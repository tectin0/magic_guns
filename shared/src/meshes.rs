use std::ops::Deref;

use bevy::{
    asset::{Asset, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Res, ResMut, Resource},
    },
    math::Vec2,
    render::{
        color::Color,
        extract_resource::ExtractResource,
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::Mesh2dHandle,
};
use bevy_rapier2d::geometry::Collider;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{custom_shader::CustomMaterial, math::triangulate};

#[derive(Component)]
pub struct SelectedEntity;

#[derive(Component)]
pub struct MapMesh {
    pub mesh_handle: Mesh2dHandle,
    pub material_handle: Handle<CustomMaterial>,
    pub collider: Option<Collider>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Vertices(Vec<[f32; 3]>);

impl MapMesh {
    pub fn new(
        collider: Option<Collider>,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<CustomMaterial>,
    ) -> Self {
        Self {
            mesh_handle,
            material_handle,
            collider,
        }
    }

    pub fn mesh_to_file(&self, meshes: &Res<Assets<Mesh>>) {
        let mesh = self.get_mesh(meshes);

        let vertex_position_attribute = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        let vertices = Vertices(vertex_position_attribute.as_float3().unwrap().to_vec());

        let bytes = rkyv::to_bytes::<_, 256>(&vertices).unwrap();

        let path = std::path::Path::new("assets/meshes");

        std::fs::create_dir_all(path).unwrap();

        let random_name = rand::random::<u128>();

        let fullpath = path.join(format!("{}.mesh", random_name));

        std::fs::write(fullpath, bytes).unwrap();
    }

    pub fn mesh_from_file(
        name: &str,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CustomMaterial>>,
    ) -> Self {
        let bytes = std::fs::read(format!("assets/meshes/{}.mesh", name)).unwrap();

        let vertices: Vertices = unsafe { rkyv::archived_root::<Vertices>(&bytes) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

        Self::mesh_from_vertices(vertices.0, meshes, materials)
    }

    pub fn meshes_from_asset_directory(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CustomMaterial>>,
    ) -> Vec<Self> {
        let meshes_path = "assets/meshes";

        let mut map_meshes = Vec::new();

        for entry in std::fs::read_dir(meshes_path).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();

            if path.is_file() {
                let bytes = std::fs::read(path).unwrap();

                let vertices: Vertices = unsafe { rkyv::archived_root::<Vertices>(&bytes) }
                    .deserialize(&mut rkyv::Infallible)
                    .unwrap();

                map_meshes.push(Self::mesh_from_vertices(vertices.0, meshes, materials));
            }
        }

        map_meshes
    }

    pub fn mesh_from_vertices(
        vertices: Vec<[f32; 3]>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CustomMaterial>>,
    ) -> Self {
        let flatten_vertices = vertices.iter().flatten().copied().collect::<Vec<f32>>();

        let indices = triangulate(&vertices);

        let primitive_topology = match indices.is_empty() {
            true => PrimitiveTopology::PointList,
            false => PrimitiveTopology::TriangleList,
        };

        let mut mesh = Mesh::new(primitive_topology);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());

        mesh.set_indices(Some(Indices::U32(indices.clone())));

        let mesh_handle = Mesh2dHandle(meshes.add(mesh.clone()));

        let mesh_handle = Mesh2dHandle(meshes.add(mesh));
        let material_handle = materials.add(CustomMaterial {
            color: Color::RED,
            positions: flatten_vertices,
            ..Default::default()
        });

        let collider = match indices.is_empty() {
            true => None,
            false => Some(Collider::trimesh(
                vertices
                    .iter()
                    .map(|vertex| Vec2::new(vertex[0], vertex[1]))
                    .collect::<Vec<Vec2>>(),
                indices
                    .chunks(3)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect::<Vec<[u32; 3]>>(),
            )),
        };

        MapMesh::new(collider, mesh_handle.clone(), material_handle.clone())
    }

    pub fn get_mesh_mut<'a>(&self, meshes: &'a ResMut<Assets<Mesh>>) -> &'a Mesh {
        meshes.get(self.mesh_handle.0.clone()).unwrap()
    }

    pub fn get_mesh<'a>(&self, meshes: &'a Res<Assets<Mesh>>) -> &'a Mesh {
        meshes.get(self.mesh_handle.0.clone()).unwrap()
    }

    pub fn get_material_mut<'a>(
        &self,
        materials: &'a ResMut<Assets<CustomMaterial>>,
    ) -> &'a CustomMaterial {
        materials.get(self.material_handle.clone()).unwrap()
    }

    pub fn get_material<'a>(
        &self,
        materials: &'a Res<Assets<CustomMaterial>>,
    ) -> &'a CustomMaterial {
        materials.get(self.material_handle.clone()).unwrap()
    }
}
