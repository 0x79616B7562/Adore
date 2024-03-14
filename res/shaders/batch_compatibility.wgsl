struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) texcoord: vec2<f32>,
    @location(3) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) texcoord: vec2<f32>,
    @location(2) texture_index: u32,
};

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: Camera;

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    out.texcoord = in.texcoord;
    out.texture_index = in.texture_index;

    return out;
}

#include_bind_groups

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var out: vec4<f32>;
    
    #include_body

    return out;
}
