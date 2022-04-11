#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;

layout (location = 0) out vec4 colorOut;


uniform float time;
uniform float dt;
uniform vec2 res;

void main()
{
    colorOut = vec4(uvIn, 1., 1.);
}