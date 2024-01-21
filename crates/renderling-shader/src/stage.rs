//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use crabslab::{Array, Id, Slab, SlabItem, ID_NONE};
use glam::{UVec2, UVec3, Vec2, Vec3, Vec4};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

use crate::{
    debug::*,
    math::IsVector,
    pbr::{self, Material},
    texture::GpuTexture,
    IsSampler, Sample2d,
};

pub mod light;

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
    // Indices that point to this vertex's joints by indexing into an array of Id<GpuEntity>
    // provided by the GpuEntity that is using this vertex
    pub joints: [u32; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y,
            joints: [0; 4],
            weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn with_position(mut self, p: impl Into<Vec3>) -> Self {
        self.position = p.into().extend(0.0);
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_uv0(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.x = uv.x;
        self.uv.y = uv.y;
        self
    }

    pub fn with_uv1(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.z = uv.x;
        self.uv.w = uv.y;
        self
    }

    pub fn with_normal(mut self, n: impl Into<Vec3>) -> Self {
        self.normal = n.into().extend(0.0);
        self
    }

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
    }

    ///// Return the matrix needed to bring vertices into the coordinate space of
    ///// the joint node.
    //pub fn get_joint_matrix(
    //    &self,
    //    i: usize,
    //    joint_ids: &[Id<GpuEntity>; 32],
    //    entities: &[GpuEntity],
    //) -> Mat4 {
    //    if i >= self.joints.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_index = self.joints[i];
    //    let joint_id = if joint_index as usize >= joint_ids.len() {
    //        Id::NONE
    //    } else {
    //        joint_ids[joint_index as usize]
    //    };
    //    if joint_id.is_none() {
    //        return Mat4::IDENTITY;
    //    }
    //    let entity_index = joint_id.index();
    //    if entity_index >= entities.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_entity = &entities[entity_index];
    //    let (t, r, s) = joint_entity.get_world_transform(entities);
    //    let trs = Mat4::from_scale_rotation_translation(s, r, t);
    //    trs * joint_entity.inverse_bind_matrix
    //}

    ///// Return the result of adding all joint matrices multiplied by their
    ///// weights for the given vertex.
    //// See the [khronos gltf viewer reference](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/47a191931461a6f2e14de48d6da0f0eb6ec2d147/source/Renderer/shaders/animation.glsl#L47)
    //pub fn get_skin_matrix(&self, joint_ids: &[Id<GpuEntity>; 32], entities:
    // &[GpuEntity]) -> Mat4 {    let mut mat = Mat4::ZERO;
    //    for i in 0..self.joints.len() {
    //        mat += self.weights[i] * self.get_joint_matrix(i, joint_ids,
    // entities);    }
    //    if mat == Mat4::ZERO {
    //        return Mat4::IDENTITY;
    //    }
    //    mat
    //}

    pub fn generate_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        let ab = a - b;
        let ac = a - c;
        ab.cross(ac).normalize()
    }

    pub fn generate_tangent(a: Vec3, a_uv: Vec2, b: Vec3, b_uv: Vec2, c: Vec3, c_uv: Vec2) -> Vec4 {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac);
        let d_uv1 = b_uv - a_uv;
        let d_uv2 = c_uv - a_uv;
        let denom = d_uv1.x * d_uv2.y - d_uv2.x * d_uv1.y;
        let denom_sign = if denom >= 0.0 { 1.0 } else { -1.0 };
        let denom = denom.abs().max(f32::EPSILON) * denom_sign;
        let f = 1.0 / denom;
        let s = f * Vec3::new(
            d_uv2.y * ab.x - d_uv1.y * ac.x,
            d_uv2.y * ab.y - d_uv1.y * ac.y,
            d_uv2.y * ab.z - d_uv1.y * ac.z,
        );
        let t = f * Vec3::new(
            d_uv1.x * ac.x - d_uv2.x * ab.x,
            d_uv1.x * ac.y - d_uv2.x * ab.y,
            d_uv1.x * ac.z - d_uv2.x * ab.z,
        );
        let n_cross_t_dot_s_sign = if n.cross(t).dot(s) >= 0.0 { 1.0 } else { -1.0 };
        (s - s.dot(n) * n)
            .alt_norm_or_zero()
            .extend(n_cross_t_dot_s_sign)
    }
}

pub fn texture_color<T: Sample2d<Sampler = S>, S: IsSampler>(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &T,
    sampler: &S,
    atlas_size: UVec2,
    slab: &[u32],
) -> Vec4 {
    let texture = slab.read(texture_id);
    let uv = texture.uv(uv, atlas_size);
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
}

/// Holds important info about the stage.
///
/// This should be the first struct in the stage's slab.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct StageLegend {
    pub atlas_size: UVec2,
    pub debug_mode: DebugMode,
    pub has_skybox: bool,
    pub has_lighting: bool,
    pub light_array: Array<light::Light>,
}

impl Default for StageLegend {
    fn default() -> Self {
        Self {
            atlas_size: Default::default(),
            debug_mode: Default::default(),
            has_skybox: Default::default(),
            has_lighting: true,
            light_array: Default::default(),
        }
    }
}

/// Pointer to a renderable unit.
#[derive(Default, SlabItem)]
pub enum Rendering {
    #[default]
    None,
    /// Render the scene using the gltf vertex and fragment shaders.
    Gltf(Id<crate::gltf::GltfRendering>),
    /// Render the scene using the sdf vertex and fragment shaders.
    Sdf(Id<crate::sdf::Sdf>),
}

/// Uber vertex shader.
///
/// This reads the "instance" by index and proxies to a specific vertex shader.
#[spirv(vertex)]
pub fn vertex(
    // Points at a `Rendering`
    #[spirv(instance_index)] instance_index: u32,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] out_instance_index: &mut u32,
    #[spirv(flat)] out_camera: &mut u32,
    #[spirv(flat)] out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in local space
    local_pos: &mut Vec3,
    // position of the vertex/fragment in world space
    world_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    *out_instance_index = instance_index;
    let rendering = slab.read(Id::<Rendering>::new(instance_index));
    match rendering {
        Rendering::None => {}
        Rendering::Gltf(unit_id) => {
            crate::gltf::vertex(
                unit_id,
                vertex_index,
                slab,
                out_camera,
                out_material,
                out_color,
                out_uv0,
                out_uv1,
                out_norm,
                out_tangent,
                out_bitangent,
                world_pos,
                clip_pos,
            );
        }
        Rendering::Sdf(_sdf_id) => {
            *local_pos = Vec3::ZERO;
            //crate::sdf::vertex(
            //    sdf_id,
            //    vertex_index,
            //    slab,
            //    out_camera,
            //    out_material,
            //    out_color,
            //    out_uv0,
            //    out_uv1,
            //    out_norm,
            //    out_tangent,
            //    out_bitangent,
            //    local_pos,
            //    world_pos,
            //    clip_pos,
            //);
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
/// Uber fragment shader.
///
/// This reads the "instance" by index and proxies to a specific vertex shader.
pub fn fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] in_instance_index: u32,
    #[spirv(flat)] in_camera: u32,
    #[spirv(flat)] in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    local_pos: Vec3,
    world_pos: Vec3,

    output: &mut Vec4,
) {
    let rendering = slab.read(Id::<Rendering>::new(in_instance_index));
    match rendering {
        Rendering::None => {}
        Rendering::Gltf(_) => {
            crate::pbr::fragment(
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefiltered,
                prefiltered_sampler,
                brdf,
                brdf_sampler,
                slab,
                in_camera,
                in_material,
                in_color,
                in_uv0,
                in_uv1,
                in_norm,
                in_tangent,
                in_bitangent,
                world_pos,
                output,
            );
        }
        Rendering::Sdf(_sdf_id) => {
            *output = local_pos.extend(1.0);
            //crate::sdf::fragment(
            //    sdf_id,
            //    atlas,
            //    atlas_sampler,
            //    irradiance,
            //    irradiance_sampler,
            //    prefiltered,
            //    prefiltered_sampler,
            //    brdf,
            //    brdf_sampler,
            //    slab,
            //    in_camera,
            //    in_material,
            //    in_color,
            //    in_uv0,
            //    in_uv1,
            //    in_norm,
            //    in_tangent,
            //    in_bitangent,
            //    local_pos,
            //    world_pos,
            //    output,
            //);
        }
    }
}

/// Returns the `StageLegend` from the stage's slab.
///
/// The `StageLegend` should be the first struct in the slab, always.
pub fn get_stage_legend(slab: &[u32]) -> StageLegend {
    slab.read(Id::new(0))
}

/// Returns the `Material` from the stage's slab.
pub fn get_material(material_index: u32, has_lighting: bool, slab: &[u32]) -> pbr::Material {
    if material_index == ID_NONE {
        // without an explicit material (or if the entire render has no lighting)
        // the entity will not participate in any lighting calculations
        pbr::Material {
            has_lighting: false,
            ..Default::default()
        }
    } else {
        let mut material = slab.read(Id::<Material>::new(material_index));
        if !has_lighting {
            material.has_lighting = false;
        }
        material
    }
}

//#[spirv(compute(threads(32)))]
///// Compute the draw calls for this frame.
/////
///// This should be called with `groupcount = (entities.len() / threads) + 1`.
//pub fn compute_cull_entities(
//    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities:
// &[GpuEntity],    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)]
// draws: &mut [DrawIndirect],    #[spirv(global_invocation_id)] global_id:
// UVec3,
//) {
//    let i = global_id.x as usize;
//
//    if i > entities.len() {
//        return;
//    }
//
//    // when the vertex count and/or instance count is 0, it effectively
// filters    // the draw call
//    let mut call = DrawIndirect {
//        vertex_count: 0,
//        instance_count: 0,
//        base_vertex: 0,
//        base_instance: i as u32,
//    };
//    let entity = &entities[i];
//    let is_visible = entity.visible != 0;
//    if entity.is_alive() && is_visible {
//        //// once naga supports atomics we can use this to compact the array
//        // let index = unsafe {
//        //    spirv_std::arch::atomic_i_increment::<
//        //        u32,
//        //        { spirv_std::memory::Scope::Device as u32 },
//        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
//        //    >(count)
//        //};
//        call.instance_count = 1;
//        call.base_vertex = entity.mesh_first_vertex;
//        call.vertex_count = entity.mesh_vertex_count;
//    }
//    draws[i] = call;
//}

#[spirv(compute(threads(32)))]
/// A shader to ensure that we can extract i8 and i16 values from a storage
/// buffer.
pub fn test_i8_i16_extraction(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let index = global_id.x as usize;
    let (value, _, _) = crate::bits::extract_i8(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
    let (value, _, _) = crate::bits::extract_i16(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
}
