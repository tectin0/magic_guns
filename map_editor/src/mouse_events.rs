use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{
        mouse::{MouseButton, MouseButtonInput},
        ButtonState,
    },
    math::Vec2,
    render::{
        camera::{self, Camera},
        mesh::Mesh,
    },
    transform::components::GlobalTransform,
    window::Window,
};

use crate::{
    base::{self, BaseMapMeshInfo, BaseMesh},
    custom_shader::CustomMaterial,
    math::IsPointInTriangle,
    MainCamera,
};

pub(crate) fn handle_lmb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut base_map_mesh_info: ResMut<BaseMapMeshInfo>,
    q_base_entity: Query<Entity, With<BaseMesh>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for event in mouse_button_input_events.read() {
        println!("LMB pressed");

        if let Some(position) = q_windows.single().cursor_position() {
            println!("Mouse position: {:?}", position);

            let (camera, camera_transform) = q_camera.single();

            let position = camera
                .viewport_to_world(camera_transform, position)
                .unwrap()
                .origin
                .truncate();

            let closest_triangle_index =
                base_map_mesh_info.cloest_triangle_to_point(position.into());

            let closest_triangle = base_map_mesh_info.triangle(closest_triangle_index);

            println!("Closest triangle: {:?}", closest_triangle);

            let center_point: Vec2 = base_map_mesh_info.center_point(closest_triangle_index);

            println!("Center point: {:?}", center_point);

            let is_point_in_triangle = closest_triangle.is_point_in_triangle(position.into());

            println!(
                "Point is in triangle {} : {}",
                closest_triangle_index, is_point_in_triangle
            );

            if is_point_in_triangle {
                base_map_mesh_info.subdivide_triangle(closest_triangle_index, position);

                let base_entity = q_base_entity.single();

                commands.entity(base_entity).insert((
                    base_map_mesh_info.make_material(&mut materials),
                    base_map_mesh_info.make_mesh(&mut meshes),
                ));
            }
        }
    }
}
