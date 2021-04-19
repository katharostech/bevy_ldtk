use std::path::PathBuf;

use bevy::{prelude::*, render::camera::Camera};
use bevy_ldtk::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup.system())
        .add_system(camera_movement.system())
        .add_system(spawn_player.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Enable hot reload
    asset_server.watch_for_changes().unwrap();

    commands
        // Spawn the map
        .spawn()
        .insert_bundle(LdtkMapBundle {
            map: asset_server.load(PathBuf::from(
                &std::env::args().nth(1).unwrap_or("map1.ldtk".into()),
            )),
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

const SPEED: f32 = 1.0;

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, SPEED, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -SPEED, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Q) {
            let scale = scale + 0.005;
            transform.scale = Vec3::new(scale, scale, 1.);
        }

        if keyboard_input.pressed(KeyCode::E) {
            let scale = scale - 0.005;
            transform.scale = Vec3::new(scale, scale, 1.);
        }

        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}

/// This system demonstrates how to get information out of the map, such as entity locations, and
/// spawn a sprite at the location of the entity
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
