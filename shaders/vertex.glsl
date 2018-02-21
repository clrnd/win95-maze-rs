#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

uniform float of;

out vec3 o_color;

void main() {
    float uf = sqrt(1 - of*of);
    gl_Position = vec4(aPos.x + of, aPos.y + uf, aPos.z, 1.0);
    o_color = aColor;
}
