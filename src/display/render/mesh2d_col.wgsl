//bevy-0.8.0/crates/bevy_sprite/src/mesh2d/

struct View {
    view_proj: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    world_position: vec3<f32>,
    near: f32,
    far: f32,
    width: f32,
    height: f32,
};


@group(0) @binding(0)
var<uniform> view: View;

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the framgent shader in location 0
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

/// Entry point for the vertex shader
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    out.clip_position = view.view_proj * vec4<f32>(vertex.position, 1.0);
    // out.clip_position = view.view_proj * vec4<f32>(round(vertex.position), 1.0);
    // out.clip_position = view.view_proj * vec4<f32>(floor(vertex.position), 1.0);
    out.color = vertex.color;
    out.uv = vertex.uv;
    return out;
}


@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;
@group(1) @binding(1)
var sprite_sampler: sampler;

// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

/// Entry point for the fragment shader
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    //uv.y=1.0-uv.y;
    var tex_color = textureSample(sprite_texture, sprite_sampler, uv);
    var c = in.color*tex_color;
    //if (c.w==0.0) {c.w=0.5;}
    //if (c.w==0.0) {c=vec4<f32>(1.0,0.0,0.0,1.0);}
    return c;
}
