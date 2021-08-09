# bevy_ldtk

[![Crates.io](https://img.shields.io/crates/v/bevy_ldtk.svg)](https://crates.io/crates/bevy_ldtk)
[![Docs.rs](https://docs.rs/bevy_ldtk/badge.svg)](https://docs.rs/bevy_ldtk)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

![screenshot](./doc/screenshot.png)

_( Tileset from ["Cavernas"] by Adam Saltsman  )_

["Cavernas"]: https://adamatomic.itch.io/cavernas

A [Bevy] plugin for loading [LDtk] tile maps.

[ldtk]: https://github.com/deepnight/ldtk

[bevy]: https://bevyengine.org

> **🚧 Maintenance Note:** This library has been merged into [Bevy Retrograde][bevy_retro]. With Bevy Retrograde [migrating] to Bevy's renderer, the plugin will be usable with out-of-the-box Bevy and will not require using with the rest of Bevy Retrograde if all you want is map loading. Updates will now be made in the Bevy Retrograde repository instead of here.
> 
> The next release of Bevy LDtk that is compatible with Bevy's renderer should be released after Bevy 0.6 is released.

[migrating]: https://github.com/katharostech/bevy_retrograde/issues/41
[bevy_retro]: https://github.com/katharostech/bevy_retrograde

## License

Bevy LDtk is licensed under the [Katharos License][k_license] which places certain restrictions
on what you are allowed to use it for. Please read and understand the terms before using Bevy
LDtk for your project.

[k_license]: https://github.com/katharostech/katharos-license

## Usage

```rust
use bevy::prelude::*;
use bevy_ldtk::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Enable hot reload
    asset_server.watch_for_changes().unwrap();

    commands
        // Spawn the map
        .spawn()
        .insert_bundle(LdtkMapBundle {
            map: asset_server.load("map1.ldtk"),
            config: LdtkMapConfig {
                set_clear_color: true,
                scale: 1.0,
                level: std::env::args()
                    .nth(2)
                    .map(|x| x.parse().unwrap())
                    .unwrap_or(0),
                center_map: false,
            },
            ..Default::default()
        });

    // And the camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
```

### Layers

When the map layers are spawned, the bottommost layer is spawned at the transform coordinate of
the `LdtkMapBundle`'s `Transform` component. Each layer after the bottom layer is placed one
unit higher on the Z axis. To have your sprites for players, etc. appear on top of the rendered
map, their Z axis translation must be higher than the map transform + the layer number that you
want it to appear above.

### LDtk Versions

| LDtk Version | Plugin Version |
| ------------ | ---------------|
| 0.8.1        | 0.4, 0.5       |
| 0.7.0        | 0.2, 0.3       |

### Bevy Versions

| Bevy Version | Plugin Version                                      |
| ------------ | --------------------------------------------------- |
| 0.4          | 0.2, 0.3, 0.4                                       |
| 0.5          | 0.5                                                 |
| master       | not officially supported, but it might work         |

## Features

- An efficient renderer that only uses 4 vertices per map layer and lays out tiles on the GPU
- Supports hot reload through the Bevy asset server integration
- Heavily commented code to help others who want to see how to make their own tilemap renderers.

## Caveats

The plugin is in relatively early stages, but it is still rather functional for many basic maps

- Many features are not supported yet, including:
  - tilesets with spacing in them
  - levels in separate files
- Occasionally some slight rendering artifacts between tiles. ( [#1] ) Not sure what causes
  those yet. Help from anybody with rendering experience would be greatly appreciated!

### Extracting Map Information

You can extract any information necessary for your game from the LDtk JSON map data. Here is an example showing how you could spawn a player.

```rust
fn spawn_player(
    mut commands: Commands,
    printed_maps: Local<Vec<Entity>>,
    query: Query<(Entity, &Handle<LdtkMap>)>,
    map_assets: Res<Assets<LdtkMap>>,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ent, handle) in query.iter() {
        // Skip any maps we have already printed the spawn location for
        if printed_maps.contains(&ent) {
            continue;
        }

        // If the map asset has finished loading
        if let Some(map) = map_assets.get(handle) {
            // This is the default level, but if you spawned a different level, put that ID here
            let level_idx = 0;

            // Get the level from the project
            let level = &map.project.levels[level_idx];

            // Find the entities layer
            let entities_layer = level
                .layer_instances
                .as_ref() // get a reference to the layer instances
                .unwrap() // Unwrap the option ( this could be None, if there are no layers )
                .iter() // Iterate over the layers
                .filter(|&x| x.__identifier == "Entities") // Filter on the name of the layer
                .next() // Get it
                .unwrap(); // Unwrap it ( would be None if it could not find a layer "MyEntities" )

            // Get the specific entity you want
            let player_start = entities_layer
                .entity_instances
                .iter() // Iterate over our entities in the layer
                .filter(|x| x.__identifier == "Player_Spawn") // Find the one we want
                .next() // Get it
                .unwrap(); // Unwrap it

            // Get the number of layers in the map and add one to it: this is how high we need to
            // spawn the player so that he is on top of all the maps
            let player_z = level.layer_instances.as_ref().unwrap().len() as f32 + 1.0;

            // Spawn the entity!
            commands.spawn().insert_bundle(SpriteBundle {
                // Set your sprite stuff
                transform: Transform::from_xyz(
                    // The player x position is the entity's x position from the map data
                    player_start.px[0] as f32,
                    // The player y position is the entity's y position from the map data, but
                    // multiplied by negative one because in the LDtk map +y means down and not up.
                    player_start.px[1] as f32 * -1.0,
                    // Spawn the player with the z value we determined earlier
                    player_z,
                ),
                material: color_materials.add(ColorMaterial {
                    texture: Some(asset_server.load("character.png")),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
    }
}
```

[#1]: https://github.com/katharostech/bevy_ldtk/issues/1
