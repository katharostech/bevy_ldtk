use std::path::PathBuf;

use bevy::{prelude::*, render::camera::Camera};
use bevy_ldtk::{LdtkMapBundle, LdtkMapConfig, LdtkPlugin, MapScale};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(LdtkMapBundle {
            map: asset_server.load(PathBuf::from(
                &std::env::args().nth(1).unwrap_or("Map.ldtk".into()),
            )),
            scale: MapScale(3.),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)),
            config: LdtkMapConfig {
                set_clear_color: true,
            },
        })
        .spawn(Camera2dBundle::default());
}

struct Test;

const SPEED: f32 = 1.0;

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::zero();
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
