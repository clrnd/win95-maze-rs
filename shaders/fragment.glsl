#version 330 core
in vec3 o_color;
in vec2 o_tex;

out vec4 FragColor;

uniform sampler2D tex1;
uniform sampler2D tex2;
uniform float t;

void main() {
    FragColor = mix(texture(tex1, o_tex), texture(tex2, o_tex), (t + 1.0)/2.0);
}
