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

uniform float toggle5;
uniform float toggle6;
uniform float toggle7;
uniform float toggle8;
uniform float toggle9;

//uniform float randomVal20;// Mask speed
uniform float accumtime;
uniform float randomVal21;// Phase offset
uniform float randomVal22;// Mask Frequency
uniform float randomVal23;// Length

uniform float beataccum;

void main()
{
    // Centered UV
    vec2 uv = (gl_FragCoord.xy - res.xy*0.5) / res.y;
    vec4 prev = texture(prevtex, uvIn);

    // MATH to tunnelize uv coordinates
    float a = atan(uv.y, uv.x)/(3.14159*2.);
    float r = 1. / length(uv);
    vec2 st = vec2(
    a,
    r
    );

    // Control parameters from random vals
    float t = accumtime - randomVal21 * 3.14159 * 2.;
    float freq = discretize(randomVal22*10, 1.) + 1;

    // Generate masks
    colorOut = vec4(0);
    if (toggle5 == 1.) {
        // Spiral scrolling
        colorOut.b += smoothstep(0.75, 0.8, sin(st.x*3.14159*2.*freq - t + st.y*3.*randomVal23));
    }
    if (toggle6 == 1.) {
        // Spiral static
        colorOut.b += smoothstep(0.7, 0.8, sin(st.x*3.14159*2.*freq + st.y*3.*randomVal23));
    }
    if (toggle7 == 1.) {
        // Rings scrolling
        colorOut.b += smoothstep(0.7, 0.8, sin(st.y*3. - t));
    }
    if (toggle8 == 1.) {
        // Rings static
        colorOut.b += smoothstep(0.7, 0.8, sin(st.y*3.));
    }
    if (toggle9 == 1.) {
        // Dots
        colorOut.b += smoothstep(0.85, 0.95, sin(st.x*3.14159*-2.*freq - t) * sin(st.y*3.14159*2. - t));
    }

    // Render tunnel UV coordinates
    colorOut.rg = mod(st, 1.);
}
