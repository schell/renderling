//! CPU side of IBL

use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use craballoc::{runtime::WgpuRuntime, slab::SlabAllocator, value::Hybrid};
use crabslab::Id;
use glam::{Mat4, Vec3};

use crate::{
    camera::Camera, convolution::shader::VertexPrefilterEnvironmentCubemapIds, skybox::Skybox,
    texture,
};

/// Image based lighting resources.
#[derive(Clone)]
pub struct Ibl {
    is_empty: Arc<AtomicBool>,
    // Cubemap texture of the pre-computed irradiance cubemap
    pub(crate) irradiance_cubemap: texture::Texture,
    // Cubemap texture and mip maps of the specular highlights,
    // where each mip level is a different roughness.
    pub(crate) prefiltered_environment_cubemap: texture::Texture,
}

impl Ibl {
    pub fn create_irradiance_and_prefilters(
        runtime: impl AsRef<WgpuRuntime>,
        skybox: &Skybox,
    ) -> Self {
        let runtime = runtime.as_ref();
        let slab = SlabAllocator::new(runtime, "ibl", wgpu::BufferUsages::VERTEX);
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let camera = Camera::new(&slab).with_projection(proj);
        let roughness = slab.new_value(0.0f32);
        let prefilter_ids = slab.new_value(VertexPrefilterEnvironmentCubemapIds {
            camera: camera.id(),
            roughness: roughness.id(),
        });

        let buffer = slab.commit();
        let mut buffer_upkeep = || {
            let possibly_new_buffer = slab.commit();
            debug_assert!(!possibly_new_buffer.is_new_this_commit());
        };

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

        let environment_cubemap = skybox.environment_cubemap_texture();

        // Convolve the environment map.
        let irradiance_cubemap = create_irradiance_map(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            environment_cubemap,
            &camera,
            views,
        );

        // Generate specular IBL pre-filtered environment map.
        let prefiltered_environment_cubemap = create_prefiltered_environment_map(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            &camera,
            &roughness,
            prefilter_ids.id(),
            environment_cubemap,
            views,
        );

        Self {
            is_empty: Arc::new(skybox.is_empty().into()),
            irradiance_cubemap,
            prefiltered_environment_cubemap,
        }
    }
    /// Create a new [`Ibl`] resource.
    pub fn new(runtime: impl AsRef<WgpuRuntime>, skybox: &Skybox) -> Self {
        let runtime = runtime.as_ref();
        let slab = SlabAllocator::new(runtime, "ibl", wgpu::BufferUsages::VERTEX);
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let camera = Camera::new(&slab).with_projection(proj);
        let roughness = slab.new_value(0.0f32);
        let prefilter_ids = slab.new_value(VertexPrefilterEnvironmentCubemapIds {
            camera: camera.id(),
            roughness: roughness.id(),
        });

        let buffer = slab.commit();
        let mut buffer_upkeep = || {
            let possibly_new_buffer = slab.commit();
            debug_assert!(!possibly_new_buffer.is_new_this_commit());
        };

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

        let environment_cubemap = skybox.environment_cubemap_texture();

        // Convolve the environment map.
        let irradiance_cubemap = create_irradiance_map(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            environment_cubemap,
            &camera,
            views,
        );

        // Generate specular IBL pre-filtered environment map.
        let prefiltered_environment_cubemap = create_prefiltered_environment_map(
            runtime,
            &buffer,
            &mut buffer_upkeep,
            &camera,
            &roughness,
            prefilter_ids.id(),
            environment_cubemap,
            views,
        );

        Self {
            is_empty: Arc::new(skybox.is_empty().into()),
            irradiance_cubemap,
            prefiltered_environment_cubemap,
        }
    }

    /// Returns whether this [`Ibl`] is empty.
    ///
    /// An [`Ibl`] is empty if it was created from an empty [`Skybox`].
    pub fn is_empty(&self) -> bool {
        self.is_empty.load(std::sync::atomic::Ordering::Relaxed)
    }
}

fn create_irradiance_map(
    runtime: impl AsRef<WgpuRuntime>,
    buffer: &wgpu::Buffer,
    buffer_upkeep: impl FnMut(),
    environment_texture: &texture::Texture,
    camera: &Camera,
    views: [Mat4; 6],
) -> texture::Texture {
    let runtime = runtime.as_ref();
    let device = &runtime.device;
    let pipeline = crate::pbr::ibl::DiffuseIrradianceConvolutionRenderPipeline::new(
        device,
        wgpu::TextureFormat::Rgba16Float,
    );

    let bindgroup = crate::pbr::ibl::diffuse_irradiance_convolution_bindgroup(
        device,
        Some("irradiance"),
        buffer,
        environment_texture,
    );

    texture::Texture::render_cubemap(
        runtime,
        &pipeline.0,
        buffer_upkeep,
        camera,
        &bindgroup,
        views,
        32,
        None,
    )
}

/// Pipeline for creating a prefiltered environment map from an existing
/// environment cubemap.
pub(crate) fn create_prefiltered_environment_pipeline_and_bindgroup(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    environment_texture: &crate::texture::Texture,
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let label = Some("prefiltered environment");
    let bindgroup_layout_desc = wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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
    };
    let bg_layout = device.create_bind_group_layout(&bindgroup_layout_desc);
    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bg_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&environment_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&environment_texture.sampler),
            },
        ],
    });
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    let vertex_linkage = crate::linkage::prefilter_environment_cubemap_vertex::linkage(device);
    let fragment_linkage = crate::linkage::prefilter_environment_cubemap_fragment::linkage(device);
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("prefiltered environment"),
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: 1,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_linkage.module,
            entry_point: Some(fragment_linkage.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
        cache: None,
    });
    (pipeline, bindgroup)
}

#[allow(clippy::too_many_arguments)]
fn create_prefiltered_environment_map(
    runtime: impl AsRef<WgpuRuntime>,
    buffer: &wgpu::Buffer,
    mut buffer_upkeep: impl FnMut(),
    camera: &Camera,
    roughness: &Hybrid<f32>,
    prefilter_id: Id<VertexPrefilterEnvironmentCubemapIds>,
    environment_texture: &texture::Texture,
    views: [Mat4; 6],
) -> texture::Texture {
    let (pipeline, bindgroup) =
        crate::pbr::ibl::create_prefiltered_environment_pipeline_and_bindgroup(
            &runtime.as_ref().device,
            buffer,
            environment_texture,
        );
    let mut cubemap_faces = Vec::new();

    for (i, view) in views.iter().enumerate() {
        for mip_level in 0..5 {
            let mip_width: u32 = 128 >> mip_level;
            let mip_height: u32 = 128 >> mip_level;

            let mut encoder =
                runtime
                    .as_ref()
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("specular convolution"),
                    });

            let cubemap_face = texture::Texture::new_with(
                runtime.as_ref(),
                Some(&format!("cubemap{i}{mip_level}prefiltered_environment")),
                Some(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC),
                None,
                wgpu::TextureFormat::Rgba16Float,
                4,
                2,
                mip_width,
                mip_height,
                1,
                &[],
            );

            // update the roughness for these mips
            roughness.set(mip_level as f32 / 4.0);
            // update the view to point at one of the cube faces
            camera.set_view(*view);
            buffer_upkeep();
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("cubemap{i}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &cubemap_face.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(&pipeline);
                render_pass.set_bind_group(0, Some(&bindgroup), &[]);
                render_pass.draw(0..36, prefilter_id.inner()..prefilter_id.inner() + 1);
            }

            runtime.as_ref().queue.submit([encoder.finish()]);
            cubemap_faces.push(cubemap_face);
        }
    }

    texture::Texture::new_cubemap_texture(
        runtime,
        Some("prefiltered environment cubemap"),
        128,
        cubemap_faces.as_slice(),
        wgpu::TextureFormat::Rgba16Float,
        5,
    )
}

pub fn diffuse_irradiance_convolution_bindgroup_layout(
    device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("convolution bindgroup"),
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

pub fn diffuse_irradiance_convolution_bindgroup(
    device: &wgpu::Device,
    label: Option<&str>,
    buffer: &wgpu::Buffer,
    // The texture to sample the environment from
    texture: &crate::texture::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &diffuse_irradiance_convolution_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
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

pub struct DiffuseIrradianceConvolutionRenderPipeline(pub wgpu::RenderPipeline);

impl DiffuseIrradianceConvolutionRenderPipeline {
    /// Create the rendering pipeline that performs a convolution.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let vertex_linkage = crate::linkage::skybox_cubemap_vertex::linkage(device);
        let fragment_linkage = crate::linkage::di_convolution_fragment::linkage(device);
        let bg_layout = diffuse_irradiance_convolution_bindgroup_layout(device);
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("convolution pipeline layout"),
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });

        DiffuseIrradianceConvolutionRenderPipeline(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("convolution pipeline"),
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
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
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
            },
        ))
    }
}
