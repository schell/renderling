//! Shader entry points.
#![no_std]
#![feature(lang_items)]
use renderling_shader::{scene, skybox, tonemapping, ui};
use spirv_std::{
    glam,
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[spirv(vertex)]
pub fn ui_vertex(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &ui::UiConstants,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] params: &ui::UiDrawParams,

    in_pos: glam::Vec2,
    in_uv: glam::Vec2,
    in_color: glam::Vec4,

    #[spirv(flat)] out_mode: &mut u32,
    out_color: &mut glam::Vec4,
    out_uv: &mut glam::Vec2,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    ui::vertex(
        constants, params, in_pos, in_uv, in_color, out_mode, out_color, out_uv, gl_pos,
    )
}

#[spirv(fragment)]
pub fn ui_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,

    #[spirv(flat)] in_mode: u32,
    in_color: glam::Vec4,
    in_uv: glam::Vec2,

    output: &mut glam::Vec4,
) {
    ui::fragment(texture, sampler, in_mode, in_color, in_uv, output)
}

#[spirv(vertex)]
pub fn main_vertex_scene(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &scene::GpuConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] vertices: &[scene::GpuVertex],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities: &[scene::GpuEntity],

    //// which entity are we drawing
    #[spirv(instance_index)] instance_id: u32,
    // which vertex on that entity
    #[spirv(vertex_index)] vertex_id: u32,

    #[spirv(flat)] out_material: &mut u32,
    out_color: &mut glam::Vec4,
    out_uv0: &mut glam::Vec2,
    out_uv1: &mut glam::Vec2,
    out_norm: &mut glam::Vec3,
    out_tangent: &mut glam::Vec3,
    out_bitangent: &mut glam::Vec3,
    out_pos: &mut glam::Vec3,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    scene::main_vertex_scene(
        instance_id,
        vertex_id,
        constants,
        vertices,
        entities,
        out_material,
        out_color,
        out_uv0,
        out_uv1,
        out_norm,
        out_tangent,
        out_bitangent,
        out_pos,
        gl_pos,
    )
}

#[spirv(fragment)]
pub fn main_fragment_scene(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &scene::GpuConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] lights: &[scene::GpuLight],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] materials: &[scene::GpuMaterial],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 2)] textures: &[scene::GpuTexture],

    #[spirv(descriptor_set = 0, binding = 5)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 6)] irradiance_sampler: &Sampler,

    //// which entity are we drawing
    #[spirv(flat)] in_material: u32,
    in_color: glam::Vec4,
    in_uv0: glam::Vec2,
    in_uv1: glam::Vec2,
    in_norm: glam::Vec3,
    in_tangent: glam::Vec3,
    in_bitangent: glam::Vec3,
    in_pos: glam::Vec3,

    output: &mut glam::Vec4,
) {
    scene::main_fragment_scene(
        atlas,
        sampler,
        irradiance,
        irradiance_sampler,
        constants,
        lights,
        materials,
        textures,
        in_material,
        in_color,
        in_uv0,
        in_uv1,
        in_norm,
        in_tangent,
        in_bitangent,
        in_pos,
        output,
    )
}

#[spirv(compute(threads(32)))]
pub fn compute_cull_entities(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities: &[scene::GpuEntity],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] draws: &mut [scene::DrawIndirect],

    #[spirv(global_invocation_id)] global_id: glam::UVec3,
) {
    scene::compute_cull_entities(entities, draws, global_id)
}

#[spirv(vertex)]
pub fn vertex_tonemapping(
    #[spirv(vertex_index)] vertex_id: u32,
    out_uv: &mut glam::Vec2,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    tonemapping::vertex(vertex_id, out_uv, gl_pos)
}

#[spirv(fragment)]
pub fn fragment_tonemapping(
    #[spirv(descriptor_set = 0, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] constants: &tonemapping::TonemapConstants,
    in_uv: glam::Vec2,

    output: &mut glam::Vec4,
) {
    tonemapping::fragment(texture, sampler, constants, in_uv, output)
}

#[spirv(vertex)]
pub fn vertex_skybox(
    #[spirv(vertex_index)] vertex_id: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &scene::GpuConstants,
    local_pos: &mut glam::Vec3,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    skybox::vertex(vertex_id, constants, local_pos, gl_pos)
}

#[spirv(vertex)]
pub fn vertex_position_passthru(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &scene::GpuConstants,
    in_pos: glam::Vec3,
    local_pos: &mut glam::Vec3,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    skybox::vertex_position_passthru(constants, in_pos, local_pos, gl_pos)
}

#[spirv(fragment)]
pub fn fragment_equirectangular(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_local_pos: glam::Vec3,
    out_color: &mut glam::Vec4,
) {
    skybox::fragment_equirectangular(texture, sampler, in_local_pos, out_color)
}

#[spirv(fragment)]
pub fn fragment_cubemap(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_local_pos: glam::Vec3,
    out_color: &mut glam::Vec4,
) {
    skybox::fragment_cubemap(texture, sampler, in_local_pos, out_color)
}

//#[spirv(fragment)]
//pub fn fragment_convolve_diffuse_irradiance(
//    #[spirv(descriptor_set = 0, binding = 1)] texture: &Cubemap,
//    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
//    in_local_pos: glam::Vec3,
//    out_color: &mut glam::Vec4,
//) {
//    convolution::fragment_convolve_diffuse_irradiance(texture, sampler, in_local_pos, out_color)
//}
