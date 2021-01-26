use anyhow::Context;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};

use crate::{LdtkMap, LdtkTileset};

/// Add asset types and asset loader to the app builder
pub(crate) fn add_assets(app: &mut AppBuilder) {
    app.add_asset::<LdtkMap>()
        .add_asset::<LdtkTileset>()
        .init_asset_loader::<LdtkMapLoader>();
}

/// An LDTK map asset loader
#[derive(Default)]
struct LdtkMapLoader;

impl AssetLoader for LdtkMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        // Create a future for the load function
        Box::pin(async move {
            // Deserialize the LDTK project file
            let project: ldtk::Project = serde_json::from_slice(bytes).context(format!(
                "Could not parse LDtk map file: {:?}",
                load_context.path()
            ))?;

            // Create a map asset
            let mut map = LdtkMap {
                project,
                tile_sets: Default::default(),
            };

            // Loop through the definitions in the project
            for def in &map.project.defs {
                // Loop through the tilesets
                for tileset in &def.tilesets {
                    // Get the path to the tileset image asset
                    let file_path = load_context
                        .path()
                        .parent()
                        .unwrap()
                        .join(&tileset.rel_path);
                    let asset_path = AssetPath::new(file_path.clone(), None);

                    // Obtain a handle to the tileset image asset
                    let handle: Handle<Texture> = load_context.get_handle(asset_path.clone());

                    // Register that image as a labeled sub-asset
                    let asset_label = format!("tileset/{}", tileset.identifier);

                    // Bevy 0.4 doesn't require a type parameter for `set_labeled_asset`
                    #[cfg(not(feature = "bevy-unstable"))]
                    load_context.set_labeled_asset(
                        &asset_label,
                        LoadedAsset::new(LdtkTileset {
                            texture: handle.clone(),
                        })
                        // Make sure that the image is loaded when our map is loaded
                        .with_dependency(asset_path),
                    );

                    // Bevy latest requires an extra type parameter
                    #[cfg(feature = "bevy-unstable")]
                    load_context.set_labeled_asset::<LdtkTileset>(
                        &asset_label,
                        LoadedAsset::new(LdtkTileset {
                            texture: handle.clone(),
                        })
                        // Make sure that the image is loaded when our map is loaded
                        .with_dependency(asset_path),
                    );

                    // Add the tileset handle to the map asset
                    map.tile_sets.insert(tileset.identifier.clone(), handle);
                }
            }

            // Set the loaded map as the default asset for this file
            load_context.set_default_asset(LoadedAsset::new(map));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        // Register this loader for .ldtk files and for .ldtk.json files.
        &["ldtk", "ldtk.json"]
    }
}
