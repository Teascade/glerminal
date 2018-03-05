#version 330 core

in vec2 position;
in vec2 texcoord;

out vec2 f_texcoord;

uniform mat4 proj_mat;

void main() {
  gl_Position = proj_mat * vec4(position, 0, 1);
  f_texcoord = texcoord;
}
