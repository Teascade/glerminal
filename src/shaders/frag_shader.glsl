#version 110

varying vec2 f_texcoord;
varying vec4 f_color;

uniform sampler2D tex;

void main() {
  gl_FragColor = texture2D(tex, f_texcoord) * f_color;
}
