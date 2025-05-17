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
    bvol::{Aabb, BoundingSphere},
    cubemap::{CubemapDescriptor, CubemapFaceDirection},
    geometry::GeometryDescriptor,
    math::{Fetch, IsAtomicSlab, IsSampler, IsVector, Sample2dArray},
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
        let view = Mat4::look_to_rh(position, direction, Vec3::Z);
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
        let depth = (z_far - z_near).abs();
        let hd = depth * 0.5;
        let projection = Mat4::orthographic_rh(-hd, hd, -hd, hd, z_near, z_far);
        let direction = parent_light_transform
            .transform_vector3(self.direction)
            .alt_norm_or_zero();
        let position = -direction * depth * 0.5;
        crate::println!("direction: {direction}");
        crate::println!("position: {position}");
        let view = Mat4::look_to_rh(position, direction, Vec3::Z);
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

    /// Returns the radius of illumination in meters.
    ///
    ///
    /// • General indoor lighting: Around 100 to 300 lux.                                   
    /// • Office lighting: Typically around 300 to 500 lux.                                 
    /// • Reading or task lighting: Around 500 to 750 lux.                                  
    /// • Detailed work (e.g., drafting, surgery): 1000 lux or more.
    pub fn radius_of_illumination(&self, minimum_illuminance_lux: f32) -> f32 {
        (self.intensity / minimum_illuminance_lux).sqrt()
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

/// A type-erased/generic light that is used as a slab pointer to a
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

/// Marker trait for abstracting over depth texture types.
pub trait IsDepth {
    type Texture;
}

pub struct DepthImage2d;
impl IsDepth for DepthImage2d {
    type Texture = Image!(2D, type=f32, sampled, depth);
}

pub struct DepthImage2dMultisampled;
impl IsDepth for DepthImage2dMultisampled {
    type Texture = Image!(2D, type=f32, sampled, depth, multisampled=true);
}

/// A tile of screen space used to cull lights.
#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
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

/// Descriptor of the light tiling operation, which culls lights by accumulating
/// them into lists that illuminate tiles of the screen.
#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
pub struct LightTilingDescriptor {
    pub depth_texture_size: UVec2,
    /// Array pointing to the lighting "tiles".
    pub tiles_array: Array<LightTile>,
}

impl LightTilingDescriptor {
    pub const TILE_SIZE: UVec2 = UVec2::new(16, 16);

    pub fn tile_dimensions(&self) -> UVec2 {
        let x = (self.depth_texture_size.x as f32 / Self::TILE_SIZE.x as f32).ceil();
        let y = (self.depth_texture_size.y as f32 / Self::TILE_SIZE.y as f32).ceil();
        UVec2::new(x as u32, y as u32)
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
    fn tile_dimensions(&self) -> UVec2 {
        self.descriptor.tile_dimensions()
    }

    /// The tile's position among all tiles.
    fn tile_pos(&self) -> UVec2 {
        self.global_id.xy() / LightTilingDescriptor::TILE_SIZE
    }

    /// The tile's index in all the [`LightTilingDescriptor`]'s `tile_array`.
    fn tile_index(&self) -> usize {
        let tile_pos = self.tile_pos();
        let tile_dimensions = self.tile_dimensions();
        (tile_pos.y * tile_dimensions.x + tile_pos.x) as usize
    }

    /// The index of the fragment within its tile.
    ///
    /// 1. The fragment position is determined from the global invocation index.
    /// 2. The fragment's relative xy position within the tile is determined.
    /// 3. The index of the fragment's tile position is calculated and returned.
    fn frag_index(&self) -> usize {
        // The fragment's xy position within its tile
        let frag_tile = self.frag_pos() % LightTilingDescriptor::TILE_SIZE;
        // The fragment's index in the tile, which will be 0..256 if TILE_SIZE is 16x16
        (frag_tile.y * LightTilingDescriptor::TILE_SIZE.x + frag_tile.x) as usize
    }

    /// Compute the min and max depth of one fragment/invocation for light tiling.
    ///
    /// Returns the **indices** of the min and max depths of the tile.
    fn compute_min_and_max_depth<S: IsAtomicSlab + ?Sized>(
        &self,
        depth_texture: &impl Fetch<UVec2, Output = Vec4>,
        _lighting_slab: &[u32],
        tiling_slab: &mut S,
    ) -> (Id<u32>, Id<u32>) {
        let frag_pos = self.frag_pos();
        // Depth frag value at the fragment position
        let frag_depth: f32 = depth_texture.fetch(frag_pos).x;
        // Fragment depth scaled to min/max of u32 values
        //
        // This is so we can compare with normal atomic ops instead of using the float extension
        let frag_depth_u32: u32 = (u32::MAX as f32 * frag_depth).round() as u32;

        // The tile's index in all the tiles
        let tile_index = self.tile_index();
        let tiling_desc = tiling_slab.read_unchecked(Id::<LightTilingDescriptor>::new(0));
        // index of the tile's min depth atomic value in the tiling slab
        let tile_id = tiling_desc.tiles_array.at(tile_index);
        let min_depth_index = tile_id + LightTile::OFFSET_OF_DEPTH_MIN;
        // index of the tile's max depth atomic value in the tiling slab
        let max_depth_index = tile_id + LightTile::OFFSET_OF_DEPTH_MAX;

        let _prev_min_depth = tiling_slab.atomic_u_min::<
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
        >(min_depth_index, frag_depth_u32);
        let _prev_max_depth = tiling_slab.atomic_u_max::<
                { spirv_std::memory::Scope::Workgroup as u32 },
                { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() }
            >(max_depth_index, frag_depth_u32);

        (min_depth_index, max_depth_index)
    }

    /// Determine whether this invocation should run.
    fn should_invoke(&self) -> bool {
        self.global_id.x < self.descriptor.depth_texture_size.x
            && self.global_id.y < self.descriptor.depth_texture_size.y
    }

    /// Returns whether this invocation's X and Y coords are divided
    /// evenly by the tile size.
    fn frag_pos_is_tile_corner(&self) -> bool {
        let frag_pos = self.frag_pos();
        frag_pos.x % LightTilingDescriptor::TILE_SIZE.x == 0
            && frag_pos.y % LightTilingDescriptor::TILE_SIZE.y == 0
    }

    fn clear_tiles<S: IsAtomicSlab + ?Sized>(&self, tiling_slab: &mut S) {
        if self.frag_pos_is_tile_corner() {
            // only continue if this is the invocation in the top-left of the tile, as
            // we only need one invocation per tile.
            let tile_index = self.tile_index();
            let tile_id = self.descriptor.tiles_array.at(tile_index);

            {
                let mut tile = tiling_slab.read(tile_id);
                tile.depth_min = u32::MAX;
                tile.depth_max = 0;
                tile.next_light_index = 0;
                tiling_slab.write(tile_id, &tile);
            }

            // index of the tile's min depth atomic value in the tiling slab
            let min_depth_index = (tile_id + LightTile::OFFSET_OF_DEPTH_MIN).index();
            // index of the tile's max depth atomic value in the tiling slab
            let max_depth_index = (tile_id + LightTile::OFFSET_OF_DEPTH_MAX).index();

            tiling_slab.write(min_depth_index.into(), &u32::MAX);
            tiling_slab.write(max_depth_index.into(), &0u32);
        }
    }

    // The difficulty here is that in SPIRV we can access `tiling_slab` atomically without wrapping it
    // in a type, but on CPU we must pass an array of (something like) `AtomicU32`. I'm not sure how to
    // model this interaction to test it on the CPU. I think I'll need something like `AtomicSlab`.
    fn compute_light_lists<S: IsAtomicSlab + ?Sized>(
        &self,
        geometry_slab: &[u32],
        lighting_slab: &[u32],
        tiling_slab: &mut S,
        min_depth_index: Id<u32>,
        max_depth_index: Id<u32>,
    ) {
        // At this point we know the depth has been computed, so now we can construct the tile's frustum
        // in clip space.
        let depth_min_u32 = tiling_slab.read_unchecked(min_depth_index);
        let depth_max_u32 = tiling_slab.read_unchecked(max_depth_index);
        let depth_min = depth_min_u32 as f32 / u32::MAX as f32;
        let depth_max = depth_max_u32 as f32 / u32::MAX as f32;

        let resolution = self.descriptor.depth_texture_size.as_vec2();
        let tile_tl_screen_space = self.tile_pos().as_vec2() * resolution;
        // The tile's aabb in screen space / viewport space.
        //
        // This is roughly the frustum.
        let tile_aabb_ss = {
            let min = tile_tl_screen_space.extend(depth_min);
            let max = (tile_tl_screen_space + LightTilingDescriptor::TILE_SIZE.as_vec2())
                .extend(depth_max);
            Aabb { min, max }
        };

        let tile_index = self.tile_index();
        let tile_id = self.descriptor.tiles_array.at(tile_index);
        let tile_lights_array = tiling_slab.read(tile_id + LightTile::OFFSET_OF_LIGHTS_ARRAY);
        let next_light_id = tile_id + LightTile::OFFSET_OF_NEXT_LIGHT_INDEX;

        let camera_id = geometry_slab.read_unchecked(
            Id::<GeometryDescriptor>::new(0) + GeometryDescriptor::OFFSET_OF_CAMERA_ID,
        );
        let camera = geometry_slab.read_unchecked(camera_id);

        let index_of_invocation_in_tile = self.frag_index();
        // List of all analytical lights in the scene
        let analytical_lights_array = lighting_slab.read_unchecked(
            Id::<LightingDescriptor>::new(0)
                + LightingDescriptor::OFFSET_OF_ANALYTICAL_LIGHTS_ARRAY,
        );
        // The number of fragments in one tile
        let count_of_fragments_in_one_tile =
            (LightTilingDescriptor::TILE_SIZE.x * LightTilingDescriptor::TILE_SIZE.y) as usize;
        // Each invocation will calculate a few lights' contribution to the tile, until all lights
        // have been visited
        for step in 0..(analytical_lights_array.len() / count_of_fragments_in_one_tile) + 1 {
            let light_index = step * count_of_fragments_in_one_tile + index_of_invocation_in_tile;
            if light_index >= analytical_lights_array.len() {
                break;
            }
            let light_id = lighting_slab.read_unchecked(analytical_lights_array.at(light_index));
            let light = lighting_slab.read_unchecked(light_id);
            let transform = geometry_slab.read(light.transform_id);
            let should_add = match light.light_type {
                LightStyle::Directional => true,
                LightStyle::Point => {
                    let point_light = lighting_slab.read(light.into_point_id());
                    crate::println!("transform: {transform:?}");
                    if transform.translation.x.is_nan() {
                        crate::println!("step: {step}");
                        crate::println!("light_index: {light_index}");
                        crate::println!("light_id: {light_id:?}");
                        crate::println!(
                            "analytical_lights_array.len: {}",
                            analytical_lights_array.len()
                        );
                        crate::println!("transform_id: {:?}", light.transform_id);
                    }
                    let center = Mat4::from(transform).transform_point3(point_light.position);
                    let radius = point_light.radius_of_illumination(1.0);
                    let sphere = BoundingSphere::new(center, radius);
                    let aabb_ss = sphere.project_onto_viewport(&camera, resolution);
                    aabb_ss.intersects_aabb(&tile_aabb_ss)
                }
                LightStyle::Spot => false,
            };

            if should_add {
                let next_index = tiling_slab.atomic_i_increment::<
                    { spirv_std::memory::Scope::Workgroup as u32 },
                    { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                >(next_light_id);
                if next_index as usize >= tile_lights_array.len() {
                    break;
                }
                tiling_slab.write(next_index.into(), &light_id);
            }
        }
    }

    // TODO: think about breaking the light tiling "compute tiles" shader up into sub-shaders.
    // It would also be possible to join or parallelize some of this work with frustum culling
    // and occlusion culling.
    fn compute_tiles<S: IsAtomicSlab + ?Sized>(
        &self,
        depth_texture: &impl Fetch<UVec2, Output = Vec4>,
        geometry_slab: &[u32],
        lighting_slab: &[u32],
        tiling_slab: &mut S,
    ) {
        self.clear_tiles(tiling_slab);
        #[cfg(gpu)]
        unsafe {
            spirv_std::arch::workgroup_memory_barrier_with_group_sync();
        }
        let (min_index, max_index) =
            self.compute_min_and_max_depth(depth_texture, lighting_slab, tiling_slab);
        #[cfg(gpu)]
        unsafe {
            spirv_std::arch::workgroup_memory_barrier_with_group_sync();
        }
        self.compute_light_lists(
            geometry_slab,
            lighting_slab,
            tiling_slab,
            min_index,
            max_index,
        );
    }
}

pub fn light_tiling_compute_tiles_impl(
    geometry_slab: &[u32],
    lighting_slab: &[u32],
    tiling_slab: &mut [u32],
    depth_texture: &impl Fetch<UVec2, Output = Vec4>,
    global_id: UVec3,
) {
    let descriptor = tiling_slab.read(Id::<LightTilingDescriptor>::new(0));
    let invocation = LightTilingInvocation::new(global_id, descriptor);
    if invocation.should_invoke() {
        invocation.compute_tiles(depth_texture, geometry_slab, lighting_slab, tiling_slab);
    }
}

/// Light culling compute shader, **without** a multisampled depth texture.
// TODO: this shader does not need the geometry slab, as the size is held in the
// tiling slab.
#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_compute_tiles(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] tiling_slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 3)] depth_texture: &<DepthImage2d as IsDepth>::Texture,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    light_tiling_compute_tiles_impl(
        geometry_slab,
        lighting_slab,
        tiling_slab,
        depth_texture,
        global_id,
    )
}

/// Light culling compute shader, with a multisampled depth texture.
#[spirv(compute(threads(16, 16, 1)))]
pub fn light_tiling_compute_tiles_multisampled(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] geometry_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lighting_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] tiling_slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 3)]
    depth_texture: &<DepthImage2dMultisampled as IsDepth>::Texture,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    light_tiling_compute_tiles_impl(
        geometry_slab,
        lighting_slab,
        tiling_slab,
        depth_texture,
        global_id,
    )
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
    fn light_tile_fragment_indices() {
        let descriptor = LightTilingDescriptor {
            depth_texture_size: UVec2::splat(200),
            tiles_array: Default::default(),
        };
        assert_eq!(
            0,
            LightTilingInvocation::new(UVec3::ZERO, descriptor).frag_index()
        );
        assert_eq!(
            1,
            LightTilingInvocation::new(UVec3::new(1, 0, 0), descriptor).frag_index()
        );
        assert_eq!(
            2,
            LightTilingInvocation::new(UVec3::new(2, 0, 0), descriptor).frag_index()
        );
        assert_eq!(
            3,
            LightTilingInvocation::new(UVec3::new(3, 0, 0), descriptor).frag_index()
        );
        assert_eq!(
            16,
            LightTilingInvocation::new(UVec3::new(0, 1, 0), descriptor).frag_index()
        );
        assert_eq!(
            17,
            LightTilingInvocation::new(UVec3::new(1, 1, 0), descriptor).frag_index()
        );
        assert_eq!(
            18,
            LightTilingInvocation::new(UVec3::new(2, 1, 0), descriptor).frag_index()
        );
    }
}
