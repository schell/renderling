#![no_std]
#![feature(lang_items)]

use renderling_shader::ui::{BlendStyle, Camera};
use spirv_std::spirv;
use spirv_std::{
    Sampler,
    glam::{mat4, vec4, Mat4, UVec4, Vec2, Vec3, Vec4},
    image::Image2d,
};

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &Camera,

    in_pos: Vec3,
    in_color: Vec4,
    in_uv: Vec2,

    in_model_matrix_0: Vec4,
    in_model_matrix_1: Vec4,
    in_model_matrix_2: Vec4,
    in_model_matrix_3: Vec4,

    out_color: &mut Vec4,
    out_uv: &mut Vec2,

    #[spirv(position)] out_pos: &mut Vec4,
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
        out_pos,
    );
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(uniform, descriptor_set = 1, binding = 0)] blend: &UVec4,
    #[spirv(descriptor_set = 1, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 2)] sampler: &Sampler,

    in_color: Vec4,
    in_uv: Vec2,

    output: &mut Vec4,
) {
    renderling_shader::ui::main_fragment(blend, texture, sampler, in_color, in_uv, output);
}
