#version 450 core// 'vs_color' is the color produced by the vertex shader
in vec4 vs_color;
out vec4 color;
void main(void) {
    color = vs_color;
}