#version 450


layout(set = 0, binding = 0) uniform Locals {
    mat4 transform;
};

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(location = 1) out vec3 normal_out;


void main() {
    gl_Position = transform * vec4(position, 1.0);
    normal_out = (transform * vec4(normal, 0.0)).xyz;
}
