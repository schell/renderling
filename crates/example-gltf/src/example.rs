use nalgebra::{Matrix4, Point3, UnitVector3, Vector3};
use renderling::{Mesh, Texture};
use wgpu::util::DeviceExt;

pub fn run() {
    log::info!("running");

    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 400,
        height: 300,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("blinn-phong lighting w/ forward pipeline");
    let window = window_builder.build(&event_loop).unwrap();
    // redefine window size to be that of the actual physical pixels
    let window_size = window.inner_size();

    // Set up wgpu
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter =
        futures_lite::future::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

    let (device, queue) = futures_lite::future::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None, // Trace path
    ))
    .unwrap();

    let format = surface
        .get_supported_formats(&adapter)
        .first()
        .copied()
        .unwrap();
    log::debug!("surface prefers {:?} texture format", format);
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &surface_config);

    let depth_texture =
        Texture::create_depth_texture(&device, window_size.width, window_size.height);

    let (_camera_buffer, camera_bindgroup) = {
        let aspect = window_size.width as f32 / window_size.height as f32;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let position = nalgebra::Point3::from([0.0, 1.0, 2.5f32]);

        let look_at = nalgebra::Point3::from([0.0, 0.0, 0.0f32]);
        let up = nalgebra::Vector3::y_axis();

        let view: [[f32; 4]; 4] = nalgebra::Matrix4::look_at_rh(&position, &look_at, &up).into();
        let projection = renderling::math::perspective_rh(fovy, aspect, znear, zfar);
        let viewproj = renderling::ViewProjection { projection, view };

        renderling::shaders::forward::create_camera_uniform(&device, viewproj, "3dcamera")
    };

    //let (document, buffers, images) = gltf::import("gltf/orange_cube.glb").unwrap();
    let (document, buffers, images) = gltf::import("gltf/pyramid.gltf").unwrap();
    log::info!("{:#?}", document);

    //let to_vertex = |ndx: &usize| -> renderling::forward::Vertex {
    //    let mut v = renderling::forward::Vertex::default();
    //    v.position = positions[*ndx].0.into();
    //    v.normal = normals[*ndx].0.into();
    //    v
    //};
    //let sphere_vertices: Vec<renderling::forward::Vertex> = cells
    //    .iter()
    //    .flat_map(|icosahedron::Triangle { a, b, c }| {
    //        let p0 = to_vertex(&a);
    //        let p1 = to_vertex(&b);
    //        let p2 = to_vertex(&c);
    //        vec![p2, p1, p0]
    //    })
    //    .collect();
    //let cube_vertices = unit_cube()
    //    .into_iter()
    //    .map(|(p, n)| {
    //        renderling::forward::Vertex::default()
    //            .with_position(p.x, p.y, p.z)
    //            .with_normal(n.x, n.y, n.z)
    //    })
    //    .collect::<Vec<_>>();

    //let sphere_mesh: Mesh = Mesh::buffer(Some("sphere mesh"), &device, &sphere_vertices, None);
    //let cube_mesh: Mesh = Mesh::buffer(Some("cube mesh"), &device, &cube_vertices, None);

    //let sphere_model_matrix: Matrix4<f32> = Matrix4::identity();
    //let sphere_normal_matrix = sphere_model_matrix
    //    .try_inverse()
    //    .unwrap()
    //    .transpose()
    //    .fixed_resize::<3, 3>(0.0);
    //let mut sphere_instance_matrix = sphere_model_matrix.as_slice().to_vec();
    //sphere_instance_matrix.append(&mut sphere_normal_matrix.as_slice().to_vec());
    //let sphere_instances_num = 1;
    //let sphere_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //    label: Some("sphere instance buffer"),
    //    contents: bytemuck::cast_slice(&sphere_instance_matrix),
    //    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //});

    //let cube_model_matrix: Matrix4<f32> = Matrix4::identity();
    //let cube_normal_matrix = cube_model_matrix
    //    .try_inverse()
    //    .unwrap()
    //    .transpose()
    //    .fixed_resize::<3, 3>(0.0);
    //let mut cube_instance_matrix = cube_model_matrix.as_slice().to_vec();
    //cube_instance_matrix.append(&mut cube_normal_matrix.as_slice().to_vec());
    //let cube_instances_num = 1;
    //let cube_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //    label: Some("cube instance buffer"),
    //    contents: bytemuck::cast_slice(&cube_instance_matrix),
    //    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //});

    //let material_diffuse_texture = Texture::new(
    //    &device,
    //    &queue,
    //    Some("diffuse material component"),
    //    None,
    //    4,
    //    1,
    //    1,
    //    &[0xff, 0xff, 0xff, 0xff],
    //);
    //let material_specular_texture = Texture::new(
    //    &device,
    //    &queue,
    //    Some("specular material component"),
    //    None,
    //    4,
    //    1,
    //    1,
    //    &[174, 174, 100, 255],
    //);
    //let material_shininess = 16.0;
    //let material = renderling::forward::MaterialUniform::new(
    //    &device,
    //    &material_diffuse_texture.view,
    //    &material_diffuse_texture.sampler,
    //    &material_specular_texture.view,
    //    &material_specular_texture.sampler,
    //    material_shininess,
    //);

    //let spot_light = renderling::forward::SpotLight {
    //    position: [0.0, 10.0, 0.0],
    //    direction: [0.0, -1.0, 0.0],
    //    inner_cutoff: std::f32::consts::PI / 3.0,
    //    attenuation: [1.0, 0.014, 0.007],
    //    outer_cutoff: std::f32::consts::PI / 2.0,
    //    ambient_color: [0.0627, 0.0627, 0.0627, 1.0],
    //    diffuse_color: [0.0627, 0.0627, 1.0, 1.0],
    //    specular_color: [0.694, 0.694, 1.0, 1.0],
    //    ..Default::default()
    //};

    //let point_light = renderling::forward::PointLight {
    //    position: [2.0, 2.0, 0.0],
    //    attenuation: [1.0, 0.14, 0.07],
    //    ambient_color: [0.1, 0.1, 0.1, 1.0],
    //    diffuse_color: [1.0, 1.0, 1.0, 1.0],
    //    specular_color: [0.5, 0.5, 0.5, 1.0],
    //    ..Default::default()
    //};

    //let lights = renderling::forward::LightsUniform::new(
    //    &device,
    //    vec![point_light],
    //    vec![spot_light],
    //    vec![],
    //);

    //lights.update_point_lights(&queue, vec![point_light]);
    //lights.update_spot_lights(&queue, vec![spot_light]);

    //let pipeline = renderling::forward::create_pipeline(&device, format).unwrap();

    //event_loop.run(move |event, _target, control_flow| {
    //    *control_flow = winit::event_loop::ControlFlow::Poll;

    //    match &event {
    //        winit::event::Event::WindowEvent { event, .. } => match &event {
    //            winit::event::WindowEvent::CloseRequested
    //            | winit::event::WindowEvent::KeyboardInput {
    //                input:
    //                    winit::event::KeyboardInput {
    //                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
    //                        ..
    //                    },
    //                ..
    //            } => *control_flow = winit::event_loop::ControlFlow::Exit,
    //            _ => {}
    //        },
    //        winit::event::Event::MainEventsCleared => {
    //            window.request_redraw();
    //        }
    //        winit::event::Event::RedrawEventsCleared => {
    //            device.poll(wgpu::Maintain::Wait);
    //        }
    //        winit::event::Event::RedrawRequested(_) => {
    //            let surface_texture = surface.get_current_texture().unwrap();
    //            let surface_texture_view =
    //                surface_texture
    //                    .texture
    //                    .create_view(&wgpu::TextureViewDescriptor {
    //                        label: Some("blinn-phong scene surface texture view"),
    //                        format: None,
    //                        dimension: None,
    //                        aspect: wgpu::TextureAspect::All,
    //                        base_mip_level: 0,
    //                        mip_level_count: None,
    //                        base_array_layer: 0,
    //                        array_layer_count: None,
    //                    });

    //            renderling::conduct_clear_pass(
    //                &device,
    //                &queue,
    //                Some("clear pass"),
    //                &surface_texture_view,
    //                Some(&depth_texture.view),
    //                wgpu::Color {
    //                    r: 0.0,
    //                    g: 0.0,
    //                    b: 0.0,
    //                    a: 1.0,
    //                },
    //            );

    //            let camera = renderling::forward::Camera {
    //                bindgroup: &camera_bindgroup,
    //            };
    //            let sphere = renderling::forward::Object {
    //                mesh_buffer: sphere_mesh.vertex_buffer.buffer.slice(..),
    //                instances: sphere_instance_buffer.slice(..),
    //                instances_range: 0..sphere_instances_num,
    //                name: Some("sphere"),
    //                draw: renderling::ObjectDraw::Default {
    //                    vertex_range: 0..sphere_mesh.vertex_buffer.len as u32,
    //                },
    //                extra: (),
    //            };

    //            let cube = renderling::forward::Object {
    //                mesh_buffer: cube_mesh.vertex_buffer.buffer.slice(..),
    //                instances: cube_instance_buffer.slice(..),
    //                instances_range: 0..cube_instances_num,
    //                name: Some("cube"),
    //                draw: renderling::ObjectDraw::Default {
    //                    vertex_range: 0..cube_mesh.vertex_buffer.len as u32,
    //                },
    //                extra: (),
    //            };

    //            let object_group = renderling::forward::ObjectGroup {
    //                material: &material.bindgroup,
    //                objects: vec![sphere, cube],
    //            };

    //            renderling::forward::render(
    //                "blinn-phong scene",
    //                &device,
    //                &queue,
    //                &pipeline,
    //                &surface_texture_view,
    //                &depth_texture.view,
    //                &lights.bindgroup,
    //                &camera,
    //                std::iter::once(&object_group),
    //            );

    //            surface_texture.present();
    //        }
    //        _ => {}
    //    }
    //})
}
