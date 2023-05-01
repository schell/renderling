//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use bitflags::bitflags;
use glam::{Mat3, Mat4, Quat, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

use crate::{math::Vec3ColorSwizzles, pbr, phong};

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
        }
    }
}

impl GpuVertex {
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
}

/// A light capable of representing a directional, point or spotlight.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuLight {
    pub position: Vec4,
    pub direction: Vec4,
    pub attenuation: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub light_type: u32,
    pub _padding0: u32,
}

impl GpuLight {
    pub const END_OF_LIGHTS: u32 = 0;
    pub const POINT_LIGHT: u32 = 1;
    pub const SPOT_LIGHT: u32 = 2;
    pub const DIRECTIONAL_LIGHT: u32 = 3;
}

/// A GPU texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuTexture {
    // top left offset of texture in the atlas
    pub offset_px: Vec2,
    // size of texture in the atlas
    pub size_px: Vec2,
}

impl GpuTexture {
    pub fn uv(&self, uv: Vec2, atlas_size: Vec2) -> Vec2 {
        let x = self.offset_px.x;
        let y = self.offset_px.y;
        let w = self.size_px.x;
        let h = self.size_px.y;
        let img_origin = Vec2::new(x, y);
        let img_size = Vec2::new(w, h);
        // convert the uv from normalized into pixel locations in the original image
        let uv_img_pixels = uv * img_size;
        // convert those into pixel locations in the atlas image
        let uv_atlas_pixels = img_origin + uv_img_pixels;
        // normalize the uvs by the atlas size
        let uv_atlas_normalized = uv_atlas_pixels / atlas_size;
        uv_atlas_normalized
    }
}

/// `u32` representing "null" or "none".
pub const ID_NONE: u32 = u32::MAX;

bitflags! {
    pub struct GpuMaterialConfig: u32 {
        /// Whether texture0 is used
        const TEXTURE0 = 1;
        /// Whether texture1 is used
        const TEXTURE1 = 1 << 1;
        /// Whether texture2 is used
        const TEXTURE2 = 1 << 2;
    }
}

impl GpuMaterialConfig {
    pub fn texture0_used(&self) -> bool {
        self.contains(Self::TEXTURE0)
    }

    pub fn texture1_used(&self) -> bool {
        self.contains(Self::TEXTURE1)
    }

    pub fn texture2_used(&self) -> bool {
        self.contains(Self::TEXTURE2)
    }
}

/// Determines the lighting to use in an ubershader.
#[repr(transparent)]
#[derive(
    Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug, bytemuck::Pod, bytemuck::Zeroable,
)]
pub struct LightingModel(u32);

impl LightingModel {
    pub const NO_LIGHTING: Self = LightingModel(0);
    pub const TEXT_LIGHTING: Self = LightingModel(1);
    pub const PHONG_LIGHTING: Self = LightingModel(2);
    pub const PBR_LIGHTING: Self = LightingModel(3);
}

/// Represents a material on the GPU.
///
/// `GpuMaterial` is capable of representing many material types.
/// Use the appropriate builder for your material type from
/// [`SceneBuilder`](crate::SceneBuilder).
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuMaterial {
    pub factor0: Vec4,
    pub factor1: Vec4,

    pub texture0: u32,
    pub texture1: u32,
    pub texture2: u32,

    pub lighting_model: LightingModel,
}

impl Default for GpuMaterial {
    fn default() -> Self {
        Self {
            factor0: Vec4::ONE,
            factor1: Vec4::ONE,
            texture0: ID_NONE,
            texture1: ID_NONE,
            texture2: ID_NONE,
            lighting_model: LightingModel::NO_LIGHTING,
        }
    }
}

/// A bundle of GPU components.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuEntity {
    // The id of this entity. `ID_MAX` means this entity is not in use.
    pub id: u32,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // The index/id of this entity's material in the material buffer.
    pub material: u32,
    // The id of this entity's parent, if it exists. `ID_NONE` means "no parent".
    pub parent: u32,
    // Whether this entity is visible. `0` is "not visible", any other value is "visible".
    pub visible: u32,
    pub padding0: [u32; 2],
    // The local translation of this entity
    pub position: Vec4,
    // The local scale of this entity
    pub scale: Vec4,
    // The local rotation of this entity
    pub rotation: Quat,
}

impl Default for GpuEntity {
    fn default() -> Self {
        Self {
            id: ID_NONE,
            mesh_first_vertex: 0,
            mesh_vertex_count: 0,
            material: ID_NONE,
            position: Vec4::ZERO,
            scale: Vec4::ONE,
            rotation: Quat::IDENTITY,
            visible: 1,
            padding0: [0, 0],
            parent: ID_NONE,
        }
    }
}

impl GpuEntity {
    pub fn is_alive(&self) -> bool {
        self.id != ID_NONE
    }

    /// Return the position, rotation and scale that describe this entity's
    /// transform in world space.
    pub fn get_world_transform(&self, entities: &[GpuEntity]) -> (Vec3, Quat, Vec3) {
        let mut position = Vec3::ZERO;
        let mut scale = Vec3::ONE;
        let mut rotation = Quat::IDENTITY;
        let mut index = self.id as usize;
        loop {
            let entity = entities[index];
            position += entity.position.xyz();
            scale *= entity.scale.xyz();
            rotation = entity.rotation * rotation;
            index = entity.parent as usize;
            if index >= entities.len() {
                break;
            }
        }
        (position, rotation, scale)
    }
}

/// Unforms/constants for a scene's worth of rendering.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuConstants {
    pub camera_projection: Mat4,
    pub camera_view: Mat4,
    pub camera_pos: Vec4,
    pub atlas_size: Vec2,
    pub padding: Vec2,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

/// Scene vertex shader.
pub fn main_vertex_scene(
    // which entity are we drawing
    instance_index: u32,
    // which vertex are we drawing
    vertex_index: u32,

    constants: &GpuConstants,
    vertices: &[GpuVertex],
    entities: &[GpuEntity],
    materials: &[GpuMaterial],
    textures: &[GpuTexture],

    out_material_config: &mut u32,
    out_material_lighting_model: &mut LightingModel,
    out_color: &mut Vec4,
    // material
    out_uv0: &mut Vec2,
    out_factor0: &mut Vec4,
    out_uv1: &mut Vec2,
    out_factor1: &mut Vec4,

    out_norm: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,

    gl_pos: &mut Vec4,
) {
    let entity = entities[instance_index as usize];
    let vertex = vertices[vertex_index as usize];
    let (position, rotation, scale) = entity.get_world_transform(entities);
    let model_matrix =
        Mat4::from_translation(position) * Mat4::from_quat(rotation) * Mat4::from_scale(scale);
    let material = if entity.material == ID_NONE {
        GpuMaterial::default()
    } else {
        materials[entity.material as usize]
    };

    let mut material_config = GpuMaterialConfig::empty();
    *out_color = vertex.color;
    *out_material_lighting_model = material.lighting_model;
    *out_factor0 = material.factor0;
    *out_factor1 = material.factor1;
    *out_uv0 = if material.texture0 == ID_NONE {
        Vec2::ZERO
    } else {
        material_config |= GpuMaterialConfig::TEXTURE0;
        let texture0 = textures[material.texture0 as usize];
        texture0.uv(vertex.uv.xy(), constants.atlas_size)
    };
    *out_uv1 = if material.texture1 == ID_NONE {
        Vec2::ZERO
    } else {
        material_config |= GpuMaterialConfig::TEXTURE1;
        let texture1 = textures[material.texture1 as usize];
        texture1.uv(vertex.uv.zw(), constants.atlas_size)
    };
    *out_material_config = material_config.bits();
    *out_norm =
        (Mat3::from_mat4(model_matrix) * (vertex.normal.xyz() / (scale * scale))).normalize();

    let view_pos = model_matrix * vertex.position.xyz().extend(1.0);
    *out_pos = view_pos.xyz();
    *gl_pos = constants.camera_projection * constants.camera_view * view_pos;
}

/// Scene fragment shader.
pub fn main_fragment_scene(
    atlas: &Image2d,
    sampler: &Sampler,

    constants: &GpuConstants,
    lights: &[GpuLight],

    in_material_config: u32,
    in_material_lighting_model: LightingModel,
    in_color: Vec4,
    in_uv0: Vec2,
    in_factor0: Vec4,
    in_uv1: Vec2,
    in_factor1: Vec4,
    in_norm: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) {
    let config = GpuMaterialConfig::from_bits_retain(in_material_config);

    let mut uv0_color: Vec4 = atlas.sample(*sampler, in_uv0);
    if !config.texture0_used() {
        uv0_color = Vec4::splat(1.0);
    }

    let mut uv1_color: Vec4 = atlas.sample(*sampler, in_uv1);
    if !config.texture1_used() {
        uv1_color = Vec4::splat(1.0);
    }

    *output = match in_material_lighting_model {
        LightingModel::PBR_LIGHTING => {
            let albedo = uv0_color * in_factor0 * in_color;
            let metallic = uv1_color.y * in_factor1.y;
            let roughness = uv1_color.z * in_factor1.z;
            let ao = 1.0;
            pbr::shade_fragment(
                constants.camera_pos.xyz(),
                in_norm,
                in_pos,
                albedo.rgb(),
                metallic,
                roughness,
                ao,
                lights,
            )
        }
        LightingModel::PHONG_LIGHTING => {
            let diffuse_color: Vec4 = uv0_color * in_color;
            let specular_color: Vec4 = uv1_color * in_color;
            phong::shade_fragment(
                &constants.camera_view,
                lights,
                diffuse_color,
                specular_color,
                in_pos,
                in_norm,
            )
        }
        LightingModel::TEXT_LIGHTING => in_color * Vec3::splat(1.0).extend(uv0_color.x),
        _unlit => in_color * uv0_color * in_factor0 * uv1_color,
    };
}

/// Compute the draw calls for this frame.
///
/// This should be called with `groupcount = (entities.len() / threads) + 1`.
pub fn compute_cull_entities(entities: &[GpuEntity], draws: &mut [DrawIndirect], global_id: UVec3) {
    let i = global_id.x as usize;

    if i > entities.len() {
        return;
    }

    // when the vertex count and/or instance count is 0, it effectively filters
    // the draw call
    let mut call = DrawIndirect {
        vertex_count: 0,
        instance_count: 0,
        base_vertex: 0,
        base_instance: i as u32,
    };
    let entity = &entities[i];
    let is_visible = entity.visible != 0;
    if entity.is_alive() && is_visible {
        //// once naga supports atomics we can use this to compact the array
        // let index = unsafe {
        //    spirv_std::arch::atomic_i_increment::<
        //        u32,
        //        { spirv_std::memory::Scope::Device as u32 },
        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
        //    >(count)
        //};
        call.instance_count = 1;
        call.base_vertex = entity.mesh_first_vertex;
        call.vertex_count = entity.mesh_vertex_count;
    }
    draws[i] = call;
}
