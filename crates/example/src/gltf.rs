//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
//!
//! This demo requires an internet connection to download the samples.
use std::time::Instant;

use renderling::{
    math::{Mat4, Vec3, Vec4},
    FontArc, GltfLoader, GlyphCache, GpuEntity, Id, Renderling, Scene, Section, Text,
    TweenTransform, UiDrawObjects, UiMode, UiScene, Write,
};
use rustc_hash::FxHashMap;
use winit::event::KeyboardInput;

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

struct App {
    loader: Option<GltfLoader>,
    entities: Vec<GpuEntity>,
    last_frame_instant: Instant,
    timestamp: f32,
    title_text_cache: GlyphCache,
    camera_text_cache: GlyphCache,

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
    fn new(r: &mut Renderling) -> Self {
        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;
        let (ww, wh) = r.get_screen_size();

        // get the font for the UI
        let bytes: Vec<u8> =
            std::fs::read("fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();
        let font = FontArc::try_from_vec(bytes).unwrap();
        let title_text_cache = r.new_glyph_cache(vec![font.clone()]);
        let camera_text_cache = r.new_glyph_cache(vec![font]);

        let builder = r.new_ui_scene().with_canvas_size(ww, wh);
        let title_text = builder.new_object().with_draw_mode(UiMode::TEXT).build();
        let camera_text = builder
            .new_object()
            .with_draw_mode(UiMode::TEXT)
            .with_position([0.0, 46.0])
            .build();
        let ui_scene = builder.build();

        // Create a placeholder scene
        let scene = r.new_scene().build().unwrap();

        renderling::setup_ui_and_scene_render_graph(
            r,
            ui_scene,
            [title_text, camera_text],
            scene,
            false,
        );

        let mut app = Self {
            timestamp: 0.0,
            loader: None,
            entities: vec![],
            title_text_cache,
            camera_text_cache,

            last_frame_instant: Instant::now(),
            radius,
            eye: Vec3::ZERO,
            phi,
            theta,
            left_mb_down,
            last_cursor_position,
        };
        app.update_title_text(r, "Drag and drop a GLTF file to view...");
        app.update_camera_text(r);
        app
    }

    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    fn update_title_text(&mut self, r: &mut Renderling, text: &str) {
        self.title_text_cache.queue(
            Section::default().add_text(
                Text::new(text)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(46.0),
            ),
        );
        let (may_mesh, may_texture) = self.title_text_cache.get_updated();
        let objects = r
            .graph
            .get_resource_mut::<UiDrawObjects>()
            .unwrap()
            .unwrap();
        let title_text = objects.get_mut(0).unwrap();
        if let Some(mesh) = may_mesh {
            title_text.set_vertices(mesh);
        }
        if let Some(texture) = may_texture {
            title_text.set_texture(&texture);
        }
    }

    fn update_camera_text(&mut self, r: &mut Renderling) {
        self.camera_text_cache.queue(
            Section::default().add_text(
                Text::new(&format!(
                    "radius: {}\nlooking at: {}\ntheta: {}\nphi: {}",
                    self.radius, self.eye, self.theta, self.phi
                ))
                .with_color([0.8, 0.8, 0.8, 1.0])
                .with_scale(32.0),
            ),
        );
        let (may_mesh, may_texture) = self.camera_text_cache.get_updated();
        let objects = r
            .graph
            .get_resource_mut::<UiDrawObjects>()
            .unwrap()
            .unwrap();
        let camera_text = objects.get_mut(1).unwrap();
        if let Some(mesh) = may_mesh {
            camera_text.set_vertices(mesh);
        }
        if let Some(texture) = may_texture {
            camera_text.set_texture(&texture);
        }
    }

    fn update_camera_view(&self, r: &mut Renderling) {
        let (w, h) = r.get_screen_size();
        let scene = r.graph.get_resource_mut::<Scene>().unwrap().unwrap();
        scene.set_camera(
            Mat4::perspective_infinite_rh(std::f32::consts::FRAC_PI_4, w as f32 / h as f32, 0.01),
            Mat4::look_at_rh(
                Self::camera_position(self.radius, self.phi, self.theta),
                self.eye,
                Vec3::Y,
            ),
        );
    }

    fn load(&mut self, r: &mut Renderling, file: impl AsRef<std::path::Path>) {
        log::info!("loading '{}'", file.as_ref().display());
        self.phi = 0.0;
        self.theta = std::f32::consts::FRAC_PI_4;
        self.left_mb_down = false;
        self.last_cursor_position = None;

        let mut builder = r.new_scene();
        let loader = builder.gltf_load(&file).unwrap();
        self.entities = builder.entities.clone();
        // find the bounding box of the model so we can display it correctly
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for primitive in loader.meshes.iter().flat_map(|meshes| meshes.iter()) {
            min = min.min(primitive.bounding_box.min);
            max = max.max(primitive.bounding_box.max);
        }

        let halfway_point = min + ((max - min).normalize() * ((max - min).length() / 2.0));
        let length = min.distance(max);
        let radius = length * 1.25;

        if loader.lights.len() == 0 {
            let _ = builder
                .new_directional_light()
                .with_diffuse_color(Vec4::ONE)
                .with_direction(Vec3::new(0.0, -1.0, 1.0))
                .build();
        }

        let name = get_name(file);
        self.update_title_text(r, &format!("{name}"));

        self.radius = radius;
        self.eye = halfway_point;
        self.last_frame_instant = Instant::now();
        self.timestamp = 0.0;
        self.loader = Some(loader);

        r.graph.add_resource(builder.build().unwrap());

        self.update_camera_view(r);
        self.update_camera_text(r);
        self.animate(r);
    }

    fn zoom(&mut self, r: &mut Renderling, delta: f32) {
        self.radius = (self.radius - (delta * RADIUS_SCROLL_DAMPENING)).max(0.0);
        self.update_camera_view(r);
        self.update_camera_text(r);
    }

    fn pan(&mut self, r: &mut Renderling, position: winit::dpi::PhysicalPosition<f64>) {
        if self.left_mb_down {
            if let Some(last_cursor_position) = self.last_cursor_position.as_ref() {
                let dx = position.x - last_cursor_position.x;
                let dy = position.y - last_cursor_position.y;

                self.phi += dx as f32 * DX_DY_DRAG_DAMPENING;

                let next_theta = self.theta - dy as f32 * DX_DY_DRAG_DAMPENING;
                self.theta = next_theta.max(0.0001).min(std::f32::consts::PI);

                self.update_camera_text(r);
            }
            self.last_cursor_position = Some(position);
            self.update_camera_view(r);
            self.update_camera_text(r);
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
        r: &mut Renderling,
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
                let scene = r.new_scene().build().unwrap();
                r.graph.add_resource(scene);
                self.loader = None;
                self.update_title_text(r, "awaiting drag and dropped `.gltf` or `.glb` file");
            }
            _ => {}
        };
    }

    fn resize(&mut self, r: &mut Renderling, width: u32, height: u32) {
        r.resize(width, height);
        r.graph
            .visit(
                |(mut ui_scene, mut scene): (Write<UiScene>, Write<Scene>)| {
                    ui_scene.set_canvas_size(width, height);
                    scene.set_camera_projection(Mat4::perspective_infinite_rh(
                        std::f32::consts::FRAC_PI_4,
                        width as f32 / height as f32,
                        0.01,
                    ));
                },
            )
            .unwrap();
    }

    fn animate(&mut self, r: &mut Renderling) {
        let now = Instant::now();
        let dt = now - self.last_frame_instant;
        self.last_frame_instant = now;
        self.timestamp += dt.as_secs_f32();

        if let Some(loader) = self.loader.as_ref() {
            for animation in loader.animations.iter() {
                let time = self.timestamp % animation.length_in_seconds();
                for (id, tfrm) in animation.get_properties_at_time(loader, time).unwrap() {
                    let mut ent = self.entities.get_mut(id.index()).unwrap();
                    ent.position = tfrm.translate.extend(0.0);
                    ent.scale = tfrm.scale.extend(0.0);
                    ent.rotation = tfrm.rotate;
                    r.graph
                        .visit(|mut scene: Write<Scene>| {
                            scene.update_entity(*ent).unwrap();
                        })
                        .unwrap();
                }
            }
        }
    }
}

fn get_name(path: impl AsRef<std::path::Path>) -> String {
    let file = path.as_ref().file_name();
    let name = file.map(|s| s.to_str()).flatten().unwrap_or("unknown");
    name.to_string()
}

/// Sets up the demo for a given model
pub fn demo(
    r: &mut Renderling,
    start: Option<impl AsRef<str>>,
) -> impl FnMut(&mut Renderling, Option<&winit::event::WindowEvent>) {
    let mut app = App::new(r);

    if let Some(file) = start {
        app.load(r, file.as_ref());
    }

    move |r, ev: Option<&winit::event::WindowEvent>| {
        if let Some(ev) = ev {
            match ev {
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    let delta = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, up) => *up,
                        winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                    };

                    app.zoom(r, delta);
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    app.pan(r, *position);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_button(*state, *button);
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    app.key_input(r, *input);
                }
                winit::event::WindowEvent::Resized(size) => {
                    app.resize(r, size.width, size.height);
                }
                winit::event::WindowEvent::DroppedFile(path) => {
                    app.load(r, path);
                }
                _ => {}
            }
        } else {
            app.update_camera_view(r);
            app.animate(r);
            r.render().unwrap();
        }
    }
}
