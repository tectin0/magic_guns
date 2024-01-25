mod bullets;
mod camera;
mod cursor;
mod map;
mod player;
mod setup;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, sprite::Material2dPlugin};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use camera::stick_camera_to_player;
use cursor::{update_cursor, CursorWorldCoords};
use map::make_map;
use player::{player_movement, player_shooting};
use setup::setup;
use shared::{custom_shader::CustomMaterialPlugin, materials::MapMaterial};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..Default::default()
        }))
        .init_resource::<CursorWorldCoords>()
        .add_plugins((
            CustomMaterialPlugin,
            Material2dPlugin::<MapMaterial>::default(),
        ))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_systems(Startup, (setup, make_map))
        .add_systems(Update, (player_movement, stick_camera_to_player))
        .add_systems(
            Update,
            player_shooting.run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(Update, update_cursor)
        // .add_systems(Update, update_bullets)
        .run();
}
