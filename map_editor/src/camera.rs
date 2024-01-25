use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    transform::components::Transform,
};

use crate::MainCamera;

pub fn camera_control(
    mut query: Query<(&mut Transform, &MainCamera)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    const SPEED: f32 = 3.0;

    for (mut transform, _) in query.iter_mut() {
        let mut translation = transform.translation;

        if keyboard_input.pressed(KeyCode::W) {
            translation.y += SPEED;
        }

        if keyboard_input.pressed(KeyCode::S) {
            translation.y -= SPEED;
        }

        if keyboard_input.pressed(KeyCode::A) {
            translation.x -= SPEED;
        }

        if keyboard_input.pressed(KeyCode::D) {
            translation.x += SPEED;
        }

        transform.translation = translation;
    }
}
