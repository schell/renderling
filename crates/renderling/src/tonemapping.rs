//! Tonemapping from an HDR texture to sRGB (most likely).
//!
//! ## References
//! * https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/tonemapping.glsl
//! * https://64.github.io/tonemapping

use crabslab::{Slab, SlabItem};
use glam::{mat3, Mat3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, spirv, Sampler};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

const GAMMA: f32 = 2.2;
const INV_GAMMA: f32 = 1.0 / GAMMA;

/// sRGB => XYZ => D65_2_D60 => AP1 => RRT_SAT
const ACESINPUT_MAT: Mat3 = mat3(
    Vec3::new(0.59719, 0.07600, 0.02840),
    Vec3::new(0.35458, 0.90834, 0.13383),
    Vec3::new(0.04823, 0.01566, 0.83777),
);

/// ODT_SAT => XYZ => D60_2_D65 => sRGB
const ACESOUTPUT_MAT: Mat3 = mat3(
    Vec3::new(1.60475, -0.10208, -0.00327),
    Vec3::new(-0.53108, 1.10813, -0.07276),
    Vec3::new(-0.07367, -0.00605, 1.07602),
);

/// Linear to sRGB approximation.
/// See http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html
pub fn linear_to_srgb(color: Vec3) -> Vec3 {
    color.powf(INV_GAMMA)
}

/// sRGB to linear approximation.
/// See http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html
pub fn srgb_to_linear(srgb_in: Vec3) -> Vec3 {
    srgb_in.powf(GAMMA)
}

/// sRGB to linear approximation.
/// See http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html
pub fn srgba_to_linear(srgb_in: Vec4) -> Vec4 {
    srgb_to_linear(srgb_in.xyz()).extend(srgb_in.w)
}

/// ACES tone map (faster approximation)
/// see: https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
pub fn tone_map_aces_narkowicz(color: Vec3) -> Vec3 {
    const A: f32 = 2.51;
    const B: f32 = 0.03;
    const C: f32 = 2.43;
    const D: f32 = 0.59;
    const E: f32 = 0.14;
    let c = (color * (A * color + B)) / (color * (C * color + D) + E);
    c.clamp(Vec3::ZERO, Vec3::ONE)
}

/// ACES filmic tone map approximation
/// see https://github.com/TheRealMJP/BakingLab/blob/master/BakingLab/ACES.hlsl
fn rrt_and_odtfit(color: Vec3) -> Vec3 {
    let a: Vec3 = color * (color + 0.0245786) - 0.000090537;
    let b: Vec3 = color * (0.983729 * color + 0.432951) + 0.238081;
    a / b
}

pub fn tone_map_aces_hill(mut color: Vec3) -> Vec3 {
    color = ACESINPUT_MAT * color;
    // Apply RRT and ODT
    color = rrt_and_odtfit(color);
    color = ACESOUTPUT_MAT * color;
    // Clamp to [0, 1]
    color = color.clamp(Vec3::ZERO, Vec3::ONE);

    color
}

pub fn tone_map_reinhard(color: Vec3) -> Vec3 {
    color / (color + Vec3::ONE)
}

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, Eq, SlabItem)]
pub struct Tonemap(u32);

impl Tonemap {
    pub const NONE: Self = Tonemap(0);
    pub const ACES_NARKOWICZ: Self = Tonemap(1);
    pub const ACES_HILL: Self = Tonemap(2);
    pub const ACES_HILL_EXPOSURE_BOOST: Self = Tonemap(3);
    pub const REINHARD: Self = Tonemap(4);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct TonemapConstants {
    pub tonemap: Tonemap,
    pub exposure: f32,
}

impl Default for TonemapConstants {
    fn default() -> Self {
        Self {
            tonemap: Tonemap::NONE,
            exposure: 1.0,
        }
    }
}

pub fn tonemap(mut color: Vec4, slab: &[u32]) -> Vec4 {
    let constants = slab.read::<TonemapConstants>(0u32.into());
    color *= constants.exposure;

    match constants.tonemap {
        Tonemap::ACES_NARKOWICZ => tone_map_aces_narkowicz(color.xyz()).extend(color.w),
        Tonemap::ACES_HILL => tone_map_aces_hill(color.xyz()).extend(color.w),
        Tonemap::ACES_HILL_EXPOSURE_BOOST => {
            // boost exposure as discussed in https://github.com/mrdoob/three.js/pull/19621
            // this factor is based on the exposure correction of Krzysztof Narkowicz in his
            // implemetation of ACES tone mapping
            tone_map_aces_hill(color.xyz() / 0.6).extend(color.w)
        }
        Tonemap::REINHARD => {
            // Use Reinhard tone mapping
            tone_map_reinhard(color.xyz()).extend(color.w)
        }
        _ => color,
    }
}

const QUAD_2D_POINTS: [(Vec2, Vec2); 6] = {
    let tl = (Vec2::new(-1.0, 1.0), Vec2::new(0.0, 0.0));
    let tr = (Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0));
    let bl = (Vec2::new(-1.0, -1.0), Vec2::new(0.0, 1.0));
    let br = (Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0));
    [tl, bl, br, tl, br, tr]
};

#[cfg(feature = "tonemapping_vertex")]
#[spirv(vertex)]
pub fn tonemapping_vertex(
    #[spirv(vertex_index)] vertex_id: u32,
    out_uv: &mut glam::Vec2,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    let (pos, uv) = QUAD_2D_POINTS[vertex_id as usize];
    *out_uv = uv;
    *gl_pos = pos.extend(0.0).extend(1.0);
}

#[cfg(feature = "tonemapping_fragment")]
#[spirv(fragment)]
pub fn tonemapping_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: glam::Vec2,
    output: &mut glam::Vec4,
) {
    let color: Vec4 = texture.sample(*sampler, in_uv);
    let color = tonemap(color, slab);
    *output = color;
}
