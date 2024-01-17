use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::{
    control::KinematicCharacterController,
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, Restitution},
};

use crate::{bullets::Bullet, cursor::CursorWorldCoords};

#[derive(Component)]
pub struct Player {}

pub fn player_shooting(
    mut commands: Commands,
    cursor_position: Res<CursorWorldCoords>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_position = player_position.single();

    let direction = cursor_position.0 - player_position.translation.truncate();

    let direction = direction.normalize();

    let bullet_size = Vec2::new(5.0, 5.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(bullet_size),
                ..Default::default()
            },
            texture: asset_server.load("pink_square.png"),
            transform: Transform::from_xyz(
                player_position.translation.x + direction.x * player_position.scale.x,
                player_position.translation.y + direction.y * player_position.scale.y,
                0.0,
            ),
            ..Default::default()
        },
        Collider::ball(bullet_size.x),
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2::new(direction.x, direction.y) * 500.0,
            ..Default::default()
        },
        Restitution::coefficient(0.7),
        Bullet {
            direction,
            speed: 500.0,
            lifetime: 1000.0,
        },
    ));
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut q_player: Query<&mut KinematicCharacterController, With<Player>>,
) {
    const SPEED: f32 = 200.0;

    let mut controller = q_player.single_mut();

    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keys.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    if keys.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    direction = direction.normalize();

    if direction.length_squared() > 0.0 {
        controller.translation = Some(direction * time.delta_seconds() * SPEED);
    }
}
