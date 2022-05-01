#version 450 core
layout (location = 0) in vec3 posIn;
layout (location = 1) in vec2 uvIn;
layout (location = 2) in vec3 normIn;

layout(binding=0) uniform sampler2D prevtex;

layout (location = 0) out vec4 colorOut;

uniform float time;
uniform float dt;
uniform vec2 res;

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

    return mat3(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c          );
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

float sdCircle( vec2 p, float r )
{
  return length(p) - r;
}

float sdCross( in vec2 p, in vec2 b, float r )
{
    p = abs(p); p = (p.y>p.x) ? p.yx : p.xy;
    vec2  q = p - b;
    float k = max(q.y,q.x);
    vec2  w = (k>0.0) ? q : vec2(b.y-p.x,-k);
    return sign(k)*length(max(w,0.0)) + r;
}

float sdOctogon( in vec2 p, in float r )
{
    const vec3 k = vec3(-0.9238795325, 0.3826834323, 0.4142135623 );
    p = abs(p);
    p -= 2.0*min(dot(vec2( k.x,k.y),p),0.0)*vec2( k.x,k.y);
    p -= 2.0*min(dot(vec2(-k.x,k.y),p),0.0)*vec2(-k.x,k.y);
    p -= vec2(clamp(p.x, -k.z*r, k.z*r), r);
    return length(p)*sign(p.y);
}

float sdEquilateralTriangle( in vec2 p )
{
    const float k = sqrt(3.0);
    p.x = abs(p.x) - 1.0;
    p.y = p.y + 1.0/k;
    if( p.x+k*p.y>0.0 ) p = vec2(p.x-k*p.y,-k*p.x-p.y)/2.0;
    p.x -= clamp( p.x, -2.0, 0.0 );
    return -length(p)*sign(p.y);
}

const float scale[5] = float[5](0.2, 0.8, 1.0, 0.8, 0.2);
vec4 laplace(vec2 fragCoord)
{
    //vec4 outVal = texture(tex, fragCoord / iResolution.xy) * 25.;
    vec4 outVal = texture(prevtex, fragCoord / res.xy) * 9.;
	for(int i = -2; i <= 2; i++)
    {
        for(int j = -2; j <= 2; j++)
        {
            vec2 uv = (fragCoord + vec2(i, j)) / res.xy;
            outVal -= texture(prevtex, uv) * scale[i + 2] * scale[j + 2];
        }
    }
    //return clamp(outVal*0.25, -0.5, 0.5);
    //return clamp(outVal *0.25, -1., 1.);
    return outVal*0.25;
}

// ###################### PATTERNS

vec4 main1(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    float Da = 1.;
    float Db = 0.3;
    float f = 0.04 + sin(length(uvf)*10.)*0.015;
    float k = .103 + length(uvf)*0.006 + sin(length(uvf)*10.)*0.015;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}

vec4 main2(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    float Da = 1. ;
    float Db = 0.2 + sin(length(uvf)*10) * 0.15;
    float f = 0.0287;
    float k = .078;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}

vec4 main3(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    /*float Da = 1.;
    float Db = 0.2;
    float f = 0.025;
    float k = .098;*/
    float Da = 1.;
    float Db = clamp(0.5 - lap.r*2, 0.1, 1);
    float f = 0.042;
    float k = .103;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}

vec4 main4(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    /*float Da = 1.;
    float Db = 0.2;
    float f = 0.025;
    float k = .098;*/
    float Da = 1.;
    float Db = 0.2;
    float f = 0.042;
    float k = .103;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}

vec4 main5(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    /*float Da = 1.;
    float Db = 0.2;
    float f = 0.025;
    float k = .098;*/
    float Da = 1.;
    //float Db = 0.4 - length(uvf)*0.2;
    float Db = 0.35 - length(uvf)*0.05;
    float f = 0.042;
    float k = .103;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7 * vec2(1, 1.7),
    0, 0.9);

    return vec4(newCon, lap.rg*30.);;
}

vec4 main6(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    float Da = 1.;
    float Db = 0.3;
    float f = 0.04 + sin(length(uvf)*10.)*0.015;
    float k = .103 + length(uvf)*0.006 + sin(length(uvf)*10.)*0.015;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}

// ###################### WIPES

uniform float randomVal10;
uniform float randomVal11;
uniform float randomVal12;
uniform float randomVal13;

uniform float wipe0;
uniform float wipe1;
uniform float wipe2;
uniform float wipe3;
uniform float wipe4;

#define WIPESTEPS (randomVal10*9+1)
#define WIPETHICCNESS (0.01 + randomVal11*0.04)

vec4 wipe0f(vec2 uv)
{
    vec2 uvc = (uv - vec2(0.5))*2;
    uvc *= vec2(res.x/res.y, 1.);

    float dist = 9999;
    float size = discretize(wipe0, WIPESTEPS);
    dist = abs(sdEquilateralTriangle(uvc/(0.25 + size))) - 0.0001;

    return vec4(0, 0.6, 0, step(-WIPETHICCNESS, -dist));
}

vec4 wipe1f(vec2 uv)
{
    vec2 uvc = (uv - vec2(0.5))*2;
    uvc *= vec2(res.x/res.y, 1.);

    float dist = 9999;

    dist = abs(sdOctogon(uvc, discretize(wipe1, WIPESTEPS))) - 0.0001;

    return vec4(0, 0.6, 0, step(-WIPETHICCNESS, -dist));
}

vec4 wipe2f(vec2 uv)
{
    vec2 uvc = (uv - vec2(0.5))*2;
    uvc *= vec2(res.x/res.y, 1.);

    float dist = 9999;

    dist = abs(sdCross(uvc, vec2(discretize(wipe2, WIPESTEPS)), 0.) - 0.0001);

    return vec4(0, 0.6, 0, step(-WIPETHICCNESS, -dist));
}

vec4 wipe3f(vec2 uv)
{
    vec2 uvc = (uv - vec2(0.5))*2;
    uvc *= vec2(res.x/res.y, 1.);

    float dist = 9999;

    dist = abs(sdCross(uvc, vec2(discretize(wipe3, WIPESTEPS), discretize(wipe3, WIPESTEPS)*0.3), 0.1) - 0.0001);

    return vec4(0, 0.6, 0, step(-WIPETHICCNESS, -dist));
}

vec4 wipe4f(vec2 uv)
{
    vec2 uvc = (uv - vec2(0.5))*2;
    uvc *= vec2(res.x/res.y, 1.);

    float dist = 9999;

    dist = abs(sdCircle(uvc, discretize(wipe4, WIPESTEPS))) - 0.0001;

    return vec4(0, 0.6, 0, step(-WIPETHICCNESS, -dist));
}

uniform float toggle0;
uniform float toggle1;
uniform float toggle2;
uniform float toggle3;
uniform float toggle4;

// ######################
void main()
{
    vec2 uvf = ((uvIn - vec2(0.5))*2);

    vec2 uv = uvIn * 1;
    vec2 uva = uv * vec2(res.x/res.y, 1.);
    vec2 uv11 = (uv - 0.5) * 2.;
    vec2 uv11a = uv11 * vec2(res.x/res.y, 1.);

    // Patterns
    if (toggle0 == 1.) {colorOut = main1(uv);}
    else if (toggle1 == 1.) {colorOut = main2(uv);}
    else if (toggle2 == 1.) {colorOut = main3(uv);}
    else if (toggle3 == 1.) {colorOut = main4(uv);}
    else if (toggle4 == 1.) {colorOut = main5(uv);}
    else {colorOut = main5(uvIn);}

    if(wipe0 > 0.) { colorOut.rg = mix(colorOut.rg, wipe0f(uv).rg, wipe0f(uv).a); }
    if(wipe1 > 0.) { colorOut.rg = mix(colorOut.rg, wipe1f(uv).rg, wipe1f(uv).a); }
    if(wipe2 > 0.) { colorOut.rg = mix(colorOut.rg, wipe2f(uv).rg, wipe2f(uv).a); }
    if(wipe3 > 0.) { colorOut.rg = mix(colorOut.rg, wipe3f(uv).rg, wipe3f(uv).a); }
    if(wipe4 > 0.) { colorOut.rg = mix(colorOut.rg, wipe4f(uv).rg, wipe4f(uv).a); }

    //colorOut = vec4(uvIn, randomVal0, 0.);
}
