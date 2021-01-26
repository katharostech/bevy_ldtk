# Bevy LDtk

A plugin to bevy for loading [LDtk] 2D tile maps. Still work-in-progress, but hopes to be usable soon.

[ldtk]: https://github.com/deepnight/ldtk

![screenshot](./doc/screenshot.png)

## Features

- An efficient renderer that only uses 4 vertices per map layer and lays out tiles on the GPU
- Heavily commented to help others who want to see how to make their own tilemap renderers.

## Caveats

The plugin is in very early development and there are some large caveats that will be fixed in later releases:

- Many features are not properly handled yet, such as advanced auto-tiling and transparency, etc.

## License

Bevy LDtk is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use it for. Please read and understand the terms before using Bevy LDtk for your project.

[k_license]: https://github.com/katharostech/katharos-licens
