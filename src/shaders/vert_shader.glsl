#version 330 core

in vec2 position;
in vec2 texcoord;
in vec4 color;
in float shakiness;

out vec2 f_texcoord;
out vec4 f_color;

uniform mat4 proj_mat;
uniform float time;

void main() {
  float x_shake = sin(time * sqrt(shakiness) * 50) * 0.02 * shakiness / 10;
  float y_shake = sin(time * sqrt(shakiness) * 40) * 0.03 * shakiness / 10;
  gl_Position = proj_mat * vec4(position + vec2(x_shake, y_shake), 0, 1);
  f_texcoord = texcoord;
  f_color = color;
}
