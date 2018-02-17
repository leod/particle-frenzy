// Large amounts of the following shaders are copied from the following awesome ggez example:
//     https://github.com/gfx-rs/gfx/blob/master/examples/support/particle/shader/

pub const GEOMETRY: &[u8] = br"#version 150 core
layout (points) in;
layout (triangle_strip, max_vertices=4) out;

in VertexData {
    vec4 color;
} VertexIn[];

out VertexData {
    vec4 color;
    vec2 uv;
} VertexOut;

layout (std140)
uniform Locals {
    float u_Aspect;
};

#define PARTICLE_RADIUS 0.05

void main()
{
    gl_Position = gl_in[0].gl_Position + vec4(-PARTICLE_RADIUS*u_Aspect, -PARTICLE_RADIUS, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(-1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(PARTICLE_RADIUS*u_Aspect, -PARTICLE_RADIUS, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(-PARTICLE_RADIUS*u_Aspect, PARTICLE_RADIUS, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(-1, 1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(PARTICLE_RADIUS*u_Aspect, PARTICLE_RADIUS, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(1, 1);
    EmitVertex();
}";

pub const VERTEX: &[u8] = br"
#version 150 core

in vec2 a_Pos;
in vec4 a_Color;

out VertexData {
    vec4 color;
} VertexOut;

void main() {
    gl_Position = vec4(a_Pos, 0, 1);
    VertexOut.color = a_Color;
}";

pub const PIXEL: &[u8] = br"
#version 150 core

in VertexData {
    vec4 color;
    vec2 uv;
} VertexIn;

out vec4 Target0;

void main() {
    float alpha = max(1-dot(VertexIn.uv, VertexIn.uv), 0);
    Target0 = vec4(VertexIn.color.xyz, VertexIn.color.w*alpha);
}";
