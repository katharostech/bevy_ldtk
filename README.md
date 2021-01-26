# Bevy LDtk

A plugin to bevy for loading [LDtk] 2D tile maps. Still work-in-progress, but hopes to be usable soon.

[ldtk]: https://github.com/deepnight/ldtk

![screenshot](./doc/screenshot.png)

## Usage

```rust
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
        .spawn(Camera2dBundle::default());
        // Spawn a map bundle
        .spawn(LdtkMapBundle {
            // Specify the path to the map asset to load
            map: asset_server.load("myMap.ldtk"),
            // You can specify a scale or leave it set to 1 for 1 to 1 pixel size
            scale: MapScale(3.),
            config: LdtkMapConfig {
                // Automatically set the clear color to the LDtk background color
                set_clear_color: true,
                // Set which level to load out of the map or leave it to 0 for the default level
                level: 0,
            },
            ..Default::default()
        })
}
```

## Features

- An efficient renderer that only uses 4 vertices per map layer and lays out tiles on the GPU
- Heavily commented to help others who want to see how to make their own tilemap renderers.

## Caveats

The plugin is in relatively early stages, but it is still rather functional for many basic maps

- Many features are not supported yet:
  - multiple overlapping autotile tiles
  - tilemaps with spacing in them
- Occasionally some slight rendering artifacts between tiles. Not sure what causes those yet.
- Hot reload is not implemented yet

If you run into anything that isn't supported that you want to use in your game open an issue or PR to help prioritize what gets implemented.

## License

Bevy LDtk is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use it for. Please read and understand the terms before using Bevy LDtk for your project.

[k_license]: https://github.com/katharostech/katharos-licens
