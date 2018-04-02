#version 330 core

in vec2 f_texcoord;
in vec4 f_color;

out vec4 color;

void main() {
  color = f_color;
}
