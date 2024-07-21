use bevy::{
    app::{App, Startup, Update},
    asset::AssetPlugin,
    log::{debug, Level, LogPlugin},
    prelude::IntoSystemConfigs,
    state::{app::AppExtStates, condition::in_state},
    DefaultPlugins,
};

use bevy::prelude::PluginGroup;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_registry_export::ExportRegistryPlugin;
use magic_guns::{
    camera::setup_camera,
    gltf::{load_gltf, spawn_gltf_objects},
    player::player_movement,
    states::GameState,
};

fn main() {
    debug!("Starting App..");

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(AssetPlugin {
                    file_path: "../assets".to_string(),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..Default::default()
                }),
        )
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .init_state::<GameState>()
        .add_systems(Startup, (load_gltf, setup_camera))
        .add_systems(
            Update,
            spawn_gltf_objects.run_if(in_state(GameState::Loading)),
        )
        .add_systems(Update, player_movement)
        .run();
}
