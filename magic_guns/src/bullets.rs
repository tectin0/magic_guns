use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        system::{Commands, Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};

use crate::{camera::MainCamera, player::Player};

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

pub fn update_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Bullet, &mut Transform, Entity), (Without<Player>, Without<MainCamera>)>,
) {
    for (mut bullet, mut transform, entity) in query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += bullet.direction.x * bullet.speed * time.delta_seconds();
            transform.translation.y += bullet.direction.y * bullet.speed * time.delta_seconds();
        }
    }
}
