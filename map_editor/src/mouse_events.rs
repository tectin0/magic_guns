use bevy::{
    asset::Assets,
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
    render::{camera::Camera, mesh::Mesh},
    transform::components::GlobalTransform,
    window::Window,
};
use bevy_rapier2d::{pipeline::QueryFilter, plugin::RapierContext};
use shared::{
    custom_shader::CustomMaterial,
    math::screen_to_rapier_coords,
    meshes::{MapMesh, SelectedEntity},
};

use crate::{ui::TopPanelRect, MainCamera};

pub fn handle_mouse_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    q_selected_map_mesh: Query<(&MapMesh, Entity), With<SelectedEntity>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    top_panel_rect: Res<TopPanelRect>,
    rapier_context: Res<RapierContext>,
) {
    for event in mouse_button_input_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.button {
            MouseButton::Left => {
                println!("LMB pressed");

                let position = match window_query_to_cursor_world_pos_without_top_panel(
                    &q_windows,
                    &top_panel_rect,
                    &q_camera,
                ) {
                    Some(position) => position,
                    None => continue,
                };

                match q_selected_map_mesh.get_single() {
                    // TODO: probably not efficient to spawn a new entity every time
                    Ok(selected_entity) => {
                        let map_mesh = selected_entity.0;

                        let mesh = map_mesh.get_mesh_mut(&meshes);
                        let _material = map_mesh.get_material_mut(&materials);

                        let vertex_position_attribute =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

                        println!("{:?}", vertex_position_attribute.as_float3().unwrap());

                        let mut vertices = vertex_position_attribute.as_float3().unwrap().to_vec();

                        vertices.push([position.x, position.y, 0.0]);

                        let map_mesh =
                            MapMesh::mesh_from_vertices(vertices, &mut meshes, &mut materials);

                        let mut entity = commands.spawn(map_mesh.into_bundle());
                        entity.insert(SelectedEntity);

                        println!("Spawned entity: {:?}", entity.id());

                        commands.entity(selected_entity.1).despawn();
                    }
                    Err(_) => {
                        let vertices = vec![[position.x, position.y, 0.0]];

                        let map_mesh =
                            MapMesh::mesh_from_vertices(vertices, &mut meshes, &mut materials);

                        let mut entity = commands.spawn(map_mesh.into_bundle());
                        entity.insert(SelectedEntity);

                        println!("Spawned entity: {:?}", entity.id());
                    }
                };
            }
            MouseButton::Right => {
                println!("RMB pressed");

                if let Some(position) = q_windows.single().cursor_position() {
                    println!("Mouse position: {:?}", position);

                    let screen = Vec2::new(q_windows.single().width(), q_windows.single().height());

                    let point = q_windows.single().cursor_position().unwrap();
                    let point = screen_to_rapier_coords(point, screen);

                    let filter = QueryFilter::default();

                    rapier_context.intersections_with_point(point, filter, |entity| {
                        // Callback called on each collider with a shape containing the point.
                        println!("The entity {:?} contains the point.", entity);
                        // Return `false` instead if we want to stop searching for other colliders containing this point.
                        true
                    });
                }
            }
            _ => {}
        }
    }
}

fn window_query_to_cursor_world_pos_without_top_panel(
    q_windows: &Query<'_, '_, &Window>,
    top_panel_rect: &Res<'_, TopPanelRect>,
    q_camera: &Query<'_, '_, (&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    q_windows.single().cursor_position().and_then(|cursor| {
        if cursor.y < top_panel_rect.0.max.y {
            return None;
        }

        let (camera, camera_transform) = q_camera.single();

        Some(
            camera
                .viewport_to_world(camera_transform, cursor)
                .unwrap()
                .origin
                .truncate(),
        )
    })
}
