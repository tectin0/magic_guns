use bevy::{
    app::{App, Startup, Update},
    asset::AssetPlugin,
    log::{debug, Level, LogPlugin},
    prelude::IntoSystemConfigs,
    state::{app::AppExtStates, condition::in_state},
    DefaultPlugins,
};

use bevy::prelude::PluginGroup;

use magic_guns::{
    gltf::{load_gltf, spawn_gltf_objects},
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
        .init_state::<GameState>()
        .add_systems(Startup, load_gltf)
        .add_systems(
            Update,
            spawn_gltf_objects.run_if(in_state(GameState::Loading)),
        )
        .run();
}
