use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_system)
        .run();
}

fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Map Editor").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}
