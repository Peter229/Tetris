#version 450

layout(location = 0) in vec3 v_tex_coords;
layout(location = 1) in vec4 v_colour;

layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2DArray t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;

void main() {
    f_color = texture(sampler2DArray(t_diffuse, s_diffuse), v_tex_coords) * v_colour;
    if (f_color.a < 0.1) {
        discard;
    }
}