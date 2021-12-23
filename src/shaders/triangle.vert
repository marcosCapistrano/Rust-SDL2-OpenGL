#version 450 core
// 'vs_color' is an output that will be sent to the next shader stage
out vec4 vs_color;
void main(void) {
    const vec4 vertices[3] = vec4[3] (vec4(0.25, -0.25, 0.5, 1.0), vec4(-0.25, -0.25, 0.5, 1.0), vec4(0.25, 0.25, 0.5, 1.0));
    const vec4 colors[] = vec4[3] (vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 0.0, 1.0), vec4(0.0, 0.0, 1.0, 1.0));
// Add 'offset' to our hard-coded vertex position
    gl_Position = vertices[gl_VertexID];
// Output a fixed value for vs_color
    vs_color = colors[gl_VertexID];
}