//! Base Mesh is 100 x 100

use bevy::{ecs::system::Resource, render::color::Color};

#[derive(Resource)]
pub struct BaseMapMeshInfo {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
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

        let mut colors: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
        colors.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 3]);

        Self {
            vertices,
            indices,
            colors,
        }
    }
}
