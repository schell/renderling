use anyhow::Context;
use wgpu::util::DeviceExt;

/// Creates a right-handed perspective projection matrix with [0,1] depth range.
fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
    assert!(z_near > 0.0 && z_far > 0.0);
    let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
    let h = cos_fov / sin_fov;
    let w = h / aspect_ratio;
    let r = z_far / (z_near - z_far);
    nalgebra::Matrix4::new(
        w,
        0.0,
        0.0,
        0.0,
        0.0,
        h,
        0.0,
        0.0,
        0.0,
        0.0,
        r,
        -1.0,
        0.0,
        0.0,
        r * z_near,
        0.0,
    )
    .transpose()
    .into()
}

fn main() -> Result<(), anyhow::Error> {
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Uncomment this line to show log traces in wgpu
    //ng::tracing_log::LogTracer::init().unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("blinn-phong lighting w/ forward pipeline");
    let window = window_builder.build(&event_loop)?;
    // redefine window size to be that of the actual physical pixels
    let window_size = window.inner_size();

    // Set up wgpu
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = smol::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
    }))
    .context("could not create adapter")?;

    let (device, queue) = smol::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None, // Trace path
    ))
    .context("adapter could not request the device and queue")?;

    let format = surface
        .get_preferred_format(&adapter)
        .context("surface is incompatible with the adapder")?;
    tracing::info!("surface prefers {:?} texture format", format);
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &surface_config);

    struct Texture {
        texture: wgpu::Texture,
        view: wgpu::TextureView,
        sampler: wgpu::Sampler,
    }

    let depth_texture = {
        let size = wgpu::Extent3d {
            width: window_size.width,
            height: window_size.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Texture {
            texture,
            view,
            sampler,
        }
    };

    let (_camera_buffer, camera_bindgroup) = {
        let aspect = 8.0 / 6.0;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let position = nalgebra::Point3::new(0.0, 1.0, 4.0f32);

        let look_at = nalgebra::Point3::new(0.0, 0.0, 0.0f32);
        let up = nalgebra::Vector3::y_axis();

        let view: [[f32; 4]; 4] = nalgebra::Matrix4::look_at_rh(&position, &look_at, &up).into();
        let projection = perspective_rh(fovy, aspect, znear, zfar);
        let viewproj = renderling::forward::ViewProjection { projection, view };

        renderling::forward::create_camera_uniform(&device, viewproj, "3dcamera")
    };

    let mut icosphere = icosahedron::Polyhedron::new_isocahedron(0.8, 5);
    icosphere.compute_triangle_normals();
    let icosahedron::Polyhedron {
        positions,
        normals,
        cells,
        ..
    } = icosphere;
    tracing::info!("icosphere created");

    let to_vertex = |ndx: &usize| -> renderling::forward::Vertex {
        let mut v = renderling::forward::Vertex::default();
        v.position = positions[*ndx].0.into();
        v.normal = normals[*ndx].0.into();
        v
    };
    let vertices: Vec<renderling::forward::Vertex> = cells
        .iter()
        .flat_map(|icosahedron::Triangle { a, b, c }| {
            let p0 = to_vertex(a);
            let p1 = to_vertex(b);
            let p2 = to_vertex(c);
            vec![p2, p1, p0]
        })
        .collect();

    pub struct MeshBuffer {
        buffer: wgpu::Buffer,
        len: usize,
    }

    pub struct Mesh {
        vertex_buffer: MeshBuffer,
        index_buffer: Option<MeshBuffer>,
    }

    impl Mesh {
        pub fn buffer<T: bytemuck::Pod>(
            device: &wgpu::Device,
            vertices: &[T],
            indices: Option<&[u16]>,
        ) -> Self {
            tracing::trace!("buffering mesh");
            let len = vertices.len();
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let vertex_buffer = MeshBuffer { buffer, len };
            let index_buffer = indices.map(|ndxs| {
                let len = ndxs.len();
                let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(ndxs),
                    usage: wgpu::BufferUsages::INDEX,
                });
                MeshBuffer { buffer, len }
            });

            Mesh {
                vertex_buffer,
                index_buffer,
            }
        }
    }

    let sphere_mesh: Mesh = Mesh::buffer(&device, &vertices, None);

    let sphere_model_matrix = nalgebra::Translation3::new(0.5, 0.0, -1.0).to_homogeneous();
    let sphere_normal_matrix = sphere_model_matrix
        .try_inverse()
        .unwrap()
        .transpose()
        .fixed_resize::<3, 3>(0.0);
    let mut sphere_instance_matrix = sphere_model_matrix.as_slice().to_vec();
    sphere_instance_matrix.append(&mut sphere_normal_matrix.as_slice().to_vec());

    let sphere_instances_num = 1;
    let sphere_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sphere instance buffer"),
        contents: bytemuck::cast_slice(&sphere_instance_matrix),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    impl Texture {
        fn create_texture(
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            label: Option<&str>,
            usage: Option<wgpu::TextureUsages>,
            color_channels: u32,
            width: u32,
            height: u32,
            data: &[u8],
        ) -> Self {
            let size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };

            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: usage.unwrap_or(
                    wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                ),
            });

            queue.write_texture(
                wgpu::ImageCopyTextureBase {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(color_channels * width),
                    rows_per_image: std::num::NonZeroU32::new(height),
                },
                size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            Texture {
                texture,
                view,
                sampler,
            }
        }
    }

    let material_diffuse_texture = Texture::create_texture(
        &device,
        &queue,
        Some("diffuse material component"),
        None,
        4,
        1,
        1,
        &[0xff, 0xff, 0xff, 0xff],
    );
    let material_specular_texture = Texture::create_texture(
        &device,
        &queue,
        Some("specular material component"),
        None,
        4,
        1,
        1,
        &[174, 174, 100, 255],
    );
    let material_shininess = 16.0;
    let material = renderling::forward::MaterialUniform::new(
        &device,
        &material_diffuse_texture.view,
        &material_diffuse_texture.sampler,
        &material_specular_texture.view,
        &material_specular_texture.sampler,
        material_shininess,
    );

    let spot_light = renderling::forward::SpotLight {
        position: [0.0, 10.0, 0.0],
        direction: [0.0, -1.0, 0.0],
        inner_cutoff: std::f32::consts::PI / 3.0,
        attenuation: [1.0, 0.014, 0.007],
        outer_cutoff: std::f32::consts::PI / 2.0,
        ambient_color: [0.0627, 0.0627, 0.0627, 1.0],
        diffuse_color: [0.0627, 0.0627, 1.0, 1.0],
        specular_color: [0.694, 0.694, 1.0, 1.0],
        ..Default::default()
    };

    let point_light = renderling::forward::PointLight {
        position: [2.0, 2.0, 0.0],
        attenuation: [1.0, 0.14, 0.07],
        ambient_color: [0.1, 0.1, 0.1, 1.0],
        diffuse_color: [1.0, 1.0, 1.0, 1.0],
        specular_color: [0.5, 0.5, 0.5, 1.0],
        ..Default::default()
    };

    let lights = renderling::forward::LightsUniform::new(
        &device,
        vec![point_light],
        vec![spot_light],
        vec![],
    );

    let pipeline = renderling::forward::create_pipeline(&device, format)?;

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match &event {
            winit::event::Event::WindowEvent { event, .. } => match &event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => {}
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::RedrawEventsCleared => {
                device.poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                let surface_frame = surface.get_current_frame().unwrap();
                let surface_texture_view =
                    surface_frame
                        .output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor {
                            label: Some("blinn-phong scene surface texture view"),
                            format: None,
                            dimension: None,
                            aspect: wgpu::TextureAspect::All,
                            base_mip_level: 0,
                            mip_level_count: None,
                            base_array_layer: 0,
                            array_layer_count: None,
                        });

                renderling::conduct_clear_pass(
                    &device,
                    &queue,
                    Some("clear pass"),
                    &surface_texture_view,
                    Some(&depth_texture.view),
                    wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                );

                let camera = renderling::forward::Camera {
                    bindgroup: &camera_bindgroup,
                };
                let object = renderling::forward::Object {
                    mesh_buffer: sphere_mesh.vertex_buffer.buffer.slice(..),
                    instances: sphere_instance_buffer.slice(..),
                    instances_range: 0..sphere_instances_num,
                    name: Some("sphere object"),
                    draw: renderling::ObjectDraw::Default {
                        vertex_range: 0..sphere_mesh.vertex_buffer.len as u32,
                    },
                };

                let object_group = renderling::forward::ObjectGroup {
                    material: &material.bindgroup,
                    objects: vec![object],
                };

                renderling::forward::render(
                    "blinn-phong scene",
                    &device,
                    &queue,
                    &pipeline,
                    &surface_texture_view,
                    &depth_texture.view,
                    &lights.bindgroup,
                    &camera,
                    std::iter::once(&object_group),
                )
            }
            _ => {}
        }
    })
}
