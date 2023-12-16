use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, ParamSet, Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::{bullets::Bullet, camera::MainCamera, cursor::CursorWorldCoords};

#[derive(Component)]
pub struct Player {}

#[derive(Resource, Default)]
pub struct PlayerPosition(Vec2);

pub fn player_shooting(
    mut commands: Commands,
    cursor_position: Res<CursorWorldCoords>,
    player_position: Res<PlayerPosition>,
    asset_server: Res<AssetServer>,
) {
    let direction = cursor_position.0 - player_position.0;

    let direction = direction.normalize();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..Default::default()
            },
            texture: asset_server.load("blue_sphere.png"),
            transform: Transform::from_xyz(player_position.0.x, player_position.0.y, 0.0),
            ..Default::default()
        },
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
    mut set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
    mut player_position: ResMut<PlayerPosition>,
) {
    const SPEED: f32 = 200.0;

    let mut player_query = set.p0();

    let mut transform = player_query.single_mut();

    let mut direction = Vec3::ZERO;

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
        transform.translation += direction * time.delta_seconds() * SPEED;
    }

    player_position.0 = transform.translation.truncate();

    drop(player_query);

    let mut camera_query = set.p1();

    let mut camera_transform = camera_query.single_mut();

    if direction.length_squared() > 0.0 {
        camera_transform.translation += direction * time.delta_seconds() * SPEED;
    }
}
