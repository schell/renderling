//! Physically based bloom.
//!
//! As described in [learnopengl.com's Physically Based Bloom article](https://learnopengl.com/Guest-Articles/2022/Phys.-Based-Bloom).
use crabslab::{Id, Slab};
use glam::{Vec2, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, spirv, Sampler};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Bloom vertex shader.
///
/// This is a pass-through vertex shader to facilitate a bloom effect.
#[spirv(vertex)]
pub fn bloom_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(instance_index)] in_id: u32,
    out_uv: &mut Vec2,
    #[spirv(flat)] out_id: &mut u32,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let i = (vertex_index % 6) as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
    *out_clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
    *out_id = in_id;
}

/// Bloom downsampling shader.
///
/// Performs successive downsampling from a source texture.
///
/// As taken from Call Of Duty method - presented at ACM Siggraph 2014.
///
/// This particular method was designed to eliminate
/// "pulsating artifacts and temporal stability issues".
#[spirv(fragment)]
pub fn bloom_downsample_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_pixel_size_id: Id<Vec2>,
    // frag_color
    downsample: &mut Vec4,
) {
    use glam::Vec3;

    let Vec2 { x, y } = slab.read(in_pixel_size_id);

    // Take 13 samples around current texel:
    // a - b - c
    // - j - k -
    // d - e - f
    // - l - m -
    // g - h - i
    // === ('e' is the current texel) ===
    let a = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y + 2.0 * y));
    let b = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y + 2.0 * y));
    let c = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y + 2.0 * y));

    let d = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y));
    let e = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y));
    let f = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y));

    let g = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y - 2.0 * y));
    let h = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y - 2.0 * y));
    let i = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y - 2.0 * y));

    let j = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y + y));
    let k = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y + y));
    let l = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y - y));
    let m = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y - y));

    // Apply weighted distribution:
    // 0.5 + 0.125 + 0.125 + 0.125 + 0.125 = 1
    // a,b,d,e * 0.125
    // b,c,e,f * 0.125
    // d,e,g,h * 0.125
    // e,f,h,i * 0.125
    // j,k,l,m * 0.5
    // This shows 5 square areas that are being sampled. But some of them overlap,
    // so to have an energy preserving downsample we need to make some adjustments.
    // The weights are the distributed so that the sum of j,k,l,m (e.g.)
    // contribute 0.5 to the final color output. The code below is written
    // to effectively yield this sum. We get:
    // 0.125*5 + 0.03125*4 + 0.0625*4 = 1
    let f1 = 0.125;
    let f2 = 0.0625;
    let f3 = 0.03125;
    let center = e * f1;
    let inner = (j + k + l + m) * f1;
    let outer = (b + d + h + f) * f2;
    let furthest = (a + c + g + i) * f3;
    let min = Vec3::splat(f32::EPSILON).extend(1.0);
    *downsample = (center + inner + outer + furthest).max(min);
}

/// Bloom upsampling shader.
///
/// This shader performs successive upsampling on a source texture.
///
/// Taken from Call Of Duty method, presented at ACM Siggraph 2014.
#[spirv(fragment)]
pub fn bloom_upsample_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] filter_radius_id: Id<Vec2>,
    // frag_color
    upsample: &mut Vec4,
) {
    // The filter kernel is applied with a radius, specified in texture
    // coordinates, so that the radius will vary across mip resolutions.
    let Vec2 { x, y } = slab.read(filter_radius_id);

    // Take 9 samples around current texel:
    // a - b - c
    // d - e - f
    // g - h - i
    // === ('e' is the current texel) ===
    let a = texture
        .sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y + y))
        .xyz();
    let b = texture
        .sample(*sampler, Vec2::new(in_uv.x, in_uv.y + y))
        .xyz();
    let c = texture
        .sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y + y))
        .xyz();

    let d = texture
        .sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y))
        .xyz();
    let e = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y)).xyz();
    let f = texture
        .sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y))
        .xyz();

    let g = texture
        .sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y - y))
        .xyz();
    let h = texture
        .sample(*sampler, Vec2::new(in_uv.x, in_uv.y - y))
        .xyz();
    let i = texture
        .sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y - y))
        .xyz();

    // Apply weighted distribution, by using a 3x3 tent filter:
    //  1   | 1 2 1 |
    // -- * | 2 4 2 |
    // 16   | 1 2 1 |
    let mut sample = e * 4.0;
    sample += (b + d + f + h) * 2.0;
    sample += a + c + g + i;
    sample *= 1.0 / 16.0;
    *upsample = sample.extend(0.5);
}

#[spirv(fragment)]
#[allow(clippy::too_many_arguments)]
/// Bloom "mix" shader.
///
/// This is the final step in applying bloom in which the computed bloom is
/// mixed with the source texture according to a strength factor.
pub fn bloom_mix_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 1)] hdr_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] hdr_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] bloom_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 4)] bloom_sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_bloom_strength_id: Id<f32>,
    frag_color: &mut Vec4,
) {
    let bloom_strength = slab.read(in_bloom_strength_id);
    let hdr = hdr_texture.sample(*hdr_sampler, in_uv).xyz();
    let bloom = bloom_texture.sample(*bloom_sampler, in_uv).xyz();
    let color = hdr.lerp(bloom, bloom_strength);
    *frag_color = color.extend(1.0)
}
