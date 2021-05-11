#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 u_matrix;
uniform mat4 perspective;

void main() {
    v_normal = transpose(inverse(mat3(u_matrix))) * normal;
    gl_Position = perspective * u_matrix * vec4(position, 1.0);
}