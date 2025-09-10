//! WASM tests.
#![allow(dead_code)]

use craballoc::{
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
};
use glam::{Vec3, Vec4};
use image::DynamicImage;
use renderling::{geometry::Vertex, prelude::*, texture::CopiedTextureBuffer};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::wasm_bindgen::prelude::*;
use wire_types::{Error, PixelType};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
/// Writes a textfile containing some system info.
///
/// If you need more info on CI etc, add it here.
async fn can_write_system_info_artifact() {
    let _ = console_log::init();

    let user_agent = web_sys::window()
        .expect_throw("no window")
        .navigator()
        .user_agent()
        .expect_throw("no user agent");
    log::info!("user_agent: {user_agent}");

    let table = std::collections::HashMap::<String, String>::from_iter(Some((
        "user_agent".to_owned(),
        user_agent,
    )));
    let file = format!("{table:#?}");
    loading_bytes::post_bin_wasm::<Result<(), wire_types::Error>>(
        "http://127.0.0.1:4000/artifact/info.txt",
        file.as_bytes(),
    )
    .await
    .unwrap_throw()
    .unwrap_throw();
}

#[wasm_bindgen_test]
async fn can_create_headless_ctx() {
    let _ctx = renderling::Context::try_new_headless(256, 256, None)
        .await
        .unwrap_throw();
}

#[wasm_bindgen_test]
async fn stage_creation() {
    let ctx = renderling::Context::try_new_headless(256, 256, None)
        .await
        .unwrap_throw();
    let _stage = ctx.new_stage();
}

fn image_from_bytes(bytes: &[u8]) -> image::DynamicImage {
    image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .expect_throw("could not guess format")
        .decode()
        .expect_throw("could not decode")
}

async fn load_test_img(path: &str) -> image::DynamicImage {
    let result = loading_bytes::load(&format!("http://127.0.0.1:4000/test_img/{path}")).await;
    let bytes = match result {
        Ok(bytes) => bytes,
        Err(e) => panic!("{e}"),
    };
    image_from_bytes(&bytes)
}

fn image_to_wire(seen: impl Into<DynamicImage>) -> wire_types::Image {
    let img: DynamicImage = seen.into();
    let width = img.width();
    let height = img.height();
    let (pixel, bytes) = match img {
        DynamicImage::ImageRgb8(image_buffer) => (PixelType::Rgb8, image_buffer.to_vec()),
        DynamicImage::ImageRgba8(image_buffer) => (PixelType::Rgba8, image_buffer.to_vec()),
        _ => panic!("Image type is not yet supported in the WASM tests"),
    };
    wire_types::Image {
        width,
        height,
        bytes,
        pixel,
    }
}

async fn assert_img_eq(filename: &str, seen: impl Into<image::DynamicImage>) {
    let wire_data = image_to_wire(seen);
    let data = serde_json::to_string(&wire_data).unwrap();
    let result = loading_bytes::post_json_wasm::<Result<(), wire_types::Error>>(
        &format!("http://127.0.0.1:4000/assert_img_eq/{filename}"),
        &data,
    )
    .await
    .unwrap();

    if let Err(Error { description }) = result {
        panic!("{description}");
    }
}

async fn save(filename: &str, seen: impl Into<image::DynamicImage>) {
    let wire_data = image_to_wire(seen);
    let data = serde_json::to_string(&wire_data).unwrap();
    let result = loading_bytes::post_json_wasm::<Result<(), wire_types::Error>>(
        &format!("http://127.0.0.1:4000/save/{filename}"),
        &data,
    )
    .await
    .unwrap();

    if let Err(Error { description }) = result {
        panic!("{description}");
    }
}

#[wasm_bindgen_test]
async fn can_load_image() {
    let _img = load_test_img("jolt.png").await;
}

#[wasm_bindgen_test]
async fn can_img_diff() {
    let a = load_test_img("jolt.png").await;
    assert_img_eq("jolt.png", a).await;

    let b = load_test_img("cmy_triangle/hdr.png").await;
    assert_img_eq("cmy_triangle/hdr.png", b).await;
}

/// Performs a clearing render pass with internal context machinery.
///
/// This tests that the context setup is correct.
#[wasm_bindgen_test]
async fn can_clear_background_sanity() {
    let instance = renderling::internal::new_instance(None);
    let (_adapter, device, queue, target) =
        renderling::internal::new_headless_device_queue_and_target(2, 2, &instance)
            .await
            .unwrap();
    let texture = target.as_texture().expect("unexpected RenderTarget");
    let view = texture.create_view(&Default::default());

    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
                resolve_target: None,
            })],
            ..Default::default()
        });
    }
    let _index = queue.submit(Some(encoder.finish()));

    let runtime = WgpuRuntime {
        device: device.into(),
        queue: queue.into(),
    };
    let buffer = CopiedTextureBuffer::new(&runtime, texture).unwrap();
    let img = buffer.convert_to_rgba().await.unwrap();
    assert_img_eq("clear.png", img).await;
}

/// Test rendering a triangle using no mesh geometry.
#[wasm_bindgen_test]
async fn implicit_isosceles_triangle() {
    let ctx = Context::headless(100, 100).await;
    let runtime = ctx.as_ref();

    fn create_pipeline(
        runtime: &WgpuRuntime,
        vmodule: &wgpu::ShaderModule,
        ventry_point: &str,
        fmodule: &wgpu::ShaderModule,
        fentry_point: &str,
    ) -> wgpu::RenderPipeline {
        runtime
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: wgpu::VertexState {
                    module: vmodule,
                    entry_point: Some(ventry_point),
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
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: fmodule,
                    entry_point: Some(fentry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                multiview: None,
                cache: None,
            })
    }
    // The first time through render with handwritten WGSL to ensure the setup works
    let hand_written_wgsl_pipeline = {
        let vertex = runtime.device.create_shader_module(wgpu::include_wgsl!(
            "../src/tutorial/implicit_isosceles_vertex.wgsl"
        ));
        let fragment = runtime
            .device
            .create_shader_module(wgpu::include_wgsl!("../src/tutorial/passthru.wgsl"));
        create_pipeline(runtime, &vertex, "main", &fragment, "main")
    };
    // The second time render with WGSL that is transpiled from Rust code and pulled in through
    // the renderling linkage machinery.
    let linkage_pipeline = {
        let vertex = renderling::linkage::implicit_isosceles_vertex::linkage(&runtime.device);
        let fragment = renderling::linkage::passthru_fragment::linkage(&runtime.device);
        create_pipeline(
            runtime,
            &vertex.module,
            vertex.entry_point,
            &fragment.module,
            fragment.entry_point,
        )
    };

    async fn render(runtime: &WgpuRuntime, frame: &Frame, pipeline: wgpu::RenderPipeline) {
        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view(),
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&pipeline);
            render_pass.draw(0..3, 0..1);
        }
        let _index = runtime.queue.submit(std::iter::once(encoder.finish()));

        let img = frame
            .read_image()
            .await
            .expect_throw("could not read frame");
        assert_img_eq("tutorial/implicit_isosceles_triangle.png", img).await;
    }

    let frame = ctx.get_next_frame().unwrap();
    render(runtime, &frame, hand_written_wgsl_pipeline).await;
    frame.present();
    let frame = ctx.get_next_frame().unwrap();
    render(runtime, &frame, linkage_pipeline).await;
}

/// Test rendering a triangle from vertices on a slab, without an instance_index.
#[wasm_bindgen_test]
async fn slabbed_vertices_no_instance() {
    let _ = console_log::init_with_level(log::Level::Debug);

    let instance = renderling::internal::new_instance(None);
    let (_adapter, device, queue, target) =
        renderling::internal::new_headless_device_queue_and_target(100, 100, &instance)
            .await
            .unwrap();
    let runtime = WgpuRuntime {
        device: device.into(),
        queue: queue.into(),
    };

    // Create our geometry on the slab.
    let slab = SlabAllocator::new(
        &runtime,
        "isosceles-triangle-no-instance",
        wgpu::BufferUsages::empty(),
    );

    let initial_vertices = [
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.0),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        Vertex {
            position: Vec3::new(0.0, 0.5, 0.0),
            color: Vec4::new(0.0, 1.0, 0.0, 1.0),
            ..Default::default()
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.0),
            color: Vec4::new(0.0, 0.0, 1.0, 1.0),
            ..Default::default()
        },
    ];

    let vertices = slab.new_array(initial_vertices);

    assert_eq!(3, vertices.len());

    // Create a bindgroup for the slab so our shader can read out the types.

    let bindgroup_layout =
        runtime
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
    let pipeline_layout = runtime
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
    let pipeline = {
        let vertex = renderling::linkage::slabbed_vertices_no_instance::linkage(&runtime.device);
        let fragment = renderling::linkage::passthru_fragment::linkage(&runtime.device);
        runtime
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
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
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment.module,
                    entry_point: Some(fragment.entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                multiview: None,
                cache: None,
            })
    };

    let slab_buffer: SlabBuffer<wgpu::Buffer> = slab.commit();

    let bindgroup = runtime
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,

                resource: slab_buffer.as_entire_binding(),
            }],
        });

    let texture = target.as_texture().expect("unexpected RenderTarget");
    let view = texture.create_view(&Default::default());
    let mut encoder = runtime
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &bindgroup, &[]);
        render_pass.draw(0..3, 0..1);
    }
    let _index = runtime.queue.submit(std::iter::once(encoder.finish()));

    let buffer = CopiedTextureBuffer::new(runtime, texture).unwrap();
    let img = buffer.convert_to_rgba().await.unwrap();
    assert_img_eq("tutorial/slabbed_isosceles_triangle_no_instance.png", img).await;
}

#[wasm_bindgen_test]
async fn slabbed_isosceles_triangle() {
    let ctx = Context::headless(100, 100).await;
    let runtime = ctx.as_ref();

    // Create our geometry on the slab.
    let slab = SlabAllocator::new(
        runtime,
        "slabbed_isosceles_triangle",
        wgpu::BufferUsages::empty(),
    );

    let geometry = vec![
        (Vec3::new(0.5, -0.5, 0.0), Vec4::new(1.0, 0.0, 0.0, 1.0)),
        (Vec3::new(0.0, 0.5, 0.0), Vec4::new(0.0, 1.0, 0.0, 1.0)),
        (Vec3::new(-0.5, -0.5, 0.0), Vec4::new(0.0, 0.0, 1.0, 1.0)),
        (Vec3::new(-1.0, 1.0, 0.0), Vec4::new(1.0, 0.0, 0.0, 1.0)),
        (Vec3::new(-1.0, 0.0, 0.0), Vec4::new(0.0, 1.0, 0.0, 1.0)),
        (Vec3::new(0.0, 1.0, 0.0), Vec4::new(0.0, 0.0, 1.0, 1.0)),
    ];
    let vertices = slab.new_array(geometry);
    let array = slab.new_value(vertices.array());

    // Create a bindgroup for the slab so our shader can read out the types.
    let bindgroup_layout =
        runtime
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
    let pipeline_layout = runtime
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

    let vertex = renderling::linkage::slabbed_vertices::linkage(&runtime.device);
    let fragment = renderling::linkage::passthru_fragment::linkage(&runtime.device);
    let pipeline = runtime
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            cache: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                module: &vertex.module,
                entry_point: Some(vertex.entry_point),
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
            multisample: wgpu::MultisampleState {
                mask: !0,
                alpha_to_coverage_enabled: false,
                count: 1,
            },
            fragment: Some(wgpu::FragmentState {
                compilation_options: Default::default(),
                module: &fragment.module,
                entry_point: Some(fragment.entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
    let slab_buffer = slab.commit();

    let bindgroup = runtime
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: slab_buffer.as_entire_binding(),
            }],
        });

    let frame = ctx.get_next_frame().unwrap();
    let mut encoder = runtime.device.create_command_encoder(&Default::default());
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &frame.view(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &bindgroup, &[]);
        let id = array.id().inner();
        render_pass.draw(0..vertices.len() as u32, id..id + 1);
    }
    runtime.queue.submit(std::iter::once(encoder.finish()));

    let img = frame
        .read_linear_image()
        .await
        .expect_throw("could not read frame");
    assert_img_eq("tutorial/slabbed_isosceles_triangle.png", img).await;
}

// #[test]
// fn slabbed_render_unit() {
//     let mut r = Renderling::headless(100, 100).unwrap();
//     let (device, queue) = r.get_device_and_queue_owned();

//     // Create our geometry on the slab.
//     // Don't worry too much about capacity, it can grow.
//     let slab = crate::slab::SlabBuffer::new(&device, 16);
//     let geometry = vec![
//         Vertex {
//             position: Vec4::new(0.5, -0.5, 0.0, 1.0),
//             color: Vec4::new(1.0, 0.0, 0.0, 1.0),
//             ..Default::default()
//         },
//         Vertex {
//             position: Vec4::new(0.0, 0.5, 0.0, 1.0),
//             color: Vec4::new(0.0, 1.0, 0.0, 1.0),
//             ..Default::default()
//         },
//         Vertex {
//             position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
//             color: Vec4::new(0.0, 0.0, 1.0, 1.0),
//             ..Default::default()
//         },
//         Vertex {
//             position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
//             color: Vec4::new(1.0, 0.0, 0.0, 1.0),
//             ..Default::default()
//         },
//         Vertex {
//             position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
//             color: Vec4::new(0.0, 1.0, 0.0, 1.0),
//             ..Default::default()
//         },
//         Vertex {
//             position: Vec4::new(0.0, 1.0, 0.0, 1.0),
//             color: Vec4::new(0.0, 0.0, 1.0, 1.0),
//             ..Default::default()
//         },
//     ];
//     let vertices = slab.append_slice(&device, &queue, &geometry);
//     let unit = RenderUnit {
//         vertices,
//         ..Default::default()
//     };
//     let unit_id = slab.append(&device, &queue, &unit);

//     // Create a bindgroup for the slab so our shader can read out the types.
//     let label = Some("slabbed isosceles triangle");
//     let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//         label,
//         entries: &[wgpu::BindGroupLayoutEntry {
//             binding: 0,
//             visibility: wgpu::ShaderStages::VERTEX,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Storage { read_only: true },
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             count: None,
//         }],
//     });
//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label,
//         bind_group_layouts: &[&bindgroup_layout],
//         push_constant_ranges: &[],
//     });

//     let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label,
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &device.create_shader_module(wgpu::include_spirv!(
//                 "linkage/tutorial-slabbed_render_unit.spv"
//             )),
//             entry_point: "tutorial::slabbed_render_unit",
//             buffers: &[],
//         },
//         primitive: wgpu::PrimitiveState {
//             topology: wgpu::PrimitiveTopology::TriangleList,
//             strip_index_format: None,
//             front_face: wgpu::FrontFace::Ccw,
//             cull_mode: None,
//             unclipped_depth: false,
//             polygon_mode: wgpu::PolygonMode::Fill,
//             conservative: false,
//         },
//         depth_stencil: Some(wgpu::DepthStencilState {
//             format: wgpu::TextureFormat::Depth32Float,
//             depth_write_enabled: true,
//             depth_compare: wgpu::CompareFunction::Less,
//             stencil: wgpu::StencilState::default(),
//             bias: wgpu::DepthBiasState::default(),
//         }),
//         multisample: wgpu::MultisampleState {
//             mask: !0,
//             alpha_to_coverage_enabled: false,
//             count: 1,
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &device.create_shader_module(wgpu::include_spirv!(
//                 "linkage/tutorial-passthru_fragment.spv"
//             )),
//             entry_point: "tutorial::passthru_fragment",
//             targets: &[Some(wgpu::ColorTargetState {
//                 format: wgpu::TextureFormat::Rgba8UnormSrgb,
//                 blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                 write_mask: wgpu::ColorWrites::ALL,
//             })],
//         }),
//         multiview: None,
//     });

//     let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
//         label,
//         layout: &bindgroup_layout,
//         entries: &[wgpu::BindGroupEntry {
//             binding: 0,
//             resource: slab.get_buffer().as_entire_binding(),
//         }],
//     });

//     struct App {
//         pipeline: wgpu::RenderPipeline,
//         bindgroup: wgpu::BindGroup,
//         unit_id: Id<RenderUnit>,
//         unit: RenderUnit,
//     }

//     let app = App {
//         pipeline,
//         bindgroup,
//         unit_id,
//         unit,
//     };
//     r.graph.add_resource(app);

//     fn render(
//         (device, queue, app, frame, depth): (
//             View<Device>,
//             View<Queue>,
//             View<App>,
//             View<FrameTextureView>,
//             View<DepthTexture>,
//         ),
//     ) -> Result<(), GraphError> {
//         let label = Some("slabbed isosceles triangle");
//         let mut encoder =
//             device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
//         {
//             let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label,
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &frame.view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
//                         store: true,
//                     },
//                 })],
//                 depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
//                     view: &depth.view,
//                     depth_ops: Some(wgpu::Operations {
//                         load: wgpu::LoadOp::Load,
//                         store: true,
//                     }),
//                     stencil_ops: None,
//                 }),
//             });
//             render_pass.set_pipeline(&app.pipeline);
//             render_pass.set_bind_group(0, &app.bindgroup, &[]);
//             render_pass.draw(
//                 0..app.unit.vertices.len() as u32,
//                 app.unit_id.inner()..app.unit_id.inner() + 1,
//             );
//         }
//         queue.submit(std::iter::once(encoder.finish()));
//         Ok(())
//     }

//     use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
//     r.graph.add_subgraph(graph!(
//         create_frame
//             < clear_frame_and_depth
//             < render
//             < copy_frame_to_post
//             < present
//     ));

//     let img = r.render_image().unwrap();
//     img_diff::assert_img_eq("tutorial/slabbed_render_unit.png", img);
// }

//     #[test]
//     fn slabbed_render_unit_camera() {
//         let mut r = Renderling::headless(100, 100).unwrap();
//         let (device, queue) = r.get_device_and_queue_owned();

//         // Create our geometry on the slab.
//         // Don't worry too much about capacity, it can grow.
//         let slab = crate::slab::SlabBuffer::new(&device, 16);
//         let geometry = vec![
//             Vertex {
//                 position: Vec4::new(0.5, -0.5, 0.0, 1.0),
//                 color: Vec4::new(1.0, 0.0, 0.0, 1.0),
//                 ..Default::default()
//             },
//             Vertex {
//                 position: Vec4::new(0.0, 0.5, 0.0, 1.0),
//                 color: Vec4::new(0.0, 1.0, 0.0, 1.0),
//                 ..Default::default()
//             },
//             Vertex {
//                 position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
//                 color: Vec4::new(0.0, 0.0, 1.0, 1.0),
//                 ..Default::default()
//             },
//             Vertex {
//                 position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
//                 color: Vec4::new(1.0, 0.0, 0.0, 1.0),
//                 ..Default::default()
//             },
//             Vertex {
//                 position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
//                 color: Vec4::new(0.0, 1.0, 0.0, 1.0),
//                 ..Default::default()
//             },
//             Vertex {
//                 position: Vec4::new(0.0, 1.0, 0.0, 1.0),
//                 color: Vec4::new(0.0, 0.0, 1.0, 1.0),
//                 ..Default::default()
//             },
//         ];
//         let vertices = slab.append_slice(&device, &queue, &geometry);
//         let (projection, view) = crate::default_ortho2d(100.0, 100.0);
//         let camera_id = slab.append(
//             &device,
//             &queue,
//             &Camera {
//                 projection,
//                 view,
//                 ..Default::default()
//             },
//         );
//         let unit = RenderUnit {
//             vertices,
//             camera: camera_id,
//             position: Vec3::new(50.0, 50.0, 0.0),
//             scale: Vec3::new(50.0, 50.0, 1.0),
//             ..Default::default()
//         };
//         let unit_id = slab.append(&device, &queue, &unit);

//         // Create a bindgroup for the slab so our shader can read out the types.
//         let label = Some("slabbed isosceles triangle");
//         let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label,
//             entries: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStages::VERTEX,
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Storage { read_only: true },
//                     has_dynamic_offset: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//             }],
//         });
//         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label,
//             bind_group_layouts: &[&bindgroup_layout],
//             push_constant_ranges: &[],
//         });

//         let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label,
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &device.create_shader_module(wgpu::include_spirv!(
//                     "linkage/tutorial-slabbed_render_unit.spv"
//                 )),
//                 entry_point: "tutorial::slabbed_render_unit",
//                 buffers: &[],
//             },
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: None,
//                 unclipped_depth: false,
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 conservative: false,
//             },
//             depth_stencil: Some(wgpu::DepthStencilState {
//                 format: wgpu::TextureFormat::Depth32Float,
//                 depth_write_enabled: true,
//                 depth_compare: wgpu::CompareFunction::Less,
//                 stencil: wgpu::StencilState::default(),
//                 bias: wgpu::DepthBiasState::default(),
//             }),
//             multisample: wgpu::MultisampleState {
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//                 count: 1,
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &device.create_shader_module(wgpu::include_spirv!(
//                     "linkage/tutorial-passthru_fragment.spv"
//                 )),
//                 entry_point: "tutorial::passthru_fragment",
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format: wgpu::TextureFormat::Rgba8UnormSrgb,
//                     blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//             }),
//             multiview: None,
//         });

//         let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label,
//             layout: &bindgroup_layout,
//             entries: &[wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: slab.get_buffer().as_entire_binding(),
//             }],
//         });

//         struct App {
//             pipeline: wgpu::RenderPipeline,
//             bindgroup: wgpu::BindGroup,
//             unit_id: Id<RenderUnit>,
//             unit: RenderUnit,
//         }

//         let app = App {
//             pipeline,
//             bindgroup,
//             unit_id,
//             unit,
//         };
//         r.graph.add_resource(app);

//         fn render(
//             (device, queue, app, frame, depth): (
//                 View<Device>,
//                 View<Queue>,
//                 View<App>,
//                 View<FrameTextureView>,
//                 View<DepthTexture>,
//             ),
//         ) -> Result<(), GraphError> {
//             let label = Some("slabbed isosceles triangle");
//             let mut encoder =
//                 device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
//             {
//                 let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                     label,
//                     color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                         view: &frame.view,
//                         resolve_target: None,
//                         ops: wgpu::Operations {
//                             load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
//                             store: true,
//                         },
//                     })],
//                     depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
//                         view: &depth.view,
//                         depth_ops: Some(wgpu::Operations {
//                             load: wgpu::LoadOp::Load,
//                             store: true,
//                         }),
//                         stencil_ops: None,
//                     }),
//                 });
//                 render_pass.set_pipeline(&app.pipeline);
//                 render_pass.set_bind_group(0, &app.bindgroup, &[]);
//                 render_pass.draw(
//                     0..app.unit.vertices.len() as u32,
//                     app.unit_id.inner()..app.unit_id.inner() + 1,
//                 );
//             }
//             queue.submit(std::iter::once(encoder.finish()));
//             Ok(())
//         }

//         use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
//         r.graph.add_subgraph(graph!(
//             create_frame
//                 < clear_frame_and_depth
//                 < render
//                 < copy_frame_to_post
//                 < present
//         ));

//         let img = r.render_image().unwrap();
//         img_diff::assert_img_eq("tutorial/slabbed_render_unit_camera.png", img);
//     }
// }

#[wasm_bindgen_test]
async fn can_clear_background() {
    let ctx = Context::try_new_headless(2, 2, None).await.unwrap();
    let stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let seen = frame.read_image().await.unwrap();
    assert_img_eq("clear.png", seen).await;
}

// #[wasm_bindgen_test]
// #[should_panic]
// async fn can_save_wrong_diffs() {
//     let img = load_test_img("jolt.png").await;
//     assert_img_eq("cmy_triangle/hdr.png", img).await;
// }

fn right_tri_vertices() -> Vec<Vertex> {
    vec![
        Vertex::default()
            .with_position([0.0, 0.0, 0.0])
            .with_color([0.0, 1.0, 1.0, 1.0]),
        Vertex::default()
            .with_position([0.0, 100.0, 0.0])
            .with_color([1.0, 1.0, 0.0, 1.0]),
        Vertex::default()
            .with_position([100.0, 0.0, 0.0])
            .with_color([1.0, 0.0, 1.0, 1.0]),
    ]
}

#[wasm_bindgen_test]
async fn can_render_hello_triangle() {
    // This is a wasm version of cmy_triangle_sanity
    let ctx = Context::try_new_headless(100, 100, None).await.unwrap();
    let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
    let (projection, view) = renderling::camera::default_ortho2d(100.0, 100.0);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);
    let _rez = stage
        .new_primitive()
        .with_vertices(stage.new_vertices(right_tri_vertices()));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame
        .read_linear_image()
        .await
        .expect_throw("could not read frame");
    assert_img_eq("cmy_triangle/hdr.png", img).await;
}
