#version 330 core
in vec2 o_tex;

out vec4 FragColor;

uniform int tex_idx;
uniform sampler2D tex;

void main() {
    FragColor = texture(tex, o_tex);
}
