use renderling::{Renderling, math::{Vec3, Vec4, Vec2, Mat4}, SceneConfig, Read, Scene, GpuCamera, Device, Queue, GpuVertex, IsGraphNode};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 100,
        height: 100,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer");
    let window = window_builder.build(&event_loop).unwrap();

    let mut r = Renderling::try_from_window(&window)
        .unwrap()
        .with_background_color(Vec3::splat(0.0).extend(1.0));
    let cfg = SceneConfig {
        max_vertices: 6,
        max_transforms: 4,
        max_entities: 2,
        max_lights: 0,
    };
    let mut scene = r
        .graph
        .visit(|(device, queue): (Read<Device>, Read<Queue>)| Scene::new(&device, &queue, cfg))
        .unwrap();
    let (projection, view) = renderling::default_ortho2d(100.0, 100.0);
    let camera = GpuCamera { projection, view };
    scene.set_camera(camera);

    // now test the textures functionality
    let img = image::io::Reader::open("img/cheetah.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let img = img.to_rgba8();

    let tex_ids = scene
        .load_images(r.get_device(), r.get_queue(), vec![img.clone()])
        .unwrap();
    assert_eq!(vec![1], tex_ids);

    let rects = scene.atlas().images();
    assert_eq!(0, rects[0].0);
    assert_eq!(1, rects[0].1.w);
    assert_eq!(1, rects[0].1.h);

    let verts = vec![
        GpuVertex {
            position: Vec4::new(0.0, 0.0, 0.0, 0.0),
            color: Vec4::new(1.0, 1.0, 0.0, 1.0),
            uv0: Vec2::new(0.0, 0.0),
            uv1: Vec2::new(0.0, 0.0),
            ..Default::default()
        },
        GpuVertex {
            position: Vec4::new(100.0, 0.0, 0.0, 0.0),
            color: Vec4::new(1.0, 0.0, 1.0, 1.0),
            uv0: Vec2::new(1.0, 0.0),
            uv1: Vec2::new(1.0, 0.0),
            ..Default::default()
        },
        GpuVertex {
            position: Vec4::new(100.0, 100.0, 0.0, 0.0),
            color: Vec4::new(0.0, 1.0, 1.0, 1.0),
            uv0: Vec2::new(1.0, 1.0),
            uv1: Vec2::new(1.0, 1.0),
            ..Default::default()
        },
    ];
    let ent = scene
        .new_entity()
        .with_meshlet(verts.clone())
        .with_transform(Mat4::IDENTITY)
        .with_texture_ids(Some(tex_ids[0]), None)
        .build()
        .unwrap();
    assert_eq!(0, ent.id);

    renderling::setup_scene_render_graph(scene, &mut r);

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
                r.get_device().poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                r.render().unwrap();
            }
            _ => {}
        }
    })
}
