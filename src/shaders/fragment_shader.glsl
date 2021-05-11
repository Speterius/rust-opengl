#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {

    vec3 N = normalize(v_normal);
    vec3 L = normalize(u_light);
    vec3 C = normalize(-v_position);

    float diffuse = max(dot(N, L), 0.0);

    vec3 half_direction = normalize(L + C);

    float specular = pow(max(dot(half_direction, N), 0.0), 16.0);

    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}