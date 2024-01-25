mod keyboard_events;

mod camera;
mod mouse_events;
mod ui;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;

use bevy_rapier2d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::camera_control;
use keyboard_events::handle_keyboard_events;

use mouse_events::handle_mouse_events;
use shared::{
    custom_shader::CustomMaterialPlugin,
    materials::{MapMaterial, MapMaterialHandle, MapMaterialPlugin},
    meshes::MapObject,
};
use ui::{ui_system, MapTextureNames, SelectedMapTextureName, TopPanelRect};

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("map_editor", log::LevelFilter::Debug)
        .with_module_level("shared", log::LevelFilter::Debug)
        .init()
        .unwrap();

    App::new()
        .init_resource::<MapTextureNames>()
        .init_resource::<SelectedMapTextureName>()
        .init_resource::<TopPanelRect>()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins((CustomMaterialPlugin, MapMaterialPlugin))
        .add_systems(Update, ui_system)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                handle_mouse_events,
                handle_keyboard_events.run_if(input_just_pressed(KeyCode::Back)),
            ),
        )
        .add_systems(Update, camera_control)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<MapMaterialHandle>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    for map_object in
        MapObject::objects_from_meshes_directory(&mut meshes, material.clone().into()).into_iter()
    {
        log::debug!("Spawning map object: {}", map_object.name);
        map_object.spawn(&mut commands);
    }
}
