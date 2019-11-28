#version 330 core
in vec2 oTex;
in vec3 oNor;

out vec4 FragColor;

uniform sampler2D tex;
uniform bool rat;
uniform bool shaded;
uniform vec3 color;
uniform int tiling;

void main() {
    if (shaded) {
        vec3 lightDir = vec3(1.0, 1.0, -1.0);
        float diffuse = max(dot(oNor, lightDir), 0.2);
        FragColor = vec4(color * diffuse * 0.2, 0.0);
    } else {
        vec4 color = texture(tex, oTex * tiling);
        // if rat and pure green, discard
        if (rat && color.rgb == vec3(0.0, 1.0, 0.0)) {
            discard;
        }
        FragColor = color;
    }
}
