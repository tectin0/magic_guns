use bevy::ecs::system::{Res, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};

pub(crate) fn ui_system(
    mut contexts: EguiContexts,
    map_texture_names: Res<MapTextureNames>,
    mut selected_map_texture_name: ResMut<SelectedMapTextureName>,
) {
    egui::TopBottomPanel::top("TopMenu").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Tile")
                .selected_text(selected_map_texture_name.0.clone())
                .show_ui(ui, |ui| {
                    for map_texture_name in map_texture_names.0.iter() {
                        ui.selectable_value(
                            &mut selected_map_texture_name.0,
                            map_texture_name.to_string(),
                            map_texture_name,
                        );
                    }
                });
        });
    });
}

#[derive(Resource)]
pub(crate) struct SelectedMapTextureName(String);

impl Default for SelectedMapTextureName {
    fn default() -> Self {
        // TODO: probably panics if `map/grey_square.png` doesn't exist
        Self("grey_square".to_string())
    }
}

#[derive(Resource)]
pub(crate) struct MapTextureNames(Vec<String>);

impl Default for MapTextureNames {
    fn default() -> Self {
        Self(list_of_map_texture_names())
    }
}

fn list_of_map_texture_names() -> Vec<String> {
    let map_directory = std::path::Path::new("assets/map");

    let mut map_texture_names = Vec::new();

    for entry in std::fs::read_dir(map_directory).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();

        let filename = filename.split('.').collect::<Vec<&str>>()[0].to_string();

        map_texture_names.push(filename);
    }

    map_texture_names
}
