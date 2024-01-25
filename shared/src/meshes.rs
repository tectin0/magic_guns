use bevy::{
    asset::{Assets, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, EntityCommands, Res, ResMut},
    },
    math::Vec2,
    render::{
        color::Color,
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::geometry::Collider;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    custom_shader::CustomMaterial,
    materials::{MapMaterial, MapMaterialHandle},
    math::triangulate,
};

#[derive(Component)]
pub struct SelectedEntity;

#[derive(Component)]
pub struct MapMesh {
    pub mesh_handle: Mesh2dHandle,
    pub material_handle: Handle<MapMaterial>,
    pub collider: Option<Collider>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Vertices(Vec<[f32; 3]>);

impl MapMesh {
    pub fn new(
        collider: Option<Collider>,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<MapMaterial>,
    ) -> Self {
        Self {
            mesh_handle,
            material_handle,
            collider,
        }
    }

    pub fn get_vertices(&self, meshes: &Res<Assets<Mesh>>) -> Vec<[f32; 3]> {
        let mesh = self.get_mesh(meshes);

        let vertex_position_attribute = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        vertex_position_attribute.as_float3().unwrap().to_vec()
    }

    pub fn get_indices(&self, meshes: &Res<Assets<Mesh>>) -> Vec<u32> {
        let mesh = self.get_mesh(meshes);

        match mesh.indices().unwrap() {
            Indices::U32(indices) => indices.to_vec(),
            Indices::U16(indices) => indices.iter().map(|index| *index as u32).collect(),
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
        material: Handle<MapMaterial>,
    ) -> Self {
        let bytes = std::fs::read(format!("assets/meshes/{}.mesh", name)).unwrap();

        let vertices: Vertices = unsafe { rkyv::archived_root::<Vertices>(&bytes) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

        Self::mesh_from_vertices(vertices.0, meshes, material)
    }

    pub fn meshes_from_asset_directory(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
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

                map_meshes.push(Self::mesh_from_vertices(
                    vertices.0,
                    meshes,
                    material.clone(),
                ));
            }
        }

        map_meshes
    }

    pub fn mesh_from_vertices(
        vertices: Vec<[f32; 3]>,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
    ) -> Self {
        let flatten_vertices = vertices.iter().flatten().copied().collect::<Vec<f32>>();

        let indices = triangulate(&vertices);

        let primitive_topology = match vertices.len() {
            1 => PrimitiveTopology::PointList,
            2 => PrimitiveTopology::LineList,
            _ => PrimitiveTopology::TriangleList,
        };

        log::debug!("primitive_topology: {:?}", primitive_topology);

        let mut mesh = Mesh::new(primitive_topology);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());

        if !indices.is_empty() {
            mesh.set_indices(Some(Indices::U32(indices.clone())));
        }

        let mesh_handle = Mesh2dHandle(meshes.add(mesh));

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

        MapMesh::new(collider, mesh_handle.clone(), material.clone())
    }

    pub fn get_mesh_mut<'a>(&self, meshes: &'a ResMut<Assets<Mesh>>) -> &'a Mesh {
        meshes.get(self.mesh_handle.0.clone()).unwrap()
    }

    pub fn get_mesh<'a>(&self, meshes: &'a Res<Assets<Mesh>>) -> &'a Mesh {
        meshes.get(self.mesh_handle.0.clone()).unwrap()
    }

    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        match self.collider {
            Some(_) => commands.spawn((
                MaterialMesh2dBundle {
                    mesh: self.mesh_handle.clone(),
                    material: self.material_handle.clone(),
                    ..Default::default()
                },
                self.collider.clone().unwrap(),
                self,
            )),
            None => commands.spawn((
                MaterialMesh2dBundle {
                    mesh: self.mesh_handle.clone(),
                    material: self.material_handle.clone(),
                    ..Default::default()
                },
                self,
            )),
        }
    }
}

#[derive(Bundle)]
pub struct MapMeshBundle {
    pub map_mesh: MapMesh,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<MapMaterial>,
}

#[derive(Bundle)]
pub struct MapMeshColliderBundle {
    pub map_mesh: MapMesh,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<MapMaterial>,
    pub collider: Collider,
}
