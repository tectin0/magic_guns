//! Base Mesh is 100 x 100

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{ResMut, Resource},
    },
    math::Vec2,
    render::{
        color::Color,
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{custom_shader::CustomMaterial, math::calc_center_points};

#[derive(Component)]
pub struct BaseMesh;

#[derive(Resource)]
pub struct BaseMapMeshInfo {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub center_points: Vec<[f32; 3]>,
    pub colors: Vec<u32>,
}

impl Default for BaseMapMeshInfo {
    fn default() -> Self {
        const SIZE: f32 = 200.0;

        let vertices = vec![
            [-SIZE, -SIZE, 0.0],
            [SIZE, -SIZE, 0.0],
            [SIZE, SIZE, 0.0],
            [-SIZE, SIZE, 0.0],
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let center_points = calc_center_points(&vertices, &indices);

        let mut colors: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
        colors.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 3]);

        Self {
            vertices,
            indices,
            center_points,
            colors,
        }
    }
}

impl BaseMapMeshInfo {
    pub fn make_material(
        &self,
        materials: &mut ResMut<Assets<CustomMaterial>>,
    ) -> Handle<CustomMaterial> {
        let flatten_positions = self
            .vertices
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<f32>>();

        let custom_material = CustomMaterial {
            color: Color::RED,
            positions: flatten_positions,
            ..Default::default()
        };

        materials.add(custom_material)
    }

    pub fn make_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
        let mut base_map_mesh = Mesh::new(PrimitiveTopology::TriangleList);

        base_map_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());

        base_map_mesh.set_indices(Some(Indices::U32(self.indices.clone())));

        println!(
            "Created Mesh with {} vertices and {} triangles",
            self.vertices.len(),
            self.indices.len() / 3
        );

        Mesh2dHandle(meshes.add(base_map_mesh))
    }

    pub fn make_bundle(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CustomMaterial>>,
    ) -> (MaterialMesh2dBundle<CustomMaterial>, BaseMesh) {
        (
            MaterialMesh2dBundle {
                mesh: self.make_mesh(meshes),
                material: self.make_material(materials),
                ..Default::default()
            },
            BaseMesh,
        )
    }

    pub fn cloest_triangle_to_point(&self, point: [f32; 2]) -> u32 {
        let mut closest_triangle = 0;
        let mut closest_distance = f32::MAX;

        for (index, center_point) in self.center_points.iter().enumerate() {
            let distance = ((center_point[0] - point[0]).powi(2)
                + (center_point[1] - point[1]).powi(2))
            .sqrt();

            if distance < closest_distance {
                closest_distance = distance;
                closest_triangle = index;
            }
        }

        closest_triangle as u32
    }

    pub fn triangle(&self, index: u32) -> [Vec2; 3] {
        let mut triangle = [Vec2::new(0.0, 0.0); 3];

        for (i, vertex_index) in self.indices[index as usize * 3..index as usize * 3 + 3]
            .iter()
            .enumerate()
        {
            triangle[i] = [
                self.vertices[*vertex_index as usize][0],
                self.vertices[*vertex_index as usize][1],
            ]
            .into();
        }

        triangle
    }

    pub fn center_point(&self, index: u32) -> Vec2 {
        let center_point = self.center_points[index as usize];

        Vec2::new(center_point[0], center_point[1])
    }

    pub fn remove_triangle(&mut self, index: u32) {
        self.indices.remove(index as usize * 3);
        self.indices.remove(index as usize * 3);
        self.indices.remove(index as usize * 3);

        self.center_points.remove(index as usize);
    }

    pub fn subdivide_triangle(&mut self, old_triangle_index: u32, new_point: Vec2) {
        let old_triangle = self.triangle(old_triangle_index);

        let old_vertices_indices = self.indices
            [old_triangle_index as usize * 3..old_triangle_index as usize * 3 + 3]
            .to_vec();

        for i in 0..3 {
            let mut new_triangle = old_triangle;

            new_triangle[i] = new_point;

            let mut new_triangle_indices = old_vertices_indices.clone();

            new_triangle_indices[i] = self.vertices.len() as u32;

            self.vertices.push([new_point.x, new_point.y, 0.0]);

            self.indices.extend_from_slice(&new_triangle_indices);
        }

        self.remove_triangle(old_triangle_index);

        self.center_points = calc_center_points(&self.vertices, &self.indices);
    }
}
