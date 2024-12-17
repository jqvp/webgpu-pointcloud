struct Uniforms {
  time: f32,
  width: f32,
  height: f32,
  pixels: f32,
  modelMatrix: mat4x4f,
  viewMatrix: mat4x4f,
};
@group(0) @binding(0) var<uniform> unif: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3f(1);
    out.clip_position = unif.viewMatrix * unif.modelMatrix * vec4f(model.position, 1.0);
    return out;
}

@fragment fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}