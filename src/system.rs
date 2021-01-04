use bevy::{prelude::*, utils::HashMap};
use ldtk::LayerType;

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
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    // Load map atlases
    for (ent, map_handle, scale, trans, config) in new_maps.iter_mut() {
        let parent = commands
            .spawn(LdtkMapTiles {
                ldtk_ent: ent,
                transform: Transform::from_scale(Vec3::splat(5.0)),
            })
            .current_entity()
            .unwrap();

        // Get the map asset, if available
        if let Some(map) = map_assets.get(map_handle) {
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

            let mut atlases = HashMap::default();

            // Load tilesets
            for (tileset_name, texture_handle) in &map.tile_sets {
                // Get the tileset info
                let tileset_info = map
                    .project
                    .defs
                    .iter()
                    .map(|x| &x.tilesets)
                    .flatten()
                    .filter(|x| &x.identifier == tileset_name)
                    .next()
                    .expect("Could not find tilset inside of map data");

                // Create a texture atlas for it
                let atlas = TextureAtlas::from_grid_with_padding(
                    texture_handle.clone(),
                    Vec2::splat(tileset_info.tile_grid_size as f32),
                    (tileset_info.px_width / tileset_info.tile_grid_size) as usize,
                    (tileset_info.px_height / tileset_info.tile_grid_size) as usize,
                    Vec2::splat(tileset_info.padding as f32),
                );
                let atlas_handle = atlas_assets.add(atlas);
                atlases.insert(tileset_info.uid, atlas_handle);
            }

            // Spawn layers for the first level
            let level = map.project.levels.get(0).expect("Expected a level");

            for (z, layer) in level.layer_instances.iter().enumerate() {
                let tiles = if let LayerType::IntGrid = layer.layer_instance_type {
                    &layer.auto_layer_tiles
                } else {
                    &layer.grid_tiles
                };

                commands
                    .spawn_batch(
                        tiles
                            .iter()
                            .map(|tile| SpriteSheetBundle {
                                texture_atlas: atlases
                                    .get(&layer.tileset_def_uid.unwrap())
                                    .unwrap()
                                    .clone(),
                                sprite: TextureAtlasSprite {
                                    index: tile.index as u32,
                                    ..Default::default()
                                },
                                transform: {
                                    let mut t = Transform::from_translation(Vec3::new(
                                        tile.layer_coord.0 as f32 * scale.0,
                                        -tile.layer_coord.1 as f32 * scale.0,
                                        0. - z as f32 * 0.01,
                                    ));

                                    t.apply_non_uniform_scale(Vec3::new(
                                        if tile.flip.x { -1.0 } else { 1.0 },
                                        if tile.flip.y { -1.0 } else { 1.0 },
                                        1.0,
                                    ));

                                    t.apply_non_uniform_scale(Vec3::splat(scale.0));

                                    t.mul_transform(*trans)
                                },
                                ..Default::default()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .with(Parent(parent));
            }

            // Mark map as having been loaded
            commands.insert_one(ent, LdtkMapHasLoaded);
        }
    }
}
