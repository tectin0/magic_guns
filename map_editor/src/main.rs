mod base;
mod custom_shader;
mod keyboard_events;
mod math;
mod mouse_events;
mod ui;

use base::{BaseMapMeshInfo, BaseMesh};
use bevy::{
    audio::Decodable,
    input::common_conditions::input_just_pressed,
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
use custom_shader::{CustomMaterial, CustomMaterialPlugin};

use keyboard_events::handle_keyboard_events;
use mouse_events::handle_lmb;
use ui::{ui_system, MapTextureNames, SelectedMapTextureName};

fn main() {
    App::new()
        .init_resource::<BaseMapMeshInfo>()
        .init_resource::<MapTextureNames>()
        .init_resource::<SelectedMapTextureName>()
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
                handle_lmb.run_if(input_just_pressed(MouseButton::Left)),
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
    base_map_mesh_info: Res<BaseMapMeshInfo>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(base_map_mesh_info.make_bundle(&mut meshes, &mut materials));
}
