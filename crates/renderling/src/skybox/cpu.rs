//! CPU-side code for skybox rendering.
use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use craballoc::{prelude::SlabAllocator, runtime::WgpuRuntime};
use glam::{Mat4, UVec2, Vec3};

use crate::{
    atlas::AtlasImage,
    camera::Camera,
    cubemap::EquirectangularImageToCubemapBlitter,
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

fn skybox_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("skybox bindgroup"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

pub(crate) fn create_skybox_bindgroup(
    device: &wgpu::Device,
    slab_buffer: &wgpu::Buffer,
    texture: &Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("skybox"),
        layout: &skybox_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: slab_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

/// Create the skybox rendering pipeline.
pub(crate) fn create_skybox_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    multisample_count: Option<u32>,
) -> SkyboxRenderPipeline {
    log::trace!("creating skybox render pipeline with format '{format:?}'");
    let vertex_linkage = crate::linkage::skybox_vertex::linkage(device);
    let fragment_linkage = crate::linkage::skybox_cubemap_fragment::linkage(device);
    let bg_layout = skybox_bindgroup_layout(device);
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("skybox pipeline layout"),
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    let msaa_sample_count = multisample_count.unwrap_or(1);
    SkyboxRenderPipeline {
        pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("skybox render pipeline"),
            layout: Some(&pp_layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: Some(vertex_linkage.entry_point),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                mask: !0,
                alpha_to_coverage_enabled: false,
                count: msaa_sample_count,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: Some(fragment_linkage.entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            multiview: None,
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
            size: UVec2::splat(1),
            format: crate::atlas::AtlasImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        };
        let s = Self::new(runtime, hdr_img);
        s.is_empty.store(true, std::sync::atomic::Ordering::Relaxed);
        s
    }

    /// Create a new `Skybox`.
    pub fn new(runtime: impl AsRef<WgpuRuntime>, hdr_img: AtlasImage) -> Self {
        let runtime = runtime.as_ref();
        log::trace!("creating skybox");

        let slab = SlabAllocator::new(runtime, "skybox-slab", wgpu::BufferUsages::VERTEX);
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let camera = Camera::new(&slab).with_projection(proj);
        let buffer = slab.commit();
        let mut buffer_upkeep = || {
            let possibly_new_buffer = slab.commit();
            debug_assert!(!possibly_new_buffer.is_new_this_commit());
        };

        let equirectangular_texture = Skybox::hdr_texture_from_atlas_image(runtime, hdr_img);
        let views = [
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            ),
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            ),
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            ),
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, -1.0, 0.0),
            ),
            Mat4::look_at_rh(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, -1.0, 0.0),
            ),
        ];

        // Create environment map.
        let environment_cubemap = Skybox::create_environment_map_from_hdr(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            &equirectangular_texture,
            &camera,
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
        let runtime = runtime.as_ref();
        Texture::new_with(
            runtime,
            Some("create hdr texture"),
            None,
            Some(runtime.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })),
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
        camera: &Camera,
        views: [Mat4; 6],
    ) -> Texture {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        let queue = &runtime.queue;
        // Create the cubemap-making pipeline.
        let pipeline =
            EquirectangularImageToCubemapBlitter::new(device, wgpu::TextureFormat::Rgba16Float);

        let resources = (
            device,
            queue,
            Some("hdr environment map"),
            wgpu::BufferUsages::VERTEX,
        );
        let bindgroup = EquirectangularImageToCubemapBlitter::create_bindgroup(
            device,
            resources.2,
            buffer,
            hdr_texture,
        );

        texture::Texture::render_cubemap(
            runtime,
            "skybox-environment",
            &pipeline.0,
            buffer_upkeep,
            camera,
            &bindgroup,
            views,
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
    use glam::Vec3;

    use crate::{context::Context, test::BlockOnFuture};

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
}
