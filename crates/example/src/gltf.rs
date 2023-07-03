//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
//!
//! This demo requires an internet connection to download the samples.
use std::time::Instant;

use renderling::{
    debug::DebugChannel,
    math::{Mat4, Vec3, Vec4},
    GltfLoader, GpuEntity, Renderling, Scene, ScreenSize, TweenProperty, UiDrawObjects, UiMode,
    UiVertex, Write,
};
use renderling_gpui::{Element, Gpui};
use winit::event::KeyboardInput;

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

mod ui;
use ui::Ui;

use self::ui::UiEvent;

const DARK_BLUE_BG_COLOR: Vec4 = Vec4::new(
    0x30 as f32 / 255.0,
    0x35 as f32 / 255.0,
    0x42 as f32 / 255.0,
    1.0,
);

struct App {
    loader: Option<GltfLoader>,
    entities: Vec<GpuEntity>,
    last_frame_instant: Instant,

    ui: Ui,
    gpui: Gpui,

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
        r.set_background_color(DARK_BLUE_BG_COLOR);
        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;
        let mut gpui = renderling_gpui::Gpui::new_from(r);
        let ui = Ui::new(&mut gpui, "fonts");
        let ui_texture = r.texture_from_wgpu_tex(gpui.get_frame_texture(), None);
        let ui_scene = r.new_ui_scene().with_canvas_size(1, 1).build();
        let ui_obj = ui_scene
            .new_object()
            .with_draw_mode(UiMode::DEFAULT)
            .with_texture(&ui_texture)
            .with_vertices(
                renderling::math::POINTS_2D_TEX_QUAD
                    .into_iter()
                    .map(|v2| UiVertex::default().with_position(*v2).with_uv(*v2)),
            )
            .build();

        // Create a placeholder scene
        let scene = r.new_scene().build().unwrap();

        renderling::setup_ui_and_scene_render_graph(r, ui_scene, [ui_obj], scene, false);

        let mut app = Self {
            loader: None,
            entities: vec![],
            ui,
            gpui,
            last_frame_instant: Instant::now(),
            radius,
            eye: Vec3::ZERO,
            phi,
            theta,
            left_mb_down,
            last_cursor_position,
        };
        app.ui
            .set_text_title("Drag and drop a GLTF file to view...");
        app.ui.set_text_camera(app.get_updated_camera_text());
        app
    }

    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    fn get_updated_camera_text(&self) -> String {
        format!(
            "position: {}\nlooking at: {}",
            Self::camera_position(self.radius, self.phi, self.theta),
            self.eye,
        )
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

        let mut builder = r.new_scene().with_debug_channel(DebugChannel::None);
        let loader = match builder.gltf_load(&file) {
            Ok(loader) => loader,
            Err(msg) => {
                self.ui.set_text_title(format!("Error: {}", msg));
                return;
            }
        };

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
            // These values were taken from the Khronos gltf-Sample-Viewer, which is a bit
            // of a reference implementation as far as GLTF viewers go.
            // See https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/renderer.js#L72
            let _key_light = builder
                .new_directional_light()
                .with_color(Vec4::ONE)
                .with_direction(Vec3::new(0.5, -0.7071, -0.5))
                .with_intensity(1.0)
                .build();
            let _fill_light = builder
                .new_directional_light()
                .with_color(Vec4::ONE)
                .with_direction(Vec3::new(-0.5, 0.7071, 0.5))
                .with_intensity(0.5)
                .build();
        }

        let name = get_name(file);
        self.ui.set_text_title(format!("{name}"));

        self.radius = radius;
        self.eye = halfway_point;
        self.last_frame_instant = Instant::now();
        self.loader = Some(loader);

        r.graph.add_resource(builder.build().unwrap());

        self.update_camera_view(r);
        self.ui.set_text_camera(self.get_updated_camera_text());
    }

    fn zoom(&mut self, r: &mut Renderling, delta: f32) {
        self.radius = (self.radius - (delta * RADIUS_SCROLL_DAMPENING)).max(0.0);
        self.update_camera_view(r);
        self.ui.set_text_camera(self.get_updated_camera_text());
    }

    fn pan(&mut self, r: &mut Renderling, position: winit::dpi::PhysicalPosition<f64>) {
        if self.left_mb_down {
            if let Some(last_cursor_position) = self.last_cursor_position.as_ref() {
                let dx = position.x - last_cursor_position.x;
                let dy = position.y - last_cursor_position.y;

                self.phi += dx as f32 * DX_DY_DRAG_DAMPENING;

                let next_theta = self.theta - dy as f32 * DX_DY_DRAG_DAMPENING;
                self.theta = next_theta.max(0.0001).min(std::f32::consts::PI);
            }
            self.last_cursor_position = Some(position);
            self.update_camera_view(r);
            self.ui.set_text_camera(self.get_updated_camera_text());
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
                self.ui
                    .set_text_title("awaiting drag and dropped `.gltf` or `.glb` file");
            }
            _ => {}
        };
    }

    fn resize(&mut self, r: &mut Renderling, width: u32, height: u32) {
        let ui_obj_tex = self.gpui.resize(width, height);
        r.resize(width, height);
        r.graph
            .visit(
                |(mut scene, mut ui_objs): (Write<Scene>, Write<UiDrawObjects>)| {
                    scene.set_camera_projection(Mat4::perspective_infinite_rh(
                        std::f32::consts::FRAC_PI_4,
                        width as f32 / height as f32,
                        0.01,
                    ));
                    // UNWRAP: safe because we always have one object to represent the UI "surface".
                    // Otherwise the app is useless.
                    let ui_obj = ui_objs.get_mut(0).unwrap();
                    ui_obj.set_texture(&ui_obj_tex);
                },
            )
            .unwrap();
    }

    fn animate(&mut self, r: &mut Renderling) {
        let now = Instant::now();
        let dt = now - self.last_frame_instant;
        self.last_frame_instant = now;
        let dt_secs = dt.as_secs_f32();
        if let Some(loader) = self.loader.as_mut() {
            for (index, animation) in loader.animations.iter_mut().enumerate() {
                let animation_len = animation.length_in_seconds();
                animation.stored_timestamp += dt_secs;
                if animation.stored_timestamp > animation_len {
                    log::trace!("animation {index} {:?} has looped", animation.name);
                }
                let time = animation.stored_timestamp % animation.length_in_seconds();
                for (id, tween_prop) in animation.get_properties_at_time(time).unwrap() {
                    let mut ent = self.entities.get_mut(id.index()).unwrap();
                    match tween_prop {
                        TweenProperty::Translation(t) => {
                            ent.position = t.extend(ent.position.w);
                        }
                        TweenProperty::Rotation(r) => {
                            ent.rotation = r;
                        }
                        TweenProperty::Scale(s) => {
                            if s == Vec3::ZERO {
                                log::trace!("scale is zero at time: {time:?}");
                                panic!("animation: {animation:#?}");
                            }
                            ent.scale = s.extend(ent.scale.w);
                        }
                        TweenProperty::MorphTargetWeights(ws) => {
                            ent.set_morph_target_weights(ws);
                        }
                    }
                    r.graph
                        .visit(|mut scene: Write<Scene>| {
                            scene.update_entity(*ent).unwrap();
                        })
                        .unwrap();
                }
                animation.stored_timestamp = time;
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

    let mut event_state = renderling_gpui::EventState::default();
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
                    log::trace!("resizing to {size:?}");
                    app.resize(r, size.width, size.height);
                    let _ = app
                        .gpui
                        .0
                        .graph
                        .get_resource::<ScreenSize>()
                        .unwrap()
                        .unwrap();
                }
                winit::event::WindowEvent::DroppedFile(path) => {
                    app.load(r, path);
                }
                _ => {}
            }

            if let Some(ev) = event_state.event_from_winit(ev) {
                let scene = r.graph.get_resource_mut::<Scene>().unwrap().unwrap();
                let channel = scene.get_debug_channel();
                let mut set_debug_channel = |mode| {
                    log::debug!("setting debug mode to {mode:?}");
                    if channel != mode {
                        scene.set_debug_channel(mode);
                    } else {
                        scene.set_debug_channel(DebugChannel::None);
                    }
                };

                match app.ui.event(ev) {
                    None => {}
                    Some(ev) => match ev {
                        UiEvent::ToggleDebugChannel(channel) => set_debug_channel(channel),
                    },
                }
            }
        } else {
            app.update_camera_view(r);
            app.animate(r);
            app.gpui.layout(&mut app.ui);
            app.gpui.render(&mut app.ui);
            r.render().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity() {
        let mut gpui = renderling_gpui::Gpui::new(600, 300); //.with_background_color(DARK_BLUE_BG_COLOR);
        let mut ui = Ui::new(&mut gpui, "../../fonts");
        ui.set_text_title("This is the title text");
        ui.set_text_camera("This is the camera text");
        gpui.layout(&mut ui);
        let _ = gpui.render_image_srgb(&mut ui);
        gpui.layout(&mut ui);
        let img = gpui.render_image_srgb(&mut ui);
        img_diff::save("example_gltf_viewer_ui_sanity.png", img);
    }
}
