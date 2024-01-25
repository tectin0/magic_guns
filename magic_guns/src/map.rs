use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use shared::{
    custom_shader::CustomMaterial,
    materials::{MapMaterial, MapMaterialHandle, STEEL_BLUE},
    meshes::{MapMesh, SelectedEntity},
};

#[derive(Component)]
pub struct MapTile {}

pub fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut map_materials: ResMut<Assets<MapMaterial>>,
) {
    let map_material_handle = map_materials.add(MapMaterial { color: STEEL_BLUE });

    let map_meshes = MapMesh::meshes_from_asset_directory(&mut meshes, map_material_handle.clone());

    for map_mesh in map_meshes {
        map_mesh.spawn(&mut commands);
    }

    commands.insert_resource(MapMaterialHandle(map_material_handle));
}
