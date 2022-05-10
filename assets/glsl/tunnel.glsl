#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;

layout(binding=0) uniform sampler2D prevtex;

layout (location = 0) out vec4 colorOut;

// UTILITIES
vec2 cexp(vec2 z)
{
    return exp(z.x) * vec2(cos(z.y), sin(z.y));
}

vec2 clog(vec2 z)
{
    return vec2(log(length(z)), atan(z.y, z.x));
}

vec2 cpow(vec2 z, float p)
{
    return cexp(clog(z) * p);
}

mat3 rot3(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat3(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s,
    oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s,
    oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c);
}

float discretize(float a, float s)
{
    return round(a*s)/s;
}
vec2 discretize(vec2 a, float s)
{
    return round(a*s)/s;
}
vec3 discretize(vec3 a, float s)
{
    return round(a*s)/s;
}
vec4 discretize(vec4 a, float s)
{
    return round(a*s)/s;
}

mat2 rot2(float a) {
    return mat2(cos(a), -sin(a), sin(a), cos(a));
}

vec3 rgb2hsv(vec3 c)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}


uniform vec2 res;
uniform float time;

void main()
{
    // Centered UV
    vec2 uv = (gl_FragCoord.xy - res.xy*0.5) / res.y;
    vec4 prev = texture(prevtex, uvIn);

    // MATH to tunnelize uv coordinates
    vec2 st = vec2(
        atan(uv.y, uv.x)/(3.14159) - time*0.13,
        1. / length(uv) - time*0.2
    );

    // Render tunnel
    colorOut = vec4(0);
    colorOut.rgb = rot3(vec3(1), st.y)*texture(prevtex, mod(st, 1.) + prev.xy*0.06).rgb*0.9;
    colorOut.xy += 0.1* mod(st, 2.) * length(uv);

    // colorOut *= 1/st.y;

    //colorOut = vec4(1);
}
