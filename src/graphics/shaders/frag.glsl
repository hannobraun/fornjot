#version 450


#define PI 3.14159265359


layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 color;


void main() {
    vec3 light = vec3(0.0, 0.0, 1.0);

    float angle = acos(dot(-light, normal));
    float f_angle = angle / (PI / 2.0);

    float f_normal = max(1.0 - f_angle, 0.0);

    color = vec4(vec3(1.0, 0.0, 0.0) * f_normal, 1.0);
}
