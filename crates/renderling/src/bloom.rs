//! Physically based bloom.
//!
//! As described in [learnopengl.com's Physically Based Bloom
//! article](https://learnopengl.com/Guest-Articles/2022/Phys.-Based-Bloom).

// #[cfg(not(target_arch = "spirv"))]
// mod cpu;
// #[cfg(not(target_arch = "spirv"))]
// pub use cpu::*;

// pub mod shader;

#[wgsl_rs::wgsl]
pub mod vertex {
    use crate::shader::constants::*;
    use wgsl_rs::std::*;

    pub struct BloomVertexInput {
        #[builtin(vertex_index)]
        vertex_index: u32,
        #[builtin(instance_index)]
        id: u32,
    }

    pub struct BloomInterStage {
        #[location(0)]
        pub uv: Vec2f,

        #[location(1)]
        #[interpolate(flat)]
        pub id: u32,

        #[builtin(position)]
        pub clip_pos: Vec4f,
    }

    /// Bloom vertex shader.
    ///
    /// This is a pass-through vertex shader to facilitate a bloom effect.
    #[vertex]
    pub fn bloom_vertex(input: BloomVertexInput) -> BloomInterStage {
        let i = (input.vertex_index % 6) as usize;
        BloomInterStage {
            uv: UV_COORD_QUAD_CCW[i],
            id: input.id,
            clip_pos: CLIP_SPACE_COORD_QUAD_CCW[i],
        }
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod sample {
    //! Downsampling and upsampling fragment shaders.

    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    use super::vertex::*;

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    texture!(group(0), binding(1), TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(2), SAMPLER: Sampler);

    /// Bloom downsampling shader.
    ///
    /// Performs successive downsampling from a source texture.
    ///
    /// As taken from Call Of Duty method - presented at ACM Siggraph 2014.
    ///
    /// This particular method was designed to eliminate
    /// "pulsating artifacts and temporal stability issues".
    #[fragment]
    pub fn bloom_downsample_fragment(input: BloomInterStage) -> Vec4f {
        let in_pixel_size: Vec2f;
        slab_read!(Vec2f, get!(SLAB), input.id, in_pixel_size);
        // TODO: once destructuring is supported in wgsl-rs, uncomment this.
        // let Vec2 { x, y } = slab.read(in_pixel_size_id);
        let x = in_pixel_size.x;
        let y = in_pixel_size.y;

        // Take 13 samples around current texel:
        // a - b - c
        // - j - k -
        // d - e - f
        // - l - m -
        // g - h - i
        // === ('e' is the current texel) ===
        let a = texture_sample(
            TEXTURE,
            SAMPLER,
            vec2f(input.uv.x - 2.0 * x, input.uv.y + 2.0 * y),
        );
        let b = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y + 2.0 * y));
        let c = texture_sample(
            TEXTURE,
            SAMPLER,
            vec2f(input.uv.x + 2.0 * x, input.uv.y + 2.0 * y),
        );

        let d = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - 2.0 * x, input.uv.y));
        let e = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y));
        let f = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + 2.0 * x, input.uv.y));

        let g = texture_sample(
            TEXTURE,
            SAMPLER,
            vec2f(input.uv.x - 2.0 * x, input.uv.y - 2.0 * y),
        );
        let h = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y - 2.0 * y));
        let i = texture_sample(
            TEXTURE,
            SAMPLER,
            vec2f(input.uv.x + 2.0 * x, input.uv.y - 2.0 * y),
        );

        let j = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - x, input.uv.y + y));
        let k = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + x, input.uv.y + y));
        let l = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - x, input.uv.y - y));
        let m = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + x, input.uv.y - y));

        // Apply weighted distribution:
        // 0.5 + 0.125 + 0.125 + 0.125 + 0.125 = 1
        // a,b,d,e * 0.125
        // b,c,e,f * 0.125
        // d,e,g,h * 0.125
        // e,f,h,i * 0.125
        // j,k,l,m * 0.5
        // This shows 5 square areas that are being sampled. But some of them
        // overlap, so to have an energy preserving downsample we need to make
        // some adjustments. The weights are the distributed so that the sum of
        // j,k,l,m (e.g.) contribute 0.5 to the final color output. The code
        // below is written to effectively yield this sum. We get:
        // 0.125*5 + 0.03125*4 + 0.0625*4 = 1
        let f1 = 0.125;
        let f2 = 0.0625;
        let f3 = 0.03125;
        let center = e * f1;
        let inner = (j + k + l + m) * f1;
        let outer = (b + d + h + f) * f2;
        let furthest = (a + c + g + i) * f3;
        // TODO: use vec3f_splat when it's available
        // let min = Vec3::splat(f32::EPSILON).extend(1.0);
        let ep = 1.1920929e-7;
        let min = vec4f(ep, ep, ep, 1.0);
        max(center + inner + outer + furthest, min)
    }

    /// Bloom upsampling shader.
    ///
    /// This shader performs successive upsampling on a source texture.
    ///
    /// Taken from Call Of Duty method, presented at ACM Siggraph 2014.
    #[fragment]
    pub fn bloom_upsample_fragment(input: BloomInterStage) -> Vec4f {
        // The filter kernel is applied with a radius, specified in texture
        // coordinates, so that the radius will vary across mip resolutions.
        // let Vec2 { x, y } = slab.read(filter_radius_id);
        let filter_radius: Vec2f;
        slab_read!(Vec2f, get!(SLAB), input.id, filter_radius);
        let x = filter_radius.x;
        let y = filter_radius.y;

        // Take 9 samples around current texel:
        // a - b - c
        // d - e - f
        // g - h - i
        // === ('e' is the current texel) ===
        let a = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - x, input.uv.y + y)).xyz();
        let b = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y + y)).xyz();
        let c = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + x, input.uv.y + y)).xyz();

        let d = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - x, input.uv.y)).xyz();
        let e = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y)).xyz();
        let f = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + x, input.uv.y)).xyz();

        let g = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x - x, input.uv.y - y)).xyz();
        let h = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x, input.uv.y - y)).xyz();
        let i = texture_sample(TEXTURE, SAMPLER, vec2f(input.uv.x + x, input.uv.y - y)).xyz();

        // Apply weighted distribution, by using a 3x3 tent filter:
        //  1   | 1 2 1 |
        // -- * | 2 4 2 |
        // 16   | 1 2 1 |
        let mut sample = e * 4.0;
        sample += (b + d + f + h) * 2.0;
        sample += a + c + g + i;
        sample *= 1.0 / 16.0;

        vec4f(sample.x, sample.y, sample.z, 0.5)
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod mix {
    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    use super::vertex::*;

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);
    texture!(group(0), binding(1), HDR_TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(2), HDR_SAMPLER: Sampler);
    texture!(group(0), binding(3), BLOOM_TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(4), BLOOM_SAMPLER: Sampler);

    #[fragment]
    /// Bloom "mix" shader.
    ///
    /// This is the final step in applying bloom in which the computed bloom is
    /// mixed with the source texture according to a strength factor.
    pub fn bloom_mix_fragment(input: BloomInterStage) -> Vec4f {
        let bloom_strength: Vec2f;
        slab_read!(Vec2f, get!(SLAB), input.id, bloom_strength);

        let hdr = texture_sample(HDR_TEXTURE, HDR_SAMPLER, input.uv).rgb();
        let bloom = texture_sample(BLOOM_TEXTURE, BLOOM_SAMPLER, input.uv).rgb();
        let s = bloom_strength.x();
        let color = mix(hdr, bloom, vec3f(s, s, s));
        vec4f(color.r(), color.g(), color.b(), 1.0)
    }
}

pub mod wgsl {
    //! wgsl-rs based bloom module.
    //!
    //! Temporary until the wgsl-rs port is done. Once the port is done this
    //! becomes the main module here.
    use core::ops::Deref;
    use std::sync::{Arc, RwLock};

    use craballoc_wgsl::{
        arena::{Arena, Value},
        buffer::SlabBuffer,
        runtime::WgpuRuntime,
    };
    use crabslab_wgsl::Id;
    use wgsl_rs::std::{vec2f, vec2u, Vec2f, Vec2u};
    use wgsl_rs_linkage_wgpu::{analysis::assemble_ir, IrModuleExt, WgpuLinkage};

    use crate::texture::{self, Texture};

    use super::{mix, sample, vertex};

    struct BloomLinkage {
        vertex: WgpuLinkage,
        sample: WgpuLinkage,
        mix: WgpuLinkage,
    }

    impl BloomLinkage {
        pub fn new() -> Self {
            let vertex_module = assemble_ir(&vertex::WGSL_SOURCE).expect("bloom vertex assembly");
            let sample_module =
                assemble_ir(&sample::WGSL_SOURCE).expect("bloom [up|down]sample assembly");
            let mix_module = assemble_ir(&mix::WGSL_SOURCE).expect("bloom mix assembly");

            BloomLinkage {
                vertex: vertex_module.generate_linkage(),
                sample: sample_module.generate_linkage(),
                mix: mix_module.generate_linkage(),
            }
        }

        fn create_bloom_downsample_pipeline(
            &mut self,
            device: &wgpu::Device,
        ) -> wgpu::RenderPipeline {
            let label = Some("bloom downsample");
            let layout = self.sample.pipeline_layout(device, label);
            let vertex_module = self.vertex.shader_module(device);
            let fragment_module = self.sample.shader_module(device);
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_module,
                    entry_point: Some("bloom_vertex"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_module,
                    entry_point: Some("bloom_downsample_fragment"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: None,
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                    compilation_options: Default::default(),
                }),
                multiview_mask: None,
                cache: None,
            })
        }

        fn create_bloom_upsample_pipeline(
            &mut self,
            device: &wgpu::Device,
        ) -> wgpu::RenderPipeline {
            let label = Some("bloom upsample");
            let layout = self.sample.pipeline_layout(device, label);
            let vertex_module = self.vertex.shader_module(device);
            let fragment_module = self.sample.shader_module(device);
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_module,
                    entry_point: Some("bloom_vertex"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_module,
                    entry_point: Some("bloom_upsample_fragment"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                    compilation_options: Default::default(),
                }),
                multiview_mask: None,
                cache: None,
            })
        }

        fn create_mix_pipeline(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
            let label = Some("bloom mix");
            let layout = self.mix.pipeline_layout(device, label);
            let vertex_module = self.vertex.shader_module(device);
            let fragment_module = self.mix.shader_module(device);
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_module,
                    entry_point: Some("bloom_vertex"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_module,
                    entry_point: Some("bloom_mix_fragment"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                    compilation_options: Default::default(),
                }),
                multiview_mask: None,
                cache: None,
            })
        }

        /// Create a bindgroup for either downsample or upsample shaders.
        fn create_sample_bindgroup(
            &mut self,
            device: &wgpu::Device,
            buffer: &wgpu::Buffer,
            tex: &Texture,
        ) -> wgpu::BindGroup {
            self.sample
                .create_bind_group_named(
                    0,
                    device,
                    &[
                        (
                            "SLAB",
                            wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                        ),
                        ("TEXTURE", wgpu::BindingResource::TextureView(&tex.view)),
                        ("SAMPLER", wgpu::BindingResource::Sampler(&tex.sampler)),
                    ],
                )
                .expect("bloom downsample bindgroup")
        }

        /// Create a sample bindgroup for each give texture.
        fn create_bindgroups(
            &mut self,
            device: &wgpu::Device,
            buffer: &wgpu::Buffer,
            textures: &[Texture],
        ) -> Vec<wgpu::BindGroup> {
            textures
                .iter()
                .map(|tex| self.create_sample_bindgroup(device, buffer, tex))
                .collect()
        }

        fn create_mix_bindgroup(
            &mut self,
            device: &wgpu::Device,
            slab_buffer: &wgpu::Buffer,
            hdr_texture: &Texture,
            bloom_texture: &Texture,
        ) -> wgpu::BindGroup {
            self.mix
                .create_bind_group_named(
                    0,
                    device,
                    &[
                        (
                            "SLAB",
                            wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                        ),
                        (
                            "HDR_TEXTURE",
                            wgpu::BindingResource::TextureView(&hdr_texture.view),
                        ),
                        (
                            "HDR_SAMPLER",
                            wgpu::BindingResource::Sampler(&hdr_texture.sampler),
                        ),
                        (
                            "BLOOM_TEXTURE",
                            wgpu::BindingResource::TextureView(&bloom_texture.view),
                        ),
                        (
                            "BLOOM_SAMPLER",
                            wgpu::BindingResource::Sampler(&bloom_texture.sampler),
                        ),
                    ],
                )
                .expect("bloom mix bindgroup")
        }
    }

    pub(crate) fn config_resolutions(resolution: Vec2u) -> impl Iterator<Item = Vec2u> {
        let num_textures = resolution.x.min(resolution.y).ilog2();
        (0..=num_textures).map(move |i| Vec2u {
            x: resolution.x >> i,
            y: resolution.y >> i,
        })
    }

    fn create_texture(
        runtime: impl AsRef<WgpuRuntime>,
        width: u32,
        height: u32,
        label: Option<&str>,
        extra_usages: wgpu::TextureUsages,
    ) -> texture::Texture {
        let sampler = runtime
            .as_ref()
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label,
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Linear,
                ..Default::default()
            });
        Texture::new_with_wgsl(
            runtime,
            label,
            Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | extra_usages,
            ),
            Some(sampler),
            wgpu::TextureFormat::Rgba16Float,
            4,
            2,
            width,
            height,
            1,
            &[],
        )
    }

    fn create_textures(
        runtime: impl AsRef<WgpuRuntime>,
        resolution: Vec2u,
    ) -> Vec<texture::Texture> {
        let resolutions = config_resolutions(resolution).collect::<Vec<_>>();
        log::trace!(
            "creating {} bloom textures at resolution {resolution:?}",
            resolutions.len()
        );
        let mut textures = vec![];
        for (
            i,
            Vec2u {
                x: width,
                y: height,
            },
        ) in resolutions.into_iter().skip(1).enumerate()
        {
            let title = format!("bloom texture[{i}]");
            let label = Some(title.as_str());
            let texture = create_texture(
                runtime.as_ref(),
                width,
                height,
                label,
                wgpu::TextureUsages::empty(),
            );
            textures.push(texture);
        }
        textures
    }

    /// Performs a "physically based" bloom effect on a texture. CPU only.
    ///
    /// Contains pipelines, down/upsampling textures, a buffer
    /// to communicate configuration to the shaders, and bindgroups.
    ///
    /// Clones of [`Bloom`] all point to the same resources.
    #[derive(Clone)]
    pub struct Bloom {
        arena: Arena<WgpuRuntime>,
        slab_buffer: SlabBuffer<wgpu::Buffer>,

        linkage: Arc<RwLock<BloomLinkage>>,

        downsample_pixel_sizes: Value<[Vec2f]>,
        downsample_pipeline: Arc<wgpu::RenderPipeline>,

        upsample_filter_radius: Value<Vec2f>,
        upsample_pipeline: Arc<wgpu::RenderPipeline>,

        textures: Arc<RwLock<Vec<texture::Texture>>>,
        bindgroups: Arc<RwLock<Vec<wgpu::BindGroup>>>,
        hdr_texture_downsample_bindgroup: Arc<RwLock<wgpu::BindGroup>>,

        mix_pipeline: Arc<wgpu::RenderPipeline>,
        mix_bindgroup: Arc<RwLock<wgpu::BindGroup>>,
        mix_texture: Arc<RwLock<Texture>>,
        mix_strength: Value<f32>,
    }

    impl Bloom {
        pub fn new(runtime: impl AsRef<WgpuRuntime>, hdr_texture: &Texture) -> Self {
            let runtime = runtime.as_ref();
            let resolution = vec2u(hdr_texture.width(), hdr_texture.height());

            let arena = Arena::new(runtime, "bloom-slab", None);
            let downsample_pixel_sizes = arena.new_array(
                config_resolutions(resolution).map(|r| 1.0 / vec2f(r.x as f32, r.y as f32)),
            );
            let upsample_filter_radius =
                arena.new_value(1.0 / vec2f(resolution.x as f32, resolution.y as f32));
            let mix_strength = arena.new_value(0.04f32);
            let slab_buffer = arena.commit();

            let mut linkage = BloomLinkage::new();

            let downsample_pipeline =
                Arc::new(linkage.create_bloom_downsample_pipeline(&runtime.device));
            let upsample_pipeline =
                Arc::new(linkage.create_bloom_upsample_pipeline(&runtime.device));
            let mix_pipeline = Arc::new(linkage.create_mix_pipeline(&runtime.device));

            let hdr_texture_downsample_bindgroup =
                linkage.create_sample_bindgroup(&runtime.device, &slab_buffer, hdr_texture);

            let mix_texture = create_texture(
                runtime,
                resolution.x,
                resolution.y,
                Some("bloom mix"),
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            );

            let textures = create_textures(runtime, resolution);
            let bindgroups = linkage.create_bindgroups(&runtime.device, &slab_buffer, &textures);

            let mix_bindgroup = linkage.create_mix_bindgroup(
                &runtime.device,
                &slab_buffer,
                hdr_texture,
                &textures[0],
            );

            Self {
                arena,
                slab_buffer,
                linkage: Arc::new(RwLock::new(linkage)),
                downsample_pixel_sizes,
                downsample_pipeline,
                upsample_filter_radius,
                upsample_pipeline,
                textures: Arc::new(RwLock::new(textures)),
                bindgroups: Arc::new(RwLock::new(bindgroups)),
                hdr_texture_downsample_bindgroup: Arc::new(RwLock::new(
                    hdr_texture_downsample_bindgroup,
                )),
                mix_pipeline,
                mix_texture: Arc::new(RwLock::new(mix_texture)),
                mix_bindgroup: Arc::new(RwLock::new(mix_bindgroup)),
                mix_strength,
            }
        }

        pub(crate) fn arena(&self) -> &Arena<WgpuRuntime> {
            &self.arena
        }

        pub fn set_mix_strength(&self, strength: f32) {
            self.mix_strength.set(strength);
        }

        pub fn get_mix_strength(&self) -> f32 {
            self.mix_strength.get()
        }

        /// Set the filter radius in pixels.
        pub fn set_filter_radius(&self, filter_radius: f32) {
            let size = self.get_size();
            let filter_radius = vec2f(filter_radius / size.x as f32, filter_radius / size.y as f32);
            self.upsample_filter_radius.set(filter_radius);
        }

        pub fn get_filter_radius(&self) -> Vec2f {
            self.upsample_filter_radius.get()
        }

        pub fn get_size(&self) -> Vec2u {
            let mix_texture = self.get_mix_texture();
            vec2u(mix_texture.width(), mix_texture.height())
        }

        /// Recreates this bloom using the new HDR texture.
        pub fn set_hdr_texture(&self, runtime: impl AsRef<WgpuRuntime>, hdr_texture: &Texture) {
            let resolution = vec2u(hdr_texture.width(), hdr_texture.height());
            let runtime = runtime.as_ref();
            let textures = create_textures(runtime, resolution);
            let mut linkage = self.linkage.write().expect("poisoned");

            *self.bindgroups.write().expect("bloom bindgroups write") =
                linkage.create_bindgroups(&runtime.device, &self.slab_buffer, &textures);
            *self
                .hdr_texture_downsample_bindgroup
                .write()
                .expect("bloom hdr downsample bindgroup write") =
                linkage.create_sample_bindgroup(&runtime.device, &self.slab_buffer, hdr_texture);
            *self.mix_texture.write().expect("bloom mix_texture write") = create_texture(
                runtime,
                resolution.x,
                resolution.y,
                Some("bloom mix"),
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            );
            *self
                .mix_bindgroup
                .write()
                .expect("bloom mix_bindgroup write") = linkage.create_mix_bindgroup(
                &runtime.device,
                &self.slab_buffer,
                hdr_texture,
                &textures[0],
            );
            *self.textures.write().expect("bloom textures write") = textures;
        }

        /// Returns a clone of the current mix texture.
        ///
        /// The mix texture is the result of mixing the bloom by the hdr using
        /// the mix strength.
        pub fn get_mix_texture(&self) -> Texture {
            // UNWRAP: not safe but we want to panic
            self.mix_texture
                .read()
                .expect("bloom mix_texture read")
                .clone()
        }

        pub(crate) fn render_downsamples(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
            struct DownsampleItem<'a> {
                view: &'a wgpu::TextureView,
                bindgroup: &'a wgpu::BindGroup,
                pixel_size: Id<Vec2f>,
            }
            // Get all the bindgroups (which are what we're reading from),
            // starting with the hdr frame.
            // Since `bindgroups` are one element greater (we pushed
            // `hdr_texture_bindgroup` to the front) the last bindgroup will
            // not be used, which is good - we don't need to read from the
            // smallest texture during downsampling. UNWRAP: not safe but we
            // want to panic
            let textures_guard = self.textures.read().expect("bloom textures read");
            let hdr_texture_downsample_bindgroup_guard = self
                .hdr_texture_downsample_bindgroup
                .read()
                .expect("bloom hdr downsample bindgroup read");
            let hdr_texture_downsample_bindgroup: &wgpu::BindGroup =
                &hdr_texture_downsample_bindgroup_guard;
            let bindgroups_guard = self.bindgroups.read().expect("bloom bindgroups read");
            let bindgroups =
                std::iter::once(hdr_texture_downsample_bindgroup).chain(bindgroups_guard.iter());
            let items = textures_guard
                .iter()
                .zip(bindgroups)
                .zip(self.downsample_pixel_sizes.array().iter())
                .map(|((tex, bindgroup), pixel_size)| DownsampleItem {
                    view: &tex.view,
                    bindgroup,
                    pixel_size,
                });
            for (
                i,
                DownsampleItem {
                    view,
                    bindgroup,
                    pixel_size,
                },
            ) in items.enumerate()
            {
                let title = format!("bloom downsample {i}");
                let label = Some(title.as_str());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                    render_pass.set_pipeline(&self.downsample_pipeline);
                    render_pass.set_bind_group(0, Some(bindgroup), &[]);
                    let id = pixel_size.into();
                    render_pass.draw(0..6, id..id + 1);
                }
                queue.submit(std::iter::once(encoder.finish()));
            }
        }

        fn render_upsamples(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
            struct UpsampleItem<'a> {
                view: &'a wgpu::TextureView,
                bindgroup: &'a wgpu::BindGroup,
            }
            // Get all the bindgroups (which are what we're reading from),
            // starting with the last mip.
            // UNWRAP: not safe but we want to panic
            let bindgroups_guard = self.bindgroups.read().expect("bloom bindgroups read");
            let bindgroups = bindgroups_guard.iter().rev();
            // Get all the texture views (which are what we're writing to),
            // starting with the second-to-last mip.
            let textures_guard = self.textures.read().expect("bloom textures read");
            let views = textures_guard.iter().rev().skip(1).map(|t| &t.view);
            let items = bindgroups
                .zip(views)
                .map(|(bindgroup, view)| UpsampleItem { view, bindgroup });
            for (i, UpsampleItem { view, bindgroup }) in items.enumerate() {
                let title = format!("bloom upsample {}", textures_guard.len() - i - 1);
                let label = Some(title.as_str());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label,
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
                        multiview_mask: None,
                    });
                    render_pass.set_pipeline(&self.upsample_pipeline);
                    render_pass.set_bind_group(0, Some(bindgroup), &[]);
                    let id = self.upsample_filter_radius.id().into();
                    render_pass.draw(0..6, id..id + 1);
                }
                queue.submit(std::iter::once(encoder.finish()));
            }
        }

        fn render_mix(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
            let label = Some("bloom mix");
            // UNWRAP: not safe but we want to panic
            let mix_texture = self.mix_texture.read().expect("bloom mix_texture read");
            let mix_bindgroup = self.mix_bindgroup.read().expect("bloom mix_bindgroup read");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &mix_texture.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                render_pass.set_pipeline(&self.mix_pipeline);
                render_pass.set_bind_group(0, Some(mix_bindgroup.deref()), &[]);
                let id = self.mix_strength.id().into();
                render_pass.draw(0..6, id..id + 1);
            }

            queue.submit(std::iter::once(encoder.finish()));
        }

        pub fn bloom(&self, runtime: impl AsRef<craballoc_wgsl::runtime::WgpuRuntime>) {
            let latest_buffer = self.arena.commit();
            assert!(
                self.slab_buffer.creation_time() == latest_buffer.creation_time(),
                "bloom slab buffer resized (it shouldn't)"
            );

            let WgpuRuntime { device, queue } = runtime.as_ref();
            self.render_downsamples(device, queue);
            self.render_upsamples(device, queue);
            self.render_mix(device, queue);
        }
    }
}

#[cfg(test)]
mod test {
    use wgsl_rs::std::{vec2f, vec2u, vec3f, Vec3f};

    use crate::{context::Context, test::BlockOnFuture, texture::Texture};

    use super::wgsl::*;

    #[test]
    fn bloom_texture_sizes_sanity() {
        let sizes = config_resolutions(vec2u(1024, 800)).collect::<Vec<_>>();
        assert_eq!(
            vec![
                vec2u(1024, 800),
                vec2u(512, 400),
                vec2u(256, 200),
                vec2u(128, 100),
                vec2u(64, 50),
                vec2u(32, 25),
                vec2u(16, 12),
                vec2u(8, 6),
                vec2u(4, 3),
                vec2u(2, 1),
            ],
            sizes
        );
        let pixel_sizes = config_resolutions(vec2u(1024, 800))
            .map(|r| 1.0 / vec2f(r.x as f32, r.y as f32))
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                vec2f(0.0009765625, 0.00125),
                vec2f(0.001953125, 0.0025),
                vec2f(0.00390625, 0.005),
                vec2f(0.0078125, 0.01),
                vec2f(0.015625, 0.02),
                vec2f(0.03125, 0.04),
                vec2f(0.0625, 0.083333336),
                vec2f(0.125, 0.16666667),
                vec2f(0.25, 0.33333334),
                vec2f(0.5, 1.0)
            ],
            pixel_sizes
        );
    }

    #[test]
    fn bloom_sanity() {
        let width = 48;
        let height = 48;
        let ctx = Context::headless(width, height).block();

        // Rgba16Float, RENDER_ATTACHMENT|TEXTURE_BINDING|COPY_DST|COPY_SRC
        // already set
        let hdr = Texture::create_hdr_texture(ctx.get_device(), width, height, 1);

        fn fill_tex(width: u32, height: u32, ctx: &Context, tex: &Texture) {
            // procedural HDR content, converted to f16 (the format is
            // Rgba16Float!)
            let mut texels = vec![0u16; (width * height * 4) as usize];
            for y in 0..height {
                for x in 0..width {
                    let i = ((y * width + x) * 4) as usize;
                    let dx = x as f32 - width as f32 * 0.5;
                    let dy = y as f32 - height as f32 * 0.5;
                    let in_blob = dx * dx + dy * dy < 16.0 * 16.0;
                    let v = [
                        0.02, // dim field
                        0.02,
                        if in_blob { 50.0 } else { 0.05 }, // bright blue blob
                        1.0,
                    ];
                    for (slot, val) in v.iter().enumerate() {
                        texels[i + slot] = half::f16::from_f32(*val).to_bits();
                    }
                }
            }
            ctx.get_queue().write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex.texture,
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
        }
        fill_tex(width, height, &ctx, &hdr);

        let runtime = craballoc_wgsl::runtime::WgpuRuntime {
            device: ctx.runtime().device.clone(),
            queue: ctx.runtime().queue.clone(),
        };

        let bloom = Bloom::new(&runtime, &hdr);
        bloom.bloom(&runtime);

        let result = bloom
            .get_mix_texture()
            .read_hdr_image(&ctx)
            .block()
            .unwrap();
        img_diff::assert_img_eq("bloom/result.exr", result);
    }

    #[test]
    /// Tests that the Bloom filter is properly integrated into `Stage`.
    fn bloom_integration() {
        let width = 256;
        let height = 128;
        let ctx = Context::headless(width, height).block();
        let stage = ctx.new_stage().with_bloom(false);
        let projection = crate::camera::perspective(width as f32, height as f32);
        let view = crate::camera::look_at(vec3f(0.0, 2.0, 18.0), Vec3f::ZERO, Vec3f::Y);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let skybox = stage
            .new_skybox_from_path("../../img/hdr/night.hdr")
            .unwrap();
        stage.use_skybox(&skybox);
        let ibl = stage.new_ibl(&skybox);
        stage.use_ibl(&ibl);

        let _doc = stage
            .load_gltf_document_from_path("../../gltf/EmissiveStrengthTest.glb")
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("bloom/without.png", img);
        frame.present();

        // now render the whole thing with default values
        stage.set_has_bloom(true);
        stage.set_bloom_mix_strength(0.1);
        stage.set_bloom_filter_radius(2.0);
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("bloom/with.png", img);
    }
}
