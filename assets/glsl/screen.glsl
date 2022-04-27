#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;

layout(binding=0) uniform sampler2D tex;

layout (location = 0) out vec4 colorOut;

void main()
{
    colorOut = texture(tex, uvIn);
}
