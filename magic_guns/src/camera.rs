use bevy::{
    ecs::{
        component::Component,
        query::{With, Without},
        system::Query,
    },
    transform::components::Transform,
};

use crate::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn stick_camera_to_player(
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let player_position = q_player.single().translation.truncate();

    let mut camera_transform = q_camera.single_mut();

    camera_transform.translation.x = player_position.x;
    camera_transform.translation.y = player_position.y;
}
