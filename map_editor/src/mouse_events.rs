use bevy::{
    asset::{Assets, Handle},
    ecs::{
        component::Component,
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
        color::Color,
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::GlobalTransform,
    window::Window,
};
use shared::{
    custom_shader::CustomMaterial,
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
) {
    for event in mouse_button_input_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.button {
            MouseButton::Left => {
                println!("LMB pressed");

                let position = match q_windows.single().cursor_position().and_then(|cursor| {
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
                }) {
                    Some(position) => position,
                    None => continue,
                };

                match q_selected_map_mesh.get_single() {
                    // TODO: probably not efficient to spawn a new entity every time
                    Ok(selected_entity) => {
                        let map_mesh = selected_entity.0;

                        let mesh = map_mesh.get_mesh_mut(&meshes);
                        let material = map_mesh.get_material_mut(&materials);

                        let vertex_position_attribute =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

                        println!("{:?}", vertex_position_attribute.as_float3().unwrap());

                        let mut vertices = vertex_position_attribute.as_float3().unwrap().to_vec();

                        vertices.push([position.x, position.y, 0.0]);

                        let map_mesh =
                            MapMesh::mesh_from_vertices(vertices, &mut meshes, &mut materials);

                        let bundle = (
                            MaterialMesh2dBundle {
                                mesh: map_mesh.mesh_handle.clone(),
                                material: map_mesh.material_handle.clone(),
                                ..Default::default()
                            },
                            map_mesh,
                            SelectedEntity,
                        );

                        let entity = commands.spawn(bundle);

                        println!("Spawned entity: {:?}", entity.id());

                        commands.entity(selected_entity.1).despawn();
                    }
                    Err(_) => {
                        let vertices = vec![[position.x, position.y, 0.0]];

                        let map_mesh =
                            MapMesh::mesh_from_vertices(vertices, &mut meshes, &mut materials);

                        let bundle = (
                            MaterialMesh2dBundle {
                                mesh: map_mesh.mesh_handle.clone(),
                                material: map_mesh.material_handle.clone(),
                                ..Default::default()
                            },
                            map_mesh,
                            SelectedEntity,
                        );

                        let entity = commands.spawn(bundle);

                        println!("Spawned entity: {:?}", entity.id());
                    }
                };
            }
            MouseButton::Right => {
                println!("RMB pressed");

                if let Some(position) = q_windows.single().cursor_position() {
                    println!("Mouse position: {:?}", position);

                    let (camera, camera_transform) = q_camera.single();

                    let position = camera
                        .viewport_to_world(camera_transform, position)
                        .unwrap()
                        .origin
                        .truncate();
                }
            }
            _ => {}
        }
    }
}
