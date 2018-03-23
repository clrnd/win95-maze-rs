#version 330 core
in vec2 oTex;
in vec3 oNor;

out vec4 FragColor;

uniform sampler2D tex;
uniform bool solid;
uniform vec3 color;

void main() {
    if (solid) {
        vec3 lightDir = vec3(1.0, 1.0, 1.0);
        float diffuse = max(dot(oNor, lightDir), 0.2);
        FragColor = vec4(color * diffuse * 0.2, 0.0);
    } else {
        FragColor = texture(tex, oTex);
    }
}
