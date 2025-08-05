
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

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>, //added
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>, //added
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view.view_proj * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    out.uv = vertex.uv; //added
    return out;
}

//added
@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;
@group(1) @binding(1)
var sprite_sampler: sampler;

//
struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>, //added
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    //uv.y=1.0-uv.y;
    var tex_color = textureSample(sprite_texture, sprite_sampler, uv);
    var c = in.color*tex_color;
    //if (c.w==0.0) {c.w=0.5;}
    //if (c.w==0.0) {c=vec4<f32>(1.0,0.0,0.0,1.0);}
    return c;

    //return in.color;
}
