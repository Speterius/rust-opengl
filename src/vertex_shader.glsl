#version 140

in vec2 position;
uniform float dx;

void main() {
    vec2 pos = position;
    gl_Position = vec4(pos.x+dx, pos.y, 0.0, 1.0);
}