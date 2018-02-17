// Large amounts of the following shaders are copied from the following awesome gfx example:
//     https://github.com/gfx-rs/gfx/blob/master/examples/support/particle/shader/

pub const GEOMETRY: &[u8] = br"#version 150 core
layout (points) in;
layout (triangle_strip, max_vertices=4) out;

in VertexData {
    float spawn_time;
    float life_time;
    vec4 color;
    vec2 size;
} VertexIn[];

out VertexData {
    vec4 color;
    vec2 uv;
} VertexOut;

uniform Globals {
    mat4 u_Transform;
    float u_Time;
};

void main() {
    if (u_Time - VertexIn[0].spawn_time < VertexIn[0].life_time) {
        vec2 size = VertexIn[0].size;

        gl_Position = u_Transform * (gl_in[0].gl_Position + vec4(-size.x, -size.y, 0, 0));
        VertexOut.color = VertexIn[0].color;
        VertexOut.uv = vec2(-1, -1);
        EmitVertex();

        gl_Position = u_Transform * (gl_in[0].gl_Position + vec4(size.x, -size.y, 0, 0));
        VertexOut.color = VertexIn[0].color;
        VertexOut.uv = vec2(1, -1);
        EmitVertex();

        gl_Position = u_Transform * (gl_in[0].gl_Position + vec4(-size.x, size.y, 0, 0));
        VertexOut.color = VertexIn[0].color;
        VertexOut.uv = vec2(-1, 1);
        EmitVertex();

        gl_Position = u_Transform * (gl_in[0].gl_Position + vec4(size.x, size.y, 0, 0));
        VertexOut.color = VertexIn[0].color;
        VertexOut.uv = vec2(1, 1);
        EmitVertex();
    }
}";

pub const VERTEX: &[u8] = br"
#version 150 core

in float a_SpawnTime;
in float a_LifeTime;
in vec2 a_Pos;
in vec2 a_Vel;
in float a_Angle;
in float a_AngularVel;
in vec3 a_Color;
in vec2 a_Size;

out VertexData {
    float spawn_time;
    float life_time;
    vec4 color;
    vec2 size;
} VertexOut;

uniform Globals {
    mat4 u_Transform;
    float u_Time;
};

void main() {
    float percent = (u_Time - a_SpawnTime) / a_LifeTime;

    gl_Position = vec4(a_Pos + a_Vel * percent, 0, 1);

    VertexOut.color = vec4(a_Color, 1.0 - percent);
    VertexOut.spawn_time = a_SpawnTime;
    VertexOut.life_time = a_LifeTime;
    VertexOut.size = a_Size;
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
