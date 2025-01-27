//! Lighting.
//!
//! Directional, point and spot lights.
//!
//! Shadow mapping.
use crabslab::{Id, Slab, SlabItem};
use glam::{Mat4, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::spirv;

use crate::{
    camera::Camera,
    math::{IsSampler, IsVector, Sample2d},
    stage::Renderlet,
    transform::Transform,
};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

/// Root descriptor of the lighting system.
#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
pub struct LightingDescriptor {
    pub shadow_map_light_transform: Id<Mat4>,
    pub bias_min: f32,
    pub bias_max: f32,
}

#[cfg(test)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ShadowMappingVertexInfo {
    pub renderlet_id: Id<Renderlet>,
    pub vertex_index: u32,
    pub vertex: crate::stage::Vertex,
    pub transform: Transform,
    pub model_matrix: Mat4,
    pub world_pos: Vec3,
    pub view_projection: Mat4,
    pub clip_pos: Vec4,
}

/// Shadow mapping vertex shader.
// Note:
// If this is taking too long to render for each renderlet, think about
// a frustum and occlusion culling pass to generate the list of renderlets.
#[spirv(vertex)]
#[allow(clippy::too_many_arguments)]
pub fn shadow_mapping_vertex(
    // Points at a `Renderlet`
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the renderlet are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    // The slab where the renderlet's geometry is staged
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    // The slab where the scene's lighting data is staged
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] light_slab: &[u32],

    #[spirv(position)] out_clip_pos: &mut Vec4,
    #[cfg(test)] out_comparison_info: &mut ShadowMappingVertexInfo,
) {
    let renderlet = geometry_slab.read_unchecked(renderlet_id);
    if !renderlet.visible {
        // put it outside the clipping frustum
        *out_clip_pos = Vec4::new(10.0, 10.0, 10.0, 1.0);
        return;
    }

    let (_vertex, _transform, _model_matrix, world_pos) =
        renderlet.get_vertex_info(vertex_index, geometry_slab);

    let lighting_descr = light_slab.read_unchecked(Id::<LightingDescriptor>::new(0));
    let light_transform = light_slab.read_unchecked(lighting_descr.shadow_map_light_transform);

    let clip_pos = light_transform * world_pos.extend(1.0);
    #[cfg(test)]
    {
        *out_comparison_info = ShadowMappingVertexInfo {
            renderlet_id,
            vertex_index,
            vertex: _vertex,
            transform: _transform,
            model_matrix: _model_matrix,
            world_pos,
            view_projection: light_transform,
            clip_pos,
        };
    }
    *out_clip_pos = clip_pos;
}

#[spirv(fragment)]
pub fn shadow_mapping_fragment(clip_pos: Vec4, frag_color: &mut Vec4) {
    *frag_color = (clip_pos.xyz() / clip_pos.w).extend(1.0);
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for SpotLight {
    fn default() -> Self {
        let white = Vec4::splat(1.0);
        let inner_cutoff = core::f32::consts::PI / 3.0;
        let outer_cutoff = core::f32::consts::PI / 2.0;
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let color = white;
        let intensity = 1.0;

        Self {
            position: Default::default(),
            direction,
            inner_cutoff,
            outer_cutoff,
            color,
            intensity,
        }
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let color = Vec4::splat(1.0);
        let intensity = 1.0;

        Self {
            direction,
            color,
            intensity,
        }
    }
}

impl DirectionalLight {
    pub fn shadow_mapping_projection_and_view(
        &self,
        parent_light_transform: &Mat4,
        camera: &Camera,
    ) -> (Mat4, Mat4) {
        // TODO: add `shadow_mapping_projection_and_view` to `SpotLight`
        let frustum = camera.frustum();
        let depth = frustum.depth();
        let hd = depth * 0.5;
        let projection = Mat4::orthographic_rh(-hd, hd, -hd, hd, 0.0, depth);
        let direction = parent_light_transform
            .transform_vector3(self.direction)
            .alt_norm_or_zero();
        let position = -direction * depth * 0.5;
        let view = Mat4::look_to_rh(position, direction, Vec3::Z);
        (projection, view)
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        let color = Vec4::splat(1.0);
        let intensity = 1.0;

        Self {
            position: Default::default(),
            color,
            intensity,
        }
    }
}

#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq)]
pub enum LightStyle {
    Directional = 0,
    Point = 1,
    Spot = 2,
}

impl SlabItem for LightStyle {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let proxy = u32::read_slab(index, slab);
        match proxy {
            0 => LightStyle::Directional,
            1 => LightStyle::Point,
            2 => LightStyle::Spot,
            _ => LightStyle::Directional,
        }
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        let proxy = *self as u32;
        proxy.write_slab(index, slab)
    }
}

/// A type-erased/generic light that is used as a slab pointer to a
/// specific light type.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq, SlabItem)]
pub struct Light {
    /// The type of the light
    pub light_type: LightStyle,
    /// The index of the light in the slab
    pub index: u32,
    /// The id of a transform to apply to the position and direction of the light.
    pub transform_id: Id<Transform>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: Id::<()>::NONE.inner(),
            transform_id: Id::NONE,
        }
    }
}

impl From<Id<DirectionalLight>> for Light {
    fn from(id: Id<DirectionalLight>) -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: id.inner(),
            transform_id: Id::NONE,
        }
    }
}

impl From<Id<SpotLight>> for Light {
    fn from(id: Id<SpotLight>) -> Self {
        Self {
            light_type: LightStyle::Spot,
            index: id.inner(),
            transform_id: Id::NONE,
        }
    }
}

impl From<Id<PointLight>> for Light {
    fn from(id: Id<PointLight>) -> Self {
        Self {
            light_type: LightStyle::Point,
            index: id.inner(),
            transform_id: Id::NONE,
        }
    }
}

impl Light {
    pub fn into_directional_id(self) -> Id<DirectionalLight> {
        Id::from(self.index)
    }

    pub fn into_spot_id(self) -> Id<SpotLight> {
        Id::from(self.index)
    }

    pub fn into_point_id(self) -> Id<PointLight> {
        Id::from(self.index)
    }
}

/// Returns shadow _intensity_ at the given position.
///
/// Returns `1.0` when the fragment is in complete shadow.
/// Returns `0.0` when the fragment is in the light.
pub fn shadow_calculation<S, T>(
    shadow_map: &T,
    shadow_map_sampler: &S,
    frag_pos_in_light_space: Vec3,
    surface_normal: Vec3,
    light_direction: Vec3,
    bias_min: f32,
    bias_max: f32,
) -> f32
where
    S: IsSampler,
    T: Sample2d<Sampler = S>,
{
    crate::println!("frag_pos_in_light_space: {frag_pos_in_light_space}");
    // The range of coordinates in the light's clip space is -1.0 to 1.0 for x and y,
    // but the texture space is [0, 1], and Y increases downward, so we do this
    // conversion to flip Y and also normalize to the range [0.0, 1.0].
    // Z should already be 0.0 to 1.0.
    let proj_coords =
        frag_pos_in_light_space * Vec3::new(0.5, -0.5, 1.0) + Vec3::new(0.5, 0.5, 0.0);
    crate::println!("proj_coords: {proj_coords}");

    // With these projected coordinates we can sample the depth map as the
    // resulting [0,1] coordinates from proj_coords directly correspond to
    // the transformed NDC coordinates from the `ShadowMap::update` render pass.
    // This gives us the closest depth from the light's point of view:
    let closest_depth = shadow_map
        .sample_by_lod(*shadow_map_sampler, proj_coords.xy(), 0.0)
        .x;
    // To get the current depth at this fragment we simply retrieve the projected vector's z
    // coordinate which equals the depth of this fragment from the light's perspective.
    let current_depth = proj_coords.z;

    // If the `current_depth`, which is the depth of the fragment from the lights POV, is
    // greater than the `closest_depth` of the shadow map at that fragment, the fragment
    // is in shadow
    crate::println!("current_depth: {current_depth}");
    crate::println!("closest_depth: {closest_depth}");
    let bias = (bias_max * (1.0 - surface_normal.dot(light_direction))).max(bias_min);

    if (current_depth - bias) > closest_depth {
        1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "gltf")]
    #[test]
    fn position_direction_sanity() {
        // With GLTF, the direction of a light is given by the light's node's transform.
        // Specifically we get the node's transform and use the rotation quaternion to
        // rotate the vector Vec3::NEG_Z - the result is our direction.

        use glam::{Mat4, Quat};
        println!("{:#?}", std::env::current_dir());
        let (document, _buffers, _images) = gltf::import("../../gltf/four_spotlights.glb").unwrap();
        for node in document.nodes() {
            println!("node: {} {:?}", node.index(), node.name());

            let gltf_transform = node.transform();
            let (translation, rotation, _scale) = gltf_transform.decomposed();
            let position = Vec3::from_array(translation);
            let direction =
                Mat4::from_quat(Quat::from_array(rotation)).transform_vector3(Vec3::NEG_Z);
            println!("position: {position}");
            println!("direction: {direction}");

            // In Blender, our lights are sitting at (0, 0, 1) pointing at -Z, +Z, +X and
            // +Y. But alas, it is a bit more complicated than that because this
            // file is exported with UP being +Y, so Z and Y have been
            // flipped...
            assert_eq!(Vec3::Y, position);
            let expected_direction = match node.name() {
                Some("light_negative_z") => Vec3::NEG_Y,
                Some("light_positive_z") => Vec3::Y,
                Some("light_positive_x") => Vec3::X,
                Some("light_positive_y") => Vec3::NEG_Z,
                n => panic!("unexpected node '{n:?}'"),
            };
            // And also there are rounding ... imprecisions...
            assert_approx_eq::assert_approx_eq!(expected_direction.x, direction.x);
            assert_approx_eq::assert_approx_eq!(expected_direction.y, direction.y);
            assert_approx_eq::assert_approx_eq!(expected_direction.z, direction.z);
        }
    }

    #[test]
    fn shadow_mapping_sanity() {
        // Test that shadow mapping is working as expected.
    }
}
