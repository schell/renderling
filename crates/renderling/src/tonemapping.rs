//! Tonemapping from an HDR texture to SDR.
//!
//! ## References
//! * <https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/tonemapping.glsl>
//! * <https://64.github.io/tonemapping>

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod shader {
    //! Tonemapping shader module.
    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    const GAMMA: f32 = 2.2;
    const INV_GAMMA: f32 = 1.0 / GAMMA;

    /// sRGB => XYZ => D65_2_D60 => AP1 => RRT_SAT
    const ACESINPUT_MAT: Mat3f = mat3x3f(
        vec3f(0.59719, 0.07600, 0.02840),
        vec3f(0.35458, 0.90834, 0.13383),
        vec3f(0.04823, 0.01566, 0.83777),
    );

    /// ODT_SAT => XYZ => D60_2_D65 => sRGB
    const ACESOUTPUT_MAT: Mat3f = mat3x3f(
        vec3f(1.60475, -0.10208, -0.00327),
        vec3f(-0.53108, 1.10813, -0.07276),
        vec3f(-0.07367, -0.00605, 1.07602),
    );

    /// Linear to sRGB approximation.
    /// See <http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html>
    pub fn linear_to_srgb(color: Vec3f) -> Vec3f {
        let inv_gamma = vec3f(INV_GAMMA, INV_GAMMA, INV_GAMMA);
        pow(color, inv_gamma)
    }

    /// sRGB to linear approximation.
    /// See <http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html>
    pub fn srgb_to_linear(srgb_in: Vec3f) -> Vec3f {
        let gamma = vec3f(GAMMA, GAMMA, GAMMA);
        pow(srgb_in, gamma)
    }

    /// sRGB to linear approximation.
    /// See <http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html>
    pub fn srgba_to_linear(srgb_in: Vec4f) -> Vec4f {
        let lin = srgb_to_linear(srgb_in.xyz());
        vec4f(lin.x, lin.y, lin.z, srgb_in.w)
    }

    /// ACES tone map (faster approximation)
    /// see: <https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve>
    pub fn tone_map_aces_narkowicz(color: Vec3f) -> Vec3f {
        const A: f32 = 2.51;
        const B: f32 = 0.03;
        const C: f32 = 2.43;
        const D: f32 = 0.59;
        const E: f32 = 0.14;
        let c = (color * (A * color + B)) / (color * (C * color + D) + E);
        clamp(c, Vec3f::ZERO, Vec3f::ONE)
    }

    /// ACES filmic tone map approximation
    /// see <https://github.com/TheRealMJP/BakingLab/blob/master/BakingLab/ACES.hlsl>
    pub fn rrt_and_odtfit(color: Vec3f) -> Vec3f {
        let a: Vec3f = color * (color + 0.0245786) - 0.000090537;
        let b: Vec3f = color * (0.983729 * color + 0.432951) + 0.238081;
        a / b
    }

    pub fn tone_map_aces_hill(color_in: Vec3f) -> Vec3f {
        let mut color = color_in;
        color = ACESINPUT_MAT * color;
        // Apply RRT and ODT
        color = rrt_and_odtfit(color);
        color = ACESOUTPUT_MAT * color;
        // Clamp to [0, 1]
        clamp(color, Vec3f::ZERO, Vec3f::ONE)
    }

    pub fn tone_map_reinhard(color: Vec3f) -> Vec3f {
        color / (color + Vec3f::ONE)
    }

    #[derive(Clone, Copy, Default, PartialEq, Eq, Wgsl, SlabItem, core::fmt::Debug)]
    #[repr(u32)]
    pub enum Tonemap {
        #[default]
        None,
        AcesNarkowicz,
        AcesHill,
        AcesHillExposureBoost,
        Reinhard,
    }

    #[derive(Clone, Copy, PartialEq, Wgsl, SlabItem)]
    pub struct TonemapConstants {
        pub tonemap: Tonemap,
        pub exposure: f32,
    }

    impl Default for TonemapConstants {
        fn default() -> Self {
            Self {
                tonemap: Tonemap::None,
                exposure: 1.0,
            }
        }
    }

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);
    texture!(group(0), binding(1), TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(2), SAMPLER: Sampler);

    pub fn tonemap(color_in: Vec4f) -> Vec4f {
        let constants: TonemapConstants;
        slab_read!(TonemapConstants, get!(SLAB), 0u32, constants);

        let mut color = color_in;
        color *= constants.exposure;

        #[allow(clippy::needless_late_init)]
        let toned: Vec3f;
        #[wgsl_allow(non_literal_match_statement_patterns)]
        match constants.tonemap {
            Tonemap::AcesNarkowicz => {
                toned = tone_map_aces_narkowicz(color.xyz());
            }
            Tonemap::AcesHill => {
                toned = tone_map_aces_hill(color.xyz());
            }
            Tonemap::AcesHillExposureBoost => {
                // boost exposure as discussed in https://github.com/mrdoob/three.js/pull/19621
                // this factor is based on the exposure correction of Krzysztof
                // Narkowicz in his implemetation of ACES tone mapping
                toned = tone_map_aces_hill(color.xyz() / 0.6);
            }
            Tonemap::Reinhard => {
                // Use Reinhard tone mapping
                toned = tone_map_reinhard(color.xyz());
            }
            _ => {
                toned = color.xyz();
            }
        }
        vec4f(toned.x, toned.y, toned.z, color.w)
    }

    const TL_POS: Vec2f = vec2f(-1.0, 1.0);
    const TR_POS: Vec2f = vec2f(1.0, 1.0);
    const BL_POS: Vec2f = vec2f(-1.0, -1.0);
    const BR_POS: Vec2f = vec2f(1.0, -1.0);

    const TL_UV: Vec2f = vec2f(0.0, 0.0);
    const TR_UV: Vec2f = vec2f(1.0, 0.0);
    const BL_UV: Vec2f = vec2f(0.0, 1.0);
    const BR_UV: Vec2f = vec2f(1.0, 1.0);

    const QUAD_2D_POS: [Vec2f; 6] = [TL_POS, BL_POS, BR_POS, TL_POS, BR_POS, TR_POS];
    const QUAD_2D_UV: [Vec2f; 6] = [TL_UV, BL_UV, BR_UV, TL_UV, BR_UV, TR_UV];

    pub struct TonemappingInterStage {
        #[location(0)]
        uv: Vec2f,
        #[builtin(position)]
        #[allow(dead_code)]
        clip_pos: Vec4f,
    }

    #[vertex]
    pub fn tonemapping_vertex(#[builtin(vertex_index)] vertex_id: u32) -> TonemappingInterStage {
        let pos = QUAD_2D_POS[vertex_id as usize];
        TonemappingInterStage {
            uv: QUAD_2D_UV[vertex_id as usize],
            clip_pos: vec4f(pos.x, pos.y, 0.0, 1.0),
        }
    }

    #[fragment]
    pub fn tonemapping_fragment(input: TonemappingInterStage) -> Vec4f {
        let color = texture_sample(TEXTURE, SAMPLER, input.uv);
        tonemap(color)
    }
}

pub mod wgsl {
    //! CPU side
    //!
    //! The wgsl-rs backed tonemapping runner: drop-in counterpart of the
    //! legacy CPU-side `Tonemapping`, built on the craballoc_wgsl runtime
    //! and the ported WGSL module.
    use std::sync::{Arc, RwLock};

    use craballoc_wgsl::{
        arena::{Arena, Value},
        runtime::WgpuRuntime,
    };
    use wgsl_rs_linkage_wgpu::{analyze_wgsl_module, WgpuLinkage};

    use crate::texture::Texture;

    use super::shader::{TonemapConstants, WGSL_SOURCE};

    /// Conducts HDR tone mapping with the wgsl-rs shader module.
    ///
    /// Writes the HDR surface texture to the (most likely) sRGB window
    /// surface. Clones of [`Tonemapping`] all reference the same internal
    /// data.
    #[derive(Clone)]
    pub struct Tonemapping {
        arena: Arena<WgpuRuntime>,
        config: Value<TonemapConstants>,
        hdr_texture: Arc<RwLock<Texture>>,
        pipeline: Arc<wgpu::RenderPipeline>,
        linkage: Arc<RwLock<WgpuLinkage>>,
    }

    impl Tonemapping {
        pub fn new(
            runtime: impl AsRef<WgpuRuntime>,
            frame_texture_format: wgpu::TextureFormat,
            hdr_texture: &Texture,
        ) -> Self {
            let runtime = runtime.as_ref();
            let arena = Arena::new(runtime, "tonemapping-slab", None);
            // The WGSL module reads the constants from slab offset zero, so
            // the config must be the first value in the arena.
            let config = arena.new_value(TonemapConstants::default());

            let mut linkage = analyze_wgsl_module(&WGSL_SOURCE).expect("tonemapping wgsl analysis");
            let wgsl_module = linkage.shader_module(&runtime.device);
            let pipeline_layout = linkage.pipeline_layout(&runtime.device, Some("tonemapping"));

            let pipeline = runtime
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("tonemapping"),
                    layout: Some(&pipeline_layout),
                    vertex: linkage
                        .vertex_entry("tonemapping_vertex")
                        .expect("tonemapping_vertex entry present")
                        .vertex_state(&wgsl_module),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    fragment: Some(wgpu::FragmentState {
                        module: &wgsl_module,
                        entry_point: Some("tonemapping_fragment"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: frame_texture_format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: Default::default(),
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                });

            Self {
                arena,
                config,
                hdr_texture: Arc::new(RwLock::new(hdr_texture.clone())),
                pipeline: Arc::new(pipeline),
                linkage: Arc::new(RwLock::new(linkage)),
            }
        }

        pub(crate) fn arena(&self) -> &Arena<WgpuRuntime> {
            &self.arena
        }

        pub fn set_hdr_texture(&self, hdr_texture: &Texture) {
            // The bind group is rebuilt on the next render against the
            // latest committed slab buffer, so only the texture changes here.
            *self
                .hdr_texture
                .write()
                .expect("tonemapping hdr_texture write") = hdr_texture.clone();
        }

        pub fn get_tonemapping_config(&self) -> TonemapConstants {
            self.config.get()
        }

        pub fn set_tonemapping_config(&self, config: TonemapConstants) {
            self.config.set(config);
        }

        /// Uploads any config mutations and renders the tonemapping pass on
        /// top of `view` (load-op Load, matching the legacy pass). The bind
        /// group is rebuilt each render against the latest committed slab
        /// buffer, so config mutations and buffer growth are both handled.
        pub fn render(&self, runtime: impl AsRef<WgpuRuntime>, view: &wgpu::TextureView) {
            let runtime = runtime.as_ref();
            let label = Some("tonemapping render");
            let slab_buffer = self.arena.commit();
            let hdr = self.hdr_texture.read().expect("tonemapping hdr read");
            let mut linkage = self.linkage.write().expect("tonemapping linkage write");
            let bind_group = linkage
                .create_bind_group_named(
                    0,
                    &runtime.device,
                    &[
                        (
                            "SLAB",
                            wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                buffer: &slab_buffer,
                                offset: 0,
                                size: None,
                            }),
                        ),
                        ("TEXTURE", wgpu::BindingResource::TextureView(&hdr.view)),
                        ("SAMPLER", wgpu::BindingResource::Sampler(&hdr.sampler)),
                    ],
                )
                .expect("tonemapping bind group");
            drop(linkage);

            let mut encoder = runtime
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    multiview_mask: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..6, 0..1);
            }

            runtime.queue.submit(std::iter::once(encoder.finish()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        shader::{Tonemap, TonemapConstants},
        wgsl,
    };
    use crate::{context::Context, test::BlockOnFuture};
    use glam::Vec4;

    /// The A/B golden test: procedurally generate an HDR image, render it
    /// through the legacy Tonemapping once per tonemap method (saving each
    /// result as a golden EXR), then render through the wgsl-rs Tonemapping
    /// on the same input and compare each result against its golden.
    #[test]
    fn golden_per_variant() {
        use crate::texture::Texture;
        use craballoc_wgsl::runtime::WgpuRuntime;

        let width = 128u32;
        let height = 64u32;
        let ctx = Context::headless(width, height).block();

        // Procedural HDR image: a red gradient into HDR range, a vertical
        // green gradient, and a 40-nit blue blob - covering dark, mid, and
        // bright regions of every curve.
        let device = ctx.get_device();
        let queue = ctx.get_queue();
        let hdr_texture = Texture::create_hdr_texture(device, width, height, 1);
        let mut texels = vec![0u16; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) as usize;
                let dx = x as f32 - width as f32 * 0.5;
                let dy = y as f32 - height as f32 * 0.5;
                let in_blob = dx * dx + dy * dy < 12.0 * 12.0;
                let px = [
                    x as f32 / width as f32 * 2.0,
                    y as f32 / height as f32,
                    if in_blob { 40.0 } else { 0.05 },
                    1.0,
                ];
                for (slot, val) in px.iter().enumerate() {
                    texels[i * 4 + slot] = half::f16::from_f32(*val).to_bits();
                }
            }
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &hdr_texture.texture,
                mip_level: 0,
                origin: Default::default(),
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&texels),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4 * 2),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let frame_format = wgpu::TextureFormat::Rgba16Float;

        let wgsl_runtime = WgpuRuntime {
            device: ctx.runtime().device.clone(),
            queue: ctx.runtime().queue.clone(),
        };
        let new = wgsl::Tonemapping::new(&wgsl_runtime, frame_format, &hdr_texture);

        // Reads back an RGBA16Float target as an f32 image.
        // Wraps the raw target and reads it back with the crate's helper.
        let read_f16_target = |texture: &wgpu::Texture| {
            Texture::from_wgpu_tex(device, texture.clone(), None, None)
                .read_hdr_image(ctx.runtime())
                .block()
                .expect("hdr readback ok")
        };

        let variants = [
            (Tonemap::None, "none"),
            (Tonemap::AcesNarkowicz, "aces_narkowicz"),
            (Tonemap::AcesHill, "aces_hill"),
            (Tonemap::AcesHillExposureBoost, "aces_hill_exposure_boost"),
            (Tonemap::Reinhard, "reinhard"),
        ];

        let golden_dir = renderling_build::test_img_dir().join("tonemapping");

        for (variant, name) in variants {
            // Render once through the wgsl-rs implementation.
            new.set_tonemapping_config(TonemapConstants {
                tonemap: variant,
                exposure: 1.0,
            });
            let output = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(format!("tonemapping-new-{name}").as_str()),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: frame_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            let output_view = output.create_view(&wgpu::TextureViewDescriptor::default());
            new.render(&wgsl_runtime, &output_view);
            let new_img = read_f16_target(&output);
            img_diff::save(format!("tonemapping/{name}_new.exr"), new_img.clone());

            // Load the golden back from disk and compare per pixel. Both
            // renders are GPU executions of the same math, so the deltas
            // should be tiny relative differences from backend float paths.
            let golden = image::open(golden_dir.join(format!("{name}.exr")))
                .unwrap()
                .into_rgba32f();
            let mut failures = 0;
            for (i, golden_px) in golden.pixels().enumerate() {
                let expected = Vec4::from(golden_px.0);
                let got = Vec4::from(
                    new_img
                        .get_pixel((i % width as usize) as u32, (i / width as usize) as u32)
                        .0,
                );
                let delta = (got - expected).abs().max_element();
                let scale = got.length().max(expected.length()).max(1.0);
                if delta > 2e-3 * scale {
                    failures += 1;
                    if failures <= 4 {
                        let x = (i % width as usize) as u32;
                        let y = (i / width as usize) as u32;
                        println!(
                            "{name} gpu({x},{y}): {got:?} vs golden {expected:?} (delta {delta})"
                        );
                    }
                }
            }
            assert_eq!(
                failures, 0,
                "{name}: {failures} pixels differ beyond tolerance"
            );
        }
    }
}
