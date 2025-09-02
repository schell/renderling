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
    spirv, Image, Sampler,
};

use crate::{
    bvol::BoundingSphere,
    geometry::{GeometryDescriptor, MorphTarget, Skin, Vertex},
    math::IsVector,
    pbr::Material,
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

/// Returned by [`Renderlet::get_vertex_info`].
pub struct VertexInfo {
    pub vertex: Vertex,
    pub transform: Transform,
    pub model_matrix: Mat4,
    pub world_pos: Vec3,
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
    pub transform_id: Id<Transform>,
    pub material_id: Id<Material>,
    pub skin_id: Id<Skin>,
    pub morph_targets: Array<Array<MorphTarget>>,
    pub morph_weights: Array<f32>,
    pub geometry_descriptor_id: Id<GeometryDescriptor>,
}

impl Default for Renderlet {
    fn default() -> Self {
        Renderlet {
            visible: true,
            vertices_array: Array::default(),
            bounds: BoundingSphere::default(),
            indices_array: Array::default(),
            transform_id: Id::NONE,
            material_id: Id::NONE,
            skin_id: Id::NONE,
            morph_targets: Array::default(),
            morph_weights: Array::default(),
            geometry_descriptor_id: Id::new(0),
        }
    }
}

impl Renderlet {
    /// Returns the vertex at the given index and its related values.
    ///
    /// These values are often used in shaders, so they are grouped together.
    pub fn get_vertex_info(&self, vertex_index: u32, geometry_slab: &[u32]) -> VertexInfo {
        let vertex = self.get_vertex(vertex_index, geometry_slab);
        let transform = self.get_transform(vertex, geometry_slab);
        let model_matrix = Mat4::from(transform);
        let world_pos = model_matrix.transform_point3(vertex.position);
        VertexInfo {
            vertex,
            transform,
            model_matrix,
            world_pos,
        }
    }
    /// Retrieve the transform of this `Renderlet`.
    ///
    /// This takes into consideration all skinning matrices.
    pub fn get_transform(&self, vertex: Vertex, slab: &[u32]) -> Transform {
        let config = slab.read_unchecked(self.geometry_descriptor_id);
        if config.has_skinning && self.skin_id.is_some() {
            let skin = slab.read(self.skin_id);
            Transform::from(skin.get_skinning_matrix(vertex, slab))
        } else {
            slab.read(self.transform_id)
        }
    }

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

#[cfg(test)]
/// A helper struct that contains all outputs of the Renderlet's PBR vertex shader.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RenderletPbrVertexInfo {
    pub renderlet: Renderlet,
    pub renderlet_id: Id<Renderlet>,
    pub vertex_index: u32,
    pub vertex: Vertex,
    pub transform: Transform,
    pub model_matrix: Mat4,
    pub view_projection: Mat4,
    pub out_color: Vec4,
    pub out_uv0: Vec2,
    pub out_uv1: Vec2,
    pub out_norm: Vec3,
    pub out_tangent: Vec3,
    pub out_bitangent: Vec3,
    pub out_pos: Vec3,
    pub out_clip_pos: Vec4,
}

/// Renderlet vertex shader.
#[spirv(vertex)]
#[allow(clippy::too_many_arguments)]
pub fn renderlet_vertex(
    // Points at a `Renderlet`
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the renderlet are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],

    #[spirv(flat)] out_renderlet: &mut Id<Renderlet>,
    // TODO: Think about placing all these out values in a G-Buffer
    // But do we have enough buffers + enough space on web?
    // ...and can we write to buffers from vertex shaders on web?
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    out_world_pos: &mut Vec3,
    #[spirv(position)] out_clip_pos: &mut Vec4,
    // test-only info struct
    #[cfg(test)] out_info: &mut RenderletPbrVertexInfo,
) {
    let renderlet = geometry_slab.read_unchecked(renderlet_id);
    if !renderlet.visible {
        // put it outside the clipping frustum
        *out_clip_pos = Vec4::new(10.0, 10.0, 10.0, 1.0);
        return;
    }

    *out_renderlet = renderlet_id;

    let VertexInfo {
        vertex,
        transform,
        model_matrix,
        world_pos,
    } = renderlet.get_vertex_info(vertex_index, geometry_slab);
    *out_color = vertex.color;
    *out_uv0 = vertex.uv0;
    *out_uv1 = vertex.uv1;
    *out_world_pos = world_pos;

    let scale2 = transform.scale * transform.scale;
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

    let camera_id = geometry_slab
        .read_unchecked(renderlet.geometry_descriptor_id + GeometryDescriptor::OFFSET_OF_CAMERA_ID);
    let camera = geometry_slab.read(camera_id);
    let clip_pos = camera.view_projection() * world_pos.extend(1.0);
    *out_clip_pos = clip_pos;
    #[cfg(test)]
    {
        *out_info = RenderletPbrVertexInfo {
            renderlet_id,
            vertex_index,
            vertex,
            transform,
            model_matrix,
            view_projection: camera.view_projection(),
            out_clip_pos: clip_pos,
            renderlet,
            out_color: *out_color,
            out_uv0: *out_uv0,
            out_uv1: *out_uv1,
            out_norm: *out_norm,
            out_tangent: *out_tangent,
            out_bitangent: *out_bitangent,
            out_pos: *out_world_pos,
        };
    }
}

/// Renderlet fragment shader
#[allow(clippy::too_many_arguments, dead_code)]
#[spirv(fragment)]
pub fn renderlet_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] material_slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] atlas_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 4)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 5)] irradiance_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 6)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 7)] prefiltered_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 8)] brdf: &Image2d,
    #[spirv(descriptor_set = 0, binding = 9)] brdf_sampler: &Sampler,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 10)] light_slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 11)] shadow_map: &Image!(2D, type=f32, sampled, arrayed),
    #[spirv(descriptor_set = 0, binding = 12)] shadow_map_sampler: &Sampler,
    #[cfg(feature = "debug-slab")]
    #[spirv(storage_buffer, descriptor_set = 0, binding = 13)]
    debug_slab: &mut [u32],

    #[spirv(flat)] renderlet_id: Id<Renderlet>,
    #[spirv(frag_coord)] frag_coord: Vec4,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    world_pos: Vec3,
    output: &mut Vec4,
) {
    // proxy to a separate impl that allows us to test on CPU
    crate::pbr::fragment_impl(
        atlas,
        atlas_sampler,
        irradiance,
        irradiance_sampler,
        prefiltered,
        prefiltered_sampler,
        brdf,
        brdf_sampler,
        shadow_map,
        shadow_map_sampler,
        geometry_slab,
        material_slab,
        light_slab,
        renderlet_id,
        frag_coord,
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

#[cfg(test)]
mod test {
    use craballoc::{prelude::SlabAllocator, runtime::CpuRuntime};
    use glam::{Mat4, Quat, Vec3};

    use crate::{math::IsMatrix, stage::NestedTransform, transform::Transform};

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

        #[expect(clippy::needless_borrows_for_generic_args, reason = "riffraff")]
        let slab = SlabAllocator::<CpuRuntime>::new(&CpuRuntime, "transform", ());
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
