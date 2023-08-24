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

/// An HDR skybox that also provides IBL cubemaps and lookups.
#[derive(Debug)]
pub struct Skybox {
    // Cubemap texture of the environment cubemap
    pub environment_cubemap: crate::Texture,
    // Bindgroup to use with the default skybox shader
    pub environment_bindgroup: wgpu::BindGroup,
    // Cubemap texture of the pre-computed irradiance cubemap
    pub irradiance_cubemap: crate::Texture,
    // Cubemap texture and mip maps of the specular highlights,
    // where each mip level is a different roughness.
    pub prefiltered_environment_cubemap: crate::Texture,
    // Texture of the pre-computed brdf integration
    pub brdf_lut: crate::Texture,
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
        let environment_cubemap = Skybox::create_environment_map_from_hdr(
            device,
            queue,
            &equirectangular_texture,
            &unit_cube_mesh,
            proj,
            views,
        );

        // Convolve the environment map.
        let irradiance_cubemap = Skybox::create_irradiance_map(
            device,
            queue,
            &environment_cubemap,
            &unit_cube_mesh,
            proj,
            views,
        );

        // Generate specular IBL pre-filtered environment map.
        let prefiltered_environment_cubemap = Skybox::create_prefiltered_environment_map(
            device,
            queue,
            &environment_cubemap,
            &unit_cube_mesh,
            proj,
            views,
        );

        let brdf_lut = Skybox::create_precomputed_brdf_texture(device, queue);

        Skybox {
            environment_bindgroup: crate::skybox::create_skybox_bindgroup(
                &device,
                &constants,
                &environment_cubemap,
            ),
            environment_cubemap,
            irradiance_cubemap,
            prefiltered_environment_cubemap,
            brdf_lut,
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
            1,
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
            Some("environment cubemap"),
            &constants,
            hdr_texture,
        );

        Self::render_cubemap(
            device,
            queue,
            "environment",
            &pipeline.0,
            &mut constants,
            &bindgroup,
            unit_cube_mesh,
            views,
            512,
            Some(9),
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
        mip_levels: Option<u32>,
    ) -> crate::Texture {
        let mut cubemap_faces = Vec::new();
        let mip_levels = mip_levels.unwrap_or(1);

        // Render every cube face.
        for i in 0..6 {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("create cubemap {label_prefix}")),
            });

            let mut cubemap_face = crate::Texture::new_with(
                device,
                queue,
                Some(&format!("cubemap{i}{label_prefix}")),
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
            let mips = cubemap_face.generate_mips(
                device,
                queue,
                Some(&format!("{label_prefix}mip")),
                mip_levels,
            );
            cubemap_faces.push(cubemap_face);
            cubemap_faces.extend(mips);
        }

        crate::Texture::new_cubemap_texture(
            device,
            queue,
            Some(&format!("{label_prefix} cubemap")),
            texture_size,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            mip_levels,
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
        let pipeline =
            crate::ibl::diffuse_irradiance::DiffuseIrradianceConvolutionRenderPipeline::new(
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
        let bindgroup = crate::ibl::diffuse_irradiance::diffuse_irradiance_convolution_bindgroup(
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
            None,
        )
    }

    fn create_prefiltered_environment_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        environment_texture: &crate::Texture,
        unit_cube_mesh: &crate::mesh::Mesh,
        proj: Mat4,
        views: [Mat4; 6],
    ) -> crate::Texture {
        let mut constants = crate::uniform::Uniform::new(
            device,
            GpuConstants {
                camera_projection: proj,
                ..Default::default()
            },
            wgpu::BufferUsages::VERTEX,
            wgpu::ShaderStages::VERTEX,
        );
        let mut roughness = Uniform::<f32>::new(
            device,
            0.0,
            wgpu::BufferUsages::empty(),
            wgpu::ShaderStages::VERTEX_FRAGMENT,
        );
        let (pipeline, bindgroup) =
            crate::ibl::prefiltered_environment::create_pipeline_and_bindgroup(
                device,
                &constants,
                &roughness,
                environment_texture,
            );

        let mut cubemap_faces = Vec::new();

        for i in 0..6 {
            for mip_level in 0..5 {
                let mip_width: u32 = 128 >> mip_level;
                let mip_height: u32 = 128 >> mip_level;

                // update the roughness for these mips
                *roughness = mip_level as f32 / 4.0;
                roughness.update(queue);

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("specular convolution"),
                });

                let cubemap_face = crate::Texture::new_with(
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

                    render_pass.set_pipeline(&pipeline);
                    render_pass.set_bind_group(0, &bindgroup, &[]);
                    unit_cube_mesh.draw(&mut render_pass);
                }

                queue.submit([encoder.finish()]);
                cubemap_faces.push(cubemap_face);
            }
        }

        crate::Texture::new_cubemap_texture(
            device,
            queue,
            Some(&format!("prefiltered environment cubemap")),
            128,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            5,
        )
    }

    fn create_precomputed_brdf_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> crate::Texture {
        #[repr(C)]
        #[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        struct Vert {
            pos: [f32; 3],
            uv: [f32; 2],
        }

        let bl = Vert {
            pos: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
        };
        let br = Vert {
            pos: [1.0, -1.0, 0.0],
            uv: [1.0, 1.0],
        };
        let tl = Vert {
            pos: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
        };
        let tr = Vert {
            pos: [1.0, 1.0, 0.0],
            uv: [1.0, 0.0],
        };

        let vertices = [bl, br, tr, bl, tr, tl];

        let screen_space_quad_mesh =
            crate::mesh::Mesh::from_vertices(device, Some("brdf_lut"), vertices);

        let vertex_module = device.create_shader_module(wgpu::include_spirv!(
            "linkage/vertex_brdf_lut_convolution.spv"
        ));
        let fragment_module = device.create_shader_module(wgpu::include_spirv!(
            "linkage/fragment_brdf_lut_convolution.spv"
        ));
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("brdf_lut_convolution"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_module,
                entry_point: "vertex_brdf_lut_convolution",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: (3 + 2) * std::mem::size_of::<f32>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32x2
                    ],
                }],
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
                module: &fragment_module,
                entry_point: "fragment_brdf_lut_convolution",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg16Float,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let framebuffer = crate::Texture::new_with(
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&pipeline);
            screen_space_quad_mesh.draw(&mut render_pass);
        }
        queue.submit([encoder.finish()]);
        framebuffer
    }
}

#[cfg(test)]
mod test {
    use glam::{Vec2, Vec3, Vec4};
    use renderling_shader::ui::{UiMode, UiVertex};

    use super::*;
    use crate::Renderling;

    #[test]
    fn hdr_texture() {
        let mut r = Renderling::headless(400, 400).unwrap();
        let hdr_data = std::fs::read("../../img/hdr/helipad.hdr").unwrap();
        let tex = Skybox::create_hdr_texture(r.get_device(), r.get_queue(), &hdr_data);
        let builder = r.new_ui_scene().with_canvas_size(400, 400);
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
        img_diff::assert_img_eq("skybox/environment.png", img);
    }

    #[test]
    fn hdr_skybox_scene() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let proj = crate::camera::perspective(600.0, 400.0);
        let view = crate::camera::look_at(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, Vec3::Y);
        let mut builder = r.new_scene().with_camera(proj, view);
        builder.add_skybox_image_from_path("../../img/hdr/resting_place.hdr");
        let scene = builder.build().unwrap();

        assert_eq!(
            wgpu::TextureFormat::Rgba16Float,
            scene.skybox.irradiance_cubemap.texture.format()
        );
        assert_eq!(
            wgpu::TextureFormat::Rgba16Float,
            scene
                .skybox
                .prefiltered_environment_cubemap
                .texture
                .format()
        );

        for i in 0..6 {
            // save out the irradiance face
            let copied_buffer = scene.skybox.irradiance_cubemap.read_from(
                r.get_device(),
                r.get_queue(),
                32,
                32,
                4,
                2,
                0,
                Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
            );
            let pixels = copied_buffer.pixels(r.get_device());
            let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                .iter()
                .map(|p| half::f16::from_bits(*p).to_f32())
                .collect::<Vec<_>>();
            assert_eq!(32 * 32 * 4, pixels.len());
            let img: image::Rgba32FImage = image::ImageBuffer::from_vec(32, 32, pixels).unwrap();
            let img = image::DynamicImage::from(img);
            let img = img.to_rgba8();
            img_diff::save(&format!("skybox/irradiance{i}.png"), img);
            for mip_level in 0..5 {
                let mip_size = 128u32 >> mip_level;
                // save out the prefiltered environment faces' mips
                let copied_buffer = scene.skybox.prefiltered_environment_cubemap.read_from(
                    r.get_device(),
                    r.get_queue(),
                    mip_size as usize,
                    mip_size as usize,
                    4,
                    2,
                    mip_level,
                    Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
                );
                let pixels = copied_buffer.pixels(r.get_device());
                let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                    .iter()
                    .map(|p| half::f16::from_bits(*p).to_f32())
                    .collect::<Vec<_>>();
                assert_eq!((mip_size * mip_size * 4) as usize, pixels.len());
                let img: image::Rgba32FImage =
                    image::ImageBuffer::from_vec(mip_size, mip_size, pixels).unwrap();
                let img = image::DynamicImage::from(img);
                let img = img.to_rgba8();
                img_diff::save(
                    &format!("skybox/prefiltered_environment_face{i}_mip{mip_level}.png"),
                    img,
                );
            }
        }

        r.setup_render_graph(Some(scene), None, [], true);
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("skybox/hdr.png", img);
    }

    #[test]
    fn precomputed_brdf() {
        assert_eq!(2, std::mem::size_of::<u16>());
        let r = Renderling::headless(32, 32).unwrap();
        let (device, queue) = r.get_device_and_queue_owned();
        let brdf_lut = Skybox::create_precomputed_brdf_texture(&device, &queue);
        assert_eq!(wgpu::TextureFormat::Rg16Float, brdf_lut.texture.format());
        let copied_buffer = brdf_lut.read(&device, &queue, 512, 512, 2, 2);
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
