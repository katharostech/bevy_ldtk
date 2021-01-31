use asset::LdtkMap;
use bevy::{
    render::pipeline::RenderPipeline, render::texture::FilterMode,
    render::texture::SamplerDescriptor, utils::HashMap,
};

use crate::*;

/// Add the Ldtk map systems to the app builder
pub(crate) fn add_systems(app: &mut AppBuilder) {
    app.add_system(process_ldtk_maps.system())
        .add_system(process_ldtk_tilesets.system())
        .add_system(hot_reload_maps.system());
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

struct LdtkMapHasLoaded;

/// This system spawns the map layers for every unloaded entity with an LDtk map
fn process_ldtk_maps(
    commands: &mut Commands,
    mut clear_color: ResMut<ClearColor>,
    mut new_maps: Query<(Entity, &Handle<LdtkMap>, &LdtkMapConfig), Without<LdtkMapHasLoaded>>,
    map_assets: Res<Assets<LdtkMap>>,
) {
    // Loop through all of the maps
    for (ent, map_handle, config) in new_maps.iter_mut() {
        // Get the map asset, if available
        if let Some(map) = map_assets.get(map_handle) {
            let project = &map.project;
            let grid_size = map.project.default_grid_size;

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

            // If the clear color should be set from the map background, set it
            if config.set_clear_color {
                *clear_color = ClearColor(
                    Color::hex(
                        level
                            .bg_color
                            .as_ref()
                            .unwrap_or(&map.project.default_level_bg_color)
                            .strip_prefix("#")
                            .expect("Invalid background color"),
                    )
                    .expect("Invalid background color"),
                );
            }

            // Loop through the layers in the selected level
            for (z, layer) in level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .rev() // Reverse the layer order so that the bottom layer is first
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
                let tiles = if !layer.auto_layer_tiles.is_empty() {
                    &layer.auto_layer_tiles
                } else if !layer.grid_tiles.is_empty() {
                    &layer.grid_tiles
                } else {
                    // Skip the layer if there are no tiles for it
                    continue;
                };

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
                            flip_bits: if tile.f.x { 1 } else { 0 } | if tile.f.y { 2 } else { 0 },
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
                let layer = commands
                    // Use the default sprite bundle with our custom render pipeline
                    .spawn(SpriteBundle {
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::new(LDTK_TILEMAP_PIPELINE_HANDLE.typed()),
                        ]),
                        ..Default::default()
                    })
                    // Add our material which the shaders will use to render the map
                    .with(LdtkTilemapLayer {
                        map_info,
                        scale: config.scale,
                        texture: tileset_texture.clone(),
                        tiles,
                        tileset_info,
                    })
                    // Add the `Handle<LdtkMap>` so that we will be able to hot reload this layer if
                    // the map changes.
                    .with(map_handle.clone())
                    .current_entity()
                    .unwrap();

                // Add the entity as a child of the LDtk map entity
                commands.push_children(ent, &[layer]);
            }

            // Mark the map as having been loaded so that we don't process it again
            commands.insert_one(ent, LdtkMapHasLoaded);
        }
    }
}

type MapEvent = AssetEvent<LdtkMap>;

/// This system watches for changes to map assets and makes sure that the map is reloaded upon
/// changes.
fn hot_reload_maps(
    commands: &mut Commands,
    // The event reader strategy is slightly different for Bevy 0.4 and the upcomming Bevy, so
    // we have to match on the feature to make sure we can support both. This should go away once Bevy
    // 0.5 is released.
    #[cfg(not(feature = "bevy-unstable"))] mut event_reader: Local<EventReader<MapEvent>>,
    #[cfg(not(feature = "bevy-unstable"))] events: Res<Events<MapEvent>>,
    #[cfg(feature = "bevy-unstable")] mut events: EventReader<MapEvent>,
    layers: Query<(Entity, &Handle<LdtkMap>), With<LdtkTilemapLayer>>,
    maps: Query<(Entity, &Handle<LdtkMap>), With<LdtkMapConfig>>,
) {
    // Here we create a simple macro that just pastes our event handler code
    macro_rules! handle_map_event {
        ($event:ident) => {
            match $event {
                // When the map asset has been modified
                AssetEvent::Modified { handle } => {
                    // Loop through all the layers in the world, find the ones that are for this map and remove them
                    for (layer_ent, map_handle) in layers.iter() {
                        if map_handle == handle {
                            commands.despawn(layer_ent);
                        }
                    }

                    // Then remove the `LdtkMapHasLoaded` component from the map so that it will be
                    // reloaded by the `process_ldtk_maps` system.
                    for (map_ent, map_handle) in maps.iter() {
                        if map_handle == handle {
                            commands.remove_one::<LdtkMapHasLoaded>(map_ent);
                        }
                    }
                }
                _ => (),
            }
        }
    }

    // Here we have to iterate over the events slightly differently for Bevy 0.4 and 0.6
    #[cfg(not(feature = "bevy-unstable"))]
    for event in event_reader.iter(&events) {
        handle_map_event!(event);
    }

    #[cfg(feature = "bevy-unstable")]
    for event in events.iter() {
        handle_map_event!(event);
    }
}
