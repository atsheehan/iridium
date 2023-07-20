#version 330

void main() {
  vec2 vertices[3];
  vertices[0] = vec2(-0.5, -0.5);
  vertices[1] = vec2(0.5, -0.5);
  vertices[2] = vec2(0.0, 0.5);

  gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
}
