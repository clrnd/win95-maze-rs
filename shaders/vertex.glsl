#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTex;

out vec3 o_color;
out vec2 o_tex;

uniform float t;

mat2 r;

void main() {
    mat2 r = mat2(cos(t), -sin(t), sin(t), cos(t));
    gl_Position = vec4(aPos.xy * r, aPos.z, 1.0);
    o_color = aColor;
    o_tex = aTex;
}
