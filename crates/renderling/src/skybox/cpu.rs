//! CPU-side code for skybox rendering.
use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use craballoc_wgsl::{arena::Arena, runtime::WgpuRuntime};
use wgsl_rs::{
    glm::{look_at_rh, perspective_rh},
    std::{vec2u_splat, vec3f, Mat4f},
};

use wgsl_rs_linkage_wgpu::IrModuleExt;

use crate::{
    atlas::AtlasImage,
    shader::camera::CameraDescriptor,
    texture::{self, Texture},
};

/// Render pipeline used to draw a skybox.
pub struct SkyboxRenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
    msaa_sample_count: u32,
}

impl SkyboxRenderPipeline {
    pub fn msaa_sample_count(&self) -> u32 {
        self.msaa_sample_count
    }
}


pub(crate) fn create_skybox_bindgroup(
    device: &wgpu::Device,
    slab_buffer: &wgpu::Buffer,
    texture: &Texture,
) -> wgpu::BindGroup {
    // The wgsl-rs module's group-0 shape (SLAB, TEXTURE, SAMPLER).
    let mut linkage = crate::skybox::render::instantiate().generate_linkage();
    linkage
        .create_bind_group_named(
            0,
            device,
            &[
                (
                    "SLAB",
                    wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: slab_buffer,
                        offset: 0,
                        size: None,
                    }),
                ),
                ("TEXTURE", wgpu::BindingResource::TextureView(&texture.view)),
                ("SAMPLER", wgpu::BindingResource::Sampler(&texture.sampler)),
            ],
        )
        .expect("skybox bind group")
}

/// Create the skybox rendering pipeline.
pub(crate) fn create_skybox_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    multisample_count: Option<u32>,
) -> SkyboxRenderPipeline {
    log::trace!("creating skybox render pipeline with format '{format:?}'");
    // The wgsl-rs module's skybox render entries (instantiate = full
    // assembled IR).
    let mut linkage = crate::skybox::render::instantiate().generate_linkage();
    let wgsl_module = linkage.shader_module(device);
    let pipeline_layout = linkage.pipeline_layout(device, Some("skybox pipeline layout"));
    let msaa_sample_count = multisample_count.unwrap_or(1);
    SkyboxRenderPipeline {
        pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            multiview_mask: None,
            label: Some("skybox render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: linkage
                .vertex_entry("vertex")
                .expect("skybox render vertex entry")
                .vertex_state(&wgsl_module),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            // The skybox writes far-plane depth (xyww), so depth must be on
            // with write and LessEqual.
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                mask: !0,
                alpha_to_coverage_enabled: false,
                count: msaa_sample_count,
            },
            fragment: Some(wgpu::FragmentState {
                module: &wgsl_module,
                entry_point: Some("fragment"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            cache: None,
        }),
        msaa_sample_count,
    }
}

/// An HDR skybox.
///
/// Skyboxes provide an environment cubemap around all your scenery
/// that acts as a background.
///
/// A [`Skybox`] can also be used to create [`Ibl`], which illuminates
/// your scene using the environment map as a light source.
///
/// All clones of a skybox point to the same underlying data.
#[derive(Debug, Clone)]
pub struct Skybox {
    is_empty: Arc<AtomicBool>,
    // Cubemap texture of the environment cubemap
    environment_cubemap: Texture,
}

impl Skybox {
    /// Create an empty, transparent skybox.
    pub fn empty(runtime: impl AsRef<WgpuRuntime>) -> Self {
        let runtime = runtime.as_ref();
        log::trace!("creating empty skybox");
        let hdr_img = AtlasImage {
            pixels: vec![0u8; 4 * 4],
            size: vec2u_splat(1).into(),
            format: crate::atlas::AtlasImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        };
        let s = Self::new(runtime, hdr_img);
        s.is_empty.store(true, std::sync::atomic::Ordering::Relaxed);
        s
    }

    /// Create a new `Skybox`.
    pub fn new(runtime: impl AsRef<WgpuRuntime>, hdr_img: AtlasImage) -> Self {
        log::trace!("creating skybox");

        let WgpuRuntime { device, queue } = runtime.as_ref().clone();
        let runtime = craballoc_wgsl::runtime::WgpuRuntime { device, queue };
        let arena = Arena::new(&runtime, "skybox-slab", Some(wgpu::BufferUsages::VERTEX));
        let proj = perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let mut cd = CameraDescriptor::IDENTITY;
        cd.set_projection(proj);
        let camera_descriptor = arena.new_value(cd);
        let buffer = arena.commit();
        let buffer_creation_time = buffer.creation_time();
        let mut buffer_upkeep = || {
            let possibly_new_buffer = arena.commit();
            debug_assert!(possibly_new_buffer.creation_time() == buffer_creation_time);
        };

        let equirectangular_texture =
            Skybox::hdr_texture_from_atlas_image(runtime.clone(), hdr_img);
        let views = [
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(1.0, 0.0, 0.0),
                vec3f(0.0, -1.0, 0.0),
            ),
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(-1.0, 0.0, 0.0),
                vec3f(0.0, -1.0, 0.0),
            ),
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(0.0, -1.0, 0.0),
                vec3f(0.0, 0.0, -1.0),
            ),
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(0.0, 1.0, 0.0),
                vec3f(0.0, 0.0, 1.0),
            ),
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(0.0, 0.0, 1.0),
                vec3f(0.0, -1.0, 0.0),
            ),
            look_at_rh(
                vec3f(0.0, 0.0, 0.0),
                vec3f(0.0, 0.0, -1.0),
                vec3f(0.0, -1.0, 0.0),
            ),
        ];

        // Create environment map.
        let environment_cubemap = Skybox::create_environment_map_from_hdr(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            &equirectangular_texture,
            |mat| {
                camera_descriptor.set(camera_descriptor.get().with_view(*mat));
            },
            views,
        );

        Skybox {
            is_empty: Arc::new(false.into()),
            environment_cubemap,
        }
    }

    /// Return a reference to the environment cubemap texture.
    pub fn environment_cubemap_texture(&self) -> &texture::Texture {
        &self.environment_cubemap
    }

    /// Convert an HDR [`AtlasImage`] into a texture.
    pub fn hdr_texture_from_atlas_image(
        runtime: impl AsRef<WgpuRuntime>,
        img: AtlasImage,
    ) -> Texture {
        Texture::new_with(
            craballoc::runtime::WgpuRuntime {
                device: runtime.as_ref().device.clone(),
                queue: runtime.as_ref().queue.clone(),
            },
            Some("create hdr texture"),
            None,
            Some(
                runtime
                    .as_ref()
                    .device
                    .create_sampler(&wgpu::SamplerDescriptor {
                        mag_filter: wgpu::FilterMode::Nearest,
                        min_filter: wgpu::FilterMode::Nearest,
                        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                        ..Default::default()
                    }),
            ),
            wgpu::TextureFormat::Rgba32Float,
            4,
            4,
            img.size.x,
            img.size.y,
            1,
            &img.pixels,
        )
    }

    /// Create an HDR equirectangular texture from bytes.
    pub fn create_hdr_texture(runtime: impl AsRef<WgpuRuntime>, hdr_data: &[u8]) -> Texture {
        let runtime = runtime.as_ref();
        let img = AtlasImage::from_hdr_bytes(hdr_data).unwrap();
        Self::hdr_texture_from_atlas_image(runtime, img)
    }

    fn create_environment_map_from_hdr(
        runtime: impl AsRef<WgpuRuntime>,
        buffer: &wgpu::Buffer,
        buffer_upkeep: impl FnMut(),
        hdr_texture: &Texture,
        // TODO: remove this after wgsl-rs port is done in favor of passing `&Camera`
        mut with_view: impl FnMut(&Mat4f),
        views: [Mat4f; 6],
    ) -> Texture {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        // The blit pipeline comes from the wgsl-rs module. The equirect
        // source is Rgba32Float (unfilterable in WebGPU), which the module
        // declares for both its texture and sampler bindings.
        let mut linkage = crate::skybox::equirect_to_cubemap::instantiate().generate_linkage();
        let wgsl_module = linkage.shader_module(device);
        let pipeline_layout = linkage.pipeline_layout(device, Some("cubemap-making pipeline"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            multiview_mask: None,
            label: Some("cubemap-making pipeline"),
            layout: Some(&pipeline_layout),
            vertex: linkage
                .vertex_entry("vertex")
                .expect("equirect_to_cubemap vertex entry")
                .vertex_state(&wgsl_module),
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
                module: &wgsl_module,
                entry_point: Some("fragment"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            cache: None,
        });

        let bindgroup = linkage
            .create_bind_group_named(
                0,
                device,
                &[
                    (
                        "SLAB",
                        wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                    ),
                    (
                        "TEXTURE",
                        wgpu::BindingResource::TextureView(&hdr_texture.view),
                    ),
                    (
                        "SAMPLER",
                        wgpu::BindingResource::Sampler(&hdr_texture.sampler),
                    ),
                ],
            )
            .expect("equirect bind group");

        let with_view_glam = |mat: &glam::Mat4| {
            let mat_wgsl = Mat4f::from(*mat);
            with_view(&mat_wgsl)
        };

        texture::Texture::render_cubemap(
            craballoc::runtime::WgpuRuntime {
                device: runtime.device.clone(),
                queue: runtime.queue.clone(),
            },
            "skybox-environment",
            &pipeline,
            buffer_upkeep,
            with_view_glam,
            &bindgroup,
            views.map(|m| m.into()),
            512,
            Some(9),
        )
    }

    /// Returns whether this skybox is empty.
    pub fn is_empty(&self) -> bool {
        self.is_empty.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod test {
    use craballoc::prelude::SlabAllocator;
    use glam::{Mat4, UVec2, Vec3};
    use wgpu::RenderPassDepthStencilAttachment;

    use crate::{
        atlas::{AtlasImage, AtlasImageFormat},
        camera::Camera,
        context::Context,
        test::BlockOnFuture,
        texture::Texture,
    };

    use super::{
        create_skybox_bindgroup, create_skybox_render_pipeline, Skybox, SkyboxRenderPipeline,
    };

    #[test]
    fn hdr_skybox_scene() {
        let ctx = Context::headless(600, 400).block();
        let proj = crate::camera::perspective(600.0, 400.0);
        let view = crate::camera::look_at(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, Vec3::Y);
        let stage = ctx.new_stage();
        let _camera = stage.new_camera().with_projection_and_view(proj, view);
        let skybox = stage
            .new_skybox_from_path("../../img/hdr/resting_place.hdr")
            .unwrap();
        stage.use_skybox(&skybox);
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_linear_image().block().unwrap();
        img_diff::assert_img_eq("skybox/hdr.png", img);
    }

    // ---- golden helpers ---------------------------------------------------
    //
    // These tests save EXR goldens for every step of the legacy skybox
    // rendering process, so the wgsl-rs port can A/B against them per step.
    // All inputs are procedural: every direction maps to a unique color, so
    // any face-order/rotation/uv bug shows up as a displaced gradient.
    //
    // NOTE: camera handles MUST stay alive until after `slab.commit()` -
    // craballoc recycles an allocation whose hybrid is dropped before the
    // commit, and a recycled source's queued updates are never written, so
    // dropped cameras leave zeroed descriptors in the buffer (w = 0 then
    // clips every fragment).

    /// A procedural equirectangular image, 64x32 f32 rgba: r = u, g = v,
    /// b = |v - 0.5| * 2 (equator band is blue, poles are white).
    fn procedural_equirect_image() -> AtlasImage {
        let width = 64usize;
        let height = 32usize;
        let mut texels = Vec::with_capacity(width * height * 4);
        for row in 0..height {
            for col in 0..width {
                let u = (col as f32 + 0.5) / width as f32;
                let v = (row as f32 + 0.5) / height as f32;
                texels.extend_from_slice(&[u, v, (v - 0.5).abs() * 2.0, 1.0]);
            }
        }
        AtlasImage {
            pixels: bytemuck::cast_slice(&texels).to_vec(),
            size: UVec2::new(width as u32, height as u32),
            format: AtlasImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        }
    }

    /// The equirectangular uv of a direction, mirroring the shader's
    /// `direction_to_equirectangular_uv` (including its rounded constants).
    fn equirect_uv(dir: Vec3) -> (f32, f32) {
        const INV_ATAN_X: f32 = 0.1591;
        const INV_ATAN_Y: f32 = 0.31830987;
        let u = dir.z.atan2(dir.x) * INV_ATAN_X + 0.5;
        let v = dir.y.asin() * INV_ATAN_Y + 0.5;
        (u, v)
    }

    /// The source texel (x, y) that nearest-sampling lands on for a uv.
    fn texel_for_uv(u: f32, v: f32) -> (usize, usize) {
        let x = ((u * 64.0).floor() as usize).clamp(0, 63);
        let y = ((v * 32.0).floor() as usize).clamp(0, 31);
        (x, y)
    }

    /// Source pixel for a texel, matching `procedural_equirect_image`.
    fn procedural_pixel(x: usize, y: usize) -> [f32; 4] {
        [
            (x as f32 + 0.5) / 64.0,
            (y as f32 + 0.5) / 32.0,
            (((y as f32 + 0.5) / 32.0 - 0.5).abs()) * 2.0,
            1.0,
        ]
    }

    /// Issue one skybox draw (36 verts, instanced by camera SLOT id) and
    /// submit. Instances select cameras by their raw slab slot id - this
    /// mirrors `Stage::render`, which draws
    /// `0..36, camera_id..camera_id + 1`.
    fn draw_skybox_pass(
        ctx: &Context,
        pipeline: &SkyboxRenderPipeline,
        bindgroup: &wgpu::BindGroup,
        camera_id: u32,
        color_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        resolve_view: Option<&wgpu::TextureView>,
    ) {
        let device = ctx.get_device();
        let label = Some("skybox golden pass");
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label,
                multiview_mask: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: resolve_view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&pipeline.pipeline);
            pass.set_bind_group(0, Some(bindgroup), &[]);
            pass.draw(0..36, camera_id..(camera_id + 1));
        }
        ctx.get_queue().submit(std::iter::once(encoder.finish()));
    }

    #[test]
    fn golden_equirect_to_cubemap_conversion() {
        let ctx = Context::headless(256, 256).block();
        let skybox = Skybox::new(&ctx, procedural_equirect_image());
        let env = skybox.environment_cubemap_texture();

        // The face cameras in `Skybox::new` render views in this order, one
        // per cubemap layer: +X, -X, -Y, +Y, +Z, -Z. The center texel of
        // each side face must be the source texel the shader's equirect uv
        // maps that axis direction to. The +Y/-Y faces are exempt from the
        // center cross-check: the atan2(z, x) equirect mapping is singular
        // at the poles, where a half-pixel screen offset swings the uv by
        // whole texels, so the pole faces carry EXR goldens only.
        let faces = [
            ("000_pos_x", Some(Vec3::new(1.0, 0.0, 0.0))),
            ("001_neg_x", Some(Vec3::new(-1.0, 0.0, 0.0))),
            ("002_neg_y", None),
            ("003_pos_y", None),
            ("004_pos_z", Some(Vec3::new(0.0, 0.0, 1.0))),
            ("005_neg_z", Some(Vec3::new(0.0, 0.0, -1.0))),
        ];
        for (i, (name, dir)) in faces.iter().enumerate() {
            let img = env
                .read_hdr_layer_image(ctx.runtime(), i as u32)
                .block()
                .unwrap();
            // Strict on purpose: any differing pixel fails (image_threshold
            // 0.0), guarding against the default 5%-of-image threshold
            // masking small regressions.
            img_diff::assert_img_eq_cfg(
                &format!("skybox/envface_{name}.exr"),
                img.clone(),
                // Relative thresholds: the wgsl-rs-rendered pixels are compared
                // against spirv-rendered goldens, so last-bit float path
                // differences get slack; any pixel still must pass.
                img_diff::DiffCfg {
                    pixel_threshold: 1.0e-3,
                    pixel_relative_threshold: 1.0e-3,
                    image_threshold: 0.0,
                    ..Default::default()
                },
            );

            let Some(dir) = dir else { continue };
            let (u, v) = equirect_uv(*dir);
            let (x, y) = texel_for_uv(u, v);
            let expected = procedural_pixel(x, y);
            let got = img.get_pixel(256, 256).0;
            // The face's center pixel is half a pixel away from the exact
            // axis direction, and the view's screen axes may mirror, so the
            // sampled texel can be a neighbor of the predicted one: tolerate
            // one source texel in the uv channels (this catches wrong-face
            // and rotation bugs, not half-texel phase).
            let tolerance = [
                1.0 / 64.0, // one source texel in the u channel
                1.0 / 32.0, // one source texel in the v channel
                0.5,        // the blue band is broad by construction
                1.0e-3,     // alpha
            ];
            for (ch, ((e, g), t)) in expected.iter().zip(got.iter()).zip(tolerance).enumerate() {
                let delta = (e - g).abs();
                assert!(
                    delta <= t,
                    "face {name} center channel {ch}: expected {e} got {g}"
                );
            }
        }
    }

    #[test]
    fn golden_skybox_screen_render() {
        let ctx = Context::headless(256, 256).block();
        let skybox = Skybox::new(&ctx, procedural_equirect_image());

        // Four cameras back to back in one slab; slot ids are the instance
        // range starts the shader reads cameras from. The handles stay bound
        // so craballoc does not recycle their slots before the commit.
        let slab = SlabAllocator::new(
            ctx.runtime(),
            "skybox-golden-slab",
            wgpu::BufferUsages::VERTEX,
        );
        let dirs = [
            ("pos_z", Vec3::new(0.0, 0.0, 1.0)),
            ("pos_x", Vec3::new(1.0, 0.0, 0.0)),
            ("neg_z", Vec3::new(0.0, 0.0, -1.0)),
            ("neg_x", Vec3::new(-1.0, 0.0, 0.0)),
        ];
        let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let _cameras: Vec<Camera> = dirs
            .iter()
            .map(|(_, dir)| {
                Camera::new(&slab).with_projection_and_view(
                    projection,
                    Mat4::look_at_rh(Vec3::ZERO, *dir, Vec3::Y),
                )
            })
            .collect();
        let camera_ids: Vec<u32> = _cameras.iter().map(|c| c.id().inner()).collect();
        let slab_buffer = slab.commit();

        let device = ctx.get_device();
        let pipeline = create_skybox_render_pipeline(device, Texture::HDR_TEXTURE_FORMAT, None);
        let bindgroup =
            create_skybox_bindgroup(device, &slab_buffer, skybox.environment_cubemap_texture());

        let color = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skybox golden color"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::HDR_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skybox golden depth"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let color_view = color.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth.create_view(&wgpu::TextureViewDescriptor::default());
        let color_img = Texture::from_wgpu_tex(device, color.clone(), None, None);

        for (i, (name, _)) in dirs.iter().enumerate() {
            draw_skybox_pass(
                &ctx,
                &pipeline,
                &bindgroup,
                camera_ids[i],
                &color_view,
                &depth_view,
                None,
            );
            let img = color_img.read_hdr_image(ctx.runtime()).block().unwrap();
            img_diff::assert_img_eq_cfg(
                &format!("skybox/screen_{name}.exr"),
                img.clone(),
                // Relative thresholds: the wgsl-rs-rendered pixels are compared
                // against spirv-rendered goldens, so last-bit float path
                // differences get slack; any pixel still must pass.
                img_diff::DiffCfg {
                    pixel_threshold: 1.0e-3,
                    pixel_relative_threshold: 1.0e-3,
                    image_threshold: 0.0,
                    ..Default::default()
                },
            );
        }

        // Depth: every skybox fragment writes far-plane depth (xyww), so the
        // whole target must be exactly 1.0 after the pass.
        let (_, _, depth_values) = Texture::read_depth_values(ctx.runtime(), &depth)
            .block()
            .unwrap();
        let far_depth = depth_values.iter().all(|d| (d - 1.0).abs() < 1.0e-5);
        assert!(far_depth, "skybox depth is not all-far");
        let mut texels = Vec::with_capacity(depth_values.len() * 4);
        for d in depth_values {
            texels.extend_from_slice(&[d, d, d, 1.0]);
        }
        let img = image::Rgba32FImage::from_raw(256, 256, texels).unwrap();
        img_diff::save("skybox/screen_depth.exr", img);
    }

    #[test]
    fn golden_skybox_screen_render_msaa4() {
        let ctx = Context::headless(256, 256).block();
        let skybox = Skybox::new(&ctx, procedural_equirect_image());

        let slab = SlabAllocator::new(
            ctx.runtime(),
            "skybox-golden-slab",
            wgpu::BufferUsages::VERTEX,
        );
        let camera = Camera::new(&slab).with_projection_and_view(
            Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0),
            Mat4::look_at_rh(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        );
        // The camera handle must still be alive at the commit (see the NOTE
        // above), so take its id before committing and keep the binding.
        let camera_id = camera.id().inner();
        let slab_buffer = slab.commit();

        let device = ctx.get_device();
        let pipeline = create_skybox_render_pipeline(device, Texture::HDR_TEXTURE_FORMAT, Some(4));
        let bindgroup =
            create_skybox_bindgroup(device, &slab_buffer, skybox.environment_cubemap_texture());

        let msaa_color = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skybox golden msaa color"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::HDR_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let resolve = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skybox golden msaa resolve"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::HDR_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let msaa_depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skybox golden msaa depth"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let msaa_view = msaa_color.create_view(&wgpu::TextureViewDescriptor::default());
        let resolve_view = resolve.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = msaa_depth.create_view(&wgpu::TextureViewDescriptor::default());
        let resolve_img = Texture::from_wgpu_tex(device, resolve.clone(), None, None);

        draw_skybox_pass(
            &ctx,
            &pipeline,
            &bindgroup,
            camera_id,
            &msaa_view,
            &depth_view,
            Some(&resolve_view),
        );
        let img = resolve_img.read_hdr_image(ctx.runtime()).block().unwrap();
        // The msaa resolve gets a slightly larger pixel threshold for
        // sample-order effects.
        img_diff::assert_img_eq_cfg(
            "skybox/screen_msaa4.exr",
            img.clone(),
            img_diff::DiffCfg {
                pixel_threshold: 1.0e-3,
                pixel_relative_threshold: 1.0e-3,
                image_threshold: 0.0,
                ..Default::default()
            },
        );
    }
}
