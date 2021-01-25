use bevy::{
    prelude::*, render::pipeline::RenderPipeline, render::texture::FilterMode,
    render::texture::SamplerDescriptor, utils::HashMap,
};

use crate::*;

/// Add the Ldtk map systems to the app builder
pub(crate) fn add_systems(app: &mut AppBuilder) {
    app.add_system(process_ldtk_maps.system())
        .add_system(process_ldtk_tilesets.system());
    // .add_system(update_textures.system());
}

#[derive(Default)]
struct State {
    // atlasLoaded: HashSet<Handle<LdtkMap>>,
}

/// Indicates that the tilesets for an [`LdtkMap`] has been loaded.
struct TilesetsLoaded;

/// A system that watches for loaded LDTK map assets and updates the texture filtering mode to
/// prevent lines between the tiles.
fn process_ldtk_tilesets(
    commands: &mut Commands,
    query: Query<(Entity, &Handle<LdtkMap>), Without<TilesetsLoaded>>,
    mut textures: ResMut<Assets<Texture>>,
    ldtk_maps: Res<Assets<LdtkMap>>,
) {
    // Loop through all of the map handles
    for (map_ent, map_handle) in query.iter() {
        // Get the map if it has loaded
        if let Some(map) = ldtk_maps.get(map_handle) {
            // Count how many textures have loaded for this map
            let mut loaded = 0;

            // Loop through all the textures
            for (_, texture_handle) in &map.tile_sets {
                // If the texture has loaded
                if let Some(texture) = textures.get_mut(texture_handle) {
                    // Make sure that the filtering mode is set to `Nearest` to prevent tiles from
                    // the tilemap from bleeding.
                    if texture.sampler.min_filter != FilterMode::Nearest {
                        texture.sampler = SamplerDescriptor {
                            min_filter: FilterMode::Nearest,
                            mag_filter: FilterMode::Nearest,
                            ..Default::default()
                        };
                    }

                    loaded += 1;
                }
            }

            // If all of the textures have loaded, add the `TilesetsLoaded` component so that
            // we don't process this map again
            if loaded == map.tile_sets.len() {
                commands.insert_one(map_ent, TilesetsLoaded);
            }
        }
    }
}

fn process_ldtk_maps(
    commands: &mut Commands,
    mut clear_color: ResMut<ClearColor>,
    mut new_maps: Query<
        (Entity, &Handle<LdtkMap>, &MapScale, &LdtkMapConfig),
        Without<LdtkMapHasLoaded>,
    >,
    map_assets: Res<Assets<LdtkMap>>,
) {
    // Loop through all of the maps
    for (ent, map_handle, scale, config) in new_maps.iter_mut() {
        // Get the map asset, if available
        if let Some(map) = map_assets.get(map_handle) {
            let project = &map.project;
            let grid_size = map.project.default_grid_size;

            // If the clear color should be set from the map background, set it
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

            // Create a hasmap mapping tileset def uid's to the tileset definition and it's texture handle
            let mut tilesets = HashMap::default();

            // Load all the tilesets
            for (tileset_name, texture_handle) in &map.tile_sets {
                // Get the tileset info
                let tileset_info = project
                    .defs
                    .iter()
                    .map(|x| &x.tilesets)
                    .flatten()
                    .filter(|x| &x.identifier == tileset_name)
                    .next()
                    .expect("Could not find tilset inside of map data");

                // Insert it into the tileset map
                tilesets.insert(tileset_info.uid, (tileset_info, texture_handle.clone()));
            }

            // Get the level that we are to display
            let level = map.project.levels.get(config.level as usize).unwrap();

            // Loop through the layers in the selected level
            for (z, layer) in level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .rev() // Reverse the layers so that they stack in the right Z order
                .enumerate()
            {
                // Get the information for the tileset associated to this layer
                let (tileset_info, tileset_texture) = if let Some(uid) = layer.__tileset_def_uid {
                    tilesets.get(&uid).expect("Missing tileset").clone()

                // Skip this layer if there is no tileset texture for it
                } else {
                    continue;
                };

                // Create a list of all the tiles in the layer
                let tiles = &layer.auto_layer_tiles;

                // Create a virtual 2D map to stick the tiles in as we read the tiles and their
                // target coordinates from the LDtk project.
                let mut tiles_map = HashMap::<(u32, u32), LdtkTilemapTileInfo>::default();

                // The width of the tileset in tiles
                let tileset_width_tiles = (tileset_info.px_wid / grid_size) as u32;

                // For every tile in the layer
                for tile in tiles {
                    // Get the x and y position of the tile in the map
                    let tileset_tile_x = (tile.src[0] / grid_size) as u32;
                    let tileset_tile_y = (tile.src[1] / grid_size) as u32;
                    // Add the tile and it's info to the (x, y) position in our tiles HashMap, and
                    // overwrite whatever tile was already there, if any.
                    // TODO: Handle automapping tiles that put multiple tiles in the same square
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

                // Go through our tiles HashMap and convert it to a 1D vector of all of the tiles'
                // information.
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

                // Initialize our map info
                let map_info = LdtkTilemapMapInfo {
                    height: (level.px_hei / map.project.default_grid_size) as u32,
                    width: (level.px_wid / map.project.default_grid_size) as u32,
                    layer_index: z as u32,
                };

                // Initialize our tileset info
                let tileset_info = LdtkTilemapTilesetInfo {
                    height: (tileset_info.px_hei / map.project.default_grid_size) as u32,
                    width: (tileset_info.px_wid / map.project.default_grid_size) as u32,
                    grid_size: map.project.default_grid_size as u32,
                };

                // Spawn the layer into the world
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

            // Mark the map as having been loaded
            commands.insert_one(ent, LdtkMapHasLoaded);
        }
    }
}
