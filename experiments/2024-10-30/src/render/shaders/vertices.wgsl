struct Uniforms {
    transform: mat4x4<f32>,
    transform_for_normals: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.transform * vec4(in.position, 1.0);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4(0.0, 0.0, 1.0, 1.0);
    return color;
}