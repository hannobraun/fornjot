#version 450


layout(set = 0, binding = 0) uniform Locals {
    mat4 transform;
};

layout(location = 0) in vec3 pos;


void main() {
    gl_Position = transform * vec4(pos, 1.0);
}
