// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage objects for rendering.
use crabslab::{Array, Id, Slab, SlabItem};
#[cfg(not(target_arch = "spirv"))]
use glam::UVec2;
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d, Image2dArray},
    spirv, Sampler,
};

use crate::{
    bvol::BoundingSphere,
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
    pub fn get_joint_matrix(&self, i: usize, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let joint_index = vertex.joints[i] as usize;
        let joint_id = slab.read(self.joints.at(joint_index));
        let joint_transform = slab.read(joint_id);
        // First apply the inverse bind matrix to bring the vertex into the joint's
        // local space, then apply the joint's current transformation to move it
        // into world space.
        let inverse_bind_matrix = slab.read(self.inverse_bind_matrices.at(joint_index));
        Mat4::from(joint_transform) * inverse_bind_matrix
    }

    pub fn get_skinning_matrix(&self, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let mut skinning_matrix = Mat4::ZERO;
        for i in 0..vertex.joints.len() {
            let joint_matrix = self.get_joint_matrix(i, vertex, slab);
            // Ensure weights are applied correctly to the joint matrix
            let weight = vertex.weights[i];
            skinning_matrix += weight * joint_matrix;
        }

        if skinning_matrix == Mat4::ZERO {
            Mat4::IDENTITY
        } else {
            skinning_matrix
        }
    }
}

/// A displacement target.
///
/// Use to displace vertices using weights defined on the mesh.
///
/// For more info on morph targets, see
/// <https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#morph-targets>
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct MorphTarget {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    // TODO: Extend MorphTargets to include UV and Color.
    // I think this would take a contribution to the `gltf` crate.
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
#[offsets]
pub struct Renderlet {
    pub visible: bool,
    pub vertices_array: Array<Vertex>,
    /// Bounding sphere of the entire renderlet, in local space.
    pub bounds: BoundingSphere,
    pub indices_array: Array<u32>,
    pub camera_id: Id<Camera>,
    pub transform_id: Id<Transform>,
    pub material_id: Id<Material>,
    pub skin_id: Id<Skin>,
    pub morph_targets: Array<Array<MorphTarget>>,
    pub morph_weights: Array<f32>,
    pub pbr_config_id: Id<PbrConfig>,
}

impl Default for Renderlet {
    fn default() -> Self {
        Renderlet {
            visible: true,
            vertices_array: Array::default(),
            bounds: BoundingSphere::default(),
            indices_array: Array::default(),
            camera_id: Id::NONE,
            transform_id: Id::NONE,
            material_id: Id::NONE,
            skin_id: Id::NONE,
            morph_targets: Array::default(),
            morph_weights: Array::default(),
            pbr_config_id: Id::new(0),
        }
    }
}

impl Renderlet {
    /// Retrieve the vertex from the slab, calculating any displacement due to
    /// morph targets.
    pub fn get_vertex(&self, vertex_index: u32, slab: &[u32]) -> Vertex {
        let index = if self.indices_array.is_null() {
            vertex_index as usize
        } else {
            slab.read(self.indices_array.at(vertex_index as usize)) as usize
        };
        let vertex_id = self.vertices_array.at(index);
        let mut vertex = slab.read_unchecked(vertex_id);
        for i in 0..self.morph_targets.len() {
            let morph_target_array = slab.read(self.morph_targets.at(i));
            let morph_target = slab.read(morph_target_array.at(index));
            let weight = slab.read(self.morph_weights.at(i));
            vertex.position += weight * morph_target.position;
            vertex.normal += weight * morph_target.normal;
            vertex.tangent += weight * morph_target.tangent.extend(0.0);
        }
        vertex
    }

    pub fn get_vertex_count(&self) -> u32 {
        if self.indices_array.is_null() {
            self.vertices_array.len() as u32
        } else {
            self.indices_array.len() as u32
        }
    }
}

/// Renderlet vertex shader.
#[spirv(vertex)]
#[allow(clippy::too_many_arguments)]
pub fn renderlet_vertex(
    // Points at a `Renderlet`
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the renderlet are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] out_renderlet: &mut Id<Renderlet>,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    out_world_pos: &mut Vec3,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let renderlet = slab.read_unchecked(renderlet_id);
    if !renderlet.visible {
        // put it outside the clipping frustum
        *out_clip_pos = Vec4::new(10.0, 10.0, 10.0, 1.0);
        return;
    }

    *out_renderlet = renderlet_id;

    let vertex = renderlet.get_vertex(vertex_index, slab);
    *out_color = vertex.color;
    *out_uv0 = vertex.uv0;
    *out_uv1 = vertex.uv1;

    let config = slab.read_unchecked(renderlet.pbr_config_id);

    let transform = if config.has_skinning && renderlet.skin_id.is_some() {
        let skin = slab.read(renderlet.skin_id);
        Transform::from(skin.get_skinning_matrix(vertex, slab))
    } else {
        slab.read(renderlet.transform_id)
    };
    let scale2 = transform.scale * transform.scale;
    let normal = vertex.normal.alt_norm_or_zero();
    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
    let model_matrix = Mat4::from(transform);
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
    *out_clip_pos = camera.view_projection() * world_pos.extend(1.0);
}

/// Renderlet fragment shader
#[allow(clippy::too_many_arguments, dead_code)]
#[spirv(fragment)]
pub fn renderlet_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2dArray,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] renderlet_id: Id<Renderlet>,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    world_pos: Vec3,
    output: &mut Vec4,
) {
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
        renderlet_id,
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

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_i_increment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] times: &u32,
) {
    let mut i = 0u32;
    loop {
        if i >= *times {
            break;
        }
        let _ = unsafe {
            spirv_std::arch::atomic_i_increment::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::NONE.bits() },
            >(global_index)
        };
        i += 1;
    }
}

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_load_and_store(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] times: &u32,
) {
    for _ in 0..*times {
        let loaded = unsafe {
            spirv_std::arch::atomic_load::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::NONE.bits() },
            >(global_index)
        };
        unsafe {
            spirv_std::arch::atomic_store::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::NONE.bits() },
            >(global_index, loaded + 2)
        };
    }
}

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_exchange(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] times: &u32,
) {
    let mut n = 0u32;
    for _ in 0..*times {
        n += unsafe {
            spirv_std::arch::atomic_exchange::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::NONE.bits() },
            >(global_index, n)
        };
    }
}

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_compare_exchange(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] times: &u32,
) {
    for n in 0..*times {
        unsafe {
            spirv_std::arch::atomic_compare_exchange::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
            >(global_index, n, 3)
        };
    }
}

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_i_decrement(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [u32],
) {
    loop {
        let i = unsafe {
            spirv_std::arch::atomic_i_decrement::<
                u32,
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::NONE.bits() },
            >(global_index)
        };
        output[i as usize] = i;
        if i == 0 {
            break;
        }
    }
}

#[cfg(feature = "test_spirv_atomics")]
#[spirv(compute(threads(32)))]
pub fn test_atomic_i_add_sub(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_index: &mut u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [u32],
) {
    let i = unsafe {
        spirv_std::arch::atomic_i_add::<
            u32,
            { spirv_std::memory::Scope::Workgroup as u32 },
            { spirv_std::memory::Semantics::NONE.bits() },
        >(global_index, 2)
    };

    output[i as usize] = unsafe {
        spirv_std::arch::atomic_i_sub::<
            u32,
            { spirv_std::memory::Scope::Workgroup as u32 },
            { spirv_std::memory::Semantics::NONE.bits() },
        >(global_index, i)
    };
}

#[cfg(test)]
mod test {
    use std::sync::Mutex;

    use glam::{Mat4, Quat, Vec3};

    use crate::{
        math::IsMatrix, slab::SlabAllocator, stage::NestedTransform, transform::Transform,
    };

    #[test]
    fn matrix_hierarchy_sanity() {
        let a: Mat4 = Transform {
            translation: Vec3::new(100.0, 100.0, 0.0),
            ..Default::default()
        }
        .into();
        let b: Mat4 = Transform {
            scale: Vec3::splat(0.5),
            ..Default::default()
        }
        .into();
        let c1 = a * b;
        let c2 = b * a;
        assert_ne!(c1, c2);
    }

    #[test]
    fn nested_transform_fox_rigging() {
        pub fn legacy_get_world_transform(tfrm: &NestedTransform) -> (Vec3, Quat, Vec3) {
            let mut mat = Mat4::IDENTITY;
            let mut local = Some(tfrm.clone());
            while let Some(t) = local.take() {
                let transform = t.get();
                mat = Mat4::from_scale_rotation_translation(
                    transform.scale,
                    transform.rotation,
                    transform.translation,
                ) * mat;
                local = t.parent();
            }
            let (s, r, t) = mat.to_scale_rotation_translation_or_id();
            (t, r, s)
        }

        let slab = SlabAllocator::<Mutex<Vec<u32>>>::default();
        let a = NestedTransform::new(&slab);
        a.set(Transform {
            translation: Vec3::splat(100.0),
            ..Default::default()
        });
        let b = NestedTransform::new(&slab);
        b.set(Transform {
            rotation: Quat::from_scaled_axis(Vec3::Z),
            ..Default::default()
        });
        let c = NestedTransform::new(&slab);
        c.set(Transform {
            scale: Vec3::splat(2.0),
            ..Default::default()
        });

        a.add_child(&b);
        b.add_child(&c);

        let Transform {
            translation,
            rotation,
            scale,
        } = c.get_global_transform();
        let global_transform = (translation, rotation, scale);
        let legacy_transform = legacy_get_world_transform(&c);
        assert_eq!(legacy_transform, global_transform);

        c.modify(|t| t.translation = Vec3::ONE);

        let all_updates = slab.get_updated_source_ids();
        assert_eq!(
            std::collections::HashSet::from_iter([
                a.get_notifier_index(),
                b.get_notifier_index(),
                c.get_notifier_index()
            ]),
            all_updates
        );

        let Transform {
            translation,
            rotation,
            scale,
        } = c.get_global_transform();
        let global_transform = (translation, rotation, scale);
        let legacy_transform = legacy_get_world_transform(&c);
        assert_eq!(legacy_transform, global_transform);
    }
}
