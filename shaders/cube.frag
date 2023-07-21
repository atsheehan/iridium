#version 330

out vec3 color;
in vec3 vertex_color;

void main() {
  color = vertex_color;
}
