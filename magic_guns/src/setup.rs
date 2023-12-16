use bevy::{
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::{camera::MainCamera, player::Player};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera {}));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            texture: asset_server.load("pink_square.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        },
        Player {},
    ));
}
