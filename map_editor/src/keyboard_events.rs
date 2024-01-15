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
use shared::custom_shader::CustomMaterial;

pub fn handle_keyboard_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,

    mut key_event_reader: EventReader<KeyboardInput>,
) {
    for event in key_event_reader.read() {
        match event.state {
            ButtonState::Pressed => {
                if let Some(key_code) = event.key_code {
                    match key_code {
                        KeyCode::Back => {}
                        _ => (),
                    }
                }
            }
            ButtonState::Released => (),
        }
    }
}
