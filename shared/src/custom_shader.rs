//! mesh2d_functions: https://github.com/bevyengine/bevy/blob/main/crates/bevy_sprite/src/mesh2d/mesh2d_functions.wgsl
//! mesh2d_manual: https://github.com/bevyengine/bevy/blob/main/examples/2d/mesh2d_manual.rs
//! shadplay shader resource (for wgsl in bevy): https://github.com/alphastrata/shadplay
//! wireframe example: https://github.com/samdauwe/webgpu-native-examples/blob/master/src/examples/wireframe_vertex_pulling.c#L58
//! wgpu tutorial: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/
//! 3d lines example: https://github.com/bevyengine/bevy/blob/8067e46049f222d37ac394745805bad98979980f/examples/3d/lines.rs

#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    pub color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
    #[storage(100, read_only)]
    pub positions: Vec<f32>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/custom.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Fill;
        Ok(())
    }
}

/// Plugin that renders [`ColoredMesh2d`]s
pub struct CustomMaterialPlugin;

/// Handle to the custom shader with a unique random ID
pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(13828845428412094821);

const CUSTOM_SHADER: &str = include_str!("../../assets/shaders/custom.wgsl");

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        // Load our custom shader
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.insert(
            COLORED_MESH2D_SHADER_HANDLE,
            Shader::from_wgsl(CUSTOM_SHADER, file!()),
        );

        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    }
}
