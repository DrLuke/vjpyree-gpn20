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

vec2 uvcscale(vec2 uv, float scale) {
    return (uv-0.5) *scale + 0.5;
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

// Randomvals
uniform float randomVal0;
uniform float randomVal0accum;
uniform float randomVal1;
uniform float randomVal2;
uniform float randomVal3;
uniform float randomVal4;
uniform float randomVal5;
uniform float randomVal6;
uniform float randomVal7;
uniform float randomVal8;
uniform float randomVal9;

#define ROTAXIS vec3(randomVal1, randomVal2, randomVal3)
#define ROTINTENS (randomVal4 * 0.2)
#define FBPHASE randomVal5
#define UVPHASE randomVal6

uniform float beat;
uniform float beatpt1;
uniform float beataccum;
uniform float beataccumpt1;

uniform float time;
uniform vec2 res;

uniform float palval;

#define PAL1 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,1.0),vec3(0.0,0.33,0.67)
#define PAL2 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,1.0),vec3(0.0,0.10,0.20)
#define PAL3 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,1.0),vec3(0.3,0.20,0.20)
#define PAL4 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,0.5),vec3(0.8,0.90,0.30)
#define PAL5 vec3(0.8,0.5,0.4),vec3(0.2,0.4,0.2),vec3(2.0,1.0,1.0),vec3(0.0,0.25,0.25)
#define PAL6 vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,0.5),vec3(0.8,0.90,0.30)

vec3 palsel(float t) {
    if (palval == 0.) {return pal(t, PAL1);}
    if (palval == 1.) {return pal(t, PAL2);}
    if (palval == 2.) {return pal(t, PAL3);}
    if (palval == 3.) {return pal(t, PAL4);}
    if (palval == 4.) {return pal(t, PAL5);}
    if (palval == 5.) {return pal(t, PAL6);}
    return vec3(0);
}

#define PAL PAL5

void main()
{
    // coordinate systems
    vec2 uv = uvIn;


    float aspect = res.x/res.y;
    uv = ((uv-0.5)*vec2(aspect, 1)*rot2(UVPHASE*2.*3.14159))/vec2(aspect, 1) +0.5;

    vec2 uvc = (uv-0.5)*2;

    uv = uvcscale(uv, 1+beatpt1*0.1);

   //uv = discretize(uv, 80);

    // Seed selector
    vec4 seed = vec4(0);
    if (toggle10 == 1.) {
        // Tunnel 1 with borders
        vec3 tun = texture(tunneltex, uv).xyz;
        seed.rgb = palsel( tun.r + tun.g + tun.b - randomVal0accum );
        seed.a = tun.b;
    } else if (toggle11 == 1.) {
        // Tunnel 2 no borders
        vec3 tun = texture(tunneltex, uv).xyz;
        seed.rgb = palsel( tun.r + tun.g  - randomVal0accum );
        seed.a = tun.b;
    } else if (toggle12 == 1.) {
        // Simple RD
        vec4 rd = texture(rdtex, uv);
        seed.rgb = palsel(length(rd.gb - randomVal0accum) );
        seed.a = smoothstep(0.,0.5, rd.b+rd.a);
    } else if (toggle13 == 1.) {
        // Double RD
        vec4 rd = texture(rdtex, uv*2);
        seed.rgb = palsel(length(rd.gb - randomVal0accum) );
        seed.a = smoothstep(0.,0.5, rd.b+rd.a);
    } else if (toggle14 == 1.) {
        // RD on tunnel, masked
        vec3 tun = texture(tunneltex, uv).xyz;
        vec4 rd = texture(rdtex, tun.rg*vec2(4,0.5));
        seed.rgb = palsel(length(rd.gb - randomVal0accum) );
        seed.a = smoothstep(0.,0.5, tun.b * (rd.b+rd.a) * (1-smoothstep(5., 20., tun.y)));
    } else if (toggle15 == 1.) {
        // RD on tunnel, unmasked
        vec3 tun = texture(tunneltex, uv).xyz;
        vec4 rd = texture(rdtex, tun.rg*vec2(4,0.5));
        seed.rgb = palsel(length(rd.gb - randomVal0accum) );
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
        feedback.rgb = texture(prevtex, uvIn + uvc*rot2(atan(prev.g, prev.r))*0.003).rgb*rot3(ROTAXIS, ROTINTENS)*0.999;
    } else if (toggle23 == 1.) {
        feedback.rgb = texture(prevtex, uvIn + vec2(length(uvc))*rot2(FBPHASE*2.*3.14159)*0.01).rgb * rot3(ROTAXIS, ROTINTENS);
    } else if (toggle24 == 1.) {
        feedback.rgb = texture(prevtex, uvcscale(uv, 1.01-beatpt1*0.02)).rgb * rot3(ROTAXIS, ROTINTENS) * 0.999;
    } else if (toggle25 == 1.) {
        feedback.rgb = texture(prevtex, uvIn*2.+vec2(time*0.01 + FBPHASE,0)).rgb*0.9;
    }

    colorOut = mix(clamp(vec4(0), vec4(1), feedback), vec4(seed.rgb, feedback.a), clamp(0, 1, seed.a));
}
