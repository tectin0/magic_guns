mod bullets;
mod camera;
mod cursor;
mod map;
mod player;
mod setup;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bullets::update_bullets;
use cursor::{cursor_world_coords, CursorWorldCoords};
use map::make_map;
use player::{player_movement, player_shooting, PlayerPosition};
use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..Default::default()
        }))
        .init_resource::<CursorWorldCoords>()
        .init_resource::<PlayerPosition>()
        .add_systems(Startup, setup)
        .add_systems(Startup, make_map)
        .add_systems(Update, player_movement)
        .add_systems(
            Update,
            player_shooting.run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(Update, cursor_world_coords)
        .add_systems(Update, update_bullets)
        .run();
}
