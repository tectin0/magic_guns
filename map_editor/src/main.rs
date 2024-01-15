mod keyboard_events;

mod mouse_events;
mod ui;

use bevy::{
    audio::Decodable,
    input::{common_conditions::input_just_pressed, mouse::MouseButtonInput},
    pbr::{
        wireframe::{Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
        ExtendedMaterial,
    },
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use keyboard_events::handle_keyboard_events;

use mouse_events::handle_mouse_events;
use shared::custom_shader::{CustomMaterial, CustomMaterialPlugin};
use ui::{ui_system, MapTextureNames, SelectedMapTextureName, TopPanelRect};

fn main() {
    App::new()
        .init_resource::<MapTextureNames>()
        .init_resource::<SelectedMapTextureName>()
        .init_resource::<TopPanelRect>()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..Default::default()
        }))
        .add_plugins((EguiPlugin, CustomMaterialPlugin))
        .add_systems(Update, ui_system)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                handle_mouse_events,
                handle_keyboard_events.run_if(input_just_pressed(KeyCode::Back)),
            ),
        )
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
