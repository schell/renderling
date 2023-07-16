//! GPU user interface.
use snafu::prelude::*;
use std::sync::Arc;

use renderling::node::FrameTextureView;
use renderling::{Device, View, RenderTarget};
use renderling::{
    FontArc, Frame, GlyphCache, Id, OwnedSection, OwnedText, Queue, Renderling, UiDrawObject,
    UiDrawObjectBuilder, UiMode, UiScene, UiVertex, WgpuStateError, ViewMut,
};
use renderling::{UiRenderPipeline, UiSceneError};

pub use renderling::math::{UVec2, Vec2, Vec4};

mod elements;
pub use elements::*;

#[derive(Debug, Snafu)]
pub enum GpuiError {}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl From<Size> for Vec2 {
    fn from(value: Size) -> Self {
        Vec2::new(value.width as f32, value.height as f32)
    }
}

impl std::ops::Add<UVec2> for Size {
    type Output = Self;

    fn add(mut self, rhs: UVec2) -> Self::Output {
        self.width += rhs.x;
        self.height += rhs.y;
        self
    }
}

impl std::ops::Sub<UVec2> for Size {
    type Output = Self;

    fn sub(mut self, rhs: UVec2) -> Self::Output {
        self.width = self.width.saturating_sub(rhs.x);
        self.height = self.height.saturating_sub(rhs.y);
        self
    }
}

impl std::ops::SubAssign<UVec2> for Size {
    fn sub_assign(&mut self, rhs: UVec2) {
        self.width = self.width.saturating_sub(rhs.x);
        self.height = self.height.saturating_sub(rhs.y);
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn contains(&self, p: impl Into<Vec2>) -> bool {
        let p = p.into();
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn union(mut self, other: Self) -> Self {
        self.add(other);
        self
    }

    pub fn add(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    WindowResized { width: u32, height: u32 },
    MouseMoved { position: UVec2, is_down: bool },
    MouseButton { position: UVec2, is_down: bool },
}

#[derive(Default)]
pub struct EventState {
    last_mouse_cursor_position: UVec2,
    mouse_button_is_down: bool,
}

#[cfg(feature = "winit")]
impl EventState {
    pub fn event_from_winit(&mut self, value: &winit::event::WindowEvent) -> Option<Event> {
        #[allow(deprecated)]
        match value {
            winit::event::WindowEvent::Resized(size) => Some(Event::WindowResized {
                width: size.width,
                height: size.height,
            }),
            winit::event::WindowEvent::Moved(_) => None,
            winit::event::WindowEvent::CloseRequested => None,
            winit::event::WindowEvent::Destroyed => None,
            winit::event::WindowEvent::DroppedFile(_) => None,
            winit::event::WindowEvent::HoveredFile(_) => None,
            winit::event::WindowEvent::HoveredFileCancelled => None,
            winit::event::WindowEvent::ReceivedCharacter(_) => None,
            winit::event::WindowEvent::Focused(_) => None,
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                input: _,
                is_synthetic: _,
            } => None,
            winit::event::WindowEvent::ModifiersChanged(_) => None,
            winit::event::WindowEvent::Ime(_) => None,
            winit::event::WindowEvent::CursorMoved {
                device_id: _,
                position,
                modifiers: _,
            } => {
                self.last_mouse_cursor_position = UVec2::new(position.x as u32, position.y as u32);
                Some(Event::MouseMoved {
                    position: self.last_mouse_cursor_position,
                    is_down: self.mouse_button_is_down,
                })
            }
            winit::event::WindowEvent::CursorEntered { device_id: _ } => None,
            winit::event::WindowEvent::CursorLeft { device_id: _ } => None,
            winit::event::WindowEvent::MouseWheel {
                device_id: _,
                delta: _,
                phase: _,
                modifiers: _,
            } => None,
            winit::event::WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
                modifiers: _,
            } => {
                match button {
                    winit::event::MouseButton::Left => Some(()),
                    winit::event::MouseButton::Right => None,
                    winit::event::MouseButton::Middle => None,
                    winit::event::MouseButton::Other(_) => None,
                }?;
                self.mouse_button_is_down = *state == winit::event::ElementState::Pressed;
                Some(Event::MouseButton {
                    position: self.last_mouse_cursor_position,
                    is_down: self.mouse_button_is_down,
                })
            }
            winit::event::WindowEvent::TouchpadPressure {
                device_id: _,
                pressure: _,
                stage: _,
            } => None,
            winit::event::WindowEvent::AxisMotion {
                device_id: _,
                axis: _,
                value: _,
            } => None,
            winit::event::WindowEvent::Touch(_) => None,
            winit::event::WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                new_inner_size: _,
            } => None,
            winit::event::WindowEvent::ThemeChanged(_) => None,
            winit::event::WindowEvent::Occluded(_) => None,
        }
    }
}

pub enum Paint<'a, 'b> {
    Drawing(Box<dyn FnOnce(&mut wgpu::RenderPass<'a>, &'a wgpu::BindGroup) + 'b>),
    Overlay(Box<dyn FnOnce(&mut wgpu::RenderPass<'a>, &'a wgpu::BindGroup) + 'b>),
}

impl<'a, 'b> Paint<'a, 'b> {
    pub fn drawing(f: impl FnOnce(&mut wgpu::RenderPass<'a>, &'a wgpu::BindGroup) + 'b) -> Self {
        Paint::Drawing(Box::new(f))
    }

    pub fn overlay(f: impl FnOnce(&mut wgpu::RenderPass<'a>, &'a wgpu::BindGroup) + 'b) -> Self {
        Paint::Overlay(Box::new(f))
    }

    fn draw(
        self,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        (match self {
            Paint::Drawing(f) => f,
            Paint::Overlay(f) => f,
        })(render_pass, default_texture_bindgroup)
    }
}

/// Implemented by every user interface element.
pub trait Element {
    type OutputEvent;

    /// Layout the element within the constraining bounding box, returning the
    /// element's actual bounding box.
    fn layout(&mut self, constraint: AABB) -> AABB;

    /// 'Paint' the element into the given render pass.
    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // render_pass: &mut wgpu::RenderPass<'a>,
        // default_texture_bindgroup: &'a wgpu::BindGroup,
    ) -> Vec<Paint<'a, 'b>>;

    /// Handle a global event and react with a local event.
    fn event(&mut self, event: Event) -> Self::OutputEvent;
}

impl<A: Element, B: Element> Element for (A, B) {
    type OutputEvent = (A::OutputEvent, B::OutputEvent);

    fn layout(&mut self, constraint: AABB) -> AABB {
        let mut aabb = AABB::default();
        aabb = aabb.union(self.0.layout(constraint));
        aabb = aabb.union(self.1.layout(constraint));
        aabb
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // render_pass: &mut wgpu::RenderPass<'a>,
        // default_texture_bindgroup: &'a wgpu::BindGroup,
    ) -> Vec<Paint<'a, 'b>> {
        let mut ps = self.0.paint(device, queue);
        ps.extend(self.1.paint(device, queue));
        ps
    }

    fn event(&mut self, event: Event) -> Self::OutputEvent {
        (self.0.event(event), self.1.event(event))
    }
}

type RenderParams = (
    View<Device>,
    View<Queue>,
    View<UiScene>,
    View<UiRenderPipeline>,
    View<FrameTextureView>,
);

/// User interface renderer.
pub struct Gpui(pub Renderling);

impl Gpui {
    /// Create a new UI renderer.
    pub fn new(width: u32, height: u32) -> Self {
        let r = Renderling::headless(width, height).unwrap();
        Self::new_from(&r)
    }

    /// Create a new UI renderer linked to the device and queue of another
    /// [`Renderling`].
    pub fn new_from(r: &renderling::Renderling) -> Self {
        let (device, queue) = r.get_device_and_queue_owned();
        let (width, height) = r.get_screen_size();
        let target = RenderTarget::Texture {
            texture: device
                .create_texture(&wgpu::TextureDescriptor {
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::COPY_SRC
                        | wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    label: None,
                    view_formats: &[],
                })
                .into(),
        };
        drop(r);
        let depth_texture = renderling::Texture::create_depth_texture(&device, width, height);
        let mut r = Renderling::new(
            target,
            depth_texture,
            device.0.clone(),
            queue.0.clone(),
            (width, height),
        );

        r.graph.add_resource({
            let fonts: Vec<FontArc> = vec![];
            fonts
        });
        let scene = r.new_ui_scene().with_canvas_size(width, height).build();
        r.graph.add_resource(scene);

        let pipeline = renderling::UiRenderPipeline(
            r.graph
                .visit(|(device, target): (View<Device>, View<RenderTarget>)| {
                    renderling::create_ui_pipeline(&device, target.format())
                })
                .unwrap(),
        );
        r.graph.add_resource(pipeline);

        fn update_scene(
            (mut scene, device, queue): (ViewMut<UiScene>, View<Device>, View<Queue>),
        ) -> Result<(), WgpuStateError> {
            scene.update(&device, &queue, []).unwrap();
            Ok(())
        }

        use renderling::{Graph, graph, node::{create_frame, clear_frame_and_depth}};
        r.graph.add_subgraph(graph!(create_frame, clear_frame_and_depth, update_scene));
        r.graph.add_barrier();
        r.graph.add_local::<RenderParams, ()>("render");
        Self(r)
    }

    /// Resize the interface's "surface" texture.
    ///
    /// Returns the new interface texture.
    pub fn resize(&mut self, width: u32, height: u32) -> renderling::Texture {
        self.0.resize(width, height);
        self.0
            .graph
            .visit(|mut ui_scene: ViewMut<UiScene>| {
                ui_scene.set_canvas_size(width, height);
            })
            .unwrap();
        let wgpu_tex = self.get_frame_texture();
        self.0.texture_from_wgpu_tex(wgpu_tex, None)
    }

    pub fn set_background_color(&mut self, color: impl Into<Vec4>) {
        self.0.set_background_color(color)
    }

    pub fn with_background_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    /// Render to an image.
    pub fn render_image(&mut self, root: &mut impl Element) -> image::DynamicImage {
        self.render_image_with_srgb(root, false)
    }

    /// Render to an image.
    pub fn render_image_srgb(&mut self, root: &mut impl Element) -> image::DynamicImage {
        self.render_image_with_srgb(root, true)
    }

    /// Render to an image.
    pub fn render_image_with_srgb(
        &mut self,
        root: &mut impl Element,
        as_srgb: bool,
    ) -> image::DynamicImage {
        self.render(root);
        let (width, height) = self.0.get_screen_size();
        let frame = self.0.graph.remove_resource::<Frame>().unwrap().unwrap();
        let device = self.0.get_device();
        let buffer = frame.copy_to_buffer(device, self.0.get_queue(), width, height);
        if as_srgb {
            // pack as srgb8
            buffer.into_image::<image::Rgba<u8>>(device).unwrap().into()
        } else {
            // pack as linear rgba8
            buffer.into_rgba(device).unwrap().into()
        }
    }

    /// Layout the root element with the internal screen size.
    pub fn layout(&self, root: &mut impl Element) {
        let (ww, wh) = self.0.get_screen_size();
        let _ = root.layout(AABB {
            min: Vec2::ZERO,
            max: Vec2::new(ww as f32, wh as f32),
        });
    }

    /// Render to the internal texture.
    pub fn render(&mut self, root: &mut impl Element) {
        let _ = self.0.graph.remove_resource::<Frame>();

        self.0.graph.run_with_local(
            |(device, queue, scene, pipeline, frame): RenderParams| -> Result<(), UiSceneError> {
                let label = Some("gpui scene render");
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&pipeline.0);
                render_pass.set_bind_group(0, scene.constants_bindgroup(), &[]);

                let paintings = root.paint(
                    &device,
                    &queue,
                    //&mut render_pass,
                    //scene.default_texture_bindgroup()
                );

                let (paintings, overlaid_paintings): (Vec<_>, Vec<_>) = paintings.into_iter().partition(|p| match p {
                    Paint::Drawing(_) => true,
                    Paint::Overlay(_) => false,
                });

                paintings.into_iter().for_each(|p| p.draw(&mut render_pass, scene.default_texture_bindgroup()));
                overlaid_paintings.into_iter().for_each(|p| p.draw(&mut render_pass, scene.default_texture_bindgroup()));

                drop(render_pass);
                queue.submit(std::iter::once(encoder.finish()));
                Ok(())
        }).unwrap();
    }

    /// Get a clone of the internal render target texture.
    ///
    /// Use this texture to composite the UI into your render graph.
    pub fn get_frame_texture(&self) -> Arc<wgpu::Texture> {
        let target = self.0.get_render_target();
        match target {
            RenderTarget::Texture { texture } => texture.clone(),
            _ => unreachable!("renderling-gpui always renders to a texture"),
        }
    }

    fn get_fonts(&self) -> &[FontArc] {
        let fonts = self
            .0
            .graph
            .get_resource::<Vec<FontArc>>()
            .unwrap()
            .unwrap();
        &fonts
    }

    pub fn add_font(&mut self, font: FontArc) -> Id<FontArc> {
        let fonts = self
            .0
            .graph
            .get_resource_mut::<Vec<FontArc>>()
            .unwrap()
            .unwrap();
        let id = Id::new(fonts.len() as u32);
        fonts.push(font);
        id
    }

    pub fn new_rectangle(&self) -> Rectangle {
        Rectangle::new()
    }

    /// Create a new default Text with the first font (added with
    /// [`Gpui::add_font`]) pre-selected.
    pub fn new_text(&self) -> Text {
        Text::new(&self.get_fonts()[0])
    }

    /// Create a new default [`Button`] with the first font (added with
    /// [`Gpui::add_font`]) pre-selected.
    pub fn new_button(&self) -> Button {
        Button::new(&self.get_fonts()[0])
    }

    /// Create a new default [`Dropdown`] with the first font (added with
    /// [`Gpui::add_font`]) pre-selected.
    pub fn new_dropdown<T: Clone + PartialEq>(&self) -> Dropdown<T> {
        let fonts = self.get_fonts();
        Dropdown::new(&fonts[0], &fonts[1])
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;

    use super::*;

    pub fn _init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .filter_module("renderling", log::LevelFilter::Trace)
            .filter_module("dagga", log::LevelFilter::Warn)
            .filter_module("broomdog", log::LevelFilter::Warn)
            .filter_module("naga", log::LevelFilter::Warn)
            .filter_module("wgpu", log::LevelFilter::Warn)
            .filter_module("wgpu_hal", log::LevelFilter::Warn)
            .try_init();
    }

    #[test]
    fn rectangle() {
        _init_logging();
        let mut ui = Gpui::new(50, 50);
        ui.set_background_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
        let mut rect = ui.new_rectangle().with_color(Vec4::ONE);
        rect.layout(AABB {
            min: Vec2::ZERO,
            max: Vec2::new(25.0, 25.0),
        });
        let img = ui.render_image(&mut rect);
        img_diff::assert_img_eq("gpui_rectangle.png", img);
        rect.set_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
        let img = ui.render_image(&mut rect);
        img_diff::assert_img_eq("gpui_rectangle2.png", img);
    }

    #[test]
    fn srgb_pixel() {
        // This tests that the process of creating an image, saving it and reading
        // back with `image` results in identical pixels.
        let lhs = image::Rgba([0.2, 0.2, 0.2, 1.0]);
        let image = image::Rgba32FImage::from_pixel(1, 1, lhs);
        let image = image::DynamicImage::from(image).into_rgb8();
        image
            .save_with_format("../../test_img/srgb_pixel.png", image::ImageFormat::Png)
            .unwrap();
        let image = image::open("../../test_img/srgb_pixel.png").unwrap();
        let image = image.into_rgba32f();
        let rhs = *image.get_pixel(0, 0);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn srgb_rectangle() {
        // This tests that the roundtrip of a color through our pipeline and back stays
        // true. It does this by creating a rectangle, painting to a texture,
        // reading the texture to a buffer, converting to an image and extracting the
        // pixel, then camparing the resulting pixel.
        _init_logging();
        let mut ui = Gpui::new(1, 1);
        ui.set_background_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
        let lhs = image::Rgba([0.2, 0.2, 0.2, 1.0]);
        let mut rect = ui
            .new_rectangle()
            .with_color(lhs.0)
            .with_size(Vec2::splat(1.0));
        ui.layout(&mut rect);
        // render_image creates an Rgba8 image
        let img = ui.render_image(&mut rect);
        let img = img.into_rgba32f();
        let rhs = *img.get_pixel(0, 0);
        // Converting from the sRGB space of the ui's surface texture introduces some
        // error so we'll compare within a margin.
        fn roughly_equal(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.005
        }
        assert!(roughly_equal(lhs.0[0], rhs.0[0]));
        assert!(roughly_equal(lhs.0[1], rhs.0[1]));
        assert!(roughly_equal(lhs.0[2], rhs.0[2]));
        assert!(roughly_equal(lhs.0[3], rhs.0[3]));
    }

    #[test]
    fn text() {
        _init_logging();
        let mut ui = Gpui::new(500, 70);
        ui.set_background_color(Vec4::new(0.0, 0.0, 0.0, 1.0));

        // We MUST load a font first
        let bytes: Vec<u8> =
            std::fs::read("../../fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();
        let font = FontArc::try_from_vec(bytes).unwrap();
        let _font_id = ui.add_font(font);

        let mut text = ui
            .new_text()
            .with_text("Hello! This is a pretty long sentence. It should wrap.")
            .with_scale(32.0)
            .with_color(Vec4::ONE);
        ui.layout(&mut text);
        let img = ui.render_image(&mut text);
        img_diff::assert_img_eq("gpui_text.png", img);

        // do it again to ensure re-layout works as expected
        ui.layout(&mut text);
        let img = ui.render_image(&mut text);
        img_diff::assert_img_eq("gpui_text.png", img);
    }

    #[test]
    fn button() {
        _init_logging();
        let mut ui = Gpui::new(165, 50);
        ui.set_background_color(Vec4::new(0.5, 0.5, 0.5, 1.0));

        // We MUST load a font first
        let bytes: Vec<u8> =
            std::fs::read("../../fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();
        let font = FontArc::try_from_vec(bytes).unwrap();
        let _font_id = ui.add_font(font);

        let mut btn = ui.new_button().with_text("Click me!").with_scale(32.0);
        ui.layout(&mut btn);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/normal.png", img);

        let may_ev_over = btn.event(Event::MouseMoved {
            position: UVec2::splat(10),
            is_down: false,
        });
        assert_eq!(Some(ButtonEvent::Over), may_ev_over);
        ui.layout(&mut btn);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/over.png", img);

        let may_ev_down = btn.event(Event::MouseButton {
            position: UVec2::splat(10),
            is_down: true,
        });
        assert_eq!(Some(ButtonEvent::Down), may_ev_down);
        ui.layout(&mut btn);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/down.png", img);

        let may_ev_up = btn.event(Event::MouseMoved {
            position: UVec2::splat(10),
            is_down: false,
        });
        assert_eq!(Some(ButtonEvent::Up), may_ev_up);
        ui.layout(&mut btn);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/over.png", img);

        let may_ev_out = btn.event(Event::MouseMoved {
            position: UVec2::splat(1000),
            is_down: false,
        });
        assert_eq!(Some(ButtonEvent::Out), may_ev_out);
        ui.layout(&mut btn);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/normal.png", img);
    }

    #[test]
    fn transparent_rectangles() {
        // This tests the quad geometry and blending by ensuring that a partially
        // transparent rectangle is uniformly transparent and layers correctly
        _init_logging();
        let mut gpui = Gpui::new(50, 50).with_background_color(Vec4::ONE);
        let rect1 = gpui
            .new_rectangle()
            .with_color(Vec3::splat(0.0).extend(0.5))
            .with_size(Vec2::splat(25.0))
            .with_origin(Vec2::splat(10.0));
        let rect2 = gpui
            .new_rectangle()
            .with_color(Vec3::splat(0.0).extend(0.5))
            .with_size(Vec2::splat(25.0))
            .with_origin(Vec2::splat(15.0));
        let mut ui = (rect2, rect1);

        gpui.layout(&mut ui);
        let img = gpui.render_image(&mut ui);
        img_diff::assert_img_eq("gpui_transparent_rectangle.png", img);

        // do it again to make sure re-layout works as expected
        gpui.layout(&mut ui);
        let img = gpui.render_image(&mut ui);
        img_diff::assert_img_eq("gpui_transparent_rectangle.png", img);
    }
}
