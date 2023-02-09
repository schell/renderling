//! Renderling's user interface shader.
use spirv_std::{
    glam::{mat4, vec4, Mat4, UVec4, Vec2, Vec3, Vec4},
    image::Image2d, Sampler,
};

pub struct Camera {
    projection: Mat4,
    view: Mat4,
}

pub fn main_vertex(
    camera: &Camera,

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

#[repr(u32)]
pub enum BlendStyle {
    ColorOnly,
    TextureOnly,
    ReplaceUvRedWithColor,
}

impl From<u32> for BlendStyle {
    fn from(value: u32) -> Self {
        match value {
            0 => BlendStyle::ColorOnly,
            1 => BlendStyle::TextureOnly,
            _ => BlendStyle::ReplaceUvRedWithColor,
        }
    }
}

pub fn main_fragment(
    blend: &UVec4,
    // TODO: confirm `SampledImage` is what we want here, and not `Image2d` + `Sampler`...
    texture: &Image2d,
    sampler: &Sampler,

    in_color: Vec4,
    in_uv: Vec2,

    output: &mut Vec4,
) {
    let uv_color: Vec4 = texture.sample(*sampler, in_uv);
    *output = match BlendStyle::from(blend.x) {
        BlendStyle::ColorOnly => in_color,
        BlendStyle::TextureOnly => uv_color,
        BlendStyle::ReplaceUvRedWithColor => {
            vec4(in_color.x, in_color.y, in_color.z, in_color.w * uv_color.x)
        }
    };
}
