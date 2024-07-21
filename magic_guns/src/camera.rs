use bevy::{
    math::Vec3,
    prelude::{default, Camera3dBundle, Commands, Component},
    transform::components::Transform,
};

#[derive(Component)]
struct CameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraMarker,
    ));
}
