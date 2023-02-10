//! PBR shader.
#![no_std]
#![feature(lang_items)]

use renderling_shader::{
    pbr::{DirectionalLights, PointLights, SpotLights},
    Camera,
};
use spirv_std::spirv;
use spirv_std::{glam::*, image::Image2d, Sampler};

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &Camera,

    in_pos: Vec3,
    in_uv: Vec2,
    in_norm: Vec3,

    in_model_matrix_0: Vec4,
    in_model_matrix_1: Vec4,
    in_model_matrix_2: Vec4,
    in_model_matrix_3: Vec4,

    in_norm_matrix_0: Vec3,
    in_norm_matrix_1: Vec3,
    in_norm_matrix_2: Vec3,

    out_pos: &mut Vec3,
    out_uv: &mut Vec2,
    out_norm: &mut Vec3,

    #[spirv(position)] gl_pos: &mut Vec4,
) {
    renderling_shader::pbr::main_vertex(
        camera,
        in_pos,
        in_uv,
        in_norm,
        in_model_matrix_0,
        in_model_matrix_1,
        in_model_matrix_2,
        in_model_matrix_3,
        in_norm_matrix_0,
        in_norm_matrix_1,
        in_norm_matrix_2,
        out_pos,
        out_uv,
        out_norm,
        gl_pos,
    );
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &Camera,

    #[spirv(descriptor_set = 1, binding = 0)] diffuse_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] diffuse_sampler: &Sampler,
    #[spirv(descriptor_set = 1, binding = 2)] specular_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 3)] specular_sampler: &Sampler,
    #[spirv(uniform, descriptor_set = 1, binding = 4)] material_shininess: &Vec4,

    #[spirv(uniform, descriptor_set = 2, binding = 0)] point_lights: &PointLights,
    #[spirv(uniform, descriptor_set = 2, binding = 1)] spot_lights: &SpotLights,
    #[spirv(uniform, descriptor_set = 2, binding = 2)] directional_lights: &DirectionalLights,

    in_pos: Vec3,
    in_uv: Vec2,
    in_norm: Vec3,

    frag_color: &mut Vec4,
) {
    renderling_shader::pbr::main_fragment(
        camera,
        diffuse_texture,
        diffuse_sampler,
        specular_texture,
        specular_sampler,
        material_shininess,
        point_lights,
        spot_lights,
        directional_lights,
        in_pos,
        in_uv,
        in_norm,
        frag_color,
    );
}
