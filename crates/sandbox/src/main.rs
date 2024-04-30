use crabslab::GrowableSlab;
use renderling::{math::UVec2, Camera, Context, Renderlet, Stage, Vertex};
use std::sync::Arc;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_size = winit::dpi::LogicalSize {
        width: 100,
        height: 100,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer");
    let window = Arc::new(window_builder.build(&event_loop).unwrap());

    let (mut ctx, mut stage) = can_read_shader_debug_logs(window.clone());
    event_loop
        .run(move |event, target| {
            target.set_control_flow(winit::event_loop::ControlFlow::Poll);

            match &event {
                winit::event::Event::WindowEvent { event, .. } => match &event {
                    winit::event::WindowEvent::CloseRequested
                    | winit::event::WindowEvent::KeyboardInput {
                        event:
                            winit::event::KeyEvent {
                                logical_key:
                                    winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                                ..
                            },
                        ..
                    } => target.exit(),

                    winit::event::WindowEvent::Resized(size) => {
                        let size = UVec2::new(size.width, size.height);
                        ctx.set_size(size);
                        stage.set_size(size)
                    }

                    winit::event::WindowEvent::RedrawRequested => {
                        ctx.get_device().poll(wgpu::Maintain::Wait);
                    }
                    _ => {}
                },
                winit::event::Event::AboutToWait => {
                    let frame = ctx.get_current_frame().unwrap();
                    stage.render(&frame.view());
                    frame.present();
                }
                _ => {}
            }
        })
        .unwrap()
}

fn can_read_shader_debug_logs(window: Arc<winit::window::Window>) -> (Context, Stage) {
    let ctx = Context::try_from_window(window).unwrap();
    let mut stage = ctx.new_stage();
    let (projection, view) = renderling::default_ortho2d(100.0, 100.0);
    let camera = stage.append(&Camera::new(projection, view));
    let geometry = stage.append_array(&[
        Vertex::default()
            .with_position([0.0, 0.0, 0.0])
            .with_color([0.0, 1.0, 1.0, 1.0]),
        Vertex::default()
            .with_position([0.0, 100.0, 0.0])
            .with_color([1.0, 1.0, 0.0, 1.0]),
        Vertex::default()
            .with_position([100.0, 0.0, 0.0])
            .with_color([1.0, 0.0, 1.0, 1.0]),
    ]);
    let _tri_id = stage.draw(Renderlet {
        camera,
        vertices: geometry,
        ..Default::default()
    });
    (ctx, stage)
}
