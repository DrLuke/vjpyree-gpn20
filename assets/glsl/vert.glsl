#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;

layout (location = 0) out vec3 posOut;
layout (location = 1) out vec2 uvOut;
layout (location = 2) out vec3 normOut;

uniform mat4 MVP;

void main()
{
    gl_Position = MVP * vec4(posIn, 1);
    posOut = (MVP * vec4(posIn, 1)).xyz;
    uvOut = uvIn;
    normOut = normIn;
}