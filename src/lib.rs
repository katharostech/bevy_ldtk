use bevy::prelude::*;

mod asset;
mod components;
mod pipeline;
mod system;

pub use components::*;
pub use pipeline::*;

use asset::add_assets;
use pipeline::configure_pipeline;
use system::add_systems;

/// Adds support for loading LDtk tile maps
#[derive(Default)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Add asssets, systems, and graphics pipeline
        add_assets(app);
        add_systems(app);
        configure_pipeline(app);
    }
}
