#version 330 core
in vec2 o_tex;

out vec4 FragColor;

// TODO
// wrong, this is per object
uniform bool is_thing;
uniform sampler2D tex1;
uniform sampler2D tex2;

void main() {
    if (is_thing) {
        FragColor = texture(tex2, o_tex);
    } else {
        FragColor = texture(tex1, o_tex);
    }
}
