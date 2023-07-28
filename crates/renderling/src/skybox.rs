//! An HDR skybox.
use glam::{Mat4, Vec3};
use renderling_shader::scene::GpuConstants;

use crate::{SceneImage, Uniform};

/// Render pipeline used to draw a skybox.
pub struct SkyboxRenderPipeline(pub wgpu::RenderPipeline);

pub fn skybox_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("skybox bindgroup"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
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

pub fn create_skybox_bindgroup(
    device: &wgpu::Device,
    constants: &Uniform<GpuConstants>,
    texture: &crate::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("skybox"),
        layout: &skybox_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: constants.buffer().as_entire_binding(),
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
pub fn create_skybox_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> SkyboxRenderPipeline {
    log::trace!("creating skybox render pipeline with format '{format:?}'");
    let vertex_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/vertex_skybox.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/fragment_cubemap.spv"));
    let bg_layout = skybox_bindgroup_layout(device);
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("skybox pipeline layout"),
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    SkyboxRenderPipeline(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("skybox pipeline"),
            layout: Some(&pp_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vertex_skybox",
                buffers: &[],
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
                module: &fragment_shader,
                entry_point: "fragment_cubemap",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        }),
    )
}

/// An HDR skybox.
pub struct Skybox {
    // Texture of the equirectangular environment
    pub equirectangular_texture: crate::Texture,
    // Texture of the environment cubemap
    pub environment_texture: crate::Texture,
    pub environment_bindgroup: wgpu::BindGroup,
    // Texture of the pre-computed irradiance cubemap
    pub irradiance_texture: crate::Texture,
    pub irradiance_bindgroup: wgpu::BindGroup,
    // pub prefiltered_environment_map: crate::Texture,
    // pub brdf_lut: crate::Texture,
}

impl Skybox {
    /// Create an empty, transparent skybox.
    pub fn empty(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        constants: &Uniform<GpuConstants>,
    ) -> Self {
        log::trace!("creating empty skybox");
        let hdr_img = SceneImage {
            pixels: vec![0u8; 4 * 4],
            width: 1,
            height: 1,
            format: crate::SceneImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        };
        Self::new(device, queue, constants, hdr_img)
    }

    /// Create a new `Skybox`.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        constants: &Uniform<GpuConstants>,
        hdr_img: SceneImage,
    ) -> Self {
        log::trace!("creating skybox");
        let equirectangular_texture = Skybox::hdr_texture_from_scene_image(device, queue, hdr_img);
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
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
        // Create unit cube for projections.
        let cube_vertices: [[f32; 3]; 8] = [
            // front
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
            // back
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ];
        let cube_elements: [u16; 36] = [
            // front
            0, 1, 2, 2, 3, 0, // right
            1, 5, 6, 6, 2, 1, // back
            7, 6, 5, 5, 4, 7, // left
            4, 0, 3, 3, 7, 4, // bottom
            4, 5, 1, 1, 0, 4, // top
            3, 2, 6, 6, 7, 3,
        ];

        let unit_cube_mesh = crate::mesh::Mesh::new(
            device,
            Some("unit cube"),
            cube_vertices,
            Some(cube_elements),
        );

        // Create environment map.
        let environment_texture = Skybox::create_environment_map_from_hdr(
            device,
            queue,
            &equirectangular_texture,
            &unit_cube_mesh,
            proj,
            views,
        );

        // Convolve the environment map.
        let irradiance_texture = Skybox::create_irradiance_map(
            device,
            queue,
            &environment_texture,
            &unit_cube_mesh,
            proj,
            views,
        );

        //// Generate specular IBL pre-filtered environment map.
        // let prefiltered_environment_map = Skybox::create_prefiltered_environment_map(
        //    device,
        //    queue,
        //    &environment_texture,
        //    &proj,
        //    &views,
        //);

        // Skybox {
        //    environment_texture,
        //    irradiance_map,
        //    prefiltered_environment_map,
        //    brdf_lut: Skybox::create_precomputed_brdf_texture(device, queue),
        //}
        log::trace!("returning skybox");
        Skybox {
            environment_bindgroup: crate::skybox::create_skybox_bindgroup(
                &device,
                &constants,
                &environment_texture,
            ),
            irradiance_bindgroup: crate::skybox::create_skybox_bindgroup(
                &device,
                &constants,
                &irradiance_texture,
            ),
            equirectangular_texture,
            environment_texture,
            irradiance_texture,
        }
    }

    /// Convert an HDR [`SceneImage`] into a texture.
    pub fn hdr_texture_from_scene_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: SceneImage,
    ) -> crate::Texture {
        crate::Texture::new_with(
            device,
            queue,
            Some("create hdr texture"),
            None,
            None,
            wgpu::TextureFormat::Rgba32Float,
            4,
            4,
            img.width,
            img.height,
            &img.pixels,
        )
    }

    /// Create an HDR equirectangular texture from bytes.
    pub fn create_hdr_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_data: &[u8],
    ) -> crate::Texture {
        let img = SceneImage::from_hdr_bytes(hdr_data).unwrap();
        Self::hdr_texture_from_scene_image(device, queue, img)
    }

    fn create_environment_map_from_hdr(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_texture: &crate::Texture,
        unit_cube_mesh: &crate::mesh::Mesh,
        proj: Mat4,
        views: [Mat4; 6],
    ) -> crate::Texture {
        // Create the cubemap-making pipeline.
        let pipeline = crate::cubemap::CubemapMakingRenderPipeline::new(
            device,
            wgpu::TextureFormat::Rgba16Float,
        );
        let mut constants = crate::uniform::Uniform::new(
            device,
            GpuConstants {
                camera_projection: proj,
                ..Default::default()
            },
            wgpu::BufferUsages::VERTEX,
            wgpu::ShaderStages::VERTEX,
        );
        let bindgroup = crate::cubemap::cubemap_making_bindgroup(
            device,
            Some("cubemap"),
            &constants,
            hdr_texture,
        );

        Self::render_cubemap(
            device,
            queue,
            "from_equirectangular",
            &pipeline.0,
            &mut constants,
            &bindgroup,
            unit_cube_mesh,
            views,
            512,
        )
    }

    fn render_cubemap(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label_prefix: &str,
        pipeline: &wgpu::RenderPipeline,
        constants: &mut Uniform<GpuConstants>,
        bindgroup: &wgpu::BindGroup,
        unit_cube_mesh: &crate::mesh::Mesh,
        views: [Mat4; 6],
        texture_size: u32,
    ) -> crate::Texture {
        let mut cubemap_faces = Vec::new();

        // Render every cube face.
        for i in 0..6 {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("create cubemap {label_prefix}")),
            });

            log::trace!("rendering cubemap face {i}");
            let cubemap_face = crate::Texture::new_with(
                device,
                queue,
                Some(&format!("cubemap{i}{label_prefix}")),
                Some(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC),
                None,
                wgpu::TextureFormat::Rgba16Float,
                4,
                2,
                texture_size,
                texture_size,
                &[],
            );

            // update the view to point at one of the cube faces
            constants.camera_view = views[i];
            constants.update(queue);

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("cubemap{i}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &cubemap_face.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, &bindgroup, &[]);
                unit_cube_mesh.draw(&mut render_pass);
            }

            queue.submit([encoder.finish()]);
            cubemap_faces.push(cubemap_face);
        }

        crate::Texture::new_cubemap_texture(
            device,
            queue,
            Some(&format!("{label_prefix} cubemap")),
            texture_size,
            texture_size,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            1,
        )
    }

    fn create_irradiance_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        environment_texture: &crate::Texture,
        unit_cube_mesh: &crate::mesh::Mesh,
        proj: Mat4,
        views: [Mat4; 6],
    ) -> crate::Texture {
        let pipeline = crate::convolution::ConvolutionRenderPipeline::new(
            device,
            wgpu::TextureFormat::Rgba16Float,
        );
        let mut constants = crate::uniform::Uniform::new(
            device,
            GpuConstants {
                camera_projection: proj,
                ..Default::default()
            },
            wgpu::BufferUsages::VERTEX,
            wgpu::ShaderStages::VERTEX,
        );
        let bindgroup = crate::convolution::convolution_bindgroup(
            device,
            Some("irradiance"),
            &constants,
            environment_texture,
        );

        Self::render_cubemap(
            device,
            queue,
            "irradiance",
            &pipeline.0,
            &mut constants,
            &bindgroup,
            unit_cube_mesh,
            views,
            32,
        )
    }

    // fn create_prefiltered_environment_map(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //    environment_texture: &crate::Texture,
    //    proj: &Mat4,
    //    views: &[Mat4; 6],
    //) -> crate::Texture {
    //    let hdr_specular_convolution_material = HdrConvolveSpecularMaterial::new(
    //        device,
    //        &HdrConvolveSpecularBindGroup {
    //            environment_texture: environment_texture,
    //            roughness: 0.0,
    //        },
    //    );

    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("HDR convolve specular"),
    //    });

    //    let mut cubemap_faces = Vec::new();

    //    for mip_level in 0..5 {
    //        let mip_width: u32 = 128 >> mip_level;
    //        let mip_height: u32 = 128 >> mip_level;

    //        let roughness = mip_level as f32 / 4.0;

    //        material_base::update_uniform_buffer(
    //            device,
    //            &hdr_specular_convolution_material.roughness_bind_group_buffer,
    //            &mut encoder,
    //            &[roughness],
    //        );

    //        for i in 0..6 {
    //            let cubemap_face = Texture::new_framebuffer_texture(
    //                device,
    //                mip_width,
    //                mip_height,
    //                wgpu::TextureFormat::Rgba16Float,
    //            );

    //            let transforms = HdrTransformBindGroup {
    //                proj_matrix: proj.to_homogeneous(),
    //                view_matrix: views[i].to_homogeneous(),
    //            };

    //            material_base::update_uniform_buffer(
    //                device,
    //
    // &hdr_specular_convolution_material.transform_bind_group_buffer,
    //                &mut encoder,
    //                &transforms,
    //            );

    //            {
    //                let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                        attachment: &cubemap_face.view,
    //                        resolve_target: None,
    //                        load_op: wgpu::LoadOp::Clear,
    //                        store_op: wgpu::StoreOp::Store,
    //                        clear_color: wgpu::Color::BLACK,
    //                    }],
    //                    depth_stencil_attachment: None,
    //                });

    // render_pass.set_pipeline(&hdr_specular_convolution_material.render_pipeline);
    //                render_pass.set_bind_group(
    //                    0,
    //                    &hdr_specular_convolution_material.transform_bind_group,
    //                    &[],
    //                );
    //                render_pass.set_bind_group(
    //                    1,
    //                    &hdr_specular_convolution_material.convolve_bind_group,
    //                    &[],
    //                );
    //                render_pass.set_bind_group(
    //                    2,
    //                    &hdr_specular_convolution_material.roughness_bind_group,
    //                    &[],
    //                );

    //                unit_cube_mesh.draw(&mut render_pass);
    //            }

    //            cubemap_faces.push(cubemap_face);
    //        }
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    Texture::new_cubemap_texture(
    //        device,
    //        queue,
    //        128,
    //        128,
    //        cubemap_faces.as_slice(),
    //        wgpu::TextureFormat::Rgba16Float,
    //        5,
    //    )
    //}

    // fn create_precomputed_brdf_texture(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //) -> crate::Texture {
    //    let screen_space_quad_vertices: [[f32; 3]; 4] = [
    //        [-1.0, -1.0, 0.0],
    //        [-1.0, 1.0, 0.0],
    //        [1.0, 1.0, 0.0],
    //        [1.0, -1.0, 0.0],
    //    ];

    //    let screen_space_quad_tex_coords: [[f32; 2]; 4] =
    //        [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

    //    let screen_space_quad_elements: [u32; 6] = [0, 1, 2, 2, 3, 0];

    //    let screen_space_quad_vertex_array = screen_space_quad_vertices
    //        .iter()
    //        .zip(screen_space_quad_tex_coords.iter())
    //        .map(|(&pos, &tex_coord)| Vertex {
    //            position: pos,
    //            normal: [0.0, 0.0, 0.0],
    //            tangent: [0.0, 0.0, 0.0, 0.0],
    //            tex_coord: tex_coord,
    //        })
    //        .collect::<Vec<Vertex>>();

    //    let screen_space_quad_mesh = Mesh::new(
    //        device,
    //        &screen_space_quad_vertex_array,
    //        Some(&screen_space_quad_elements),
    //    );

    //    let brdf_mat = HdrConvolveBrdfMaterial::new(device);

    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("brdf_precompute"),
    //    });

    //    let transforms = HdrTransformBindGroup {
    //        proj_matrix: Matrix3::identity().to_homogeneous(),
    //        view_matrix: Similarity3::identity().to_homogeneous(),
    //    };

    //    material_base::update_uniform_buffer(
    //        device,
    //        &brdf_mat.transform_bind_group_buffer,
    //        &mut encoder,
    //        &transforms,
    //    );

    //    let framebuffer =
    //        Texture::new_framebuffer_texture(device, 512, 512,
    // wgpu::TextureFormat::Rg16Float);

    //    {
    //        let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                attachment: &framebuffer.view,
    //                resolve_target: None,
    //                load_op: wgpu::LoadOp::Clear,
    //                store_op: wgpu::StoreOp::Store,
    //                clear_color: wgpu::Color::BLACK,
    //            }],
    //            depth_stencil_attachment: None,
    //        });

    //        render_pass.set_pipeline(&brdf_mat.render_pipeline);
    //        render_pass.set_bind_group(0, &brdf_mat.transform_bind_group, &[]);

    //        screen_space_quad_mesh.draw(&mut render_pass);
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    Texture::new_texture_from_framebuffer(
    //        device,
    //        queue,
    //        512,
    //        512,
    //        &framebuffer,
    //        wgpu::TextureFormat::Rg16Float,
    //        wgpu::AddressMode::ClampToEdge,
    //    )
    //}
}

#[cfg(test)]
mod test {
    use glam::{Vec2, Vec3, Vec4};
    use renderling_shader::ui::{UiMode, UiVertex};

    use super::*;
    use crate::{BufferDimensions, CopiedTextureBuffer, Renderling};

    #[test]
    fn hdr_texture() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let hdr_data = std::fs::read("../../img/hdr/helipad.hdr").unwrap();
        let tex = Skybox::create_hdr_texture(r.get_device(), r.get_queue(), &hdr_data);
        let builder = r.new_ui_scene().with_canvas_size(600, 400);
        let obj = builder
            .new_object()
            .with_draw_mode(UiMode::DEFAULT)
            .with_texture(&tex)
            .with_vertices({
                let tl = UiVertex {
                    position: Vec2::new(0.0, 0.0),
                    uv: Vec2::new(0.0, 0.0),
                    color: Vec4::ONE,
                };
                let tr = UiVertex {
                    position: Vec2::new(1.0, 0.0),
                    uv: Vec2::new(1.0, 0.0),
                    color: Vec4::ONE,
                };
                let bl = UiVertex {
                    position: Vec2::new(0.0, 1.0),
                    uv: Vec2::new(0.0, 1.0),
                    color: Vec4::ONE,
                };
                let br = UiVertex {
                    position: Vec2::new(1.0, 1.0),
                    uv: Vec2::new(1.0, 1.0),
                    color: Vec4::ONE,
                };
                [tl, bl, tr, tr, bl, br]
            })
            .with_scale([400.0, 400.0])
            .build();
        let scene = builder.build();
        r.setup_render_graph(None, Some(scene), [obj], true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("hdr_texture.png", img);
    }

    #[test]
    fn skybox_irradiance_map() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let (device, queue) = r.get_device_and_queue_owned();
        let constants = Uniform::new(
            &device,
            GpuConstants {
                camera_projection: Mat4::perspective_rh(
                    std::f32::consts::FRAC_PI_4,
                    6.0 / 4.0,
                    0.01,
                    10.0,
                ),
                ..Default::default()
            },
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        );
        let hdr_img = SceneImage::from_hdr_path("../../img/hdr/helipad.hdr").unwrap();
        let skybox = Skybox::new(&device, &queue, &constants, hdr_img);
        let dimensions = BufferDimensions::new(4, 2, 32, 32);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("read skybox irradiance"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let source = skybox.irradiance_texture.texture.as_image_copy();
        encoder.copy_texture_to_buffer(
            source,
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(dimensions.padded_bytes_per_row as u32),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: dimensions.width as u32,
                height: dimensions.height as u32,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(std::iter::once(encoder.finish()));
        let copied = CopiedTextureBuffer {
            dimensions,
            buffer,
            format: skybox.irradiance_texture.texture.format(),
        };
        let scene_img = copied.into_scene_image(&device).unwrap();
        let img = scene_img.into_rgba8().unwrap();
        img_diff::save("skybox_irradiance0.png", img);
    }

    #[test]
    fn hdr_skybox_scene() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let proj = crate::camera::perspective(600.0, 400.0);
        let view = crate::camera::look_at(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, Vec3::Y);
        let mut builder = r.new_scene().with_camera(proj, view);
        builder.add_skybox_image_from_path("../../img/hdr/helipad.hdr");
        let scene = builder.build().unwrap();
        r.setup_render_graph(Some(scene), None, [], true);
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("hdr_skybox_scene.png", img);
    }
}
