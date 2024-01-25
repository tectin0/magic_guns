use std::ops::Deref;

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, EntityCommands, Res, ResMut},
    },
    math::Vec2,
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::geometry::Collider;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{materials::MapMaterial, math::triangulate};

#[derive(Component)]
pub struct SelectedEntity;

#[derive(Component)]
pub struct MapObject {
    pub name: String,
    pub mesh_handle: Mesh2dHandle,
    pub material_handle: Handle<MapMaterial>,
    pub collider: Option<Collider>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Vertices(Vec<[f32; 3]>);

const OBJECT_DIRECTORY: &str = "assets/objects";
const MESHES_DIRECTORY: &str = "assets/meshes";

impl MapObject {
    pub fn new(
        collider: Option<Collider>,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<MapMaterial>,
    ) -> Self {
        Self {
            name: "unnamed".to_string(),
            mesh_handle,
            material_handle,
            collider,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();

        self
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

    pub fn object_to_file(&self, meshes: &Res<Assets<Mesh>>) {
        let mesh = self.get_mesh(meshes);

        let vertex_position_attribute = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        let vertices = Vertices(vertex_position_attribute.as_float3().unwrap().to_vec());

        let bytes = rkyv::to_bytes::<_, 256>(&vertices).unwrap();

        let path = std::path::Path::new(OBJECT_DIRECTORY);

        std::fs::create_dir_all(path).unwrap();

        let fullpath = path.join(format!("{}.bin", self.name));

        if self.name == "unnamed" {
            log::warn!("unnamed object saved to {:?}", fullpath);
        }

        std::fs::write(fullpath, bytes).unwrap();
    }

    pub fn object_from_file(
        name: &str,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
    ) -> Self {
        let bytes = std::fs::read(format!("{}/{}.bin", OBJECT_DIRECTORY, name)).unwrap();

        let vertices: Vertices = unsafe { rkyv::archived_root::<Vertices>(&bytes) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

        Self::map_object_from_vertices(vertices.0, meshes, material)
    }

    // objects saved as binary format
    pub fn objects_from_objects_directory(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
    ) -> Vec<Self> {
        let mut map_objects = Vec::new();

        for entry in std::fs::read_dir(OBJECT_DIRECTORY).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();

            if path.is_file() {
                let bytes = std::fs::read(path).unwrap();

                let vertices: Vertices = unsafe { rkyv::archived_root::<Vertices>(&bytes) }
                    .deserialize(&mut rkyv::Infallible)
                    .unwrap();

                map_objects.push(Self::map_object_from_vertices(
                    vertices.0,
                    meshes,
                    material.clone(),
                ));
            }
        }

        map_objects
    }

    // meshes exported from (e.g.) blender as list of vertices and indices
    pub fn objects_from_meshes_directory(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
    ) -> Vec<Self> {
        let mut map_objects = Vec::new();

        for entry in std::fs::read_dir(MESHES_DIRECTORY).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();

            if path.is_file() {
                let mesh = ImportableMesh::from(MeshPath(path.to_str().unwrap().to_string()));
                let collider = mesh.make_trimesh_collider();

                let mesh_handle = Mesh2dHandle(meshes.add(mesh.into()));

                map_objects.push(
                    Self::new(Some(collider), mesh_handle.clone(), material.clone())
                        .name(path.file_stem().unwrap().to_str().unwrap()),
                );
            }
        }

        map_objects
    }

    pub fn map_object_from_vertices(
        vertices: Vec<[f32; 3]>,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<MapMaterial>,
    ) -> Self {
        let _flatten_vertices = vertices.iter().flatten().copied().collect::<Vec<f32>>();

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

        MapObject::new(collider, mesh_handle.clone(), material.clone())
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
    pub map_mesh: MapObject,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<MapMaterial>,
}

#[derive(Bundle)]
pub struct MapMeshColliderBundle {
    pub map_mesh: MapObject,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<MapMaterial>,
    pub collider: Collider,
}

/// First Line: Vertices
/// Second Line: Indices
/// Example File in assets/meshes:
/// -0.8778809309005737 -1.8117322921752930.13798338174819946 -1.5531271696090698-0.547441303730011 0.403771877288818360.15953385829925537 0.8275967240333557-0.05597031116485596 1.711163878440857-1.2298710346221924 2.0847043991088867-1.0 -1.01.0 -1.0-1.0 1.01.0 1.0
/// 0 1 3 2 2 3 4 5 0 1 3 2
pub struct MeshPath(pub String);

impl From<&str> for MeshPath {
    fn from(path: &str) -> Self {
        Self(path.to_string())
    }
}

pub struct ImportableMesh(pub Mesh);

impl std::ops::Deref for ImportableMesh {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Mesh> for ImportableMesh {
    fn into(self) -> Mesh {
        self.0
    }
}

impl From<MeshPath> for ImportableMesh {
    fn from(path: MeshPath) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let file = std::fs::read_to_string(path.0).unwrap();

        let mut lines = file.lines();

        let vertices_line = lines.next().unwrap();
        let indices_line = lines.next().unwrap();

        for vertex in vertices_line.split(' ') {
            if vertex.is_empty() {
                continue;
            }

            vertices.push(vertex.parse::<f32>().unwrap());
        }

        for index in indices_line.split(' ') {
            if index.is_empty() {
                continue;
            }

            indices.push(index.parse::<u32>().unwrap());
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let vertices = vertices
            .chunks(2)
            .map(|chunk| [chunk[0], chunk[1], 0.0])
            .collect::<Vec<[f32; 3]>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh.set_indices(Some(Indices::U32(indices)));

        Self(mesh)
    }
}

impl ImportableMesh {
    pub fn make_trimesh_collider(&self) -> Collider {
        let vertices = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap()
            .to_vec();

        let indices = match self.indices().unwrap() {
            Indices::U32(indices) => indices.to_vec(),
            Indices::U16(indices) => indices.iter().map(|index| *index as u32).collect(),
        };

        Collider::trimesh(
            vertices
                .iter()
                .map(|vertex| Vec2::new(vertex[0], vertex[1]))
                .collect::<Vec<Vec2>>(),
            indices
                .chunks(3)
                .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                .collect::<Vec<[u32; 3]>>(),
        )
    }
}
