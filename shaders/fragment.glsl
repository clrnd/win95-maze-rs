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
        vec3 norm = normalize(oNor);
        float diffuse = max(dot(norm, lightDir), 0.1);
        FragColor = vec4(color * diffuse, 0.0);
    } else {
        FragColor = texture(tex, oTex);
    }
}
