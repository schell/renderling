//! Lighting.
//!
//! Directional, point and spot lights.
//!
//! Shadow mapping.
//!
//! Tiling.
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{Mat4, UVec2, UVec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
#[cfg(gpu)]
use spirv_std::num_traits::Float;
use spirv_std::{spirv, Image};

use crate::{
    atlas::{AtlasDescriptor, AtlasTexture},
    cubemap::{CubemapDescriptor, CubemapFaceDirection},
    geometry::GeometryDescriptor,
    math::{Fetch, IsSampler, IsVector, Sample2dArray},
    stage::Renderlet,
    transform::Transform,
};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

#[cfg(cpu)]
mod shadow_map;
#[cfg(cpu)]
pub use shadow_map::*;

#[cfg(cpu)]
mod tiling;
#[cfg(cpu)]
pub use tiling::*;

/// Root descriptor of the lighting system.
#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
#[offsets]
pub struct LightingDescriptor {
    /// List of all analytical lights in the scene.
    pub analytical_lights_array: Array<Id<Light>>,
    /// Shadow mapping atlas info.
    pub shadow_map_atlas_descriptor_id: Id<AtlasDescriptor>,
    /// `Id` of the [`ShadowMapDescriptor`] to use when updating
    /// a shadow map.
    ///
    /// This changes from each run of the `shadow_mapping_vertex`.
    pub update_shadow_map_id: Id<ShadowMapDescriptor>,
    /// The index of the shadow map atlas texture to update.
    pub update_shadow_map_texture_index: u32,
    /// `Id` of the [`LightTilingDescriptor`] to use when performing
    /// light tiling.
    pub light_tiling_descriptor_id: Id<LightTilingDescriptor>,
}

#[derive(Clone, Copy, SlabItem, core::fmt::Debug)]
pub struct ShadowMapDescriptor {
    pub light_space_transforms_array: Array<Mat4>,
    /// Near plane of the projection matrix
    pub z_near: f32,
    /// Far plane of the projection matrix
    pub z_far: f32,
    /// Pointers to the atlas textures where the shadow map depth
    /// data is stored.
    ///
    /// This will be an array of one `Id` for directional and spot lights,
    /// and an array of four `Id`s for a point light.
    pub atlas_textures_array: Array<Id<AtlasTexture>>,
    pub bias_min: f32,
    pub bias_max: f32,
    pub pcf_samples: u32,
}

impl Default for ShadowMapDescriptor {
    fn default() -> Self {
        Self {
            light_space_transforms_array: Default::default(),
            z_near: Default::default(),
            z_far: Default::default(),
            atlas_textures_array: Default::default(),
            bias_min: 0.0005,
            bias_max: 0.005,
            pcf_samples: 4,
        }
    }
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
///
/// It is assumed that a [`LightingDescriptor`] is stored at `Id(0)` of the
/// `light_slab`.
///
/// This shader reads the [`LightingDescriptor`] to find the shadow map to
/// be updated, then determines the clip positions to emit based on the
/// shadow map's atlas texture.
///
/// It then renders the renderlet into the designated atlas frame.
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
        *out_clip_pos = Vec4::new(100.0, 100.0, 100.0, 1.0);
        return;
    }

    let (_vertex, _transform, _model_matrix, world_pos) =
        renderlet.get_vertex_info(vertex_index, geometry_slab);

    let lighting_desc = light_slab.read_unchecked(Id::<LightingDescriptor>::new(0));
    let shadow_desc = light_slab.read_unchecked(lighting_desc.update_shadow_map_id);
    let light_space_transform_id = shadow_desc
        .light_space_transforms_array
        .at(lighting_desc.update_shadow_map_texture_index as usize);
    let light_space_transform = light_slab.read_unchecked(light_space_transform_id);
    let clip_pos = light_space_transform * world_pos.extend(1.0);
    #[cfg(test)]
    {
        *out_comparison_info = ShadowMappingVertexInfo {
            renderlet_id,
            vertex_index,
            vertex: _vertex,
            transform: _transform,
            model_matrix: _model_matrix,
            world_pos,
            view_projection: light_space_transform,
            clip_pos,
        };
    }
    *out_clip_pos = clip_pos;
}

#[spirv(fragment)]
pub fn shadow_mapping_fragment(clip_pos: Vec4, frag_color: &mut Vec4) {
    *frag_color = (clip_pos.xyz() / clip_pos.w).extend(1.0);
}

/// Contains values needed to determine the outgoing radiance of a fragment.
///
/// For more info, see the **Spotlight** section of the
/// [learnopengl](https://learnopengl.com/Lighting/Light-casters)
/// article.
#[derive(Clone, Copy, Default, core::fmt::Debug)]
pub struct SpotLightCalculation {
    /// Position of the light in world space
    pub light_position: Vec3,
    /// Position of the fragment in world space
    pub frag_position: Vec3,
    /// Unit vector (LightDir) pointing from the fragment to the light
    pub frag_to_light: Vec3,
    /// Distance from the fragment to the light
    pub frag_to_light_distance: f32,
    /// Unit vector (SpotDir) direction that the light is pointing in
    pub light_direction: Vec3,
    /// The cosine of the cutoff angle (Phi ϕ) that specifies the spotlight's radius.
    ///
    /// Everything inside this angle is lit by the spotlight.
    pub cos_inner_cutoff: f32,
    /// The cosine of the cutoff angle (Gamma γ) that specifies the spotlight's outer radius.
    ///
    /// Everything outside this angle is not lit by the spotlight.
    ///
    /// Fragments between `inner_cutoff` and `outer_cutoff` have an intensity
    /// between `1.0` and `0.0`.
    pub cos_outer_cutoff: f32,
    /// Whether the fragment is inside the `inner_cutoff` cone.
    pub fragment_is_inside_inner_cone: bool,
    /// Whether the fragment is inside the `outer_cutoff` cone.
    pub fragment_is_inside_outer_cone: bool,
    /// `outer_cutoff` - `inner_cutoff`
    pub epsilon: f32,
    /// Cosine of the angle (Theta θ) between `frag_to_light` (LightDir) vector and the
    /// `light_direction` (SpotDir) vector.
    ///
    /// θ  should be smaller than `outer_cutoff` (Gamma γ) to be
    /// inside the spotlight, but since these are all cosines of angles, we actually
    /// compare using `>`.
    pub cos_theta: f32,
    pub contribution_unclamped: f32,
    /// The intensity level between `0.0` and `1.0` that should be used to determine
    /// outgoing radiance.
    pub contribution: f32,
}

impl SpotLightCalculation {
    /// Calculate the values required to determine outgoing radiance of a spot light.
    pub fn new(
        spot_light_descriptor: SpotLightDescriptor,
        node_transform: Mat4,
        fragment_world_position: Vec3,
    ) -> Self {
        let light_position = node_transform.transform_point3(spot_light_descriptor.position);
        let frag_position = fragment_world_position;
        let frag_to_light = light_position - frag_position;
        let frag_to_light_distance = frag_to_light.length();
        if frag_to_light_distance == 0.0 {
            crate::println!("frag_to_light_distance: {frag_to_light_distance}");
            return Self::default();
        }
        let frag_to_light = frag_to_light.alt_norm_or_zero();
        let light_direction = node_transform
            .transform_vector3(spot_light_descriptor.direction)
            .alt_norm_or_zero();
        let cos_inner_cutoff = spot_light_descriptor.inner_cutoff.cos();
        let cos_outer_cutoff = spot_light_descriptor.outer_cutoff.cos();
        let epsilon = cos_inner_cutoff - cos_outer_cutoff;
        let cos_theta = frag_to_light.dot(-light_direction);
        let fragment_is_inside_inner_cone = cos_theta > cos_inner_cutoff;
        let fragment_is_inside_outer_cone = cos_theta > cos_outer_cutoff;
        let contribution_unclamped = (cos_theta - cos_outer_cutoff) / epsilon;
        let contribution = contribution_unclamped.clamp(0.0, 1.0);
        Self {
            light_position,
            frag_position,
            frag_to_light,
            frag_to_light_distance,
            light_direction,
            cos_inner_cutoff,
            cos_outer_cutoff,
            fragment_is_inside_inner_cone,
            fragment_is_inside_outer_cone,
            epsilon,
            cos_theta,
            contribution_unclamped,
            contribution,
        }
    }
}

/// Description of a spot light.
///
/// ## Tips
///
/// If your spotlight is not illuminating your scenery, ensure that the
/// `inner_cutoff` and `outer_cutoff` values are "correct". `outer_cutoff`
/// should be _greater than_ `inner_cutoff` and the values should be a large
/// enough to cover at least one pixel at the distance between the light and
/// the scenery.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct SpotLightDescriptor {
    pub position: Vec3,
    pub direction: Vec3,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for SpotLightDescriptor {
    fn default() -> Self {
        let white = Vec4::splat(1.0);
        let inner_cutoff = 0.077143565;
        let outer_cutoff = 0.09075713;
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

impl SpotLightDescriptor {
    pub fn shadow_mapping_projection_and_view(
        &self,
        parent_light_transform: &Mat4,
        z_near: f32,
        z_far: f32,
    ) -> (Mat4, Mat4) {
        let fovy = 2.0 * self.outer_cutoff;
        let aspect = 1.0;
        let projection = Mat4::perspective_rh(fovy, aspect, z_near, z_far);
        let direction = parent_light_transform
            .transform_vector3(self.direction)
            .alt_norm_or_zero();
        let position = parent_light_transform.transform_point3(self.position);
        let up = direction.orthonormal_vectors()[0];
        let view = Mat4::look_to_rh(position, direction, up);
        (projection, view)
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct DirectionalLightDescriptor {
    pub direction: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for DirectionalLightDescriptor {
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

impl DirectionalLightDescriptor {
    pub fn shadow_mapping_projection_and_view(
        &self,
        parent_light_transform: &Mat4,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        // TODO: in `DirectionalLightDescriptor::shadow_mapping_projection_and_view`, take Frustum
        // as a parameter and then figure out the minimal view projection that includes that frustum
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> (Mat4, Mat4) {
        crate::println!("descriptor: {self:#?}");
        let depth = (z_far - z_near).abs();
        let hd = depth * 0.5;
        let projection = Mat4::orthographic_rh(-hd, hd, -hd, hd, z_near, z_far);
        let direction = parent_light_transform
            .transform_vector3(self.direction)
            .alt_norm_or_zero();
        let position = -direction * depth * 0.5;
        crate::println!("direction: {direction}");
        crate::println!("position: {position}");
        let up = direction.orthonormal_vectors()[0];
        let view = Mat4::look_to_rh(position, direction, up);
        (projection, view)
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct PointLightDescriptor {
    pub position: Vec3,
    pub color: Vec4,
    /// Expressed as candelas.
    pub intensity: f32,
}

impl Default for PointLightDescriptor {
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

impl PointLightDescriptor {
    pub fn shadow_mapping_view_matrix(
        &self,
        face_index: usize,
        parent_light_transform: &Mat4,
    ) -> Mat4 {
        let eye = parent_light_transform.transform_point3(self.position);
        let mut face = CubemapFaceDirection::FACES[face_index];
        face.eye = eye;
        face.view()
    }

    pub fn shadow_mapping_projection_matrix(z_near: f32, z_far: f32) -> Mat4 {
        Mat4::perspective_lh(core::f32::consts::FRAC_PI_2, 1.0, z_near, z_far)
    }

    pub fn shadow_mapping_projection_and_view_matrices(
        &self,
        parent_light_transform: &Mat4,
        z_near: f32,
        z_far: f32,
    ) -> (Mat4, [Mat4; 6]) {
        let p = Self::shadow_mapping_projection_matrix(z_near, z_far);
        let eye = parent_light_transform.transform_point3(self.position);
        (
            p,
            CubemapFaceDirection::FACES.map(|mut face| {
                face.eye = eye;
                face.view()
            }),
        )
    }
}

/// Returns the radius of illumination in meters.
///
/// * Moonlight: < 1 lux.
///   - Full moon on a clear night: 0.25 lux.
///   - Quarter moon: 0.01 lux
///   - Starlight overcast moonless night sky: 0.0001 lux.
/// * General indoor lighting: Around 100 to 300 lux.                                   
/// * Office lighting: Typically around 300 to 500 lux.                                 
/// * Reading or task lighting: Around 500 to 750 lux.                                  
/// * Detailed work (e.g., drafting, surgery): 1000 lux or more.
pub fn radius_of_illumination(intensity_candelas: f32, minimum_illuminance_lux: f32) -> f32 {
    (intensity_candelas / minimum_illuminance_lux).sqrt()
}

#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq)]
pub enum LightStyle {
    Directional = 0,
    Point = 1,
    Spot = 2,
}

impl core::fmt::Display for LightStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LightStyle::Directional => f.write_str("directional"),
            LightStyle::Point => f.write_str("point"),
            LightStyle::Spot => f.write_str("spot"),
        }
    }
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

/// A generic light that is used as a slab pointer to a
/// specific light type.
// TODO: rename to `LightDescriptor`
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq, SlabItem)]
pub struct Light {
    /// The type of the light
    pub light_type: LightStyle,
    /// The index of the light in the lighting slab
    pub index: u32,
    /// The id of a transform to apply to the position and direction of the light.
    ///
    /// This `Id` points to a transform on the geometry slab.
    pub transform_id: Id<Transform>,
    /// The id of the shadow map in use by this light.
    pub shadow_map_desc_id: Id<ShadowMapDescriptor>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: Id::<()>::NONE.inner(),
            transform_id: Id::NONE,
            shadow_map_desc_id: Id::NONE,
        }
    }
}

impl From<Id<DirectionalLightDescriptor>> for Light {
    fn from(id: Id<DirectionalLightDescriptor>) -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: id.inner(),
            transform_id: Id::NONE,
            shadow_map_desc_id: Id::NONE,
        }
    }
}

impl From<Id<SpotLightDescriptor>> for Light {
    fn from(id: Id<SpotLightDescriptor>) -> Self {
        Self {
            light_type: LightStyle::Spot,
            index: id.inner(),
            transform_id: Id::NONE,
            shadow_map_desc_id: Id::NONE,
        }
    }
}

impl From<Id<PointLightDescriptor>> for Light {
    fn from(id: Id<PointLightDescriptor>) -> Self {
        Self {
            light_type: LightStyle::Point,
            index: id.inner(),
            transform_id: Id::NONE,
            shadow_map_desc_id: Id::NONE,
        }
    }
}

impl Light {
    pub fn into_directional_id(self) -> Id<DirectionalLightDescriptor> {
        Id::from(self.index)
    }

    pub fn into_spot_id(self) -> Id<SpotLightDescriptor> {
        Id::from(self.index)
    }

    pub fn into_point_id(self) -> Id<PointLightDescriptor> {
        Id::from(self.index)
    }
}

/// Parameters to the shadow mapping calculation function.
///
/// This is mostly just to appease clippy.
pub struct ShadowCalculation {
    pub shadow_map_desc: ShadowMapDescriptor,
    pub shadow_map_atlas_size: UVec2,
    pub surface_normal_in_world_space: Vec3,
    pub frag_pos_in_world_space: Vec3,
    pub frag_to_light_in_world_space: Vec3,
    pub bias_min: f32,
    pub bias_max: f32,
    pub pcf_samples: u32,
}

impl ShadowCalculation {
    /// Reads various required parameters from the slab and creates a `ShadowCalculation`.
    pub fn new(
        light_slab: &[u32],
        light: crate::prelude::Light,
        in_pos: Vec3,
        surface_normal: Vec3,
        light_direction: Vec3,
    ) -> Self {
        let shadow_map_desc = light_slab.read_unchecked(light.shadow_map_desc_id);
        let atlas_size = {
            let lighting_desc_id = Id::<LightingDescriptor>::new(0);
            let atlas_desc_id = light_slab.read_unchecked(
                lighting_desc_id + LightingDescriptor::OFFSET_OF_SHADOW_MAP_ATLAS_DESCRIPTOR_ID,
            );
            let atlas_desc = light_slab.read_unchecked(atlas_desc_id);
            atlas_desc.size
        };

        ShadowCalculation {
            shadow_map_desc,
            shadow_map_atlas_size: atlas_size.xy(),
            surface_normal_in_world_space: surface_normal,
            frag_pos_in_world_space: in_pos,
            frag_to_light_in_world_space: light_direction,
            bias_min: shadow_map_desc.bias_min,
            bias_max: shadow_map_desc.bias_max,
            pcf_samples: shadow_map_desc.pcf_samples,
        }
    }

    fn get_atlas_texture_at(&self, light_slab: &[u32], index: usize) -> AtlasTexture {
        let atlas_texture_id =
            light_slab.read_unchecked(self.shadow_map_desc.atlas_textures_array.at(index));
        light_slab.read_unchecked(atlas_texture_id)
    }

    fn get_frag_pos_in_light_space(&self, light_slab: &[u32], index: usize) -> Vec3 {
        let light_space_transform_id = self.shadow_map_desc.light_space_transforms_array.at(index);
        let light_space_transform = light_slab.read_unchecked(light_space_transform_id);
        light_space_transform.project_point3(self.frag_pos_in_world_space)
    }

    /// Returns shadow _intensity_ for directional and spot lights.
    ///
    /// Returns `0.0` when the fragment is in full light.
    /// Returns `1.0` when the fragment is in full shadow.
    pub fn run_directional_or_spot<T, S>(
        &self,
        light_slab: &[u32],
        shadow_map: &T,
        shadow_map_sampler: &S,
    ) -> f32
    where
        S: IsSampler,
        T: Sample2dArray<Sampler = S>,
    {
        let ShadowCalculation {
            shadow_map_desc: _,
            shadow_map_atlas_size,
            frag_pos_in_world_space: _,
            surface_normal_in_world_space: surface_normal,
            frag_to_light_in_world_space: light_direction,
            bias_min,
            bias_max,
            pcf_samples,
        } = self;
        let frag_pos_in_light_space = self.get_frag_pos_in_light_space(light_slab, 0);
        crate::println!("frag_pos_in_light_space: {frag_pos_in_light_space}");
        if !crate::math::is_inside_clip_space(frag_pos_in_light_space.xyz()) {
            return 0.0;
        }
        // The range of coordinates in the light's clip space is -1.0 to 1.0 for x and y,
        // but the texture space is [0, 1], and Y increases downward, so we do this
        // conversion to flip Y and also normalize to the range [0.0, 1.0].
        // Z should already be 0.0 to 1.0.
        let proj_coords_uv = (frag_pos_in_light_space.xy() * Vec2::new(1.0, -1.0)
            + Vec2::splat(1.0))
            * Vec2::splat(0.5);
        crate::println!("proj_coords_uv: {proj_coords_uv}");

        let shadow_map_atlas_texture = self.get_atlas_texture_at(light_slab, 0);
        // With these projected coordinates we can sample the depth map as the
        // resulting [0,1] coordinates from proj_coords directly correspond to
        // the transformed NDC coordinates from the `ShadowMap::update` render pass.
        // This gives us the closest depth from the light's point of view:
        let pcf_samples_2 = *pcf_samples as i32 / 2;
        let texel_size = 1.0
            / Vec2::new(
                shadow_map_atlas_texture.size_px.x as f32,
                shadow_map_atlas_texture.size_px.y as f32,
            );
        let mut shadow = 0.0f32;
        let mut total = 0.0f32;
        for x in -pcf_samples_2..=pcf_samples_2 {
            for y in -pcf_samples_2..=pcf_samples_2 {
                let proj_coords = shadow_map_atlas_texture.uv(
                    proj_coords_uv + Vec2::new(x as f32, y as f32) * texel_size,
                    *shadow_map_atlas_size,
                );
                let shadow_map_depth = shadow_map
                    .sample_by_lod(*shadow_map_sampler, proj_coords, 0.0)
                    .x;
                // To get the current depth at this fragment we simply retrieve the projected vector's z
                // coordinate which equals the depth of this fragment from the light's perspective.
                let fragment_depth = frag_pos_in_light_space.z;

                // If the `current_depth`, which is the depth of the fragment from the lights POV, is
                // greater than the `closest_depth` of the shadow map at that fragment, the fragment
                // is in shadow
                crate::println!("current_depth: {fragment_depth}");
                crate::println!("closest_depth: {shadow_map_depth}");
                let bias = (bias_max * (1.0 - surface_normal.dot(*light_direction))).max(*bias_min);

                if (fragment_depth - bias) >= shadow_map_depth {
                    shadow += 1.0
                }
                total += 1.0;
            }
        }
        shadow / total.max(1.0)
    }

    pub const POINT_SAMPLE_OFFSET_DIRECTIONS: [Vec3; 21] = [
        Vec3::ZERO,
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, -1.0, 1.0),
        Vec3::new(0.0, -1.0, -1.0),
        Vec3::new(0.0, 1.0, -1.0),
    ];
    /// Returns shadow _intensity_ for point lights.
    ///
    /// Returns `0.0` when the fragment is in full light.
    /// Returns `1.0` when the fragment is in full shadow.
    pub fn run_point<T, S>(
        &self,
        light_slab: &[u32],
        shadow_map: &T,
        shadow_map_sampler: &S,
        light_pos_in_world_space: Vec3,
    ) -> f32
    where
        S: IsSampler,
        T: Sample2dArray<Sampler = S>,
    {
        let ShadowCalculation {
            shadow_map_desc,
            shadow_map_atlas_size,
            frag_pos_in_world_space,
            surface_normal_in_world_space: surface_normal,
            frag_to_light_in_world_space: frag_to_light,
            bias_min,
            bias_max,
            pcf_samples,
        } = self;

        let light_to_frag_dir = frag_pos_in_world_space - light_pos_in_world_space;
        crate::println!("light_to_frag_dir: {light_to_frag_dir}");

        let pcf_samplesf = (*pcf_samples as f32)
            .max(1.0)
            .min(Self::POINT_SAMPLE_OFFSET_DIRECTIONS.len() as f32);
        let pcf_samples = pcf_samplesf as usize;
        let view_distance = light_to_frag_dir.length();
        let disk_radius = (1.0 + view_distance / shadow_map_desc.z_far) / 25.0;
        let mut shadow = 0.0f32;
        for i in 0..pcf_samples {
            let sample_offset = Self::POINT_SAMPLE_OFFSET_DIRECTIONS[i] * disk_radius;
            crate::println!("sample_offset: {sample_offset}");
            let sample_dir = (light_to_frag_dir + sample_offset).alt_norm_or_zero();
            let (face_index, uv) = CubemapDescriptor::get_face_index_and_uv(sample_dir);
            crate::println!("face_index: {face_index}",);
            crate::println!("uv: {uv}");
            let frag_pos_in_light_space = self.get_frag_pos_in_light_space(light_slab, face_index);
            let face_texture = self.get_atlas_texture_at(light_slab, face_index);
            let uv_tex = face_texture.uv(uv, *shadow_map_atlas_size);
            let shadow_map_depth = shadow_map.sample_by_lod(*shadow_map_sampler, uv_tex, 0.0).x;
            let fragment_depth = frag_pos_in_light_space.z;
            let bias = (bias_max * (1.0 - surface_normal.dot(*frag_to_light))).max(*bias_min);
            if (fragment_depth - bias) > shadow_map_depth {
                shadow += 1.0
            }
        }

        shadow / pcf_samplesf
    }
}

/// Depth pre-pass for the light tiling feature.
///
/// This shader writes all staged [`Renderlet`]'s depth into a buffer.
///
/// This shader is very much like [`shadow_mapping_vertex`], except that
/// shader gets its projection+view matrix from the light stored in a
/// `ShadowMapDescriptor`.
///
/// Here we want to render as normal forward pass would, with the `Renderlet`'s view
/// and the [`Camera`]'s projection.
///
/// ## Note
/// This shader will likely be expanded to include parts of occlusion culling and order
/// independent transparency.
#[spirv(vertex)]
pub fn light_tiling_depth_pre_pass(
    // Points at a `Renderlet`.
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the renderlet are we rendering?
    #[spirv(vertex_index)] vertex_index: u32,
    // The slab where the renderlet's geometry is staged
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    // Output clip coords
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let renderlet = geometry_slab.read_unchecked(renderlet_id);
    if !renderlet.visible {
        // put it outside the clipping frustum
        *out_clip_pos = Vec3::splat(100.0).extend(1.0);
        return;
    }

    let camera_id = geometry_slab
        .read_unchecked(Id::<GeometryDescriptor>::new(0) + GeometryDescriptor::OFFSET_OF_CAMERA_ID);
    let camera = geometry_slab.read_unchecked(camera_id);

    let (_vertex, _transform, _model_matrix, world_pos) =
        renderlet.get_vertex_info(vertex_index, geometry_slab);

    *out_clip_pos = camera.view_projection() * world_pos.extend(1.0);
}

pub type DepthImage2d = Image!(2D, type=f32, sampled, depth);

pub type DepthImage2dMultisampled = Image!(2D, type=f32, sampled, depth, multisampled=true);

/// A tile of screen space used to cull lights.
#[derive(Clone, Copy, Default, SlabItem)]
#[offsets]
pub struct LightTile {
    /// Minimum depth of objects found within the frustum of the tile.
    pub depth_min: u32,
    /// Maximum depth of objects foudn within the frustum of the tile.
    pub depth_max: u32,
    /// The count of lights in this tile.
    ///
    /// Also, the next available light index.
    pub next_light_index: u32,
    /// List of light ids that intersect this tile's frustum.
    pub lights_array: Array<Id<Light>>,
}

impl core::fmt::Debug for LightTile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LightTile")
            .field("depth_min", &dequantize_depth_u32_to_f32(self.depth_min))
            .field("depth_max", &dequantize_depth_u32_to_f32(self.depth_max))
            .field("next_light_index", &self.next_light_index)
            .field("lights_array", &self.lights_array)
            .finish()
    }
}

/// Descriptor of the light tiling operation, which culls lights by accumulating
/// them into lists that illuminate tiles of the screen.
#[derive(Clone, Copy, SlabItem, core::fmt::Debug)]
pub struct LightTilingDescriptor {
    /// Size of the [`Stage`]'s depth texture.
    pub depth_texture_size: UVec2,
    /// Configurable tile size.
    pub tile_size: u32,
    /// Array pointing to the lighting "tiles".
    pub tiles_array: Array<LightTile>,
    /// Minimum illuminance.
    ///
    /// Used to determine whether a light illuminates a tile.
    pub minimum_illuminance_lux: f32,
}

impl Default for LightTilingDescriptor {
    fn default() -> Self {
        Self {
            depth_texture_size: Default::default(),
            tile_size: 16,
            tiles_array: Default::default(),
            minimum_illuminance_lux: 0.1,
        }
    }
}

impl LightTilingDescriptor {
    /// Returns the dimensions of the grid of tiles.
    pub fn tile_grid_size(&self) -> UVec2 {
        let dims_f32 = self.depth_texture_size.as_vec2() / self.tile_size as f32;
        dims_f32.ceil().as_uvec2()
    }

    pub fn tile_coord_for_fragment(&self, frag_coord: Vec2) -> UVec2 {
        let frag_coord = frag_coord.as_uvec2();
        frag_coord / self.tile_size
    }

    pub fn tile_index_for_fragment(&self, frag_coord: Vec2) -> usize {
        let tile_coord = self.tile_coord_for_fragment(frag_coord);
        let tile_dimensions = self.tile_grid_size();
        (tile_coord.y * tile_dimensions.x + tile_coord.x) as usize
    }
}

/// Quantizes a fragment depth from `f32` to `u32`.
pub fn quantize_depth_f32_to_u32(depth: f32) -> u32 {
    (u32::MAX as f32 * depth).round() as u32
}

/// Reconstructs a previously quantized depth from a `u32`.
pub fn dequantize_depth_u32_to_f32(depth: u32) -> f32 {
    depth as f32 / u32::MAX as f32
}

/// Helper for determining the next light to check during an
/// invocation of the light list computation.
struct NextLightIndex {
    current_step: usize,
    tile_size: u32,
    lights: Array<Id<Light>>,
    global_id: UVec3,
}

impl Iterator for NextLightIndex {
    type Item = Id<Id<Light>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.next_index();
        self.current_step += 1;
        if next_index < self.lights.len() {
            Some(self.lights.at(next_index))
        } else {
            None
        }
    }
}

impl NextLightIndex {
    pub fn new(
        global_id: UVec3,
        tile_size: u32,
        analytical_lights_array: Array<Id<Light>>,
    ) -> Self {
        Self {
            current_step: 0,
            tile_size,
            lights: analytical_lights_array,
            global_id,
        }
    }

    pub fn next_index(&self) -> usize {
        // Determine the xy coord of this invocation within the _tile_
        let frag_tile_xy = self.global_id.xy() % self.tile_size;
        // Determine the index of this invocation within the _tile_
        let offset = frag_tile_xy.y * self.tile_size + frag_tile_xy.x;
        let stride = (self.tile_size * self.tile_size) as usize;
        self.current_step * stride + offset as usize
    }
}

struct LightTilingInvocation {
    global_id: UVec3,
    descriptor: LightTilingDescriptor,
}

impl LightTilingInvocation {
    fn new(global_id: UVec3, descriptor: LightTilingDescriptor) -> Self {
        Self {
            global_id,
            descriptor,
        }
    }

    /// The fragment's position.
    ///
    /// X range is 0 to (width - 1), Y range is 0 to (height - 1).
    fn frag_pos(&self) -> UVec2 {
        self.global_id.xy()
    }

    /// The number of tiles in X and Y within the depth texture.
    fn tile_grid_size(&self) -> UVec2 {
        self.descriptor.tile_grid_size()
    }

    /// The tile's coordinate among all tiles in the tile grid.
    ///
    /// The units are in tile x y.
    fn tile_coord(&self) -> UVec2 {
        self.global_id.xy() / self.descriptor.tile_size
    }

    /// The tile's index in all the [`LightTilingDescriptor`]'s `tile_array`.
    fn tile_index(&self) -> usize {
        let tile_pos = self.tile_coord();
        let tile_dimensions = self.tile_grid_size();
        (tile_pos.y * tile_dimensions.x + tile_pos.x) as usize
    }

    /// The tile's normalized min and max.
    fn tile_ndc_min_max(&self) -> (Vec2, Vec2) {
        let grid_size = self.tile_grid_size();
        let min_coord = self.tile_coord().as_vec2();
        let max_coord = min_coord + 1.0;
        let min = crate::math::convert_pixel_to_ndc(min_coord, grid_size);
        let max = crate::math::convert_pixel_to_ndc(max_coord, grid_size);
        (min, max)
    }

    /// The tile's normalized midpoint.
    fn tile_ndc_midpoint(&self) -> Vec2 {
        let min_coord = self.tile_coord().as_vec2();
        let mid_coord = min_coord + 0.5;
        crate::math::convert_pixel_to_ndc(mid_coord, self.tile_grid_size())
    }

    /// Compute the min and max depth of one fragment/invocation for light tiling.
    ///
    /// The min and max is stored in a tile on lighting slab.
    fn compute_min_and_max_depth(
        &self,
        depth_texture: &impl Fetch<UVec2, Output = Vec4>,
        lighting_slab: &mut [u32],
    ) {
        let frag_pos = self.frag_pos();
        let depth_texture_size = self.descriptor.depth_texture_size;
        if frag_pos.x >= depth_texture_size.x || frag_pos.y >= depth_texture_size.y {
            return;
        }
        // Depth frag value at the fragment position
        let frag_depth: f32 = depth_texture.fetch(frag_pos).x;
        // Fragment depth scaled to min/max of u32 values
        //
        // This is so we can compare with normal atomic ops instead of using the float extension
        let frag_depth_u32 = quantize_depth_f32_to_u32(frag_depth);

        // The tile's index in all the tiles
        let tile_index = self.tile_index();
        let lighting_desc = lighting_slab.read_unchecked(Id::<LightingDescriptor>::new(0));
        let tiling_desc = lighting_slab.read_unchecked(lighting_desc.light_tiling_descriptor_id);
        // index of the tile's min depth atomic value in the lighting slab
        let tile_id = tiling_desc.tiles_array.at(tile_index);
        let min_depth_index = tile_id + LightTile::OFFSET_OF_DEPTH_MIN;
        // index of the tile's max depth atomic value in the lighting slab
        let max_depth_index = tile_id + LightTile::OFFSET_OF_DEPTH_MAX;

        let _prev_min_depth = crate::sync::atomic_u_min::<
            { spirv_std::memory::Scope::Workgroup as u32 },
            { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
        >(lighting_slab, min_depth_index, frag_depth_u32);
        let _prev_max_depth = crate::sync::atomic_u_max::<
            { spirv_std::memory::Scope::Workgroup as u32 },
            { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
        >(lighting_slab, max_depth_index, frag_depth_u32);
    }

    /// Determine whether this invocation should run.
    fn should_invoke(&self) -> bool {
        self.global_id.x < self.descriptor.depth_texture_size.x
            && self.global_id.y < self.descriptor.depth_texture_size.y
    }

    /// Clears one tile.
    ///
    /// ## Note
    /// This is only valid to call from the [`light_tiling_clear_tiles`] shader.
    fn clear_tile(&self, lighting_slab: &mut [u32]) {
        let dimensions = self.tile_grid_size();
        let index = (self.global_id.y * dimensions.x + self.global_id.x) as usize;
        if index < self.descriptor.tiles_array.len() {
            let tile_id = self.descriptor.tiles_array.at(index);
            let mut tile = lighting_slab.read(tile_id);
            tile.depth_min = u32::MAX;
            tile.depth_max = 0;
            tile.next_light_index = 0;
            lighting_slab.write(tile_id, &tile);
            // Zero out the light list and the ratings
            for id in tile.lights_array.iter() {
                lighting_slab.write(id, &Id::NONE);
            }
        }
    }

    // The difficulty here is that in SPIRV we can access `lighting_slab` atomically without wrapping it
    // in a type, but on CPU we must pass an array of (something like) `AtomicU32`. I'm not sure how to
    // model this interaction to test it on the CPU.
    fn compute_light_lists(&self, geometry_slab: &[u32], lighting_slab: &mut [u32]) {
        let index = self.tile_index();
        let tile_id = self.descriptor.tiles_array.at(index);
        // Construct the tile's frustum in clip space.
        let depth_min_u32 = lighting_slab.read_unchecked(tile_id + LightTile::OFFSET_OF_DEPTH_MIN);
        let depth_max_u32 = lighting_slab.read_unchecked(tile_id + LightTile::OFFSET_OF_DEPTH_MAX);
        let depth_min = dequantize_depth_u32_to_f32(depth_min_u32);
        let depth_max = dequantize_depth_u32_to_f32(depth_max_u32);

        if depth_min == depth_max {
            // If we would construct a frustum with zero volume, abort.
            //
            // See <http://renderling.xyz/articles/live/light_tiling.html#zero-volume-frustum-optimization>
            // for more info.
            return;
        }

        let camera_id = geometry_slab.read_unchecked(
            Id::<GeometryDescriptor>::new(0) + GeometryDescriptor::OFFSET_OF_CAMERA_ID,
        );
        let camera = geometry_slab.read_unchecked(camera_id);

        // let (ndc_tile_min, ndc_tile_max) = self.tile_ndc_min_max();
        // // This is the AABB frustum, in NDC coords
        // let ndc_tile_aabb = Aabb::new(
        //     ndc_tile_min.extend(depth_min),
        //     ndc_tile_max.extend(depth_max),
        // );

        // Get the frustum (here simplified to a line) in world coords, since we'll be
        // using it to compare against the radius of illumination of each light
        let tile_ndc_midpoint = self.tile_ndc_midpoint();
        let tile_line_ndc = (
            tile_ndc_midpoint.extend(depth_min),
            tile_ndc_midpoint.extend(depth_max),
        );
        let inverse_viewproj = camera.view_projection().inverse();
        let tile_line = (
            inverse_viewproj.project_point3(tile_line_ndc.0),
            inverse_viewproj.project_point3(tile_line_ndc.1),
        );

        let tile_index = self.tile_index();
        let tile_id = self.descriptor.tiles_array.at(tile_index);
        let tile_lights_array = lighting_slab.read(tile_id + LightTile::OFFSET_OF_LIGHTS_ARRAY);
        let next_light_id = tile_id + LightTile::OFFSET_OF_NEXT_LIGHT_INDEX;

        // List of all analytical lights in the scene
        let analytical_lights_array = lighting_slab.read_unchecked(
            Id::<LightingDescriptor>::new(0)
                + LightingDescriptor::OFFSET_OF_ANALYTICAL_LIGHTS_ARRAY,
        );

        // Each invocation will calculate a few lights' contribution to the tile, until all lights
        // have been visited
        let next_light = NextLightIndex::new(
            self.global_id,
            self.descriptor.tile_size,
            analytical_lights_array,
        );
        for id_of_light_id in next_light {
            let light_id = lighting_slab.read_unchecked(id_of_light_id);
            let light = lighting_slab.read_unchecked(light_id);
            let transform = lighting_slab.read(light.transform_id);
            // Get the distance to the light in world coords, and the
            // intensity of the light.
            let (distance, intensity_candelas) = match light.light_type {
                LightStyle::Directional => {
                    let directional_light = lighting_slab.read(light.into_directional_id());
                    (0.0, directional_light.intensity)
                }
                LightStyle::Point => {
                    let point_light = lighting_slab.read(light.into_point_id());
                    let center = Mat4::from(transform).transform_point3(point_light.position);
                    let distance = crate::math::distance_to_line(center, tile_line.0, tile_line.1);
                    (distance, point_light.intensity)
                }
                LightStyle::Spot => {
                    // TODO: take into consideration the direction the spot light is pointing
                    let spot_light = lighting_slab.read(light.into_spot_id());
                    let center = Mat4::from(transform).transform_point3(spot_light.position);
                    let distance = crate::math::distance_to_line(center, tile_line.0, tile_line.1);
                    (distance, spot_light.intensity)
                }
            };

            let radius =
                radius_of_illumination(intensity_candelas, self.descriptor.minimum_illuminance_lux);
            let should_add = radius >= distance;
            if should_add {
                // If the light should be added to the bin, get the next available index in the bin,
                // then write the id of the light into that index.
                let next_index = crate::sync::atomic_i_increment::<
                    { spirv_std::memory::Scope::Workgroup as u32 },
                    { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                >(lighting_slab, next_light_id);
                if next_index as usize >= tile_lights_array.len() {
                    // We've already filled the bin, so abort.
                    //
                    // TODO: Figure out a better way to handle light tile list overrun.
                    break;
                } else {
                    // Get the id that corresponds to the next available index in the ratings bin
                    let binned_light_id = tile_lights_array.at(next_index as usize);
                    // Write to that location
                    lighting_slab.write(binned_light_id, &light_id);
                }
            }
        }
    }
}

#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_clear_tiles(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let lighting_descriptor = lighting_slab.read(Id::<LightingDescriptor>::new(0));
    let light_tiling_descriptor =
        lighting_slab.read(lighting_descriptor.light_tiling_descriptor_id);
    let invocation = LightTilingInvocation::new(global_id, light_tiling_descriptor);
    invocation.clear_tile(lighting_slab);
}

/// Compute the min and max depth value for a tile.
///
/// This shader must be called **once for each fragment in the depth texture**.
#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_compute_tile_min_and_max_depth(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 2)] depth_texture: &DepthImage2d,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let lighting_descriptor = lighting_slab.read(Id::<LightingDescriptor>::new(0));
    let light_tiling_descriptor =
        lighting_slab.read(lighting_descriptor.light_tiling_descriptor_id);
    let invocation = LightTilingInvocation::new(global_id, light_tiling_descriptor);
    invocation.compute_min_and_max_depth(depth_texture, lighting_slab);
}

/// Compute the min and max depth value for a tile, multisampled.
///
/// This shader must be called **once for each fragment in the depth texture**.
#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_compute_tile_min_and_max_depth_multisampled(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 2)] depth_texture: &DepthImage2dMultisampled,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let lighting_descriptor = lighting_slab.read(Id::<LightingDescriptor>::new(0));
    let light_tiling_descriptor =
        lighting_slab.read(lighting_descriptor.light_tiling_descriptor_id);
    let invocation = LightTilingInvocation::new(global_id, light_tiling_descriptor);
    invocation.compute_min_and_max_depth(depth_texture, lighting_slab);
}

#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_bin_lights(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let lighting_descriptor = lighting_slab.read(Id::<LightingDescriptor>::new(0));
    let light_tiling_descriptor =
        lighting_slab.read(lighting_descriptor.light_tiling_descriptor_id);
    let invocation = LightTilingInvocation::new(global_id, light_tiling_descriptor);
    if invocation.should_invoke() {
        invocation.compute_light_lists(geometry_slab, lighting_slab);
    }
}

#[cfg(test)]
mod test {
    use crate::math::GpuRng;

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
    /// Test that we can determine if a point is inside clip space or not.
    fn clip_space_bounds_sanity() {
        let inside = Vec3::ONE;
        assert!(
            crate::math::is_inside_clip_space(inside),
            "should be inside"
        );
        let inside = Vec3::new(0.5, -0.5, -0.8);
        assert!(
            crate::math::is_inside_clip_space(inside),
            "should be inside"
        );
        let outside = Vec3::new(0.5, 0.0, 1.3);
        assert!(
            !crate::math::is_inside_clip_space(outside),
            "should be outside"
        );
    }

    #[test]
    fn finding_orthogonal_vectors_sanity() {
        const THRESHOLD: f32 = f32::EPSILON * 3.0;

        let mut prng = GpuRng::new(0);
        for _ in 0..100 {
            let v2 = prng.gen_vec2(Vec2::splat(-100.0), Vec2::splat(100.0));
            let v2_ortho = v2.orthonormal_vectors();
            let v2_dot = v2.dot(v2_ortho);
            if v2_dot.abs() >= THRESHOLD {
                panic!("{v2} • {v2_ortho} < {THRESHOLD}, saw {v2_dot}")
            }

            let v3 = prng
                .gen_vec3(Vec3::splat(-100.0), Vec3::splat(100.0))
                .alt_norm_or_zero();
            for v3_ortho in v3.orthonormal_vectors() {
                let v3_dot = v3.dot(v3_ortho);
                if v3_dot.abs() >= THRESHOLD {
                    panic!("{v3} • {v3_ortho} < {THRESHOLD}, saw {v3_dot}");
                }
            }
        }
    }

    #[test]
    fn next_light_sanity() {
        {
            let lights_array = Array::new(0, 1);
            // When there's only one light we only need one invocation to check that one light
            // (per tile)
            let mut next_light = NextLightIndex::new(UVec3::new(0, 0, 0), 16, lights_array);
            assert_eq!(Some(0u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next invocation won't check anything
            let mut next_light = NextLightIndex::new(UVec3::new(1, 0, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
            // Neither will the next row
            let mut next_light = NextLightIndex::new(UVec3::new(0, 1, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
        }
        {
            let lights_array = Array::new(0, 2);
            // When there's two lights we need two invocations
            let mut next_light = NextLightIndex::new(UVec3::new(0, 0, 0), 16, lights_array);
            assert_eq!(Some(0u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next invocation checks the second light
            let mut next_light = NextLightIndex::new(UVec3::new(1, 0, 0), 16, lights_array);
            assert_eq!(Some(1u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next one doesn't check anything
            let mut next_light = NextLightIndex::new(UVec3::new(2, 0, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
        }
        {
            // With 256 lights (16*16), each fragment in the tile checks exactly one light
            let lights_array = Array::new(0, 16 * 16);
            let mut checked_lights = vec![];
            for y in 0..16 {
                for x in 0..16 {
                    let mut next_light = NextLightIndex::new(UVec3::new(x, y, 0), 16, lights_array);
                    let next_index = next_light.next_index();
                    let checked_light = next_light.next().unwrap();
                    assert_eq!(next_index, checked_light.index());
                    checked_lights.push(checked_light);
                    assert_eq!(None, next_light.next());
                }
            }
            println!("checked_lights: {checked_lights:#?}");
            assert_eq!(256, checked_lights.len());
        }
    }

    #[test]
    fn frag_coord_to_tile_index() {
        let tiling_desc = LightTilingDescriptor {
            depth_texture_size: UVec2::new(1024, 800),
            ..Default::default()
        };
        for x in 0..16 {
            let index = tiling_desc.tile_index_for_fragment(Vec2::new(x as f32, 0.0));
            assert_eq!(0, index);
        }
        let index = tiling_desc.tile_index_for_fragment(Vec2::new(16.0, 0.0));
        assert_eq!(1, index);
        let index = tiling_desc.tile_index_for_fragment(Vec2::new(0.0, 16.0));
        assert_eq!(1024 / 16, index);

        let tiling_desc = LightTilingDescriptor {
            depth_texture_size: UVec2::new(
                (10.0 * 2.0f32.powi(8)) as u32,
                (9.0 * 2.0f32.powi(8)) as u32,
            ),
            ..Default::default()
        };
        let frag_coord = Vec2::new(1917.0, 979.0);
        let tile_coord = tiling_desc.tile_coord_for_fragment(frag_coord);
        assert_eq!(UVec2::new(119, 61), tile_coord);
    }

    #[test]
    fn light_tiling_invocation_tile_aabb() {
        let invocation = LightTilingInvocation {
            global_id: UVec3::new(446, 1342, 1),
            descriptor: LightTilingDescriptor {
                depth_texture_size: UVec2::new(2560, 2304),
                ..Default::default()
            },
        };
        let (min, max) = invocation.tile_ndc_min_max();
        println!("min: {min}");
        println!("max: {max}");
        assert!(min.x < 0.0, "min: {min}");
        assert!(min.y < 0.0, "min: {min}");
    }
}
