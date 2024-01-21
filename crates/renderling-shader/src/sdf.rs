//! SDF shaders.
//!
//! This module puts together the `renderling-shader-sdf` crate and the
//! physically-based fragment shader.
use crabslab::{Id, Slab, SlabItem};
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

use crate::{
    math::IsVector, pbr::Material, stage::Vertex, Camera, IsSampler, Sample2d, SampleCube,
    Transform,
};

pub use renderling_shader_sdf::*;

//pub fn vertex(
//    sdf_id: Id<Sdf>,
//    // Which vertex within the render unit are we rendering
//    vertex_index: u32,
//    slab: &[u32],
//    out_camera: &mut u32,
//    out_material: &mut u32,
//    out_color: &mut Vec4,
//    out_uv0: &mut Vec2,
//    out_uv1: &mut Vec2,
//    out_norm: &mut Vec3,
//    out_tangent: &mut Vec3,
//    out_bitangent: &mut Vec3,
//    local_pos: &mut Vec3,
//    world_pos: &mut Vec3,
//    clip_pos: &mut Vec4,
//) {
//    let Sdf {
//        shape,
//        transform,
//        material: material_id,
//        camera: camera_id,
//    } = slab.read(sdf_id);
//
//    let vertex = shape.get_vertex(vertex_index as usize, slab);
//    *local_pos = vertex.position.xyz();
//    let model_matrix = Mat4::from_scale_rotation_translation(
//        transform.scale,
//        transform.rotation,
//        transform.translation,
//    );
//    let scale2 = transform.scale * transform.scale;
//    let normal = vertex.normal.xyz().alt_norm_or_zero();
//    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
//    let normal_w: Vec3 = (model_matrix * (normal / scale2).extend(0.0))
//        .xyz()
//        .alt_norm_or_zero();
//    let tangent_w: Vec3 = (model_matrix * tangent.extend(0.0))
//        .xyz()
//        .alt_norm_or_zero();
//    let bitangent_w = normal_w.cross(tangent_w) * if vertex.tangent.w >= 0.0 { 1.0 } else { -1.0 };
//    *out_tangent = tangent_w;
//    *out_bitangent = bitangent_w;
//    *out_norm = normal_w;
//    let view_pos = model_matrix * vertex.position.xyz().extend(1.0);
//    *world_pos = view_pos.xyz();
//    let camera = slab.read(camera_id);
//    *out_camera = camera_id.into();
//    *clip_pos = camera.projection * camera.view * view_pos;
//
//    *out_material = material_id.into();
//    *out_color = Vec4::ONE;
//    *out_uv0 = Vec2::ZERO;
//    *out_uv1 = Vec2::ZERO;
//}
//
//pub fn fragment<T, C, S>(
//    sdf_id: Id<Sdf>,
//
//    atlas: &T,
//    atlas_sampler: &S,
//    irradiance: &C,
//    irradiance_sampler: &S,
//    prefiltered: &C,
//    prefiltered_sampler: &S,
//    brdf: &T,
//    brdf_sampler: &S,
//
//    slab: &[u32],
//
//    in_camera: u32,
//    in_material: u32,
//    in_color: Vec4,
//    in_uv0: Vec2,
//    in_uv1: Vec2,
//    in_norm: Vec3,
//    in_tangent: Vec3,
//    in_bitangent: Vec3,
//    local_pos: Vec3,
//    world_pos: Vec3,
//
//    output: &mut Vec4,
//) where
//    T: Sample2d<Sampler = S>,
//    C: SampleCube<Sampler = S>,
//    S: IsSampler,
//{
//    let sdf = slab.read(Id::<Sdf>::from(sdf_id));
//    let distance = sdf.distance(local_pos.xy(), slab);
//    let change = spirv_std::arch::fwidth(distance) * 0.5;
//    let aa_cutoff = smoothstep(distance, change, -change);
//    let color = Vec4::new(aa_cutoff, aa_cutoff, aa_cutoff, 1.0);
//    crate::pbr::fragment(
//        atlas,
//        atlas_sampler,
//        irradiance,
//        irradiance_sampler,
//        prefiltered,
//        prefiltered_sampler,
//        brdf,
//        brdf_sampler,
//        slab,
//        in_camera,
//        in_material,
//        in_color * color,
//        in_uv0,
//        in_uv1,
//        in_norm,
//        in_tangent,
//        in_bitangent,
//        world_pos,
//        output,
//    );
//}
