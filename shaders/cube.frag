#version 150

out vec4 color;
in vec2 vertex_tex_coord;
in vec3 vertex_normal;

uniform sampler2D tex_sampler;

const float PI = 3.1415926535897932384626433832795;

const float sun_heading = PI / 3;
const float sun_pitch = PI / 3;

const float diffuse_factor = 0.5;
const float ambient_factor = 0.5;

void main() {
  vec3 sun_direction = vec3(sin(sun_pitch) * cos(sun_heading), sin(sun_pitch) * sin(sun_heading), cos(sun_pitch));

  vec4 frag_color = texture(tex_sampler, vertex_tex_coord);
  vec4 diffuse_color = frag_color * max(dot(vertex_normal, sun_direction), 0.0) * diffuse_factor;
  vec4 ambient_color = frag_color * ambient_factor;

  color = diffuse_color + ambient_color;
}
