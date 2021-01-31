# Bevy LDtk

[![Crates.io](https://img.shields.io/crates/v/bevy_ldtk.svg)](https://crates.io/crates/bevy_ldtk)
[![Docs.rs](https://docs.rs/bevy_ldtk/badge.svg)](https://docs.rs/bevy_ldtk)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

A [Bevy] plugin for loading [LDtk] 2D tile maps.

[ldtk]: https://github.com/deepnight/ldtk
[bevy]: https://bevyengine.org

![screenshot](./doc/screenshot.png)

_( Screenshot assets from the **CanariPack 8BIT TopDown** art pack by **Johan Vinet** )_

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

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // Spawn a camera
        .spawn(Camera2dBundle::default())
        // Spawn a map bundle
        .spawn(LdtkMapBundle {
            // Specify the path to the map asset to load
            map: asset_server.load("map1.ldtk"),
            config: LdtkMapConfig {
                // Automatically set the clear color to the LDtk background color
                set_clear_color: true,
                // You can specify a scale or leave it set to 1 for 1 to 1 pixel size
                scale: 3.0,
                // Set which level to load out of the map or leave it to 0 for the default level
                level: 0,
            },
            ..Default::default()
        });
}
```

### Layers

When the map layers are spawned, the bottommost layer is spawned at the transform coordinate of the `LdtkMapBundle`'s `Transform` component. Each layer after the bottom layer is placed one unit higher on the Z axis. To have your sprites for players, etc. appear on top of the rendered map, their Z axis translation must be higher than the map transform + the layer number that you want it to appear above.

### Running the Example

```bash
cargo run --example display_map
```

You can also copy your own LDtk maps into the assets folder and then run them by specifying the map file name and the level. For instance, this will load the second level from `map2.ldtk`:

```bash
cargo run --example display_map -- map2.ldtk 1
```

### Bevy Versions

| Bevy Version | Plugin Version                                 |
| ------------ | ---------------------------------------------- |
| 0.4          | 0.2                                            |
| master       | with the `bevy-unstable` feature ( see below ) |

#### Using Bevy From Master

You can use this crate with Bevy master by adding a patch to your `Cargo.toml` and by adding the `bevy-unstable` feature to this crate:

```toml
[dependencies]
# Bevy version must be set to "0.4" and we will
# override it in the patch below.
bevy = "0.4"
bevy_ldtk = { version = "0.2", features = ["bevy-unstable"] }

[patch.crates-io]
bevy = { git = "https://github.com/bevyengine/bevy.git" }
```

Note that as Bevy master may or may not introduce breaking API changes, this crate may or may not compile when using the `bevy-unstable` feature.

## Features

- An efficient renderer that only uses 4 vertices per map layer and lays out tiles on the GPU
- Supports hot reload through the Bevy asset server integration
- Heavily commented code to help others who want to see how to make their own tilemap renderers.

## Caveats

This plugin is in relatively early stages of development, and while it can load many basic maps, there are some caveats:

- Many features are not supported yet:
  - multiple overlapping autotile tiles
  - tilemaps with spacing in them
  - levels in separate files
- Occasionally some slight rendering artifacts between tiles. Not sure what causes those yet.

If you run into anything that isn't supported that you want to use in your game open an issue or PR to help prioritize what gets implemented.

## License

Bevy LDtk is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use it for. Please read and understand the terms before using Bevy LDtk for your project.

[k_license]: https://github.com/katharostech/katharos-licens
