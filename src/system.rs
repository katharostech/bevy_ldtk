use bevy::{
    prelude::*, render::pipeline::RenderPipeline, render::render_graph::base::MainPass,
    render::texture::FilterMode, render::texture::SamplerDescriptor, sprite::QUAD_HANDLE,
    utils::HashMap,
};

use crate::*;

pub(crate) fn add_systems(app: &mut AppBuilder) {
    app.add_system(process_ldtk_maps.system());
}

#[derive(Default)]
struct State {
    // atlasLoaded: HashSet<Handle<LdtkMap>>,
}

fn process_ldtk_maps(
    commands: &mut Commands,
    mut clear_color: ResMut<ClearColor>,
    mut new_maps: Query<
        (
            Entity,
            &Handle<LdtkMap>,
            &MapScale,
            &Transform,
            &LdtkMapConfig,
        ),
        Without<LdtkMapHasLoaded>,
    >,
    map_assets: Res<Assets<LdtkMap>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // Load map atlases
    for (ent, map_handle, scale, trans, config) in new_maps.iter_mut() {
        // let parent = commands
        //     .spawn(LdtkMapTiles {
        //         ldtk_ent: ent,
        //         transform: Transform::from_scale(Vec3::splat(5.0)),
        //     })
        //     .current_entity()
        //     .unwrap();

        // Get the map asset, if available
        if let Some(map) = map_assets.get(map_handle) {
            let project = &map.project;
            let grid_size = map.project.default_grid_size;

            // Set the background color
            if config.set_clear_color {
                *clear_color = ClearColor(
                    Color::hex(
                        &map.project
                            .bg_color
                            .strip_prefix("#")
                            .expect("Invalid background color"),
                    )
                    .expect("Invalid background color"),
                );
            }

            let mut tilesets = HashMap::default();

            // Load tilesets
            for (tileset_name, texture_handle) in &map.tile_sets {
                // Update the sampler mode for the texture to avoid bleeding edges
                if let Some(texture) = textures.get_mut(texture_handle) {
                    texture.sampler = SamplerDescriptor {
                        min_filter: FilterMode::Nearest,
                        mag_filter: FilterMode::Nearest,
                        ..Default::default()
                    };
                }

                // Get the tileset info
                let tileset_info = project
                    .defs
                    .iter()
                    .map(|x| &x.tilesets)
                    .flatten()
                    .filter(|x| &x.identifier == tileset_name)
                    .next()
                    .expect("Could not find tilset inside of map data");

                tilesets.insert(tileset_info.uid, (tileset_info, texture_handle.clone()));

                // // Create a texture atlas for it
                // let atlas = TextureAtlas::from_grid_with_padding(
                //     texture_handle.clone(),
                //     Vec2::splat(tileset_info.tile_grid_size as f32),
                //     (tileset_info.px_width / tileset_info.tile_grid_size) as usize,
                //     (tileset_info.px_height / tileset_info.tile_grid_size) as usize,
                //     Vec2::splat(tileset_info.padding as f32),
                // );
                // let atlas_handle = atlas_assets.add(atlas);
                // atlases.insert(tileset_info.uid, atlas_handle);
            }

            // Spawn layers for the first level
            let level = map.project.levels.get(0).unwrap();

            for (z, layer) in level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .rev() // Reverse the layers so that they stack in the right Z order
                .enumerate()
            {
                // let layer_def = get_def!(layers, layer_instance.layer_def_uid);

                // FIXME: actually grab the right tilesheet
                let (tileset_info, tileset_texture) = if let Some(uid) = layer.__tileset_def_uid {
                    tilesets.get(&uid).expect("Missing tileset").clone()
                } else {
                    continue;
                };

                let tiles = &layer.auto_layer_tiles;

                // Create a virtual 2D map to stick the tiles in
                let mut tiles_map = HashMap::<(u32, u32), LdtkTilemapTileInfo>::default();
                let tileset_width_tiles = (tileset_info.px_wid / grid_size) as u32;

                for tile in tiles {
                    let tileset_tile_x = (tile.src[0] / grid_size) as u32;
                    let tileset_tile_y = (tile.src[1] / grid_size) as u32;
                    tiles_map.insert(
                        (
                            (tile.px[0] / grid_size) as u32,
                            (tile.px[1] / grid_size) as u32,
                        ),
                        LdtkTilemapTileInfo {
                            tile_index: tileset_tile_y * tileset_width_tiles + tileset_tile_x,
                            flip_bits: if tile.flip.x { 1 } else { 0 }
                                | if tile.flip.y { 2 } else { 0 },
                        },
                    );
                }
                // Go through all the tiles and create a flat vector from it
                let mut tiles = Vec::new();
                for y in 0..layer.__c_hei as u32 {
                    for x in (0..layer.__c_wid as u32).rev() {
                        tiles.push(tiles_map.get(&(x, y)).map(Clone::clone).unwrap_or(
                            LdtkTilemapTileInfo {
                                flip_bits: 0,
                                tile_index: u32::MAX,
                            },
                        ))
                    }
                }

                let map_info = LdtkTilemapMapInfo {
                    height: (level.px_hei / map.project.default_grid_size) as u32,
                    width: (level.px_wid / map.project.default_grid_size) as u32,
                    layer_index: z as u32,
                };

                let tileset_info = LdtkTilemapTilesetInfo {
                    height: (tileset_info.px_hei / map.project.default_grid_size) as u32,
                    width: (tileset_info.px_wid / map.project.default_grid_size) as u32,
                    grid_size: map.project.default_grid_size as u32,
                };

                // Add layer
                commands
                    .spawn(SpriteBundle {
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::new(LDTK_TILEMAP_PIPELINE_HANDLE.typed()),
                        ]),
                        ..Default::default()
                    })
                    .with(LdtkTilemapMaterial {
                        map_info,
                        scale: scale.0,
                        texture: tileset_texture.clone(),
                        tiles,
                        tileset_info,
                    })
                    .with(Parent(ent));
            }

            // Mark map as having been loaded
            commands.insert_one(ent, LdtkMapHasLoaded);
        }
    }
}
