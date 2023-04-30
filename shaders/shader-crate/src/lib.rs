//! Shader entry points.
#![no_std]
#![feature(lang_items)]
use renderling_shader::scene;
use spirv_std::{glam, image::Image2d, spirv, Sampler};

#[spirv(vertex)]
pub fn main_vertex_scene(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &scene::GpuConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] vertices: &[scene::GpuVertex],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities: &[scene::GpuEntity],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] materials: &[scene::GpuMaterial],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 2)] textures: &[scene::GpuTexture],

    //// which entity are we drawing
    #[spirv(instance_index)] instance_id: u32,
    // which vertex on that entity
    #[spirv(vertex_index)] vertex_id: u32,

    #[spirv(flat)] out_material_config: &mut u32,
    #[spirv(flat)] out_material_lighting_model: &mut scene::LightingModel,
    out_color: &mut glam::Vec4,
    out_uv0: &mut glam::Vec2,
    out_factor0: &mut glam::Vec4,
    out_uv1: &mut glam::Vec2,
    out_factor1: &mut glam::Vec4,
    out_norm: &mut glam::Vec3,
    out_pos: &mut glam::Vec3,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    scene::main_vertex_scene(
        instance_id,
        vertex_id,
        constants,
        vertices,
        entities,
        materials,
        textures,
        out_material_config,
        out_material_lighting_model,
        out_color,
        out_uv0,
        out_factor0,
        out_uv1,
        out_factor1,
        out_norm,
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

    #[spirv(flat)] in_material_config: u32,
    #[spirv(flat)] in_material_lighting_model: scene::LightingModel,
    in_color: glam::Vec4,
    in_uv0: glam::Vec2,
    in_factor0: glam::Vec4,
    in_uv1: glam::Vec2,
    in_factor1: glam::Vec4,
    in_norm: glam::Vec3,
    in_pos: glam::Vec3,

    output: &mut glam::Vec4,
) {
    scene::main_fragment_scene(
        atlas,
        sampler,
        constants,
        lights,
        in_material_config,
        in_material_lighting_model,
        in_color,
        in_uv0,
        in_factor0,
        in_uv1,
        in_factor1,
        in_norm,
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

///// Just a test for atomics in Naga.
//#[spirv(compute(threads(32)))]
//pub fn compute_atomics(
//    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] data: &mut [u32],
//    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sum: &mut u32,
//    #[spirv(global_invocation_id)] global_id: glam::UVec3,
//) {
//    let index = global_id.x as usize;
//    if index > data.len() {
//        return;
//    }
//
//    let n =
//        unsafe {
//            spirv_std::arch::atomic_load::<
//                    u32,
//                { spirv_std::memory::Scope::Device as u32 },
//                { spirv_std::memory::Semantics::NONE.bits() as u32 },
//                >(&data[index])
//        };
//    *sum = n;
//    //unsafe {
//    //    spirv_std::arch::atomic_i_add::<
//    //        u32,
//    //        { spirv_std::memory::Scope::Device as u32 },
//    //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
//    //    >(sum, n)
//    //};
//}
