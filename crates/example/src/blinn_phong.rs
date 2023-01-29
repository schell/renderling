use nalgebra::{Matrix4, Point3, UnitVector3, Vector3};
use renderling::{MeshBuilder, Renderling, Texture, WgpuState};
use wgpu::util::DeviceExt;

/// Points around the unit cube.
///
///    yb          1_____2     _____
///    |           /    /|    /|    |  (same box, left and front sides removed)
///    |___x     0/___3/ |   /7|____|6
///   /    g      |    | /   | /    /
/// z/r           |____|/   4|/____/5
pub fn unit_points() -> [Point3<f32>; 8] {
    let p0 = Point3::from([-0.5, 0.5, 0.5]);
    let p1 = Point3::from([-0.5, 0.5, -0.5]);
    let p2 = Point3::from([0.5, 0.5, -0.5]);
    let p3 = Point3::from([0.5, 0.5, 0.5]);

    let p4 = Point3::from([-0.5, -0.5, 0.5]);
    let p7 = Point3::from([-0.5, -0.5, -0.5]);
    let p6 = Point3::from([0.5, -0.5, -0.5]);
    let p5 = Point3::from([0.5, -0.5, 0.5]);

    [p0, p1, p2, p3, p4, p5, p6, p7]
}

pub fn triangle_face_normal(
    p1: &Point3<f32>,
    p2: &Point3<f32>,
    p3: &Point3<f32>,
) -> UnitVector3<f32> {
    let a = p1 - p2;
    let b = p1 - p3;
    let n: Vector3<f32> = a.cross(&b);
    UnitVector3::new_normalize(n)
}

pub fn unit_cube() -> Vec<(Point3<f32>, UnitVector3<f32>)> {
    let points = unit_points();
    let triangles: [(usize, usize, usize); 12] = [
        (0, 1, 2),
        (0, 2, 3), // top
        (0, 3, 4),
        (4, 3, 5), // front
        (3, 2, 6),
        (3, 6, 5), // right
        (1, 0, 7),
        (7, 0, 4), // left
        (4, 5, 6),
        (4, 6, 7), // bottom
        (2, 1, 7),
        (2, 7, 6), // back
    ];
    triangles
        .iter()
        .flat_map(|(a, b, c)| {
            let a = points[*a];
            let b = points[*b];
            let c = points[*c];
            let n = triangle_face_normal(&c, &b, &a);
            vec![(a, n), (b, n), (c, n)]
        })
        .collect::<Vec<_>>()
}

pub fn run() -> Result<(), anyhow::Error> {
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Uncomment this line to show log traces in wgpu
    //ng::tracing_log::LogTracer::init().unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 400,
        height: 300,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("blinn-phong lighting w/ forward pipeline");
    let window = window_builder.build(&event_loop)?;
    // redefine window size to be that of the actual physical pixels
    let window_size = window.inner_size();

    // Set up wgpu
    let mut wgpu_state = WgpuState::from_window(&window).unwrap();
    // Get our ui renderling
    let mut ui: Renderling = wgpu_state.new_ui_renderling();

    let ui_camera = ui.new_camera().with_projection_ortho2d().build();
    let triangle = ui
        .new_object()
        .with_camera(&ui_camera)
        .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                    renderling::Vertex::default()
                        .with_position(10.0, 10.0, 0.5)
                        .with_color(0.0, 1.0, 1.0, 1.0),
                    renderling::Vertex::default()
                        .with_position(window_size.width as f32 - 10.0, 10.0, 0.5)
                        .with_color(1.0, 0.0, 1.0, 1.0),
                    renderling::Vertex::default()
                        .with_position(10.0, window_size.height as f32 - 10.0, 0.5)
                        .with_color(1.0, 1.0, 0.0, 1.0),
                ]))
        .build()
        .unwrap();

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
                wgpu_state.device.poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                let (frame, depth) = wgpu_state.next_frame().unwrap();
                wgpu_state.clear(&frame, Some(&depth));
                ui.update().unwrap();
                ui.render(&frame, &depth).unwrap();
                wgpu_state.present().unwrap();
            }
            _ => {}
        }
    })

    //let (_camera_buffer, camera_bindgroup) = {
    //    let aspect = window_size.width as f32 / window_size.height as f32;
    //    let fovy = std::f32::consts::PI / 4.0;
    //    let znear = 0.1;
    //    let zfar = 100.0;
    //    let position = nalgebra::Point3::from([0.0, 1.0, 2.5f32]);

    //    let look_at = nalgebra::Point3::from([0.0, 0.0, 0.0f32]);
    //    let up = nalgebra::Vector3::y_axis();

    //    let view: [[f32; 4]; 4] = nalgebra::Matrix4::look_at_rh(&position, &look_at, &up).into();
    //    let projection = renderling::math::perspective_rh(fovy, aspect, znear, zfar);
    //    let viewproj = renderling::ViewProjection { projection, view };

    //    renderling::forward::create_camera_uniform(&wgpu_state.device, viewproj, "3dcamera")
    //};

    //let mut icosphere = icosahedron::Polyhedron::new_isocahedron(0.65, 5);
    //icosphere.compute_triangle_normals();
    //let icosahedron::Polyhedron {
    //    positions,
    //    normals,
    //    cells,
    //    ..
    //} = icosphere;
    //tracing::info!("icosphere created");

    //let to_vertex = |ndx: &usize| -> renderling::forward::Vertex {
    //    let mut v = renderling::forward::Vertex::default();
    //    v.position = positions[*ndx].0.into();
    //    v.normal = normals[*ndx].0.into();
    //    v
    //};
    //let sphere_vertices = cells.iter().flat_map(|icosahedron::Triangle { a, b, c }| {
    //    let p0 = to_vertex(&a);
    //    let p1 = to_vertex(&b);
    //    let p2 = to_vertex(&c);
    //    vec![p2, p1, p0]
    //});
    //let cube_vertices = unit_cube().into_iter().map(|(p, n)| {
    //    renderling::forward::Vertex::default()
    //        .with_position(p.x, p.y, p.z)
    //        .with_normal(n.x, n.y, n.z)
    //});

    //let sphere_mesh = MeshBuilder::default()
    //    .with_vertices(sphere_vertices)
    //    .build(Some("sphere mesh"), &wgpu_state.device);
    //let cube_mesh = MeshBuilder::default()
    //    .with_vertices(cube_vertices)
    //    .build(Some("cube mesh"), &wgpu_state.device);

    //let sphere_model_matrix: Matrix4<f32> = Matrix4::identity();
    //let sphere_normal_matrix = sphere_model_matrix
    //    .try_inverse()
    //    .unwrap()
    //    .transpose()
    //    .fixed_resize::<3, 3>(0.0);
    //let mut sphere_instance_matrix = sphere_model_matrix.as_slice().to_vec();
    //sphere_instance_matrix.append(&mut sphere_normal_matrix.as_slice().to_vec());
    //let sphere_instances_num = 1;
    //let sphere_instance_buffer =
    //    wgpu_state
    //        .device
    //        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //            label: Some("sphere instance buffer"),
    //            contents: bytemuck::cast_slice(&sphere_instance_matrix),
    //            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //        });

    //let cube_model_matrix: Matrix4<f32> = Matrix4::identity();
    //let cube_normal_matrix = cube_model_matrix
    //    .try_inverse()
    //    .unwrap()
    //    .transpose()
    //    .fixed_resize::<3, 3>(0.0);
    //let mut cube_instance_matrix = cube_model_matrix.as_slice().to_vec();
    //cube_instance_matrix.append(&mut cube_normal_matrix.as_slice().to_vec());
    //let cube_instances_num = 1;
    //let cube_instance_buffer =
    //    wgpu_state
    //        .device
    //        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //            label: Some("cube instance buffer"),
    //            contents: bytemuck::cast_slice(&cube_instance_matrix),
    //            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //        });

    //let material_diffuse_texture = Texture::new(
    //    &wgpu_state.device,
    //    &wgpu_state.queue,
    //    Some("diffuse material component"),
    //    None,
    //    4,
    //    1,
    //    1,
    //    &[0xff, 0xff, 0xff, 0xff],
    //);
    //let material_specular_texture = Texture::new(
    //    &wgpu_state.device,
    //    &wgpu_state.queue,
    //    Some("specular material component"),
    //    None,
    //    4,
    //    1,
    //    1,
    //    &[174, 174, 100, 255],
    //);
    //let material_shininess = 16.0;
    //let material = renderling::forward::MaterialUniform::new(
    //    &wgpu_state.device,
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
    //    &wgpu_state.device,
    //    vec![point_light],
    //    vec![spot_light],
    //    vec![],
    //);

    //lights.update_point_lights(&wgpu_state.queue, vec![point_light]);
    //lights.update_spot_lights(&wgpu_state.queue, vec![spot_light]);

    //let pipeline =
    //    renderling::forward::create_pipeline(&wgpu_state.device, wgpu_state.target.format())?;

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
    //            wgpu_state.device.poll(wgpu::Maintain::Wait);
    //        }
    //        winit::event::Event::RedrawRequested(_) => {
    //            wgpu_state.prepare_target_frame().unwrap();
    //            let target_frame = wgpu_state.current_target_frame.take().unwrap();
    //            let frame_texture = target_frame.texture();
    //            let frame_texture_view = frame_texture.create_view(&wgpu::TextureViewDescriptor {
    //                label: Some("blinn-phong scene surface texture view"),
    //                format: None,
    //                dimension: None,
    //                aspect: wgpu::TextureAspect::All,
    //                base_mip_level: 0,
    //                mip_level_count: None,
    //                base_array_layer: 0,
    //                array_layer_count: None,
    //            });

    //            renderling::conduct_clear_pass(
    //                &wgpu_state.device,
    //                &wgpu_state.queue,
    //                Some("clear pass"),
    //                &frame_texture_view,
    //                Some(&wgpu_state.depth_texture.view),
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
    //                &wgpu_state.device,
    //                &wgpu_state.queue,
    //                &pipeline,
    //                &frame_texture_view,
    //                &wgpu_state.depth_texture.view,
    //                &lights.bindgroup,
    //                &camera,
    //                std::iter::once(&object_group),
    //            );

    //            target_frame.present();
    //        }
    //        _ => {}
    //    }
    //})
}
