@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vertex(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return transform * vec4(position, 1.0);
}

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}