//! Runs through all the gltf sample models to test and show-off renderling's gltf capabilities.
//!
//! This demo requires an internet connection to download the samples.
use renderling::{
    math::{Mat4, Vec3},
    Camera, FontArc, ForwardPipeline, GltfLoader, GlyphCache, Object, Renderling, Section, Text,
    UiPipeline, WgpuState,
};
use winit::event::KeyboardInput;

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

const MODELS: [(&str, &str); 3] = [
    // standard
    (
        "Box",
        "https://github.com/KhronosGroup/glTF-Sample-Models/raw/master/2.0/Box/glTF-Binary/Box.glb",
    ),
    (
        "Box Interleaved",
        "https://github.com/KhronosGroup/glTF-Sample-Models/raw/master/2.0/BoxInterleaved/glTF-Binary/BoxInterleaved.glb"
    ),
    (
        "Box Textured",
        "https://github.com/KhronosGroup/glTF-Sample-Models/raw/master/2.0/BoxTextured/glTF-Binary/BoxTextured.glb"
    ),
];

struct App {
    index: usize,

    ui: Renderling<UiPipeline>,
    text: Object,
    ui_camera: Camera,
    cache: GlyphCache,

    forward: Renderling<ForwardPipeline>,
    forward_camera: Camera,

    loader: GltfLoader,

    // distance from the origin
    radius: f32,
    // anglular position on a circle `radius` away from the origin on x,z
    phi: f32,
    // angular distance from y axis
    theta: f32,
    // is the left mouse button down
    left_mb_down: bool,
    // cursor position
    last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
}

impl App {
    fn new(gpu: &mut WgpuState, starting_index: usize) -> Self {
        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;
        let window_size = gpu.get_size();
        let mut ui: Renderling<UiPipeline> = gpu.new_ui_renderling();
        let ui_camera = ui.new_camera().with_projection_ortho2d().build();
        let text = ui.new_object().build().unwrap();
        // get the font for the UI
        let bytes: Vec<u8> = std::fs::read("fonts/Font Awesome 6 Free-Regular-400.otf").unwrap();

        let font = FontArc::try_from_vec(bytes).unwrap();
        let cache = gpu.new_glyph_cache(vec![font]);

        let mut forward = gpu.new_forward_renderling_with(Some(wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
            unclipped_depth: false,
        }));
        let forward_camera = forward
            .new_camera()
            .with_projection_perspective()
            .with_view(Mat4::look_at_rh(
                Self::camera_position(radius, phi, theta),
                Vec3::ZERO,
                Vec3::Y,
            ))
            .build();
        let loader = gpu.new_gltf_loader();

        let mut app = Self {
            index: starting_index,
            ui,
            text,
            ui_camera,
            cache,
            forward,
            forward_camera,
            loader,
            radius,
            phi,
            theta,
            left_mb_down,
            last_cursor_position,
        };
        let (name, url) = MODELS[app.index];
        app.load(url);
        app.update_ui(name);
        app
    }

    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    fn update_ui(&mut self, name: &str) {
        self.cache.queue(
            Section::default().add_text(
                Text::new(name)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(64.0),
            ),
        );

        let (material, mesh) = self.cache.get_updated();

        if let Some(material) = material {
            self.text.set_material(material);
        }
        if let Some(mesh) = mesh {
            self.text.set_mesh(mesh);
        }
    }

    fn load(&mut self, url: &str) {
        self.loader.unload();

        self.radius = 6.0;
        self.phi = 0.0;
        self.theta = std::f32::consts::FRAC_PI_4;
        self.left_mb_down = false;
        self.last_cursor_position = None;

        let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        let (document, buffers, images) = gltf::import_slice(bytes).unwrap();
        self.loader
            .load_scene(None, &mut self.forward, &document, &buffers, &images)
            .unwrap();
    }

    fn zoom(&mut self, delta: f32) {
        self.radius -= delta * RADIUS_SCROLL_DAMPENING;
        self.forward_camera.set_view(Mat4::look_at_rh(
            Self::camera_position(self.radius, self.phi, self.theta),
            Vec3::ZERO,
            Vec3::Y,
        ));
    }

    fn pan(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        if self.left_mb_down {
            if let Some(last_cursor_position) = self.last_cursor_position.as_ref() {
                let dx = position.x - last_cursor_position.x;
                let dy = position.y - last_cursor_position.y;

                self.phi += dx as f32 * DX_DY_DRAG_DAMPENING;

                let next_theta = self.theta - dy as f32 * DX_DY_DRAG_DAMPENING;
                self.theta = next_theta.max(0.0001).min(std::f32::consts::PI);

                self.forward_camera.set_view(Mat4::look_at_rh(
                    Self::camera_position(self.radius, self.phi, self.theta),
                    Vec3::ZERO,
                    Vec3::Y,
                ));
            }
            self.last_cursor_position = Some(position);
        }
    }

    fn mouse_button(
        &mut self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) {
        if matches!(button, winit::event::MouseButton::Left) {
            self.left_mb_down = matches!(state, winit::event::ElementState::Pressed);
            if !self.left_mb_down {
                self.last_cursor_position = None;
            }
        }
    }

    fn key_input(
        &mut self,
        gpu: &mut WgpuState,
        KeyboardInput {
            state,
            virtual_keycode,
            ..
        }: winit::event::KeyboardInput,
    ) {
        if matches!(state, winit::event::ElementState::Pressed) {
            return;
        }
        let should_load = match virtual_keycode {
            Some(winit::event::VirtualKeyCode::Left) => {
                self.index = (self.index + MODELS.len() - 1) % MODELS.len();
                true
            }
            Some(winit::event::VirtualKeyCode::Right) => {
                self.index = (self.index + 1) % MODELS.len();
                true
            }
            _ => false,
        };
        if should_load {
            let (name, url) = MODELS[self.index];
            self.update_ui(name);
            self.loader.unload();
            self.forward.update().unwrap();
            self.render(gpu);

            self.load(url);
        }
    }

    fn resize(&mut self, gpu: &mut WgpuState, width: u32, height: u32) {
        gpu.resize((width, height));
        self.ui_camera.set_projection(Mat4::orthographic_rh(
            0.0,
            width as f32,
            height as f32,
            0.0,
            1.0,
            -1.0,
        ));
        self.forward_camera.set_projection(Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_4,
            width as f32 / height as f32,
            0.1,
            100.0,
        ));
    }

    fn render(&mut self, gpu: &mut WgpuState) {
        let (frame, depth) = gpu.next_frame_cleared().unwrap();

        self.forward.update().unwrap();
        self.forward.render(&frame, &depth).unwrap();

        // just clear the depth texture, because we want to render the 2d UI over the 3d background
        gpu.clear(None, Some(&depth));
        self.ui.update().unwrap();
        self.ui.render(&frame, &depth).unwrap();

        gpu.present().unwrap();
    }
}

/// Sets up the demo for a given model
pub fn demo(
    gpu: &mut WgpuState,
    start: Option<impl AsRef<str>>,
) -> impl FnMut(&mut WgpuState, Option<&winit::event::WindowEvent>) {
    // find which model we're going to view
    let index = start
        .map(|input_name| {
            for (i, (name, _)) in MODELS.iter().enumerate() {
                if input_name.as_ref().to_lowercase() == name.to_lowercase() {
                    return Some(i);
                }
            }
            None
        })
        .flatten()
        .unwrap_or_else(|| 0);

    let mut app = App::new(gpu, index);

    move |gpu, ev: Option<&winit::event::WindowEvent>| {
        if let Some(ev) = ev {
            match ev {
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    let delta = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, up) => *up,
                        winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                    };

                    app.zoom(delta);
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    app.pan(*position);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_button(*state, *button);
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    app.key_input(gpu, *input);
                }
                winit::event::WindowEvent::Resized(size) => {
                    app.resize(gpu, size.width, size.height);
                }
                _ => {}
            }
        } else {
            app.render(gpu);
        }
    }
}
