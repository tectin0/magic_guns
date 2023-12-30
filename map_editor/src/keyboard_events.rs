use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonState,
    },
    render::mesh::Mesh,
};

use crate::{
    base::{BaseMapMeshInfo, BaseMesh},
    custom_shader::CustomMaterial,
};

pub fn handle_keyboard_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut base_map_mesh_info: ResMut<BaseMapMeshInfo>,
    q_base_entity: Query<Entity, With<BaseMesh>>,
    mut key_event_reader: EventReader<KeyboardInput>,
) {
    for event in key_event_reader.read() {
        match event.state {
            ButtonState::Pressed => {
                if let Some(key_code) = event.key_code {
                    match key_code {
                        KeyCode::Back => {
                            *base_map_mesh_info = BaseMapMeshInfo::default();

                            let base_entity = q_base_entity.single();

                            commands.entity(base_entity).insert((
                                base_map_mesh_info.make_material(&mut materials),
                                base_map_mesh_info.make_mesh(&mut meshes),
                            ));
                        }
                        _ => (),
                    }
                }
            }
            ButtonState::Released => (),
        }
    }
}
