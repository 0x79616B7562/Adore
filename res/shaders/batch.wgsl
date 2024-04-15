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

@group(1) @binding(0) var textures: binding_array<texture_2d<f32>, #capacity>;
@group(1) @binding(1) var texture_sampler: binding_array<sampler, #capacity>;
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(textures[in.texture_index], texture_sampler[in.texture_index], in.texcoord) * in.color;
}
