#version 140

in vec2 f_texcoord;
in vec4 f_color;

out vec4 color;

uniform sampler2D tex;

void main() {
  color = texture2D(tex, f_texcoord) * f_color;
}
