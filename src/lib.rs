use bevy::prelude::*;

mod asset;
mod components;
mod system;

pub use components::*;

use asset::add_assets;
use system::add_systems;

/// Adds support for loading LDtk tile maps
#[derive(Default)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Add assets and systems
        add_assets(app);
        add_systems(app);
    }
}
