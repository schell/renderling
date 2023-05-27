//! GPU user interface.

use std::sync::Arc;

use renderling::node::FrameTextureView;
use renderling::{graph::IsGraphNode, Device, Read, RenderTarget};
use renderling::{
    FontArc, FontId, Frame, GlyphCache, Id, OwnedSection, OwnedText, Queue, Renderling,
    UiDrawObject, UiDrawObjectBuilder, UiMode, UiScene, UiVertex, WgpuStateError, Write,
};
use renderling::{UiRenderPipeline, UiSceneError};

pub use renderling::math::{UVec2, Vec2, Vec4};

mod elements;
pub use elements::*;

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
pub struct SizeConstraint {
    min: Size,
    max: Size,
}

impl std::ops::Sub<UVec2> for SizeConstraint {
    type Output = Self;

    fn sub(mut self, rhs: UVec2) -> Self::Output {
        self.min -= rhs;
        self.max -= rhs;
        self
    }
}

impl std::ops::SubAssign<UVec2> for SizeConstraint {
    fn sub_assign(&mut self, rhs: UVec2) {
        self.min -= rhs;
        self.max -= rhs;
    }
}

impl From<Size> for SizeConstraint {
    fn from(value: Size) -> Self {
        SizeConstraint {
            min: value,
            max: value,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn contains(&self, p: impl Into<Vec2>) -> bool {
        let p = p.into();
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

pub enum Event {
    MouseMoved { position: UVec2 },
    MouseButton {
        position: UVec2,
        is_down: bool
    }
}

pub trait Element {
    type OutputEvent;

    fn layout(&mut self, constraint: SizeConstraint) -> Size;
    fn paint<'a, 'b: 'a>(
        &'b mut self,
        origin: Vec2,
        size: Vec2,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    );
    fn event(&mut self, event: Event) -> Option<Self::OutputEvent>;
}

type RenderParams = (
    Read<Device>,
    Read<Queue>,
    Read<UiScene>,
    Read<UiRenderPipeline>,
    Read<FrameTextureView>,
);

/// User interface renderer.
pub struct Gpui(Renderling);

impl Gpui {
    /// Create a new UI renderer.
    pub fn new(width: u32, height: u32) -> Self {
        let mut r = renderling::Renderling::headless(width, height).unwrap();
        r.graph.add_resource({
            let fonts: Vec<FontArc> = vec![];
            fonts
        });
        let scene = r.new_ui_scene().with_canvas_size(width, height).build();
        r.graph.add_resource(scene);

        let pipeline = renderling::UiRenderPipeline(
            r.graph
                .visit(|(device, target): (Read<Device>, Read<RenderTarget>)| {
                    renderling::create_ui_pipeline(&device, target.format())
                })
                .unwrap(),
        );
        r.graph.add_resource(pipeline);

        r.graph.add_node(
            renderling::node::create_frame
                .into_node()
                .with_name("create_frame"),
        );
        r.graph.add_node(
            renderling::node::clear_frame_and_depth
                .into_node()
                .with_name("clear_frame_and_depth"),
        );

        fn update_scene(
            (mut scene, device, queue): (Write<UiScene>, Read<Device>, Read<Queue>),
        ) -> Result<(), WgpuStateError> {
            scene.update(&device, &queue, []).unwrap();
            Ok(())
        }
        r.graph
            .add_node(update_scene.into_node().with_name("ui_scene_update"));

        r.graph.add_barrier();

        r.graph.add_local::<RenderParams, ()>();
        Self(r)
    }

    pub fn set_background_color(&mut self, color: impl Into<Vec4>) {
        self.0.set_background_color(color)
    }

    /// Render to an image.
    pub fn render_image(&mut self, root: &mut impl Element) -> image::DynamicImage {
        self.render(root);
        let (width, height) = self.0.get_screen_size();
        let frame = self.0.graph.remove_resource::<Frame>().unwrap().unwrap();
        let device = self.0.get_device();
        let buffer = frame.copy_to_buffer(device, self.0.get_queue(), width, height);
        buffer.into_image::<image::Rgba<u8>>(device).unwrap()
    }

    /// Render to the internal texture.
    pub fn render(&mut self, root: &mut impl Element) {
        log::trace!("rendering");
        let _ = self.0.graph.remove_resource::<Frame>();
        let (width, height) = self.0.get_screen_size();
        let size = root.layout(SizeConstraint {
            min: Size {
                width: 0,
                height: 0,
            },
            max: Size { width, height },
        });

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

                root.paint(
                    Vec2::ZERO,
                    size.into(),
                    &device,
                    &queue,
                    &mut render_pass,
                    scene.default_texture_bindgroup()
                );

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
            RenderTarget::Surface {
                surface: _,
                surface_config: _,
            } => unreachable!("renderling-gpui does not render to a surface"),
            RenderTarget::Texture { texture } => texture.clone(),
        }
    }

    fn get_fonts(&self) -> Vec<FontArc> {
        let fonts = self
            .0
            .graph
            .get_resource::<Vec<FontArc>>()
            .unwrap()
            .unwrap();
        fonts.clone()
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

    pub fn new_rectangle(&self, size: Size, color: Vec4) -> Rectangle {
        Rectangle::new(size, color)
    }

    pub fn new_text(&self) -> Text {
        Text::new(&self.0, self.get_fonts())
    }

    pub fn new_button(&self) -> Button {
        Button::new(&self.0, self.get_fonts())
    }
}

#[cfg(test)]
mod test {
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
        let mut rect = ui.new_rectangle(
            Size {
                width: 25,
                height: 25,
            },
            Vec4::ONE,
        );
        let img = ui.render_image(&mut rect);
        img_diff::assert_img_eq("gpui_rectangle.png", img);
        rect.set_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
        let img = ui.render_image(&mut rect);
        img_diff::assert_img_eq("gpui_rectangle2.png", img);
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
        let font_id = ui.add_font(font);

        let mut text = ui.new_text();
        text.add_text(
            "Hello! This is a pretty long sentence. It should wrap.",
            32.0,
            Vec4::ONE,
            font_id,
        );
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
        let font_id = ui.add_font(font);

        let mut btn = ui.new_button();
        btn.add_text("Click me!", 32.0, font_id);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/normal.png", img);

        let may_ev_over = btn.event(Event::MouseMoved {
            position: UVec2::splat(10),
        });
        assert_eq!(Some(ButtonEvent::Over), may_ev_over);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/over.png", img);

        let may_ev_down = btn.event(Event::MouseButton { position: UVec2::splat(10), is_down: true });
        assert_eq!(Some(ButtonEvent::Down), may_ev_down);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/down.png", img);

        let may_ev_up = btn.event(Event::MouseMoved {
            position: UVec2::splat(10),
        });
        assert_eq!(Some(ButtonEvent::Up), may_ev_up);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/over.png", img);

        let may_ev_out = btn.event(Event::MouseMoved {
            position: UVec2::splat(1000),
        });
        assert_eq!(Some(ButtonEvent::Out), may_ev_out);
        let img = ui.render_image(&mut btn);
        img_diff::assert_img_eq("gpui_button/normal.png", img);
    }
}
