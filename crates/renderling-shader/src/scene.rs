//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{Mat3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles, mat3};
use spirv_std::{image::Image2d, Sampler};

use crate::{math::Vec3ColorSwizzles, pbr, phong};

mod id;
pub use id::*;

mod wrap;
pub use wrap::*;

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y
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

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
    }
}

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightType(u32);

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Display for LightType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            &Self::END_OF_LIGHTS => "end of lights",
            &Self::POINT_LIGHT => "point light",
            &Self::SPOT_LIGHT => "spot light",
            &Self::DIRECTIONAL_LIGHT => "directional light",
            _ => "unsupported light",
        };
        f.write_str(s)
    }
}

impl LightType {
    pub const END_OF_LIGHTS: Self = Self(0);
    pub const POINT_LIGHT: Self = Self(1);
    pub const SPOT_LIGHT: Self = Self(2);
    pub const DIRECTIONAL_LIGHT: Self = Self(3);
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
    pub light_type: LightType,
    pub _padding0: u32,
}

/// A GPU texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuTexture {
    // The top left offset of texture in the atlas
    pub offset_px: UVec2,
    // The size of texture in the atlas
    pub size_px: UVec2,
    // How `s` edges should be handled in texture addressing.
    pub mode_s: TextureAddressMode,
    // How `t` edges should be handled in texture addressing.
    pub mode_t: TextureAddressMode,
}

impl GpuTexture {
    /// Transform the given `uv` coordinates for this texture's address mode
    /// and placement in the atlas of the given size.
    pub fn uv(&self, mut uv: Vec2, atlas_size: UVec2) -> Vec2 {
        uv.x = wrap::wrap(uv.x, self.mode_s);
        uv.y = wrap::wrap(uv.y, self.mode_t);

        // get the pixel index of the uv coordinate in terms of the original image
        let mut px_index_s = (uv.x * self.size_px.x as f32) as u32;
        let mut px_index_t = (uv.y * self.size_px.y as f32) as u32;

        // convert the pixel index from image to atlas space
        px_index_s += self.offset_px.x;
        px_index_t += self.offset_px.y;

        let sx = atlas_size.x as f32;
        let sy = atlas_size.y as f32;
        // normalize the pixels by dividing by the atlas size
        let uv_s = px_index_s as f32 / sx;
        let uv_t = px_index_t as f32 / sy;

        let st = Vec2::new(uv_s, uv_t);
        st
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

    pub texture0: Id<GpuTexture>,
    pub texture1: Id<GpuTexture>,
    pub texture2: Id<GpuTexture>,
    pub texture3: Id<GpuTexture>,

    pub texture0_tex_coord: u32,
    pub texture1_tex_coord: u32,
    pub texture2_tex_coord: u32,
    pub texture3_tex_coord: u32,

    pub lighting_model: LightingModel,
    pub padding: [u32; 3]
}

impl Default for GpuMaterial {
    fn default() -> Self {
        Self {
            factor0: Vec4::ONE,
            factor1: Vec4::ONE,
            texture0: Id::NONE,
            texture1: Id::NONE,
            texture2: Id::NONE,
            texture3: Id::NONE,
            texture0_tex_coord: 0,
            texture1_tex_coord: 0,
            texture2_tex_coord: 0,
            texture3_tex_coord: 0,
            lighting_model: LightingModel::NO_LIGHTING,
            padding: [0; 3]
        }
    }
}

/// A bundle of GPU components.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuEntity {
    // The id of this entity. `Id::NONE` means this entity is not in use.
    pub id: Id<GpuEntity>,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // The index/id of this entity's material in the material buffer.
    pub material: Id<GpuMaterial>,
    // The id of this entity's parent, if it exists. `Id::NONE` means "no parent".
    pub parent: Id<GpuEntity>,
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
            id: Id::NONE,
            mesh_first_vertex: 0,
            mesh_vertex_count: 0,
            material: Id::NONE,
            position: Vec4::ZERO,
            scale: Vec4::ONE,
            rotation: Quat::IDENTITY,
            visible: 1,
            padding0: [0, 0],
            parent: Id::NONE,
        }
    }
}

impl From<&GpuEntity> for Id<GpuEntity> {
    fn from(value: &GpuEntity) -> Self {
        value.id
    }
}

impl From<GpuEntity> for Id<GpuEntity> {
    fn from(value: GpuEntity) -> Self {
        value.id
    }
}

impl GpuEntity {
    pub fn is_alive(&self) -> bool {
        !self.id.is_none()
    }

    /// Return the position, rotation and scale that describe this entity's
    /// transform in world space.
    pub fn get_world_transform(&self, entities: &[GpuEntity]) -> (Vec3, Quat, Vec3) {
        let mut position = Vec3::ZERO;
        let mut scale = Vec3::ONE;
        let mut rotation = Quat::IDENTITY;
        let mut id = self.id;
        loop {
            let entity = entities[id.index()];
            position += entity.position.xyz();
            scale *= entity.scale.xyz();
            rotation = entity.rotation * rotation;
            id = entity.parent;
            if id.index() >= entities.len() {
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
    pub atlas_size: UVec2,
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

fn texture_color(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &Image2d,
    sampler: &Sampler,
    atlas_size: UVec2,
    textures: &[GpuTexture],
) -> Vec4 {
    let texture = if texture_id.is_some() {
        GpuTexture::default()
    } else {
        textures[texture_id.index()]
    };

    let uv = texture.uv(uv, atlas_size);
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
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

    out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,

    gl_pos: &mut Vec4,
) {
    let entity = entities[instance_index as usize];
    let vertex = vertices[vertex_index as usize];
    let (position, rotation, scale) = entity.get_world_transform(entities);
    let model_matrix =
        Mat4::from_translation(position) * Mat4::from_quat(rotation) * Mat4::from_scale(scale);

    *out_material = entity.material.into();
    *out_color = vertex.color;
    *out_uv0 = vertex.uv.xy();
    *out_uv1 = vertex.uv.zw();

    let scale2 = scale * scale;
    let model_matrix3 = Mat3::from_mat4(model_matrix);
    let t = (model_matrix3 * (vertex.tangent.xyz() / scale2)).normalize_or_zero();
    let n = (model_matrix3 * (vertex.normal.xyz() / scale2)).normalize_or_zero();
    let b = t.cross(n) * vertex.tangent.w;

    *out_tangent = t;
    *out_bitangent = b;
    *out_norm = n;

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
    materials: &[GpuMaterial],
    textures: &[GpuTexture],

    in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) {
    let material = if in_material == ID_NONE {
        GpuMaterial::default()
    } else {
        materials[in_material as usize]
    };

    let texture0_uv = if material.texture0_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color0 = texture_color(
        material.texture0,
        texture0_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let texture1_uv = if material.texture1_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color1 = texture_color(
        material.texture1,
        texture1_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let texture2_uv = if material.texture2_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color2 = texture_color(
        material.texture2,
        texture2_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let norm = if material.texture2.is_none() {
        // there is no normal map, use the normal normal ;)
        in_norm
    } else {
        let tbn = mat3(in_tangent, in_bitangent, in_norm);
        // convert the normal from color coords to tangent space -1,1
        let norm = tex_color2.xyz() * 2.0 - 1.0;
        // convert the normal from tangent space to world space
        (tbn * norm).normalize_or_zero()
    };

    *output = match material.lighting_model {
        LightingModel::PBR_LIGHTING => {
            let albedo = tex_color0 * material.factor0 * in_color;
            let (metallic, roughness) = if material.texture1.is_none() {
                (material.factor1.y, material.factor1.z)
            } else {
                (tex_color1.y, tex_color1.z)
            };
            let ao = 1.0;
            pbr::shade_fragment(
                constants.camera_pos.xyz(),
                norm,
                in_pos,
                albedo.rgb(),
                metallic,
                roughness,
                ao,
                lights,
            )
        }
        LightingModel::PHONG_LIGHTING => {
            let diffuse_color: Vec4 = tex_color0 * in_color;
            let specular_color: Vec4 = tex_color1 * in_color;
            phong::shade_fragment(
                &constants.camera_view,
                lights,
                diffuse_color,
                specular_color,
                in_pos,
                norm,
            )
        }
        LightingModel::TEXT_LIGHTING => in_color * Vec3::splat(1.0).extend(tex_color0.x),
        _unlit => in_color * tex_color0 * material.factor0 * tex_color1,
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
