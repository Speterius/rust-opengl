#version 140

in vec3 position;
in vec3 normal;
in vec3 texcoord;

out vec3 v_normal;
out vec2 v_texcoord;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 1.0);
    v_normal = normal;
}