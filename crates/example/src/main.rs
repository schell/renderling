use std::time::Instant;

use renderling::{
    math::{Quat, Vec3, Vec4},
    ForwardVertex, MeshBuilder, Renderling, Transform, UiPipeline, WgpuState,
};

fn run() -> Result<(), anyhow::Error> {
    env_logger::Builder::default()
        .filter_module("example", log::LevelFilter::Trace)
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
    let mut ui: Renderling<UiPipeline> = gpu.new_ui_renderling();

    let _ui_camera = ui.new_camera().with_projection_ortho2d().build();
    let _triangle = ui
        .new_object()
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
    let _forward_camera = forward
        .new_camera()
        .with_projection_perspective()
        .with_look_at(Vec3::new(0.0, 1.0, 2.5), Vec3::ZERO, Vec3::Y)
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
    let cube_vertices = renderling::math::unit_cube().into_iter().map(|(p, n)| {
        ForwardVertex::default()
            .with_position(p.x, p.y, p.z)
            .with_normal(n.x, n.y, n.z)
    });

    let _sphere = forward
        .new_object()
        .with_mesh_builder(MeshBuilder::default().with_vertices(sphere_vertices))
        .build()
        .unwrap();
    let cube = forward
        .new_object()
        .with_mesh_builder(MeshBuilder::default().with_vertices(cube_vertices))
        .build()
        .unwrap();

    let _spot_light = forward
        .new_spot_light()
        .with_position(Vec3::new(0.0, 10.0, 0.0))
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_ambient_color(Vec3::splat(0.0627).extend(1.0))
        .with_diffuse_color(Vec4::new(
            0.0627,
            0.0627,
            1.0,
            1.0,
        ))
        .with_specular_color(Vec4::new(
            0.694,
            0.694,
            1.0,
            1.0,
        ))
        .build();

    let _point_light = forward
        .new_point_light()
        .with_position(Vec3::new(2.0, 2.0, 0.0))
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(Vec3::splat(0.1).extend(1.0))
        .with_diffuse_color(Vec4::splat(1.0))
        .with_specular_color(Vec3::splat(0.5).extend(1.0))
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
                cube.set_transform(
                    Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Y, rotation_y)),
                );

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

fn main() {
    run().unwrap();
}
