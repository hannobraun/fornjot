#version 450


layout(set = 0, binding = 0) uniform Locals {
    mat4 transform;
};

layout(location = 0) in vec2 pos;


void main() {
    gl_Position = transform * vec4(pos, 0.0, 1.0);
}
