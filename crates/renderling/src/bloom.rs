//! Physically based bloom.
//!
//! As described in [learnopengl.com's Physically Based Bloom article](https://learnopengl.com/Guest-Articles/2022/Phys.-Based-Bloom).
use crabslab::{Id, Slab, SlabItem};
use glam::{UVec2, Vec2, Vec4};
use spirv_std::{image::Image2d, spirv, Sampler};

#[derive(Clone, Copy, SlabItem)]
pub struct BloomConfig {
    pub resolution: UVec2,
    pub upsample_filter_radius: Vec2,
}

impl Default for BloomConfig {
    fn default() -> Self {
        Self {
            resolution: UVec2::ONE,
            upsample_filter_radius: Vec2::ONE,
        }
    }
}

#[cfg(feature = "bloom_vertex")]
/// A passthru vertex shader to facilitate a bloom effect.
#[spirv(vertex)]
pub fn bloom_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(instance_index)] bloom_cfg_id: Id<BloomConfig>,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    out_uv: &mut Vec2,
    #[spirv(flat)] out_config: &mut BloomConfig,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let i = (vertex_index % 6) as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
    *out_clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
    *out_config = slab.read(bloom_cfg_id);
}

#[cfg(feature = "bloom_downsample_fragment")]
/// Performs downsampling on a texture.
///
/// As taken from Call Of Duty method - presented at ACM Siggraph 2014.
///
/// This particular method was customly designed to eliminate
/// "pulsating artifacts and temporal stability issues".
#[spirv(fragment)]
pub fn bloom_downsample_fragment(
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_config: BloomConfig,
    // frag_color
    downsample: &mut Vec4,
) {
    let Vec2 { x, y } =
        1.0 / Vec2::new(in_config.resolution.x as f32, in_config.resolution.y as f32);

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
    *downsample = (e * 0.125)
        + ((a + c + g + i) * 0.03125)
        + ((b + d + f + h) * 0.0625)
        + ((j + k + l + m) * 0.125);
}

#[cfg(feature = "bloom_upsample_fragment")]
/// This shader performs upsampling on a texture.
/// Taken from Call Of Duty method, presented at ACM Siggraph 2014.
#[spirv(fragment)]
pub fn bloom_upsample_fragment(
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_config: BloomConfig,
    // frag_color
    upsample: &mut Vec4,
) {
    // The filter kernel is applied with a radius, specified in texture
    // coordinates, so that the radius will vary across mip resolutions.
    let Vec2 { x, y } = in_config.upsample_filter_radius;

    // Take 9 samples around current texel:
    // a - b - c
    // d - e - f
    // g - h - i
    // === ('e' is the current texel) ===
    let a = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y + y));
    let b = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y + y));
    let c = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y + y));

    let d = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y));
    let e = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y));
    let f = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y));

    let g = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y - y));
    let h = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y - y));
    let i = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y - y));

    // Apply weighted distribution, by using a 3x3 tent filter:
    //  1   | 1 2 1 |
    // -- * | 2 4 2 |
    // 16   | 1 2 1 |
    let mut sample = e * 4.0;
    sample += (b + d + f + h) * 2.0;
    sample += a + c + g + i;
    sample *= 1.0 / 16.0;
    *upsample = sample;
}

#[cfg(not(target_arch = "spirv"))]
mod cpu {
    use std::sync::Arc;

    use super::*;
    use crate::{
        slab::{Hybrid, SlabManager},
        texture,
    };

    fn create_bloom_downscale_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        todo!()
    }

    fn create_bloom_upscale_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        todo!()
    }

    fn create_textures(device: &wgpu::Device, num_textures: u32) -> Vec<texture::Texture> {
        todo!()
    }

    /// Performs a "physically based" bloom effect on a texture.
    ///
    /// Contains pipelines, down/upsampling textures, a buffer
    /// to communicate configuration to the shaders, and bindgroups.
    pub struct Bloom {
        slab: SlabManager,
        config: Hybrid<BloomConfig>,
        downsample_pipeline: Arc<wgpu::RenderPipeline>,
        upsample_pipeline: Arc<wgpu::RenderPipeline>,
        textures: Vec<texture::Texture>,
        //bindgroup: Arc<wgpu::BindGroup>,
    }

    impl Bloom {
        pub fn new(
            device: &crate::Device,
            queue: &crate::Queue,
            format: wgpu::TextureFormat,
            resolution: UVec2,
            num_levels: u32,
        ) -> Self {
            let mut slab = SlabManager::new(device, queue);
            let config = slab.new_hybrid(BloomConfig {
                resolution,
                upsample_filter_radius: Vec2::ONE,
            });
            let textures = create_textures(device, num_levels);
            let downsample_pipeline = Arc::new(create_bloom_downscale_pipeline(device));
            let upsample_pipeline = Arc::new(create_bloom_upscale_pipeline(device));
            Self {
                slab,
                config,
                downsample_pipeline,
                upsample_pipeline,
                textures,
            }
        }
    }
}
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;
