#version 330 core
in vec2 oTex;

out vec4 FragColor;

uniform sampler2D tex;
uniform bool solid;
uniform vec3 color;

void main() {
    if (solid) {
        FragColor = vec4(color, 0.0);
    } else {
        FragColor = texture(tex, oTex);
    }
}
