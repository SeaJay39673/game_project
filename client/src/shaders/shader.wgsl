struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,

    @location(3) m0: vec4<f32>,
    @location(4) m1: vec4<f32>,
    @location(5) m2: vec4<f32>,
    @location(6) m3: vec4<f32>,
    @location(7) instance_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@group(0) @binding(0)
var tex: texture_2d<f32>;

@group(0) @binding(1)
var tex_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(
        input.m0,
        input.m1,
        input.m2,
        input.m3,
    );
    
    var output: VertexOutput;
    output.position = model *  vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    output.color = input.color * input.instance_color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(tex, tex_sampler, input.uv);
    return tex_color * input.color;
}