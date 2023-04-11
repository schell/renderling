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
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] vertices: &[scene::GpuVertex],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] transforms: &[glam::Mat4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] entities: &[scene::GpuEntity],

    //// which entity are we drawing
    #[spirv(instance_index)] instance_id: u32,
    // which vertex on that entity
    #[spirv(vertex_index)] vertex_id: u32,

    out_color: &mut glam::Vec4,
    out_uv: &mut glam::Vec2,
    #[spirv(position)] out_pos: &mut glam::Vec4,
) {
    scene::main_vertex_scene(
        instance_id,
        vertex_id,
        camera,
        vertices,
        transforms,
        entities,
        out_color,
        out_uv,
        out_pos,
    )
}

#[spirv(fragment)]
pub fn main_fragment_scene(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,

    in_color: glam::Vec4,
    in_uv: glam::Vec2,

    output: &mut glam::Vec4,
) {
    scene::main_fragment_scene(
        atlas,
        sampler,
        in_color,
        in_uv,
        output,
    )
}

#[spirv(compute(threads(32)))]
pub fn compute_cull_entities(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] meshes: &[scene::GpuMeshlet],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] entities: &[scene::GpuEntity],

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] draws: &mut [scene::DrawIndirect],

    #[spirv(global_invocation_id)] global_id: glam::UVec3,
) {
    scene::compute_cull_entities(
        meshes, entities, draws, global_id,
    )
}
