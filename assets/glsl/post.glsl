#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;


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

// License information applies only to the "pal" function and all it's uses
// The MIT License
// https://www.youtube.com/c/InigoQuilez
// https://iquilezles.org/
// Copyright © 2015 Inigo Quilez
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// From: https://www.shadertoy.com/view/ll2GD3
vec3 pal( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d )
{
    return a + b*cos( 6.28318*(c*t+d) );
}

// -----------------------------------------------------

layout(binding=0) uniform sampler2D prevtex;
layout(binding=1) uniform sampler2D rdtex;
layout(binding=2) uniform sampler2D tunneltex;

layout (location = 0) out vec4 colorOut;

// Pixel seed toggles
uniform float toggle10;
uniform float toggle11;
uniform float toggle12;
uniform float toggle13;
uniform float toggle14;
uniform float toggle15;

// Feedback toggles
uniform float toggle20;
uniform float toggle21;
uniform float toggle22;
uniform float toggle23;
uniform float toggle24;
uniform float toggle25;

uniform float beat;
uniform float beataccum;

#define PAL1 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,1.0),vec3(0.0,0.33,0.67)

void main()
{
    // coordinate systems
    vec2 uv = uvIn;
    vec2 uvc = (uvIn-0.5)*2;

    // Seed selector
    vec4 seed = vec4(0);
    if (toggle10 == 1.) {
        // Tunnel 1 with borders
        vec3 tun = texture(tunneltex, uvIn).xyz;
        seed.rgb = pal( tun.r + tun.g + tun.b, PAL1 );
        seed.a = tun.b;
    } else if (toggle11 == 1.) {
        // Tunnel 2 no borders
        vec3 tun = texture(tunneltex, uvIn).xyz;
        seed.rgb = pal( tun.r + tun.g, PAL1 );
        seed.a = tun.b;
    } else if (toggle12 == 1.) {
        // Simple RD
        vec4 rd = texture(rdtex, uvIn);
        seed.rgb = pal(length(rd.gb), PAL1 );
        seed.a = smoothstep(0.,0.5, rd.b+rd.a);
    } else if (toggle13 == 1.) {
        // Double RD
        vec4 rd = texture(rdtex, uvIn*2);
        seed.rgb = pal(length(rd.gb), PAL1 );
        seed.a = smoothstep(0.,0.5, rd.b+rd.a);
    } else if (toggle14 == 1.) {
        // RD on tunnel, masked
        vec3 tun = texture(tunneltex, uvIn).xyz;
        vec4 rd = texture(rdtex, tun.rg*vec2(4,0.5));
        seed.rgb = pal(length(rd.gb), PAL1 );
        seed.a = smoothstep(0.,0.5, tun.b * (rd.b+rd.a) * (1-smoothstep(5., 20., tun.y)));
    } else if (toggle15 == 1.) {
        // RD on tunnel, unmasked
        vec3 tun = texture(tunneltex, uvIn).xyz;
        vec4 rd = texture(rdtex, tun.rg*vec2(4,0.5));
        seed.rgb = pal(length(rd.gb), PAL1 );
        seed.a = smoothstep(0.,0.5, rd.b+rd.a) * (1-smoothstep(5., 20., tun.y));
    }

    // Feedback selector
    vec4 feedback = vec4(0);
    vec4 prev = texture(prevtex, uvIn);
    if (toggle20 == 1.) {
        // Clear
    } else if (toggle21 == 1.) {
        // Dark clouds
        feedback.rgb = texture(prevtex, uvIn + uvc*rot2(atan(prev.g, prev.r))*0.003).rgb*rot3(vec3(prev.rgb), length(uvIn-0.5)*length(prev))*0.999;
    } else if (toggle22 == 1.) {

    } else if (toggle23 == 1.) {

    } else if (toggle24 == 1.) {

    } else if (toggle25 == 1.) {

    }

    /*vec4 prev = texture(prevtex, uvIn);
    prev = texture(prevtex, uvIn-(prev.rg-0.3)*0.02);
    vec4 rd = texture(rdtex, uvIn);

    // RD masks
    float rdMask1 = smoothstep(0.2, 0.5, rd.g);
    float rdMask2 = smoothstep(0.2, 0.7, rd.b);


    colorOut.rgb = vec3(rdMask1);

    colorOut.rgb = mix(rd.rgb*rot3(vec3(1), rd.g), prev.rgb*rot3(vec3(1), 0.15), 1.-rdMask1);

    //colorOut.rgb = vec3(1.-rd.r, 0, rd.g);
    */

    colorOut = mix(clamp(vec4(0), vec4(1), feedback), vec4(seed.rgb, feedback.a), clamp(0, 1, seed.a));
}
