//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage objects for rendering.
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{Mat4, UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

use crate::{
    camera::Camera,
    math::IsVector,
    pbr::{Material, PbrConfig},
    transform::Transform,
};

#[allow(unused_imports)]
use spirv_std::num_traits::Float;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

#[cfg(all(feature = "gltf", not(target_arch = "spirv")))]
mod gltf_support;
#[cfg(all(feature = "gltf", not(target_arch = "spirv")))]
pub use gltf_support::*;

/// A vertex skin.
///
/// For more info on vertex skinning, see
/// <https://github.khronos.org/glTF-Tutorials/gltfTutorial/gltfTutorial_019_SimpleSkin.html>
#[derive(Clone, Copy, Default, SlabItem)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct Skin {
    // Ids of the skeleton nodes' global transforms used as joints in this skin.
    pub joints: Array<Id<Transform>>,
    // Contains the 4x4 inverse-bind matrices.
    //
    // When is none, each matrix is assumed to be the 4x4 identity matrix
    // which implies that the inverse-bind matrices were pre-applied.
    pub inverse_bind_matrices: Array<Mat4>,
}

impl Skin {
    pub fn get_transform(&self, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let mut mat = Mat4::ZERO;
        for i in 0..vertex.joints.len() {
            let joint_weight = vertex.weights[i];
            if joint_weight == 0.0 {
                continue;
            }

            let inverse_bind_matrix = slab.read(self.inverse_bind_matrices.at(i));

            let joint_index = vertex.joints[i] as usize;
            let joint_id = slab.read(self.joints.at(joint_index));
            let joint_matrix = Mat4::from(slab.read(joint_id)) * inverse_bind_matrix;

            mat += joint_matrix * joint_weight;
        }
        if mat == Mat4::ZERO {
            Mat4::IDENTITY
        } else {
            mat
        }
    }
}

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
    pub uv0: Vec2,
    pub uv1: Vec2,
    pub normal: Vec3,
    pub tangent: Vec4,
    // Indices that point to this vertex's 'joint' transforms.
    pub joints: [u32; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::ONE,
            uv0: Vec2::ZERO,
            uv1: Vec2::ZERO,
            normal: Vec3::Z,
            tangent: Vec4::Y,
            joints: [0; 4],
            weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn with_position(mut self, p: impl Into<Vec3>) -> Self {
        self.position = p.into();
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_uv0(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv0 = uv.into();
        self
    }

    pub fn with_uv1(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv1 = uv.into();
        self
    }

    pub fn with_normal(mut self, n: impl Into<Vec3>) -> Self {
        self.normal = n.into();
        self
    }

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
    }

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

/// A draw call used to render some geometry.
///
/// ## Note
/// The default implentation returns a `Renderlet` with `pbr_config` set to
/// `Id::new(0)`. This corresponds to the `PbrConfig` that is maintained by
/// the [`Stage`].
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Renderlet {
    pub visible: bool,
    pub vertices_array: Array<Vertex>,
    pub indices_array: Array<u32>,
    pub camera_id: Id<Camera>,
    pub transform_id: Id<Transform>,
    pub material_id: Id<Material>,
    pub skin_id: Id<Skin>,
    pub pbr_config_id: Id<PbrConfig>,
    pub debug_index: u32,
}

impl Default for Renderlet {
    fn default() -> Self {
        Renderlet {
            visible: true,
            vertices_array: Array::default(),
            indices_array: Array::default(),
            camera_id: Id::NONE,
            transform_id: Id::NONE,
            material_id: Id::NONE,
            skin_id: Id::NONE,
            pbr_config_id: Id::new(0),
            debug_index: 0,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct RenderletVertexLog {
    pub renderlet_id: Id<Renderlet>,
    pub debug_index: u32,
    pub vertex_index: u32,
    pub started: bool,
    pub completed: bool,
}

impl RenderletVertexLog {
    pub fn new(renderlet_id: Id<Renderlet>, debug_index: u32, vertex_index: u32) -> Self {
        let mut log = Self::default();
        log.renderlet_id = renderlet_id;
        log.debug_index = debug_index;
        log.vertex_index = vertex_index;
        log
    }

    pub fn write(&self, debug: &mut [u32]) {
        let log_id: Id<RenderletVertexLog> =
            Id::new(self.debug_index + self.vertex_index * RenderletVertexLog::SLAB_SIZE as u32);
        debug.write(log_id, self);
    }
}

#[cfg(feature = "renderlet_vertex")]
/// Renderlet vertex shader.
#[spirv(vertex)]
#[allow(dead_code)]
pub fn renderlet_vertex(
    // Points at a `Renderlet`
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the renderlet are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] debug: &mut [u32],

    #[spirv(flat)] out_camera: &mut Id<Camera>,
    #[spirv(flat)] out_material: &mut Id<Material>,
    #[spirv(flat)] out_pbr_config: &mut Id<PbrConfig>,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    out_world_pos: &mut Vec3,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    use crate::math::IsMatrix;

    let renderlet = slab.read_unchecked(renderlet_id);

    let mut vertex_log = RenderletVertexLog::new(renderlet_id, renderlet.debug_index, vertex_index);
    vertex_log.started = true;
    vertex_log.write(debug);

    *out_camera = renderlet.camera_id;
    *out_material = renderlet.material_id;
    *out_pbr_config = renderlet.pbr_config_id;

    let index = if renderlet.indices_array.is_null() {
        vertex_index as usize
    } else {
        slab.read(renderlet.indices_array.at(vertex_index as usize)) as usize
    };
    let vertex_id = renderlet.vertices_array.at(index as usize);
    let vertex = slab.read_unchecked(vertex_id);
    *out_color = vertex.color;
    *out_uv0 = vertex.uv0;
    *out_uv1 = vertex.uv1;

    let model_matrix = if renderlet.skin_id.is_some() {
        let skin = slab.read(renderlet.skin_id);
        skin.get_transform(vertex, slab)
    } else {
        let t = slab.read(renderlet.transform_id);
        Mat4::from(t)
    };
    let (scale, _, _) = model_matrix.to_scale_rotation_translation_or_id();
    let scale2 = scale * scale;
    let normal = vertex.normal.alt_norm_or_zero();
    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
    let normal_w: Vec3 = (model_matrix * (normal / scale2).extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    *out_norm = normal_w;

    let tangent_w: Vec3 = (model_matrix * tangent.extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    *out_tangent = tangent_w;

    let bitangent_w = normal_w.cross(tangent_w) * if vertex.tangent.w >= 0.0 { 1.0 } else { -1.0 };
    *out_bitangent = bitangent_w;

    let world_pos = model_matrix.transform_point3(vertex.position);
    *out_world_pos = world_pos;

    let camera = slab.read(renderlet.camera_id);
    *out_clip_pos = camera.projection * camera.view * world_pos.extend(1.0);

    vertex_log.completed = true;
    vertex_log.write(debug);
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct RenderletFragmentLog {
    pub frag_coord: Vec4,
    pub resolution: UVec2,
    pub started: bool,
    pub completed: bool,
}

impl RenderletFragmentLog {
    pub fn new(frag_coord: Vec4, resolution: UVec2) -> Self {
        let mut log = Self::default();
        log.frag_coord = frag_coord;
        log.resolution = resolution;
        log
    }

    pub fn write(&self, debug: &mut [u32]) {
        let x = self.frag_coord.x as u32;
        let y = self.frag_coord.y as u32;
        let w = self.resolution.x;
        let fragment_index = y * w + x;
        let index = fragment_index * RenderletFragmentLog::SLAB_SIZE as u32;
        let log_id: Id<RenderletFragmentLog> = Id::new(index);
        debug.write(log_id, self);
    }
}

#[cfg(feature = "renderlet_fragment")]
/// Renderlet fragment shader
#[allow(clippy::too_many_arguments, dead_code)]
#[spirv(fragment)]
pub fn renderlet_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] debug: &mut [u32],
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(flat)] in_camera: Id<Camera>,
    #[spirv(flat)] in_material: Id<Material>,
    #[spirv(flat)] in_pbr_config: Id<PbrConfig>,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    world_pos: Vec3,
    output: &mut Vec4,
) {
    let pbr_config = slab.read(in_pbr_config);
    let mut log = RenderletFragmentLog::new(frag_coord, pbr_config.resolution);
    log.started = true;
    log.write(debug);

    crate::pbr::fragment_impl(
        atlas,
        atlas_sampler,
        irradiance,
        irradiance_sampler,
        prefiltered,
        prefiltered_sampler,
        brdf,
        brdf_sampler,
        slab,
        pbr_config,
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

    log.completed = true;
    log.write(debug);
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

#[cfg(feature = "test_i8_16_extraction")]
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
