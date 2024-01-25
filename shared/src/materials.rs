use std::ops::Deref;

use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets, Handle},
    ecs::system::{Resource},
    reflect::TypePath,
    render::{
        color::Color,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, PrimitiveTopology, RenderPipelineDescriptor, Shader,
            ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

#[derive(Resource)]
pub struct MapMaterialHandle(pub Handle<MapMaterial>);

impl Deref for MapMaterialHandle {
    type Target = Handle<MapMaterial>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct MapMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/basic.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let polygon_mode = match key.mesh_key.primitive_topology() {
            PrimitiveTopology::TriangleList => PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip => PolygonMode::Fill,
            PrimitiveTopology::LineList => PolygonMode::Line,
            PrimitiveTopology::LineStrip => PolygonMode::Line,
            PrimitiveTopology::PointList => PolygonMode::Point,
        };

        log::debug!("polygon_mode: {:?}", polygon_mode);

        descriptor.primitive.polygon_mode = polygon_mode;
        Ok(())
    }
}

pub const STEEL_BLUE: Color = Color::rgb(0.27, 0.51, 0.71);

pub struct MapMaterialPlugin;

/// Handle to the custom shader with a unique random ID
pub const MAP_MATERIAL_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(83726845423112024821);

const CUSTOM_SHADER: &str = include_str!("../../assets/shaders/basic.wgsl");

impl Plugin for MapMaterialPlugin {
    fn build(&self, app: &mut App) {
        // Load our custom shader
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.insert(
            MAP_MATERIAL_SHADER_HANDLE,
            Shader::from_wgsl(CUSTOM_SHADER, file!()),
        );

        app.add_plugins(Material2dPlugin::<MapMaterial>::default());

        let map_material_handle = app
            .world
            .resource_mut::<Assets<MapMaterial>>()
            .add(MapMaterial { color: STEEL_BLUE });

        app.world
            .insert_resource(MapMaterialHandle(map_material_handle));
    }
}
