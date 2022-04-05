#version 150

in vec3 to;
in float intensity;

out vec4 color;

uniform vec3 light;

void main() {
    float brightness = dot(normalize(to), normalize(light));
    vec3 dark_color;
    vec3 regular_color;
    if (intensity > 0.7) {
        dark_color = mix(vec3(0.2, 0.6, 0.1), vec3(0.0, 0.3, 0.0), (intensity - 0.7) * 10.0 / 3.0);
    } else if (intensity <= 0.7 && intensity > 0.6) {
        dark_color = mix(vec3(0.2, 0.2, 0.2), vec3(0.2, 0.6, 0.1), (intensity - 0.6) * 10.0);
    } else if (intensity <= 0.6) {
        dark_color = mix(vec3(0.9, 0.9, 0.9), vec3(0.2, 0.2, 0.2), intensity * (1.0 / 0.6));

    }

    regular_color = vec3(dark_color.x * 0.6, dark_color.y * 0.6, dark_color.z * 0.6);

    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}