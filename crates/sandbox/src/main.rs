use crabslab::GrowableSlab;
use renderling::{
    math::{Vec2, Vec3, Vec4},
    pbr::Material,
    AtlasImage, Camera, Renderling, TextureAddressMode, Transform, Vertex,
};
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

    let mut r = atlas_uv_mapping(window.clone());
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
                        r.resize(size.width, size.height);
                    }

                    winit::event::WindowEvent::RedrawRequested => {
                        r.get_device().poll(wgpu::Maintain::Wait);
                    }
                    _ => {}
                },
                winit::event::Event::AboutToWait => {
                    r.render().unwrap();
                }
                _ => {}
            }
        })
        .unwrap()
}

fn atlas_uv_mapping(window: Arc<winit::window::Window>) -> Renderling {
    let mut r = Renderling::try_from_window(window)
        .unwrap()
        .with_background_color(Vec4::new(1.0, 0.0, 1.0, 1.0));
    let mut stage = r.new_stage();
    stage.configure_graph(&mut r, false);
    stage.set_debug_mode(renderling::pbr::debug::DebugMode::Brdf);
    let (projection, view) = renderling::default_ortho2d(32.0, 32.0);
    let camera = stage.append(&Camera {
        projection,
        view,
        ..Default::default()
    });
    let dirt = AtlasImage::from_path("img/dirt.jpg").unwrap_or_else(|e| panic!("{e}"));
    let sandstone = AtlasImage::from_path("img/sandstone.png").unwrap();
    let texels = AtlasImage::from_path("test_img/atlas/uv_mapping.png").unwrap();
    let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
    let mut texels_tex = textures[2];
    texels_tex.modes.s = TextureAddressMode::ClampToEdge;
    texels_tex.modes.t = TextureAddressMode::ClampToEdge;
    let texels_tex_id = stage.append(&texels_tex);
    let material_id = stage.append(&Material {
        albedo_texture: texels_tex_id,
        has_lighting: false,
        ..Default::default()
    });
    let mesh = stage
        .new_mesh()
        .with_primitive(
            {
                let tl = Vertex::default()
                    .with_position(Vec3::ZERO)
                    .with_uv0(Vec2::ZERO);
                let tr = Vertex::default()
                    .with_position(Vec3::new(1.0, 0.0, 0.0))
                    .with_uv0(Vec2::new(1.0, 0.0));
                let bl = Vertex::default()
                    .with_position(Vec3::new(0.0, 1.0, 0.0))
                    .with_uv0(Vec2::new(0.0, 1.0));
                let br = Vertex::default()
                    .with_position(Vec3::new(1.0, 1.0, 0.0))
                    .with_uv0(Vec2::splat(1.0));
                vec![tl, bl, br, tl, br, tr]
            },
            [],
            material_id,
        )
        .build();
    let mesh = stage.append(&mesh);
    let node = stage.append(&renderling::gltf::GltfNode {
        mesh,
        ..Default::default()
    });
    let transform = stage.append(&Transform {
        scale: Vec3::new(32.0, 32.0, 1.0),
        ..Default::default()
    });
    let node_path = stage.append_array(&[node]);
    let _unit = stage.draw_gltf_rendering(&renderling::gltf::GltfRendering {
        camera,
        transform,
        node_path,
        vertex_count: 6,
        ..Default::default()
    });

    //let img = r.render_image().unwrap();
    r
}
