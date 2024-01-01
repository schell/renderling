//! Traditional 2d rendering.
//!
//! This is mostly for rendering text.

use crabslab::SlabItem;
use glam::{Mat4, UVec2, Vec2, Vec4};
use spirv_std::{image::Image2d, spirv, Sampler};

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct UiVertex {
    pub position: Vec2,
    pub uv: Vec2,
    pub color: Vec4,
}

impl Default for UiVertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            uv: Default::default(),
            color: Vec4::ONE,
        }
    }
}

impl UiVertex {
    pub fn with_position(mut self, p: impl Into<Vec2>) -> Self {
        self.position = p.into();
        self
    }

    pub fn with_uv(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv = uv.into();
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }
}

#[repr(C)]
#[derive(Clone, Copy, SlabItem)]
pub struct UiConstants {
    pub canvas_size: UVec2,
    pub camera_translation: Vec2,
}

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct UiMode(pub u32);

impl UiMode {
    pub const DEFAULT: Self = UiMode(0);
    pub const TEXT: Self = UiMode(3);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UiDrawParams {
    pub translation: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub mode: UiMode,
}

impl Default for UiDrawParams {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            scale: Vec2::ONE,
            rotation: Default::default(),
            mode: Default::default(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[spirv(vertex)]
pub fn vertex(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &UiConstants,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] params: &UiDrawParams,

    in_pos: Vec2,
    in_uv: Vec2,
    in_color: Vec4,

    #[spirv(flat)] out_mode: &mut u32,
    out_color: &mut Vec4,
    out_uv: &mut Vec2,

    #[spirv(position)] gl_pos: &mut Vec4,
) {
    *out_mode = params.mode.0;
    *out_color = in_color;
    *out_uv = in_uv;

    let model = Mat4::from_translation(params.translation.extend(0.0))
        * Mat4::from_rotation_z(params.rotation)
        * Mat4::from_scale(params.scale.extend(1.0));
    let view = Mat4::from_translation(constants.camera_translation.extend(0.0));
    let proj = Mat4::orthographic_rh(
        0.0,
        constants.canvas_size.x as f32,
        constants.canvas_size.y as f32,
        0.0,
        -1.0,
        1.0,
    );

    *gl_pos = proj * view * model * Vec4::new(in_pos.x, in_pos.y, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn fragment(
    #[spirv(descriptor_set = 1, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,

    #[spirv(flat)] in_mode: u32,
    in_color: Vec4,
    in_uv: Vec2,

    output: &mut Vec4,
) {
    let mode = UiMode(in_mode);
    let uv_color: Vec4 = texture.sample(*sampler, in_uv);
    *output = match mode {
        UiMode::TEXT => Vec4::new(in_color.x, in_color.y, in_color.z, in_color.w * uv_color.x),
        _ => in_color * uv_color,
    }
}
