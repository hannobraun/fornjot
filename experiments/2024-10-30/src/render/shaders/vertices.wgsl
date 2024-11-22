struct Uniforms {
    transform: mat4x4<f32>,
    transform_for_normals: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) center: vec3<f32>,
    @location(2) radius: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) point: vec3<f32>,
    @location(1) center: vec3<f32>,
    @location(2) radius: f32,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = uniforms.transform * vec4(in.position, 1.0);

    out.point = in.position;
    out.center = in.center;
    out.radius = in.radius;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if length(in.center - in.point) > in.radius {
        discard;
    }

    var color = vec4(0.5, 0.5, 0.5, 1.0);
    return color;
}
