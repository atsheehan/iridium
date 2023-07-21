#version 150

out vec4 color;
in vec2 vertex_tex_coord;

uniform sampler2D tex_sampler;

void main() {
  color = texture(tex_sampler, vertex_tex_coord);
}
