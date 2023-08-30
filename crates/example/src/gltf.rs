//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use instant::Instant;

use renderling::{
    debug::DebugChannel,
    math::{Mat4, Vec3, Vec4},
    GltfLoader, GpuEntity, RenderGraphConfig, Renderling, Scene, SceneImage, ScreenSize,
    TweenProperty, UiDrawObjects, UiMode, UiVertex, ViewMut,
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

pub enum SupportedFileType {
    Gltf,
    Hdr,
}

pub fn is_file_supported(file: impl AsRef<std::path::Path>) -> Option<SupportedFileType> {
    log::info!("loading '{}'", file.as_ref().display());
    let ext = file.as_ref().extension()?;
    Some(match ext.to_str()? {
        "hdr" => SupportedFileType::Hdr,
        _ => SupportedFileType::Gltf,
    })
}

struct App {
    loader: Option<GltfLoader>,
    entities: Vec<GpuEntity>,
    last_frame_instant: Instant,
    default_skybox_image_bytes: Vec<u8>,
    loads: Arc<Mutex<HashMap<std::path::PathBuf, Vec<u8>>>>,

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
    async fn new(r: &mut Renderling) -> Self {
        let default_skybox_image_bytes = loading_bytes::load("img/hdr/night.hdr").await.unwrap();
        r.set_background_color(DARK_BLUE_BG_COLOR);
        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;
        let mut gpui = renderling_gpui::Gpui::new_from(r);
        let ui = Ui::new(&mut gpui, "fonts").await;
        let ui_texture = r.texture_from_wgpu_tex(gpui.get_frame_texture(), None);
        let ui_scene = r.new_ui_scene().with_canvas_size(1, 1).build();
        let ui_obj = ui_scene
            .new_object()
            .with_draw_mode(UiMode::DEFAULT)
            .with_texture(&ui_texture)
            .with_vertices(
                renderling::math::POINTS_2D_TEX_QUAD
                    .into_iter()
                    .map(|v2| UiVertex::default().with_position(v2).with_uv(v2)),
            )
            .build();

        // Create a placeholder scene
        let scene = r.new_scene().build().unwrap();
        // r.setup_render_graph(Some(scene), Some(ui_scene), [ui_obj], false);
        r.setup_render_graph(RenderGraphConfig {
            scene: Some(scene),
            ui: Some(ui_scene),
            objs: vec![ui_obj],
            ..Default::default()
        });

        let mut app = Self {
            loader: None,
            entities: vec![],
            default_skybox_image_bytes,
            loads: Arc::new(Mutex::new(HashMap::default())),
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
        // convert from spherical to cartesian
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

    fn load_hdr_skybox(&mut self, r: &mut Renderling, bytes: Vec<u8>) {
        let img = SceneImage::from_hdr_bytes(&bytes).unwrap();
        let scene = r.graph.get_resource_mut::<Scene>().unwrap().unwrap();
        scene.set_skybox_img(Some(img));
        self.default_skybox_image_bytes = bytes;
    }

    fn load_gltf_model(
        &mut self,
        r: &mut Renderling,
        path: impl AsRef<std::path::Path>,
        bytes: Vec<u8>,
    ) {
        self.phi = 0.0;
        self.theta = std::f32::consts::FRAC_PI_4;
        self.left_mb_down = false;
        self.last_cursor_position = None;

        let mut builder = r
            .new_scene()
            .with_skybox_image_from_bytes(&self.default_skybox_image_bytes)
            .with_debug_channel(DebugChannel::None);
        // let cross_loader =
        // builder.gltf_load("/Users/schell/code/renderling/gltf/origin_cross.glb").
        // unwrap();
        let loader = match builder.gltf_load_bytes(bytes) {
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
        for node in loader.nodes.iter() {
            let entity = self.entities.get(node.entity_id.index()).unwrap();
            let (translation, rotation, scale) = entity.get_world_transform(&self.entities);
            let tfrm = Mat4::from_scale_rotation_translation(scale, rotation, translation);
            if let Some(mesh_index) = node.gltf_mesh_index {
                for primitive in loader.meshes.get(mesh_index).unwrap().iter() {
                    let bbmin = tfrm.transform_point3(primitive.bounding_box.min);
                    let bbmax = tfrm.transform_point3(primitive.bounding_box.max);
                    min = min.min(bbmin);
                    max = max.max(bbmax);
                }
            }
        }

        let halfway_point = min + ((max - min).normalize() * ((max - min).length() / 2.0));
        let length = min.distance(max);
        let radius = length * 1.25;

        let name = get_name(path);
        self.ui.set_text_title(format!("{name}"));

        self.radius = radius;
        self.eye = halfway_point;
        self.last_frame_instant = Instant::now();
        self.loader = Some(loader);

        r.graph.add_resource(builder.build().unwrap());

        self.update_camera_view(r);
        self.ui.set_text_camera(self.get_updated_camera_text());
    }

    fn tick_loads(&mut self, r: &mut Renderling) {
        let loaded = std::mem::take(&mut *self.loads.lock().unwrap());
        for (path, bytes) in loaded.into_iter() {
            match is_file_supported(&path) {
                Some(SupportedFileType::Gltf) => self.load_gltf_model(r, path, bytes),
                Some(SupportedFileType::Hdr) => self.load_hdr_skybox(r, bytes),
                None => {}
            }
        }
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

    /// Queues a load operation.
    fn load(&mut self, path: &str) {
        let path = std::path::PathBuf::from(path);
        let loads = self.loads.clone();

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let path_str = format!("{}", path.display());
                let bytes = loading_bytes::load(&path_str).await.unwrap();
                let mut loads = loads.lock().unwrap();
                loads.insert(path, bytes);
                log::debug!("loaded {path_str}");
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::thread::spawn(move || {
                let bytes = std::fs::read(&path).unwrap();
                let mut loads = loads.lock().unwrap();
                loads.insert(path, bytes);
            });
        }
    }

    fn resize(&mut self, r: &mut Renderling, width: u32, height: u32) {
        let ui_obj_tex = self.gpui.resize(width, height);
        r.resize(width, height);
        r.graph
            .visit(
                |(mut scene, mut ui_objs): (ViewMut<Scene>, ViewMut<UiDrawObjects>)| {
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
                    let ent = self.entities.get_mut(id.index()).unwrap();
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
                        .visit(|mut scene: ViewMut<Scene>| {
                            scene.update_entity(*ent).unwrap();
                        })
                        .unwrap();
                }
                animation.stored_timestamp = time;
                break;
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
pub async fn demo(
    r: &mut Renderling,
    model: Option<impl AsRef<str>>,
    skybox: Option<impl AsRef<str>>,
) -> impl FnMut(&mut Renderling, Option<&winit::event::WindowEvent>) {
    let mut app = App::new(r).await;
    if let Some(file) = model {
        app.load(file.as_ref());
    }
    if let Some(file) = skybox {
        app.load(file.as_ref());
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
                    log::trace!("got dropped file event: {}", path.display());
                    let path = format!("{}", path.display());
                    app.load(&path);
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
            app.tick_loads(r);
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
        let mut ui = futures_lite::future::block_on(Ui::new(&mut gpui, "../../fonts"));
        ui.set_text_title("This is the title text");
        ui.set_text_camera("This is the camera text");
        gpui.layout(&mut ui);
        let _ = gpui.render_image_srgb(&mut ui);
        gpui.layout(&mut ui);
        let img = gpui.render_image_srgb(&mut ui);
        img_diff::save("example_gltf_viewer_ui_sanity.png", img);
    }
}
