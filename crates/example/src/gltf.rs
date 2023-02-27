//! Runs through all the gltf sample models to test and show-off renderling's gltf capabilities.
//!
//! This demo requires an internet connection to download the samples.
use core::f32;
use std::time::Instant;

use renderling::{
    math::{Mat4, Vec3},
    Camera, FontArc, ForwardPipeline, GltfLoader, GlyphCache, Object, Renderling, Section, Text,
    UiPipeline, WgpuState,
};
use winit::event::KeyboardInput;

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

#[derive(Default)]
struct Ui {
    title_text: String,
}

struct App {
    renderling_ui: Renderling<UiPipeline>,
    renderling_forward: Renderling<ForwardPipeline>,

    ui: Ui,

    text_title: Object,
    text_camera: Object,
    ui_camera: Camera,
    cache: GlyphCache,

    forward_camera: Camera,

    loader: GltfLoader,
    last_frame_instant: Instant,
    timestamp: f32,

    // look at
    eye: Vec3,
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
    fn new(gpu: &mut WgpuState) -> Self {
        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;
        let window_size = gpu.get_size();
        let mut ui: Renderling<UiPipeline> = gpu.new_ui_renderling();
        let ui_camera = ui.new_camera().with_projection_ortho2d().build();
        let text = ui.new_object().build().unwrap();
        let text_camera = ui.new_object().build().unwrap();
        // get the font for the UI
        let bytes: Vec<u8> =
            std::fs::read("fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();

        let font = FontArc::try_from_vec(bytes).unwrap();
        let cache = gpu.new_glyph_cache(vec![font]);

        let mut forward = gpu.new_forward_renderling_with(Some(wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, //Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
            unclipped_depth: false,
        }));
        let forward_camera = forward
            .new_camera()
            .with_projection(Mat4::perspective_infinite_rh(
                std::f32::consts::FRAC_PI_4,
                window_size.0 as f32 / window_size.1 as f32,
                0.01,
            ))
            .with_view(Mat4::look_at_rh(
                Self::camera_position(radius, phi, theta),
                Vec3::ZERO,
                Vec3::Y,
            ))
            .build();
        let loader = gpu.new_gltf_loader();

        let mut app = Self {
            timestamp: 0.0,
            renderling_ui: ui,
            text_title: text,
            text_camera,
            ui_camera,
            cache,
            renderling_forward: forward,
            forward_camera,
            loader,
            last_frame_instant: Instant::now(),
            radius,
            eye: Vec3::ZERO,
            phi,
            theta,
            left_mb_down,
            last_cursor_position,
            ui: Ui {
                title_text: "drag and drop a `.gltf` or `.glb` file to load".to_string(),
            },
        };
        app.update_ui();
        app
    }

    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    fn update_ui(&mut self) {
        self.cache.queue(
            Section::default()
                .add_text(
                    Text::new(&self.ui.title_text)
                        .with_color([1.0, 1.0, 1.0, 1.0])
                        .with_scale(46.0),
                )
                .add_text(Text::new("\n"))
                .add_text(
                    Text::new(&format!("distance: {}, center: {}", self.radius, self.eye))
                        .with_color([0.8, 0.8, 0.8, 1.0])
                        .with_scale(32.0),
                ),
        );

        let (material, mesh) = self.cache.get_updated();

        if let Some(material) = material {
            self.text_title.set_material(material);
        }
        if let Some(mesh) = mesh {
            self.text_title.set_mesh(mesh);
        }
    }

    fn update_camera(&mut self) {
        self.forward_camera.set_view(Mat4::look_at_rh(
            Self::camera_position(self.radius, self.phi, self.theta),
            self.eye,
            Vec3::Y,
        ));
    }

    fn update(&mut self) {
        self.animate();
        self.update_camera();
        self.update_ui();
    }

    fn load(&mut self, file: impl AsRef<std::path::Path>) {
        self.loader.unload();

        self.phi = 0.0;
        self.theta = std::f32::consts::FRAC_PI_4;
        self.left_mb_down = false;
        self.last_cursor_position = None;

        let (document, buffers, images) = gltf::import(&file).unwrap();
        self.loader
            .load(&mut self.renderling_forward, &document, &buffers, &images)
            .unwrap();

        if self.loader.lights().count() == 0 {
            let name = get_name(file);
            self.ui.title_text = format!("{name} (unlit)");
        }

        // find the bounding box of the model so we can display it correctly
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for node in document.nodes() {
            if let Some(mesh) = node.mesh() {
                for primitive in mesh.primitives() {
                    let aabb = primitive.bounding_box();
                    min.x = min.x.min(aabb.min[0]);
                    min.y = min.y.min(aabb.min[1]);
                    min.z = min.z.min(aabb.min[2]);
                    max.x = max.x.max(aabb.max[0]);
                    max.y = max.y.max(aabb.max[1]);
                    max.z = max.z.max(aabb.max[2]);
                }
            }
        }

        let halfway_point = min + ((max - min).normalize() * ((max - min).length() / 2.0));
        let length = min.distance(max);
        let radius = length * 1.25;

        self.radius = radius;
        self.eye = halfway_point;
        self.last_frame_instant = Instant::now();
        self.timestamp = 0.0;
        self.update();
    }

    fn zoom(&mut self, delta: f32) {
        self.radius = (self.radius - (delta * RADIUS_SCROLL_DAMPENING)).max(0.0);
        self.update();
    }

    fn pan(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        if self.left_mb_down {
            if let Some(last_cursor_position) = self.last_cursor_position.as_ref() {
                let dx = position.x - last_cursor_position.x;
                let dy = position.y - last_cursor_position.y;

                self.phi += dx as f32 * DX_DY_DRAG_DAMPENING;

                let next_theta = self.theta - dy as f32 * DX_DY_DRAG_DAMPENING;
                self.theta = next_theta.max(0.0001).min(std::f32::consts::PI);

                self.update();
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
        _: &mut WgpuState,
        KeyboardInput {
            state,
            virtual_keycode,
            ..
        }: winit::event::KeyboardInput,
    ) {
        if matches!(state, winit::event::ElementState::Pressed) {
            return;
        }
        match virtual_keycode {
            Some(winit::event::VirtualKeyCode::Space) => {
                // clear all objects, cameras and lights
                self.loader.unload();
                self.ui.title_text = "awaiting drag and dropped `.gltf` or `.glb` file".to_string();
                self.update();
            }
            _ => {}
        };
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
        self.forward_camera
            .set_projection(Mat4::perspective_infinite_rh(
                std::f32::consts::FRAC_PI_4,
                width as f32 / height as f32,
                0.01,
            ));
    }

    fn animate(&mut self) {
        let now = Instant::now();
        let dt = now - self.last_frame_instant;
        self.last_frame_instant = now;
        self.timestamp += dt.as_secs_f32();

        for animation in self.loader.animations() {
            let time = self.timestamp % animation.length_in_seconds();
            for tween in animation.tweens.iter() {
                let prop = if let Some(prop) = tween.interpolate(time).unwrap() {
                    prop
                } else if time >= tween.length_in_seconds() {
                    tween.get_last_keyframe_property().unwrap()
                } else {
                    tween.get_first_keyframe_property().unwrap()
                };
                let node = self.loader.get_node(tween.target_node_index).unwrap();
                node.set_tween_property(prop);
            };
        }
    }

    fn render(&mut self, gpu: &mut WgpuState) {
        let (frame, depth) = gpu.next_frame_cleared().unwrap();

        self.renderling_forward.update().unwrap();
        self.renderling_forward.render(&frame, &depth).unwrap();

        // just clear the depth texture, because we want to render the 2d UI over the 3d background
        gpu.clear(None, Some(&depth));
        self.renderling_ui.update().unwrap();
        self.renderling_ui.render(&frame, &depth).unwrap();

        gpu.present().unwrap();
    }
}

fn get_name(path: impl AsRef<std::path::Path>) -> String {
    let file = path.as_ref().file_name();
    let name = file.map(|s| s.to_str()).flatten().unwrap_or("unknown");
    name.to_string()
}

/// Sets up the demo for a given model
pub fn demo(
    gpu: &mut WgpuState,
    start: Option<impl AsRef<str>>,
) -> impl FnMut(&mut WgpuState, Option<&winit::event::WindowEvent>) {
    let mut app = App::new(gpu);

    if let Some(file) = start {
        app.load(file.as_ref());
    }

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
                winit::event::WindowEvent::DroppedFile(path) => {
                    app.ui.title_text = get_name(&path);
                    app.update_ui();
                    app.load(path);
                }
                _ => {}
            }
        } else {
            app.update();
            app.render(gpu);
        }
    }
}
