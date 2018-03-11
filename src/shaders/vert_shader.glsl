#version 330 core

in vec2 position;
in vec2 texcoord;
in vec4 color;

out vec2 f_texcoord;
out vec4 f_color;

uniform mat4 proj_mat;

void main() {
  gl_Position = proj_mat * vec4(position, 0, 1);
  f_texcoord = texcoord;
  f_color = color;
}
