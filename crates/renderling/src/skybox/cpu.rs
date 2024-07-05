//! CPU-side code for skybox rendering.
use crabslab::Id;
use glam::{Mat4, UVec2, Vec3};

use crate::{
    atlas::AtlasImage,
    camera::Camera,
    convolution::VertexPrefilterEnvironmentCubemapIds,
    slab::{Hybrid, SlabAllocator},
    texture::Texture,
};

/// Render pipeline used to draw a skybox.
pub struct SkyboxRenderPipeline(pub wgpu::RenderPipeline);

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
    SkyboxRenderPipeline(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("skybox render pipeline"),
            layout: Some(&pp_layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: vertex_linkage.entry_point,
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
                count: 1,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: fragment_linkage.entry_point,
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
    )
}

/// An HDR skybox that also provides IBL cubemaps and lookups.
///
/// A clone of a skybox is a reference to the same skybox.
///
/// Only available on the CPU. Not available in shaders.
#[derive(Debug, Clone)]
pub struct Skybox {
    // Cubemap texture of the environment cubemap
    pub environment_cubemap: Texture,
    // Cubemap texture of the pre-computed irradiance cubemap
    pub irradiance_cubemap: Texture,
    // Cubemap texture and mip maps of the specular highlights,
    // where each mip level is a different roughness.
    pub prefiltered_environment_cubemap: Texture,
    // Texture of the pre-computed brdf integration
    pub brdf_lut: Texture,
    // `Id` of the camera to use for rendering the skybox.
    //
    // The camera is used to determine the orientation of the skybox.
    pub camera: crabslab::Id<Camera>,
}

impl Skybox {
    /// Create an empty, transparent skybox.
    pub fn empty(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        log::trace!("creating empty skybox");
        let hdr_img = AtlasImage {
            pixels: vec![0u8; 4 * 4],
            size: UVec2::splat(1),
            format: crate::atlas::AtlasImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        };
        Self::new(device, queue, hdr_img, crabslab::Id::<Camera>::NONE)
    }

    /// Create a new `Skybox`.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_img: AtlasImage,
        camera_id: crate::slab::Id<Camera>,
    ) -> Self {
        log::trace!("creating skybox");

        let slab = SlabAllocator::<wgpu::Buffer>::default();

        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let camera = slab.new_value(Camera::default().with_projection(proj));
        let roughness = slab.new_value(0.0f32);
        let prefilter_ids = slab.new_value(VertexPrefilterEnvironmentCubemapIds {
            camera: camera.id(),
            roughness: roughness.id(),
        });

        let buffer =
            slab.get_updated_buffer((device, queue, Some("skybox"), wgpu::BufferUsages::VERTEX));
        let mut buffer_upkeep = || {
            let maybe_resized_buffer = slab.upkeep((
                device,
                queue,
                Some("skybox-upkeep"),
                wgpu::BufferUsages::VERTEX,
            ));
            debug_assert!(maybe_resized_buffer.is_none());
        };

        let equirectangular_texture = Skybox::hdr_texture_from_atlas_image(device, queue, hdr_img);
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
            device,
            queue,
            &buffer,
            &mut buffer_upkeep,
            &equirectangular_texture,
            &camera,
            views,
        );

        // Convolve the environment map.
        let irradiance_cubemap = Skybox::create_irradiance_map(
            device,
            queue,
            &buffer,
            &mut buffer_upkeep,
            &environment_cubemap,
            &camera,
            views,
        );

        // Generate specular IBL pre-filtered environment map.
        let prefiltered_environment_cubemap = Skybox::create_prefiltered_environment_map(
            device,
            queue,
            &buffer,
            &mut buffer_upkeep,
            &camera,
            &roughness,
            prefilter_ids.id(),
            &environment_cubemap,
            views,
        );

        let brdf_lut = Skybox::create_precomputed_brdf_texture(device, queue);

        Skybox {
            environment_cubemap,
            irradiance_cubemap,
            prefiltered_environment_cubemap,
            brdf_lut,
            camera: camera_id,
        }
    }

    /// Convert an HDR [`AtlasImage`] into a texture.
    pub fn hdr_texture_from_atlas_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: AtlasImage,
    ) -> Texture {
        Texture::new_with(
            device,
            queue,
            Some("create hdr texture"),
            None,
            Some(device.create_sampler(&wgpu::SamplerDescriptor {
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
    pub fn create_hdr_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_data: &[u8],
    ) -> Texture {
        let img = AtlasImage::from_hdr_bytes(hdr_data).unwrap();
        Self::hdr_texture_from_atlas_image(device, queue, img)
    }

    fn create_environment_map_from_hdr(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        buffer_upkeep: impl FnMut(),
        hdr_texture: &Texture,
        camera: &Hybrid<Camera>,
        views: [Mat4; 6],
    ) -> Texture {
        // Create the cubemap-making pipeline.
        let pipeline = crate::cubemap::CubemapMakingRenderPipeline::new(
            device,
            wgpu::TextureFormat::Rgba16Float,
        );

        let resources = (
            device,
            queue,
            Some("hdr environment map"),
            wgpu::BufferUsages::VERTEX,
        );
        let bindgroup =
            crate::cubemap::cubemap_making_bindgroup(device, resources.2, buffer, hdr_texture);

        Self::render_cubemap(
            device,
            queue,
            &pipeline.0,
            buffer_upkeep,
            camera,
            &bindgroup,
            views,
            512,
            Some(9),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_cubemap(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pipeline: &wgpu::RenderPipeline,
        mut buffer_upkeep: impl FnMut(),
        camera: &Hybrid<Camera>,
        bindgroup: &wgpu::BindGroup,
        views: [Mat4; 6],
        texture_size: u32,
        mip_levels: Option<u32>,
    ) -> Texture {
        let mut cubemap_faces = Vec::new();
        let mip_levels = mip_levels.unwrap_or(1);

        // Render every cube face.
        for (i, view) in views.iter().enumerate() {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("cubemap{i}")),
            });

            let mut cubemap_face = Texture::new_with(
                device,
                queue,
                Some(&format!("cubemap{i}")),
                Some(
                    wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::COPY_SRC
                        | wgpu::TextureUsages::COPY_DST
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                ),
                None,
                wgpu::TextureFormat::Rgba16Float,
                4,
                2,
                texture_size,
                texture_size,
                1,
                &[],
            );

            // update the view to point at one of the cube faces
            camera.modify(|c| c.set_view(*view));
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
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, bindgroup, &[]);
                render_pass.draw(0..36, 0..1);
            }

            queue.submit([encoder.finish()]);
            let mips = cubemap_face.generate_mips(device, queue, Some("cubemap mips"), mip_levels);
            cubemap_faces.push(cubemap_face);
            cubemap_faces.extend(mips);
        }

        Texture::new_cubemap_texture(
            device,
            queue,
            Some("skybox cubemap"),
            texture_size,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            mip_levels,
        )
    }

    fn create_irradiance_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        buffer_upkeep: impl FnMut(),
        environment_texture: &Texture,
        camera: &Hybrid<Camera>,
        views: [Mat4; 6],
    ) -> Texture {
        let pipeline =
            crate::ibl::diffuse_irradiance::DiffuseIrradianceConvolutionRenderPipeline::new(
                device,
                wgpu::TextureFormat::Rgba16Float,
            );

        let bindgroup = crate::ibl::diffuse_irradiance::diffuse_irradiance_convolution_bindgroup(
            device,
            Some("irradiance"),
            buffer,
            environment_texture,
        );

        Self::render_cubemap(
            device,
            queue,
            &pipeline.0,
            buffer_upkeep,
            camera,
            &bindgroup,
            views,
            32,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn create_prefiltered_environment_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        mut buffer_upkeep: impl FnMut(),
        camera: &Hybrid<Camera>,
        roughness: &Hybrid<f32>,
        prefilter_id: Id<VertexPrefilterEnvironmentCubemapIds>,
        environment_texture: &Texture,
        views: [Mat4; 6],
    ) -> Texture {
        let (pipeline, bindgroup) =
            crate::ibl::prefiltered_environment::create_pipeline_and_bindgroup(
                device,
                buffer,
                environment_texture,
            );
        let mut cubemap_faces = Vec::new();

        for (i, view) in views.iter().enumerate() {
            for mip_level in 0..5 {
                let mip_width: u32 = 128 >> mip_level;
                let mip_height: u32 = 128 >> mip_level;

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("specular convolution"),
                });

                let cubemap_face = Texture::new_with(
                    device,
                    queue,
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
                camera.modify(|c| c.set_view(*view));
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
                        })],
                        depth_stencil_attachment: None,
                        ..Default::default()
                    });

                    render_pass.set_pipeline(&pipeline);
                    render_pass.set_bind_group(0, &bindgroup, &[]);
                    render_pass.draw(0..36, prefilter_id.inner()..prefilter_id.inner() + 1);
                }

                queue.submit([encoder.finish()]);
                cubemap_faces.push(cubemap_face);
            }
        }

        Texture::new_cubemap_texture(
            device,
            queue,
            Some("prefiltered environment cubemap"),
            128,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            5,
        )
    }

    fn create_precomputed_brdf_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let vertex_linkage = crate::linkage::brdf_lut_convolution_vertex::linkage(device);
        let fragment_linkage = crate::linkage::brdf_lut_convolution_fragment::linkage(device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("brdf_lut_convolution"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: vertex_linkage.entry_point,
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
                entry_point: fragment_linkage.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg16Float,
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

        let framebuffer = Texture::new_with(
            device,
            queue,
            Some("brdf_lut"),
            Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
            ),
            None,
            wgpu::TextureFormat::Rg16Float,
            2,
            2,
            512,
            512,
            1,
            &[],
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("brdf_lut_convolution"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &framebuffer.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            render_pass.set_pipeline(&pipeline);
            render_pass.draw(0..6, 0..1);
        }
        queue.submit([encoder.finish()]);
        framebuffer
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;

    use super::*;
    use crate::Context;

    #[test]
    fn hdr_skybox_scene() {
        let ctx = Context::headless(600, 400);

        let proj = crate::camera::perspective(600.0, 400.0);
        let view = crate::camera::look_at(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, Vec3::Y);

        let stage = ctx.new_stage();

        let camera = stage.new_value(Camera::new(proj, view));
        let skybox = stage
            .new_skybox_from_path("../../img/hdr/resting_place.hdr", camera.id())
            .unwrap();

        assert_eq!(
            wgpu::TextureFormat::Rgba16Float,
            skybox.irradiance_cubemap.texture.format()
        );
        assert_eq!(
            wgpu::TextureFormat::Rgba16Float,
            skybox.prefiltered_environment_cubemap.texture.format()
        );

        for i in 0..6 {
            // save out the irradiance face
            let copied_buffer = Texture::read_from(
                &skybox.irradiance_cubemap.texture,
                ctx.get_device(),
                ctx.get_queue(),
                32,
                32,
                4,
                2,
                0,
                Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
            );
            let pixels = copied_buffer.pixels(ctx.get_device());
            let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                .iter()
                .map(|p| half::f16::from_bits(*p).to_f32())
                .collect::<Vec<_>>();
            assert_eq!(32 * 32 * 4, pixels.len());
            let img: image::Rgba32FImage = image::ImageBuffer::from_vec(32, 32, pixels).unwrap();
            let img = image::DynamicImage::from(img);
            let img = img.to_rgba8();
            img_diff::assert_img_eq(&format!("skybox/irradiance{i}.png"), img);
            for mip_level in 0..5 {
                let mip_size = 128u32 >> mip_level;
                // save out the prefiltered environment faces' mips
                let copied_buffer = Texture::read_from(
                    &skybox.prefiltered_environment_cubemap.texture,
                    ctx.get_device(),
                    ctx.get_queue(),
                    mip_size as usize,
                    mip_size as usize,
                    4,
                    2,
                    mip_level,
                    Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
                );
                let pixels = copied_buffer.pixels(ctx.get_device());
                let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                    .iter()
                    .map(|p| half::f16::from_bits(*p).to_f32())
                    .collect::<Vec<_>>();
                assert_eq!((mip_size * mip_size * 4) as usize, pixels.len());
                let img: image::Rgba32FImage =
                    image::ImageBuffer::from_vec(mip_size, mip_size, pixels).unwrap();
                let img = image::DynamicImage::from(img);
                let img = img.to_rgba8();
                img_diff::assert_img_eq(
                    &format!("skybox/prefiltered_environment_face{i}_mip{mip_level}.png"),
                    img,
                );
            }
        }

        stage.set_skybox(skybox);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_linear_image().unwrap();
        img_diff::assert_img_eq("skybox/hdr.png", img);
    }

    #[test]
    fn precomputed_brdf() {
        assert_eq!(2, std::mem::size_of::<u16>());
        let r = Context::headless(32, 32);
        let (device, queue) = r.get_device_and_queue_owned();
        let brdf_lut = Skybox::create_precomputed_brdf_texture(&device, &queue);
        assert_eq!(wgpu::TextureFormat::Rg16Float, brdf_lut.texture.format());
        let copied_buffer = Texture::read(&brdf_lut.texture, &device, &queue, 512, 512, 2, 2);
        let pixels = copied_buffer.pixels(&device);
        let pixels: Vec<f32> = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
            .iter()
            .copied()
            .map(|bits| half::f16::from_bits(bits).to_f32())
            .collect();
        assert_eq!(512 * 512 * 2, pixels.len());
        let pixels: Vec<f32> = pixels
            .chunks_exact(2)
            .flat_map(|pixel| match pixel {
                [r, g] => [*r, *g, 0.0, 1.0],
                _ => unreachable!(),
            })
            .collect();

        let img: image::ImageBuffer<image::Rgba<f32>, Vec<f32>> =
            image::ImageBuffer::from_vec(512, 512, pixels).unwrap();
        let img = image::DynamicImage::from(img);
        let img = img.into_rgba8();
        img_diff::assert_img_eq("skybox/brdf_lut.png", img);
    }
}
