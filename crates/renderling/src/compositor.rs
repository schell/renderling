//! Compositor for alpha-blending a source texture onto a target framebuffer.
//!
//! This is used by the `renderling-ui` crate to overlay MSAA-resolved UI
//! content onto a 3D scene without overwriting existing framebuffer content.

use glam::{Vec2, Vec4};
use spirv_std::{image::Image2d, spirv, Sampler};

/// Fullscreen quad vertex shader for compositing.
///
/// Generates 6 vertices (two triangles) covering the full clip-space quad
/// and passes through UV coordinates for texture sampling.
#[spirv(vertex)]
pub fn compositor_vertex(
    #[spirv(vertex_index)] vertex_id: u32,
    out_uv: &mut Vec2,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    let i = vertex_id as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
    *out_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
}

/// Passthrough fragment shader for compositing.
///
/// Samples the source texture at the given UV and outputs the color.
/// Alpha blending is handled by the pipeline's blend state, not the shader.
#[spirv(fragment)]
pub fn compositor_fragment(
    #[spirv(descriptor_set = 0, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    in_uv: Vec2,
    frag_color: &mut Vec4,
) {
    *frag_color = texture.sample(*sampler, in_uv);
}

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;
