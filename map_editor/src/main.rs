mod base;
mod custom_shader;

use base::BaseMapMeshInfo;
use bevy::{
    pbr::wireframe::{Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    sprite::Mesh2dHandle,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use custom_shader::{ColoredMesh2d, ColoredMesh2dPlugin};

fn main() {
    App::new()
        .init_resource::<BaseMapMeshInfo>()
        .init_resource::<MapTextureNames>()
        .init_resource::<SelectedMapTextureName>()
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE,
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../assets".to_string(),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // WARN this is a native only feature. It will not work with webgl or webgpu
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                }),
        )
        .add_plugins((EguiPlugin, WireframePlugin))
        .add_systems(Update, ui_system)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    base_map_mesh_info: Res<BaseMapMeshInfo>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut base_map_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    base_map_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        base_map_mesh_info.vertices.clone(),
    );

    base_map_mesh.set_indices(Some(Indices::U32(base_map_mesh_info.indices.clone())));

    // base_map_mesh.insert_attribute(
    //     MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
    //     base_map_mesh_info.colors.clone(),
    // );

    let material = ColorMaterial {
        color: Color::rgb(1.0, 1.0, 1.0),
        texture: Some(asset_server.load("map/grey_square.png")),
        ..Default::default()
    };

    commands.spawn((ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(base_map_mesh)),
        material: materials.add(material),
        ..default()
    },));
}

fn ui_system(
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
struct SelectedMapTextureName(String);

impl Default for SelectedMapTextureName {
    fn default() -> Self {
        // TODO: probably panics if `map/grey_square.png` doesn't exist
        Self("grey_square".to_string())
    }
}

#[derive(Resource)]
struct MapTextureNames(Vec<String>);

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
