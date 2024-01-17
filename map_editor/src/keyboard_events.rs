use bevy::{
    asset::Assets,
    ecs::{
        event::EventReader,
        system::{Commands, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonState,
    },
    render::mesh::Mesh,
};
use shared::custom_shader::CustomMaterial;

pub fn handle_keyboard_events(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<CustomMaterial>>,

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
