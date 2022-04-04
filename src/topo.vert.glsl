#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out float intensity;

uniform mat4 model;
uniform mat4 perspective;

void main() {
    v_normal = transpose(inverse(mat3(model))) * surface;
    intensity = position.z;
    vec3 normalized_position = vec3(position.x, position.y, position.z * 0.2);
    gl_Position = perspective * model * vec4(normalized_position, 1.0);
}