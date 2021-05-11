#version 140

in vec3 v_normal;
in vec2 v_texcoord;

out vec4 color;

void main() {
    color = vec4((v_normal + 1) /2, 1.0);
}
