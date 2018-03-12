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
  float x_shake = sin(time * shakiness * 50) * 0.002 * shakiness;
  float y_shake = sin(time * shakiness * 40) * 0.004 * shakiness;
  gl_Position = proj_mat * vec4(position + vec2(x_shake, y_shake), 0, 1);
  f_texcoord = texcoord;
  f_color = color;
}
