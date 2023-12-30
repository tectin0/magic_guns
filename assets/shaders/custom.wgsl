// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::mesh2d_functions
// #import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_render::view::View

struct CustomMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0) var<uniform> material: CustomMaterial;
@group(1) @binding(1) var color_texture: texture_2d<f32>;
@group(1) @binding(2) var color_sampler: sampler;
@group(1) @binding(100) var<storage, read> positions: array<f32>;

struct VertexInput {
    @builtin(instance_index) instance_index : u32,
    @builtin(vertex_index) vertex_index : u32,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the fragment shader in location 0
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let pos_x = positions[3u * vertex.vertex_index + 0u];
    let pos_y = positions[3u * vertex.vertex_index + 1u];
    let pos_z = positions[3u * vertex.vertex_index + 2u];

    let position = vec3<f32>(pos_x, pos_y, pos_z);

    // Project the world position of the mesh into screen position
    let model = mesh2d_functions::get_model_matrix(vertex.instance_index);
    out.clip_position = mesh2d_functions::mesh2d_position_local_to_clip(model, vec4<f32>(position, 1.0));
    
    out.color = material.color;
    
    return out;
}

// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    @location(0) color: vec4<f32>,
};

struct FragmentOutput {
    // The color of the fragment
    @location(0) color: vec4<f32>,
};

/// Entry point for the fragment shader
@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    var output : FragmentOutput;
    output.color = in.color;

    return output;
}