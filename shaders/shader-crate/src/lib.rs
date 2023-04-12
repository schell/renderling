//! Shader entry points.
#![no_std]
#![feature(lang_items)]
use renderling_shader::scene;
use spirv_std::{glam, image::Image2d, spirv, Sampler};

mod ui;
pub use ui::*;

mod pbr;
pub use pbr::*;

#[spirv(vertex)]
pub fn main_vertex_scene(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] camera: &scene::GpuCamera,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] vertices: &[scene::GpuVertex],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] transforms: &[glam::Mat4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] entities: &[scene::GpuEntity],

    //// which entity are we drawing
    #[spirv(instance_index)] instance_id: u32,
    // which vertex on that entity
    #[spirv(vertex_index)] vertex_id: u32,

    #[spirv(flat)] out_lighting_model: &mut u32,
    out_color: &mut glam::Vec4,
    #[spirv(flat)] out_tex_ids: &mut glam::UVec2,
    out_uv0: &mut glam::Vec2,
    out_uv1: &mut glam::Vec2,
    out_norm: &mut glam::Vec3,
    out_pos: &mut glam::Vec3,
    #[spirv(position)] gl_pos: &mut glam::Vec4
) {
    scene::main_vertex_scene(
        instance_id,
        vertex_id,
        camera,
        vertices,
        transforms,
        entities,
        out_lighting_model,
        out_color,
        out_tex_ids,
        out_uv0,
        out_uv1,
        out_norm,
        out_pos,
        gl_pos
    )
}

#[spirv(fragment)]
pub fn main_fragment_scene(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] camera: &scene::GpuCamera,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[scene::GpuLight],

    #[spirv(flat)] in_lighting_model: u32,
    in_color: glam::Vec4,
    #[spirv(flat)] in_tex_ids: glam::UVec2,
    in_uv0: glam::Vec2,
    in_uv1: glam::Vec2,
    in_norm: glam::Vec3,
    in_pos: glam::Vec3,

    output: &mut glam::Vec4,
) {
    scene::main_fragment_scene(
        atlas,
        sampler,
        camera,
        lights,
        in_lighting_model,
        in_color,
        in_tex_ids,
        in_uv0,
        in_uv1,
        in_norm,
        in_pos,
        output,
    )
}

#[spirv(compute(threads(32)))]
pub fn compute_cull_entities(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] entities: &[scene::GpuEntity],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] draws: &mut [scene::DrawIndirect],

    #[spirv(global_invocation_id)] global_id: glam::UVec3,
) {
    scene::compute_cull_entities(
        entities, draws, global_id,
    )
}
