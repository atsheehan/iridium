#version 330

const float NEAR = 0.0;
const float FAR = 10000.0;

uniform vec3 position = vec3(2.0, 3.0, 6.0);

out vec3 vertex_color;

void main() {
  vec3 near_bottom_left = vec3(0.0, 0.0, 0.0);
  vec3 near_bottom_right = vec3(1.0, 0.0, 0.0);
  vec3 near_top_left = vec3(0.0, 1.0, 0.0);
  vec3 near_top_right = vec3(1.0, 1.0, 0.0);
  vec3 far_bottom_left = vec3(0.0, 0.0, 1.0);
  vec3 far_bottom_right = vec3(1.0, 0.0, 1.0);
  vec3 far_top_left = vec3(0.0, 1.0, 1.0);
  vec3 far_top_right = vec3(1.0, 1.0, 1.0);

  vec3 vertices[36];
  // Front side
  vertices[0] = near_bottom_left;
  vertices[1] = near_bottom_right;
  vertices[2] = near_top_right;

  vertices[3] = near_top_right;
  vertices[4] = near_top_left;
  vertices[5] = near_bottom_left;

  // Right side
  vertices[6] = near_bottom_right;
  vertices[7] = far_bottom_right;
  vertices[8] = far_top_right;

  vertices[9] = far_top_right;
  vertices[10] = near_top_right;
  vertices[11] = near_bottom_right;

  // Far side
  vertices[12] = far_bottom_right;
  vertices[13] = far_bottom_left;
  vertices[14] = far_top_left;

  vertices[15] = far_top_left;
  vertices[16] = far_top_right;
  vertices[17] = far_bottom_right;

  // Left side
  vertices[18] = far_bottom_left;
  vertices[19] = near_bottom_left;
  vertices[20] = near_top_left;

  vertices[21] = near_top_left;
  vertices[22] = far_top_left;
  vertices[23] = far_bottom_left;

  // Top side
  vertices[24] = near_top_left;
  vertices[25] = near_top_right;
  vertices[26] = far_top_right;

  vertices[27] = far_top_right;
  vertices[28] = far_top_left;
  vertices[29] = near_top_left;

  // Bottom side
  vertices[30] = far_bottom_left;
  vertices[31] = far_bottom_right;
  vertices[32] = near_bottom_right;

  vertices[33] = near_bottom_right;
  vertices[34] = near_bottom_left;
  vertices[35] = far_bottom_left;

  vec3 colors[6];
  colors[0] = vec3(0.0, 1.0, 0.0);
  colors[1] = vec3(1.0, 0.0, 0.0);
  colors[2] = vec3(0.0, 0.0, 1.0);
  colors[3] = vec3(1.0, 0.0, 1.0);
  colors[4] = vec3(0.0, 1.0, 1.0);
  colors[5] = vec3(1.0, 1.0, 0.0);

  mat4 world_to_clip_transform =
    mat4(1.0, 0.0, 0.0, 0.0,
         0.0, 1.0, 0.0, 0.0,
         0.0, 0.0, 1.0, 0.0,
         0.0, 0.0, 1.0, 0.0);

  mat4 model_to_world_transform =
    mat4(1.0, 0.0, 0.0, position.x,
         0.0, 1.0, 0.0, position.y,
         0.0, 0.0, 1.0, position.z,
         0.0, 0.0, 0.0, 1.0);

  gl_Position = vec4(vertices[gl_VertexID], 1.0) * model_to_world_transform * world_to_clip_transform;

  vertex_color = colors[gl_VertexID / 6];
}
