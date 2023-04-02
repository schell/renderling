//! Shader wrapper for user interface shaders.
use renderling_shader::{CameraRaw, ui::ShaderColorBlend};
use spirv_std::spirv;
use spirv_std::{glam::*, image::Image2d, Sampler};

#[spirv(vertex)]
pub fn vertex_ui(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &CameraRaw,

    in_pos: Vec3,
    in_color: Vec4,
    in_uv: Vec2,

    in_model_matrix_0: Vec4,
    in_model_matrix_1: Vec4,
    in_model_matrix_2: Vec4,
    in_model_matrix_3: Vec4,

    out_color: &mut Vec4,
    out_uv: &mut Vec2,

    #[spirv(position)] gl_pos: &mut Vec4,
) {
    renderling_shader::ui::main_vertex(
        camera,
        in_pos,
        in_color,
        in_uv,
        in_model_matrix_0,
        in_model_matrix_1,
        in_model_matrix_2,
        in_model_matrix_3,
        out_color,
        out_uv,
        gl_pos,
    );
}

#[spirv(fragment)]
pub fn fragment_ui(
    #[spirv(uniform, descriptor_set = 1, binding = 0)] blend: &ShaderColorBlend,
    #[spirv(descriptor_set = 1, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 2)] sampler: &Sampler,

    in_color: Vec4,
    in_uv: Vec2,

    frag_color: &mut Vec4,
) {
    renderling_shader::ui::main_fragment(blend, texture, sampler, in_color, in_uv, frag_color);
}
