//! CPU side of the cubemap module.
use std::sync::Arc;

use glam::{Mat4, UVec2, Vec3, Vec4};
use image::GenericImageView;

use crate::{
    camera::Camera,
    stage::{Stage, StageRendering},
    texture::Texture,
};

use super::{CubemapDescriptor, CubemapFaceDirection};

pub fn cpu_sample_cubemap(cubemap: &[image::DynamicImage; 6], coord: Vec3) -> Vec4 {
    let coord = coord.normalize_or(Vec3::X);
    let (face_index, uv) = CubemapDescriptor::get_face_index_and_uv(coord);

    // Get the selected image
    let image = &cubemap[face_index];

    // Convert 2D UV to pixel coordinates
    let (width, height) = image.dimensions();
    let px = uv.x * (width as f32 - 1.0);
    let py = uv.y * (height as f32 - 1.0);

    // Sample using the nearest neighbor for simplicity
    let image::Rgba([r, g, b, a]) = image.get_pixel(px.round() as u32, py.round() as u32);

    // Convert the sampled pixel to Vec4
    Vec4::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

/// A cubemap that acts as a render target for an entire scene.
///
/// Use this to create and update a skybox with scene geometry.
pub struct SceneCubemap {
    pipeline: Arc<wgpu::RenderPipeline>,
    cubemap_texture: wgpu::Texture,
    depth_texture: crate::texture::Texture,
    clear_color: wgpu::Color,
}

impl SceneCubemap {
    pub fn new(
        device: &wgpu::Device,
        size: UVec2,
        format: wgpu::TextureFormat,
        clear_color: Vec4,
    ) -> Self {
        let label = Some("scene-to-cubemap");
        let cubemap_texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let depth_texture = Texture::create_depth_texture(device, size.x, size.y, 1, label);
        let pipeline = Arc::new(Stage::create_stage_render_pipeline(device, format, 1));
        Self {
            pipeline,
            cubemap_texture,
            depth_texture,
            clear_color: wgpu::Color {
                r: clear_color.x as f64,
                g: clear_color.y as f64,
                b: clear_color.z as f64,
                a: clear_color.w as f64,
            },
        }
    }

    pub fn run(&self, stage: &Stage) {
        // create a camera for our cube
        let camera = stage.new_value(Camera::default());

        let mut prev_camera_ids = vec![];
        for rlet in stage.renderlets_iter() {
            if let Some(rlet) = rlet.upgrade() {
                let mut guard = rlet.lock();
                prev_camera_ids.push(guard.camera_id);
                // Overwrite the renderlet's camera
                guard.camera_id = camera.id();
            }
        }

        // By setting this to 90 degrees (PI/2 radians) we make sure the viewing field
        // is exactly large enough to fill a single face of the cubemap such that all
        // faces align correctly to each other at the edges.
        let fovy = std::f32::consts::FRAC_PI_2;
        let aspect = self.cubemap_texture.width() as f32 / self.cubemap_texture.height() as f32;
        let projection = Mat4::perspective_lh(fovy, aspect, 1.0, 25.0);
        // Render each face by rendering the scene from each camera angle into the cubemap
        for (i, face) in CubemapFaceDirection::FACES.iter().enumerate() {
            // Update the camera angle, no need to sync as calling `Stage::render` does this
            // implicitly
            camera.modify(|c| c.set_projection_and_view(projection, face.view()));
            let label_s = format!("scene-to-cubemap-{i}");
            let view = self
                .cubemap_texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&label_s),
                    base_array_layer: i as u32,
                    array_layer_count: Some(1),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    ..Default::default()
                });
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            };
            let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            };
            let (_, _) = StageRendering {
                pipeline: &self.pipeline,
                stage,
                color_attachment,
                depth_stencil_attachment,
            }
            .run();
        }
    }
}

/// A render pipeline for blitting an equirectangular image as a cubemap.
pub struct EquirectangularImageToCubemapBlitter(pub wgpu::RenderPipeline);

impl EquirectangularImageToCubemapBlitter {
    pub fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cubemap-making bindgroup"),
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
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }

    pub fn create_bindgroup(
        device: &wgpu::Device,
        label: Option<&str>,
        buffer: &wgpu::Buffer,
        // The texture to sample the environment from
        texture: &Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &Self::create_bindgroup_layout(device),
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

    /// Create the rendering pipeline that creates cubemaps from equirectangular
    /// images.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        log::trace!("creating cubemap-making render pipeline with format '{format:?}'");
        let vertex_linkage = crate::linkage::skybox_cubemap_vertex::linkage(device);
        let fragment_linkage = crate::linkage::skybox_equirectangular_fragment::linkage(device);
        let bg_layout = Self::create_bindgroup_layout(device);
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cubemap-making pipeline layout"),
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        EquirectangularImageToCubemapBlitter(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("cubemap-making pipeline"),
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

#[cfg(test)]
mod test {
    use craballoc::slab::SlabAllocator;
    use glam::Vec4;
    use image::GenericImageView;

    use crate::{
        math::{UNIT_INDICES, UNIT_POINTS},
        stage::{Renderlet, Vertex},
    };

    use super::*;

    #[test]
    fn hand_rolled_cubemap_sampling() {
        let width = 256;
        let height = 256;
        let ctx = crate::Context::headless(width, height);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::ZERO)
            .with_lighting(false)
            .with_msaa_sample_count(4);
        let camera =
            stage.new_value(
                Camera::default_perspective(width as f32, height as f32)
                    .with_view(Mat4::look_at_rh(Vec3::splat(3.0), Vec3::ZERO, Vec3::Y)),
            );
        // geometry is the "clip cube" where colors are normalized 3d space coords
        let vertices = stage.new_array(UNIT_POINTS.map(|unit_cube_point| {
            Vertex::default()
                // multiply by 2.0 because the unit cube's AABB bounds are at 0.5, and we want 1.0
                .with_position(unit_cube_point * 2.0)
                // "normalize" (really "shift") the space coord from [-0.5, 0.5] to [0.0, 1.0]
                .with_color((unit_cube_point + 0.5).extend(1.0))
        }));
        let indices = stage.new_array(UNIT_INDICES.map(|u| u as u32));
        let renderlet = stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: camera.id(),
            ..Default::default()
        });
        stage.add_renderlet(&renderlet);

        let scene_cubemap = SceneCubemap::new(
            ctx.get_device(),
            UVec2::new(width, height),
            wgpu::TextureFormat::Rgba8Unorm,
            Vec4::ZERO,
        );

        scene_cubemap.run(&stage);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("cubemap/hand_rolled_cubemap_sampling/cube.png", img);
        frame.present();

        let slab = SlabAllocator::new(&ctx, wgpu::BufferUsages::empty());
        let uv = slab.new_value(Vec3::ZERO);
        let buffer = slab.commit();
        let label = Some("cubemap-sampling-test");
        let bind_group_layout =
            ctx.get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label,
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
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                            count: None,
                        },
                    ],
                });
        let cubemap_sampling_pipeline_layout =
            ctx.get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });
        let vertex = crate::linkage::cubemap_sampling_test_vertex::linkage(ctx.get_device());
        let fragment = crate::linkage::cubemap_sampling_test_fragment::linkage(ctx.get_device());
        let cubemap_sampling_pipeline =
            ctx.get_device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label,
                    layout: Some(&cubemap_sampling_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vertex.module,
                        entry_point: Some(vertex.entry_point),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
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
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &fragment.module,
                        entry_point: Some(fragment.entry_point),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            blend: None,
                            write_mask: wgpu::ColorWrites::all(),
                        })],
                    }),
                    multiview: None,
                    cache: None,
                });

        let cubemap_view =
            scene_cubemap
                .cubemap_texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label,
                    dimension: Some(wgpu::TextureViewDimension::Cube),
                    ..Default::default()
                });
        let cubemap_sampler = ctx.get_device().create_sampler(&wgpu::SamplerDescriptor {
            label,
            ..Default::default()
        });
        let bind_group = ctx
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&cubemap_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&cubemap_sampler),
                    },
                ],
            });
        let render_target = ctx.get_device().create_texture(&wgpu::TextureDescriptor {
            label,
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let render_target_view = render_target.create_view(&wgpu::TextureViewDescriptor::default());

        let sample = |dir: Vec3| -> Vec4 {
            uv.set(dir.normalize_or(Vec3::ZERO));
            slab.commit();

            let mut encoder = ctx
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &render_target_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(&cubemap_sampling_pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..6, 0..1);
            }
            let submission_index = ctx.get_queue().submit(Some(encoder.finish()));
            ctx.get_device()
                .poll(wgpu::Maintain::wait_for(submission_index));

            let img = Texture::read(&ctx, &render_target, 1, 1, 4, 1)
                .into_image::<u8, image::Rgba<u8>>(ctx.get_device())
                .unwrap();
            let image::Rgba([r, g, b, a]) = img.get_pixel(0, 0);
            Vec4::new(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            )
        };

        fn index_to_face_string(index: usize) -> &'static str {
            match index {
                0 => "+X",
                1 => "-X",
                2 => "+Y",
                3 => "-Y",
                4 => "+Z",
                5 => "-Z",
                _ => "?",
            }
        }

        let mut cpu_cubemap = vec![];
        for i in 0..6 {
            let img = Texture::read_from(
                &ctx,
                &scene_cubemap.cubemap_texture,
                width as usize,
                height as usize,
                4,
                1,
                0,
                Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
            )
            .into_image::<u8, image::Rgba<u8>>(ctx.get_device())
            .unwrap();

            img_diff::assert_img_eq(
                &format!(
                    "cubemap/hand_rolled_cubemap_sampling/face_{}.png",
                    index_to_face_string(i as usize)
                ),
                img.clone(),
            );

            cpu_cubemap.push(img);
        }
        let cpu_cubemap = [
            cpu_cubemap.remove(0),
            cpu_cubemap.remove(0),
            cpu_cubemap.remove(0),
            cpu_cubemap.remove(0),
            cpu_cubemap.remove(0),
            cpu_cubemap.remove(0),
        ];

        {
            // assert a few sanity checks on the cpu cubemap
            println!("x samples sanity");
            let x_samples_uv = [
                UVec2::ZERO,
                UVec2::new(255, 0),
                UVec2::new(127, 127),
                UVec2::new(255, 255),
                UVec2::new(0, 255),
            ];

            for uv in x_samples_uv {
                let image::Rgba([r, g, b, a]) = cpu_cubemap[0].get_pixel(uv.x, uv.y);
                println!("uv: {uv}");
                println!("rgba: {r} {g} {b} {a}");
            }
        }

        let mut uvs = vec![
            // start with cardinal directions
            Vec3::X,
            Vec3::NEG_X,
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::Z,
            Vec3::NEG_Z,
        ];

        // add corners to the uvs to sample
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let uv = Vec3::new(x, y, z);
                    uvs.push(uv);
                }
            }
        }

        // add in some deterministic pseudo-randomn points
        {
            let order = acorn_prng::Order::new(666);
            let seed = acorn_prng::Seed::new(1_000_000);
            let mut prng = acorn_prng::Acorn::new(order, seed);
            let mut rf32 = move || {
                let u = prng.generate_u32_between_range(0..=u32::MAX);
                f32::from_bits(u)
            };
            let mut rxvec3 = { || Vec3::new(f32::MAX, rf32(), rf32()).normalize_or(Vec3::X) };
            // let mut rvec3 = || Vec3::new(rf32(), rf32(), rf32());
            uvs.extend((0..20).map(|_| rxvec3()));
        }

        // add zero
        uvs.push(Vec3::ZERO);

        const THRESHOLD: f32 = 0.005;
        for uv in uvs.into_iter() {
            let nuv = uv.normalize_or(Vec3::X);
            let color = sample(uv);
            let (face_index, uv2d) =
                CubemapDescriptor::get_face_index_and_uv(uv.normalize_or(Vec3::X));
            let px = (uv2d.x * (width as f32 - 1.0)).round() as u32;
            let py = (uv2d.y * (height as f32 - 1.0)).round() as u32;
            let puv = UVec2::new(px, py);
            let cpu_color = cpu_sample_cubemap(&cpu_cubemap, uv);
            let dir_string = index_to_face_string(face_index);
            println!(
                "__uv: {uv},\n\
                 _nuv: {nuv},\n\
                 _gpu: {color}\n\
                 _cpu: {cpu_color}\n\
                 from: {dir_string}({face_index}) {uv2d} {puv}\n"
            );
            let cmp = pretty_assertions::Comparison::new(&color, &cpu_color);
            let distance = color.distance(cpu_color);
            if distance > THRESHOLD {
                println!("distance: {distance}");
                println!("{cmp}");
                panic!("distance {distance} greater than {THRESHOLD}");
            }
        }
    }
}
