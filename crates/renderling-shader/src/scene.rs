//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{Mat3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::math::Vec3ColorSwizzles;

/// A vertex in a mesh.
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug)
)]
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

/// Calculate attenuation
///
/// attenuation.x: constant
/// attenuation.y: linear
/// attenuation.z: quadratic
pub fn attenuate(attenuation: Vec3, distance: f32) -> f32 {
    let level = attenuation.x + attenuation.y * distance + attenuation.z * (distance * distance);
    if level == 0.0 {
        // no attenuation
        1.0
    } else {
        1.0 / level
    }
}

#[repr(C)]
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
    const END_OF_LIGHTS: u32 = 0;
    pub const POINT_LIGHT: u32 = 1;
    pub const SPOT_LIGHT: u32 = 2;
    pub const DIRECTIONAL_LIGHT: u32 = 3;

    /// Calculate a point light's color contribution to a fragment.
    pub fn color_phong_point(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        let light_pos: Vec3 = (view * self.position.xyz().extend(1.0)).xyz();
        let vertex_to_light = light_pos - vertex_pos;
        let vertex_to_light_distance = vertex_to_light.length();

        let light_dir: Vec3 = vertex_to_light.normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = vertex_to_light_distance;
        let attenuation: f32 = attenuate(self.attenuation.xyz(), distance);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation;
        diffuse *= attenuation;
        specular *= attenuation;

        ambient + diffuse + specular
    }

    // Calculate a spotlight's color contribution to a fragment.
    pub fn color_phong_spot(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction.xyz() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_pos: Vec3 = (view * self.position.xyz().extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation.xyz(), distance);
        // spotlight intensity
        let direction: Vec3 = (-(view * self.direction.xyz().extend(0.0)).xyz()).normalize();
        let theta: f32 = light_dir.dot(direction);
        let epsilon: f32 = self.inner_cutoff - self.outer_cutoff;
        let intensity: f32 = ((theta - self.outer_cutoff) / epsilon).clamp(0.0, 1.0);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation * intensity;
        diffuse *= attenuation * intensity;
        specular *= attenuation * intensity;

        ambient + diffuse + specular
    }

    // Calculate a directional light's color contribution to a fragment.
    pub fn color_phong_directional(
        &self,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction.xyz() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_dir: Vec3 = (-(view * self.direction.xyz().extend(0.0)).xyz()).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // combine results
        let ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient + diffuse + specular
    }
}

/// Type has no inhabitants.
pub struct LightingModel;

impl LightingModel {
    pub const NO_LIGHTING: u32 = 0;
    pub const PHONG_LIGHTING: u32 = 1;
}

/// A GPU texture.
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug)
)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
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
pub const ID_NONE: u32 = 1024;

/// A bundle of GPU components.
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug)
)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuEntity {
    // The id of this entity. `ID_MAX` means this entity is not in use.
    pub id: u32,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // The id of this entity's first texture in the atlas.
    pub texture0: u32,
    // The id of this entity's second texture in the atlas.
    pub texture1: u32,
    // The lighting model used for shading this object.
    pub lighting: u32,
    // The id of this entity's parent, if it exists. `ID_NONE` means "no parent".
    pub parent: u32,
    pub padding0: u32,
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
            position: Vec4::ZERO,
            scale: Vec4::ONE,
            rotation: Quat::IDENTITY,
            padding0: 0,
            texture0: 0,
            texture1: 0,
            lighting: LightingModel::NO_LIGHTING,
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
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug)
)]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuConstants {
    pub camera_projection: Mat4,
    pub camera_view: Mat4,
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
    textures: &[GpuTexture],

    out_lighting_model: &mut u32,
    out_color: &mut Vec4,
    out_tex_ids: &mut UVec2,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_pos: &mut Vec3,

    gl_pos: &mut Vec4,
) {
    let entity = entities[instance_index as usize];
    let vertex = vertices[vertex_index as usize];
    let (position, rotation, scale) = entity.get_world_transform(entities);
    let model_matrix =
        Mat4::from_translation(position) * Mat4::from_quat(rotation) * Mat4::from_scale(scale);
    let texture0 = textures[entity.texture0 as usize];
    let texture1 = textures[entity.texture1 as usize];

    *out_lighting_model = entity.lighting;
    *out_color = vertex.color;
    *out_tex_ids = UVec2::new(entity.texture0, entity.texture1);
    *out_uv0 = texture0.uv(vertex.uv.xy(), constants.atlas_size);
    *out_uv1 = texture1.uv(vertex.uv.zw(), constants.atlas_size);
    *out_norm = (Mat3::from_mat4(constants.camera_view)
        * Mat3::from_mat4(model_matrix)
        * (vertex.normal.xyz() / (scale * scale)))
        .normalize();

    let view_pos = constants.camera_view * model_matrix * vertex.position.xyz().extend(1.0);
    *out_pos = view_pos.xyz();
    *gl_pos = constants.camera_projection * view_pos;
}

/// Scene fragment shader.
pub fn main_fragment_scene(
    atlas: &Image2d,
    sampler: &Sampler,

    constants: &GpuConstants,
    lights: &[GpuLight],

    in_lighting_model: u32,
    in_color: Vec4,
    in_tex_ids: UVec2,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) {
    let mut uv0_color: Vec4 = atlas.sample(*sampler, in_uv0);
    if in_tex_ids.x == 0 {
        uv0_color = Vec4::splat(1.0);
    }
    let mut uv1_color: Vec4 = atlas.sample(*sampler, in_uv1);
    if in_tex_ids.y == 0 {
        uv1_color = Vec4::splat(1.0);
    }

    *output = match in_lighting_model {
        LightingModel::PHONG_LIGHTING => {
            let diffuse_color: Vec4 = uv0_color * in_color;
            let specular_color: Vec4 = uv1_color * in_color;
            lighting_phong(
                &constants.camera_view,
                lights,
                diffuse_color,
                specular_color,
                in_pos,
                in_norm,
            )
        }
        LightingModel::NO_LIGHTING | _ => in_color * uv0_color * uv1_color,
    };
}

fn lighting_phong(
    camera_view: &Mat4,
    lights: &[GpuLight],
    diffuse_color: Vec4,
    specular_color: Vec4,
    in_pos: Vec3,
    in_norm: Vec3,
) -> Vec4 {
    if lights.is_empty() || lights[0].light_type == GpuLight::END_OF_LIGHTS {
        // the scene is unlit, so we should provide some default
        let desaturated_norm = in_norm.abs().dot(Vec3::new(0.2126, 0.7152, 0.0722));
        return (diffuse_color.rgb() * desaturated_norm).extend(1.0);
    }

    let norm: Vec3 = in_norm.normalize_or_zero();
    let camera_to_frag_dir: Vec3 = (-in_pos).normalize_or_zero();
    let mut color: Vec3 = Vec3::ZERO;
    for i in 0..lights.len() {
        let light = lights[i];
        match light.light_type {
            GpuLight::END_OF_LIGHTS => {
                break;
            }
            GpuLight::DIRECTIONAL_LIGHT => {
                color += light.color_phong_directional(
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            GpuLight::POINT_LIGHT => {
                color += light.color_phong_point(
                    in_pos,
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            GpuLight::SPOT_LIGHT => {
                color += light.color_phong_spot(
                    in_pos,
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            _ => {}
        }
    }

    color.extend(1.0)
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
    // at first we'll just draw everything into the draw indirect buffer
    let is_visible = true;
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
