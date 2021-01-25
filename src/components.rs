use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};

#[derive(Default, Bundle)]
pub struct LdtkMapBundle {
    pub map: Handle<LdtkMap>,
    pub scale: MapScale,
    pub transform: Transform,
    pub config: LdtkMapConfig,
}

/// Configuration for how to spawn and display the Ldtk map
#[derive(Default)]
pub struct LdtkMapConfig {
    /// Whether or not to set the clear color of the screen to match the background color of the
    /// LDtk map.
    pub set_clear_color: bool,
    /// Which level from the LDtk project to display, if there are more than one level.
    pub level: usize,
}

#[derive(Bundle)]
pub struct LdtkMapTiles {
    pub ldtk_ent: Entity,
    pub transform: Transform,
}

#[derive(Default, Debug)]
pub struct LdtkMapHasLoaded;

pub struct MapScale(pub f32);

impl Default for MapScale {
    fn default() -> Self {
        MapScale(1.0)
    }
}

impl From<f32> for MapScale {
    fn from(x: f32) -> Self {
        MapScale(x)
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "15676b9f-730b-4707-b1f6-a03e480d8ca0"]
pub struct LdtkTileset {
    pub texture: Handle<Texture>,
}

#[derive(TypeUuid)]
#[uuid = "abd7b6d9-633f-4322-a8f4-e5f011cae9c6"]
pub struct LdtkMap {
    pub project: ldtk::Project,
    pub tile_sets: HashMap<String, Handle<Texture>>,
}
