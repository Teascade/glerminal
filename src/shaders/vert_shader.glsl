#version 330 core

in vec2 position;
in vec2 texcoord;

out vec2 f_texcoord;

void main() {
  gl_Position = vec4(position, 0, 1);
  f_texcoord = texcoord;
}
