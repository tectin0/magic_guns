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
    materials::MapMaterialHandle,
    math::screen_to_rapier_coords,
    meshes::{MapObject, SelectedEntity},
};

use crate::{ui::TopPanelRect, MainCamera};

pub fn handle_mouse_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<MapMaterialHandle>,
    q_selected_map_mesh: Query<(&MapObject, Entity), With<SelectedEntity>>,
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

                        let vertex_position_attribute =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

                        let mut vertices = vertex_position_attribute.as_float3().unwrap().to_vec();

                        vertices.push([position.x, position.y, 0.0]);

                        log::debug!("vertices: {:?}", vertices);

                        let map_mesh = MapObject::map_object_from_vertices(
                            vertices,
                            &mut meshes,
                            material.clone().into(),
                        );

                        let mut entity = map_mesh.spawn(&mut commands);
                        entity.insert(SelectedEntity);

                        log::debug!("Spawned entity: {:?}", entity.id());

                        commands.entity(selected_entity.1).despawn();
                    }
                    Err(_) => {
                        let vertices = vec![[position.x, position.y, 0.0]];

                        log::debug!("vertices: {:?}", vertices);

                        let map_mesh = MapObject::map_object_from_vertices(
                            vertices,
                            &mut meshes,
                            material.clone().into(),
                        );

                        let mut entity = map_mesh.spawn(&mut commands);
                        entity.insert(SelectedEntity);

                        log::debug!("Spawned entity: {:?}", entity.id());
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

                    let mut selected_entity = None;

                    rapier_context.intersections_with_point(point, filter, |entity| {
                        selected_entity = Some(entity);

                        // Return `false` if we want to stop searching for other colliders containing this point.
                        false
                    });

                    match selected_entity {
                        Some(selected_entity) => {
                            println!("Selected entity: {:?}", selected_entity);

                            let currently_selected_entity = match q_selected_map_mesh.get_single() {
                                Ok(selected_entity) => Some(selected_entity.1),
                                Err(_) => None,
                            };

                            if currently_selected_entity != Some(selected_entity) {
                                if let Some(currently_selected_entity) = currently_selected_entity {
                                    commands
                                        .entity(currently_selected_entity)
                                        .remove::<SelectedEntity>();
                                }

                                commands.entity(selected_entity).insert(SelectedEntity);
                            }
                        }
                        None => {
                            if let Ok(selected_entity) = q_selected_map_mesh.get_single() {
                                commands
                                    .entity(selected_entity.1)
                                    .remove::<SelectedEntity>();
                            }
                        }
                    }
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
