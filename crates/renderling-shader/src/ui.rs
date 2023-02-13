//! Renderling's user interface shader.
use glam::{mat4, vec4, Mat4, Vec2, Vec3, Vec4};
use spirv_std::{image::Image2d, Sampler};

use crate::ShaderCamera;

pub fn main_vertex(
    camera: &ShaderCamera,

    in_pos: Vec3,
    in_color: Vec4,
    in_uv: Vec2,

    in_model_matrix_0: Vec4,
    in_model_matrix_1: Vec4,
    in_model_matrix_2: Vec4,
    in_model_matrix_3: Vec4,

    out_color: &mut Vec4,
    out_uv: &mut Vec2,
    out_pos: &mut Vec4,
) {
    let model: Mat4 = mat4(
        in_model_matrix_0,
        in_model_matrix_1,
        in_model_matrix_2,
        in_model_matrix_3,
    );
    *out_color = in_color;
    *out_uv = in_uv;
    *out_pos = camera.projection * camera.view * model * vec4(in_pos.x, in_pos.y, in_pos.z, 1.0);
}

/// Variants of uv/color blending.
///
/// This determines how UV and Color coords are blended
/// together.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum UiColorBlend {
    /// The mesh should be colored only with its color attribute
    ColorOnly = 0,
    /// The mesh should be colored only with its uv vertex attribute
    UvOnly = 1,
    /// The mesh should replace uv red with its color vertex attribute.
    ///
    /// This is used for colored text.
    ReplaceRedUvWithColor = 2,
}

impl From<&ShaderColorBlend> for UiColorBlend {
    fn from(ShaderColorBlend { inner }: &ShaderColorBlend) -> Self {
        match inner {
            0 => UiColorBlend::ColorOnly,
            1 => UiColorBlend::UvOnly,
            _ => UiColorBlend::ReplaceRedUvWithColor,
        }
    }
}

impl From<UiColorBlend> for ShaderColorBlend {
    fn from(value: UiColorBlend) -> Self {
        ShaderColorBlend {
            inner: value as u32,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct ShaderColorBlend {
    inner: u32,
}

pub fn main_fragment(
    blend: &ShaderColorBlend,
    texture: &Image2d,
    sampler: &Sampler,

    in_color: Vec4,
    in_uv: Vec2,

    output: &mut Vec4,
) {
    let uv_color: Vec4 = texture.sample(*sampler, in_uv);
    *output = match UiColorBlend::from(blend) {
        UiColorBlend::ColorOnly => in_color,
        UiColorBlend::UvOnly => uv_color,
        UiColorBlend::ReplaceRedUvWithColor => {
            vec4(in_color.x, in_color.y, in_color.z, in_color.w * uv_color.x)
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn ui_color() {
        assert!(super::UiColorBlend::ReplaceRedUvWithColor as u32 == 2);
    }
}
