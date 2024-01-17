use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use shared::{
    custom_shader::CustomMaterial,
    meshes::{MapMesh, SelectedEntity},
};

#[derive(Component)]
pub struct MapTile {}

pub fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let map_meshes = MapMesh::meshes_from_asset_directory(&mut meshes, &mut custom_materials);

    for map_mesh in map_meshes {
        commands.spawn(map_mesh.into_bundle());
    }
}
