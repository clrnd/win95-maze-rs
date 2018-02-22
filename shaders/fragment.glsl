#version 330 core
in vec3 o_color;
in vec2 o_tex;

out vec4 FragColor;

uniform sampler2D tex1;
uniform sampler2D tex2;

void main() {
    FragColor = mix(texture(tex1, o_tex), texture(tex2, o_tex), 0.5);
}
