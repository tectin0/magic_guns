use bevy::{
    ecs::{
        query::With,
        system::{Query, ResMut, Resource},
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

use crate::camera::MainCamera;

#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

pub fn update_cursor(
    mut cursor_world_coords: ResMut<CursorWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_world_coords.0 = world_position;
    }
}
