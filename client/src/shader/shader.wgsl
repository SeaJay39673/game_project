struct Uniforms { model: mat4x4<f32>, };
struct UvUniform { uv_min: vec2<f32>, uv_max: vec2<f32>, };

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<uniform> uv_data: UvUniform;
@group(0) @binding(2) var my_texture: texture_2d<f32>;
@group(0) @binding(3) var my_sampler: sampler;

struct VertexInput { @location(0) position: vec2<f32>, };
struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>, };

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform quad position to world
    out.position = uniforms.model * vec4<f32>(input.position, 0.0, 1.0);

    // Map quad [-1,1] → [0,1]
    let local_uv = input.position * 0.5 + vec2<f32>(0.5, 0.5);

    // Interpolate UV linearly using top-left origin atlas
    out.uv = uv_data.uv_min + local_uv * (uv_data.uv_max - uv_data.uv_min);

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(my_texture, my_sampler, input.uv);
}
