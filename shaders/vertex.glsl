#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTex;
layout (location = 2) in vec3 aNor;

out vec2 oTex;
out vec3 oNor;

uniform bool rat;
uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main() {
    if (rat) {
        // reset rotation part of the model view matrix
        mat4 mv = mat4(1.0);
        mat4 tmp = view * model;
        mv[3] = tmp[3];
        gl_Position = proj * mv * vec4(aPos, 1.0);
    } else {
        gl_Position = proj * view * model * vec4(aPos, 1.0);
    }
    oTex = aTex;
    oNor = mat3(transpose(inverse(model))) * aNor;
}
