use std::time::Instant;

use nalgebra::{Point3, UnitQuaternion, UnitVector3, Vector3};
use renderling::{ForwardVertex, MeshBuilder, Renderling, WgpuState, WorldTransform};

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
    env_logger::Builder::default()
        .filter_module("renderling", log::LevelFilter::Debug)
        .filter_module("naga", log::LevelFilter::Warn)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .init();

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
    let mut gpu = WgpuState::from_window(&window).unwrap();
    // Get our ui renderling
    let mut ui: Renderling = gpu.new_ui_renderling();

    let ui_camera = ui.new_camera().with_projection_ortho2d().build();
    let _triangle = ui
        .new_object()
        .with_camera(&ui_camera)
        .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                    renderling::UiVertex::default()
                        .with_position(10.0, 10.0, 0.0)
                        .with_color(0.0, 1.0, 1.0, 1.0),
                    renderling::UiVertex::default()
                        .with_position(window_size.width as f32 - 10.0, 10.0, 0.0)
                        .with_color(1.0, 0.0, 1.0, 1.0),
                    renderling::UiVertex::default()
                        .with_position(10.0, window_size.height as f32 - 10.0, 0.0)
                        .with_color(1.0, 1.0, 0.0, 1.0),
                ]))
        .build()
        .unwrap();

    let mut forward = gpu.new_forward_renderling();
    let forward_camera = forward
        .new_camera()
        .with_projection_perspective()
        .with_look_at(Point3::new(0.0, 1.0, 2.5), Point3::origin(), Vector3::y())
        .build();

    let mut icosphere = icosahedron::Polyhedron::new_isocahedron(0.65, 5);
    icosphere.compute_triangle_normals();
    let icosahedron::Polyhedron {
        positions,
        normals,
        cells,
        ..
    } = icosphere;
    log::info!("icosphere created");

    let to_vertex = |ndx: &usize| -> ForwardVertex {
        let mut v = ForwardVertex::default();
        v.position = positions[*ndx].0.into();
        v.normal = normals[*ndx].0.into();
        v
    };
    let sphere_vertices = cells.iter().flat_map(|icosahedron::Triangle { a, b, c }| {
        let p0 = to_vertex(&a);
        let p1 = to_vertex(&b);
        let p2 = to_vertex(&c);
        vec![p2, p1, p0]
    });
    let cube_vertices = unit_cube().into_iter().map(|(p, n)| {
        ForwardVertex::default()
            .with_position(p.x, p.y, p.z)
            .with_normal(n.x, n.y, n.z)
    });

    let _sphere = forward
        .new_object()
        .with_camera(&forward_camera)
        .with_mesh_builder(MeshBuilder::default().with_vertices(sphere_vertices))
        .build()
        .unwrap();
    let cube = forward
        .new_object()
        .with_camera(&forward_camera)
        .with_mesh_builder(MeshBuilder::default().with_vertices(cube_vertices))
        .build()
        .unwrap();

    let _spot_light = forward
        .new_spot_light()
        .with_position(Point3::new(0.0, 10.0, 0.0))
        .with_direction(Vector3::new(0.0, -1.0, 0.0))
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_ambient_color(wgpu::Color {
            r: 0.0627,
            g: 0.0627,
            b: 0.0627,
            a: 1.0,
        })
        .with_diffuse_color(wgpu::Color {
            r: 0.0627,
            g: 0.0627,
            b: 1.0,
            a: 1.0,
        })
        .with_specular_color(wgpu::Color {
            r: 0.694,
            g: 0.694,
            b: 1.0,
            a: 1.0,
        })
        .build();

    let _point_light = forward
        .new_point_light()
        .with_position(Point3::new(2.0, 2.0, 0.0))
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        })
        .with_diffuse_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
        .with_specular_color(wgpu::Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        })
        .build();

    let mut last_frame = Instant::now();
    let rotation_speed = std::f32::consts::FRAC_PI_4; // per second
    let mut rotation_y = 0.0;

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
                gpu.device.poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                // rotate the cube a bit
                let now = Instant::now();
                let dt = now - last_frame;
                let rotation = rotation_speed * dt.as_secs_f32();
                rotation_y += rotation;
                last_frame = now;
                cube.set_transform(WorldTransform::default().with_rotation(
                    UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation_y),
                ));

                let (frame, depth) = gpu.next_frame().unwrap();
                gpu.clear(Some(&frame), Some(&depth));
                ui.update().unwrap();
                ui.render(&frame, &depth).unwrap();

                gpu.clear(None, Some(&depth));
                forward.update().unwrap();
                forward.render(&frame, &depth).unwrap();
                gpu.present().unwrap();
            }
            _ => {}
        }
    })
}
