use bevy::prelude::*;

use crate::asset::LdtkMap;

/// A component bundle for spawning an LDtk map
#[derive(Default, Bundle)]
pub struct LdtkMapBundle {
    /// The handle to a map asset
    pub map: Handle<LdtkMap>,
    /// The transform of the map
    pub transform: Transform,
    /// The map configuration settings
    pub config: LdtkMapConfig,
    /// The global transformls
    pub global_transform: GlobalTransform,
}

/// Configuration for how to display the Ldtk map
pub struct LdtkMapConfig {
    /// Whether or not to set the clear color of the screen to match the background color of the
    /// LDtk map.
    pub set_clear_color: bool,
    /// Which level from the LDtk project to display, if there are more than one level.
    pub level: usize,
    /// The scale of the pixels in the tilemap. A scale of 1 means that 1 pixel in the map should
    /// equal 1 pixel on the screen.
    pub scale: f32,
}

impl Default for LdtkMapConfig {
    fn default() -> Self {
        LdtkMapConfig {
            set_clear_color: false,
            level: 0,
            scale: 1.0,
        }
    }
}
