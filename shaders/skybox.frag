#version 330

uniform samplerCube skybox;
in vec3 frag_position;

out vec4 color;

void main() {
  color = texture(skybox, frag_position);
}
