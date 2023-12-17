use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::Rng;

#[derive(Component)]
pub struct MapTile {}

pub fn make_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let number_map_tiles = 1000;

    let mut rng = rand::thread_rng();

    for _ in 0..number_map_tiles {
        let x = rng.gen_range(-500.0..500.0);
        let y = rng.gen_range(-500.0..500.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                texture: asset_server.load("map/grey_square.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            MapTile {},
        ));
    }
}
