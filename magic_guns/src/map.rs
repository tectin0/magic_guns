use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    math::Vec2,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle},
    transform::components::Transform,
};
use bevy_rapier2d::geometry::Collider;
use rand::Rng;
use shared::{
    custom_shader::CustomMaterial,
    meshes::{MapMesh, SelectedEntity},
};

#[derive(Component)]
pub struct MapTile {}

pub fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // TODO: unncessary
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let map_meshes = MapMesh::meshes_from_asset_directory(&mut meshes, &mut custom_materials);

    for map_mesh in map_meshes {
        let collider = map_mesh.collider.clone();

        let bundle = (
            MaterialMesh2dBundle {
                mesh: map_mesh.mesh_handle.clone(),
                material: materials.add(ColorMaterial {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            },
            map_mesh,
            SelectedEntity,
        );

        let mut entity = commands.spawn(bundle);

        if let Some(collider) = collider {
            entity.insert(collider);
        }
    }
}
