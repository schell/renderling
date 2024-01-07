//! GPU user interface.
use crabslab::{GrowableSlab, Slab, SlabItem};
use snafu::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use renderling::{
    shader::stage::{Camera, RenderUnit},
    Device, Frame, Id, Queue, RenderTarget, Renderling, Stage, View, ViewMut, WgpuStateError,
};

/*
UiDrawObject,
UiDrawObjectBuilder, UiMode, UiRenderPipeline, UiScene, UiSceneError, UiVertex,
*/

pub use renderling::math::{UVec2, Vec2, Vec4};

mod elements;
pub use elements::*;

mod text;
pub use text::*;

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

/// Implemented by every user interface element.
pub trait Element {
    type OutputEvent;
    /// Update the element within the constraining bounding box, returning the
    /// element's actual bounding box.
    ///
    /// The element is responsible for updating its own state on the GPU
    /// and the state of its children.
    fn update(&mut self, gpui: &mut Gpui, constraint: AABB) -> AABB;
    //fn delete(self, gpui: &mut Gpui);

    /// Handle a global event and react with a local event.
    fn event(&mut self, event: Event) -> Self::OutputEvent;
}

impl<A: Element, B: Element> Element for (A, B) {
    type OutputEvent = (A::OutputEvent, B::OutputEvent);

    fn update(&mut self, gpui: &mut Gpui, constraint: AABB) -> AABB {
        let mut aabb = AABB::default();
        aabb = aabb.union(self.0.update(gpui, constraint));
        aabb = aabb.union(self.1.update(gpui, constraint));
        aabb
    }

    fn event(&mut self, event: Event) -> Self::OutputEvent {
        (self.0.event(event), self.1.event(event))
    }
}

/// Helper for writing new interface elements.
struct ElementUpdates {
    writes: Vec<(usize, Vec<u32>)>,
}

impl ElementUpdates {
    pub fn write<T: Default + SlabItem>(&mut self, id: Id<T>, t: &T) {
        self.writes.push((id.index(), {
            let mut v = vec![0; T::slab_size()];
            let _index = v.write_indexed(t, 0);
            v
        }))
    }

    pub fn apply(&mut self, slab: &mut impl Slab) {
        for (index, data) in std::mem::take(&mut self.writes).into_iter() {
            let _ = slab.write_indexed_slice(data.as_slice(), index);
        }
    }
}

/// User interface renderer.
pub struct Gpui {
    pub stage: Stage,
    pub renderling: Renderling,
    pub fonts: Vec<FontArc>,
    pub camera: Id<Camera>,
}

impl Deref for Gpui {
    type Target = Renderling;

    fn deref(&self) -> &Self::Target {
        &self.renderling
    }
}

impl DerefMut for Gpui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.renderling
    }
}

impl crabslab::Slab for Gpui {
    fn len(&self) -> usize {
        self.stage.len()
    }

    fn read<T: crabslab::SlabItem + Default>(&self, id: Id<T>) -> T {
        self.stage.read(id)
    }

    fn write_indexed<T: crabslab::SlabItem>(&mut self, t: &T, index: usize) -> usize {
        self.stage.write_indexed(t, index)
    }

    fn write_indexed_slice<T: crabslab::SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        self.stage.write_indexed_slice(t, index)
    }
}

impl crabslab::GrowableSlab for Gpui {
    fn capacity(&self) -> usize {
        self.stage.capacity()
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        self.stage.reserve_capacity(capacity)
    }

    fn increment_len(&mut self, n: usize) -> usize {
        self.stage.increment_len(n)
    }
}

impl Gpui {
    /// Create a new UI renderer.
    pub fn new(width: u32, height: u32) -> Self {
        let mut r = Renderling::headless(width, height);
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = renderling::default_ortho2d(width as f32, height as f32);
        let camera = stage.append(&Camera::new(projection, view));
        Gpui {
            stage,
            renderling: r,
            fonts: vec![],
            camera,
        }
    }

    /// Layout the root element with the internal screen size.
    pub fn layout(&mut self, root: &mut impl Element) {
        let (ww, wh) = self.renderling.get_screen_size();
        let _ = root.update(
            self,
            AABB {
                min: Vec2::ZERO,
                max: Vec2::new(ww as f32, wh as f32),
            },
        );
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
        &self.fonts
    }

    pub fn add_font(&mut self, font: FontArc) -> Id<FontArc> {
        let id = Id::new(self.fonts.len() as u32);
        self.fonts.push(font);
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
        rect.layout(
            &mut ui,
            AABB {
                min: Vec2::ZERO,
                max: Vec2::new(25.0, 25.0),
            },
        );
        let img = ui.render_image().unwrap();
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
