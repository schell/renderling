//! User interface for the gltf viewer.
use renderling_gpui::{Button, Element, Gpui, Text, Vec2, Vec4, AABB};

pub enum UiEvent {
    ToggleDebugNormals
}

pub struct Ui {
    text_title: Text,
    text_camera: Text,

    btn_debug_normal: Button,
}

impl Ui {
    pub fn new(gpui: &mut Gpui) -> Self {
        Ui {
            text_title: gpui
                .new_text()
                .with_text("Title")
                .with_scale(64.0)
                .with_color(Vec4::ONE),
            text_camera: gpui
                .new_text()
                .with_text("Camera")
                .with_scale(32.0)
                .with_color(Vec4::ONE),
            btn_debug_normal: gpui
                .new_button()
                .with_text("Debug Normals")
                .with_scale(32.0)
        }
    }

    pub fn set_text_title(&mut self, text: impl Into<String>) {
        self.text_title.set_text(text);
    }

    pub fn set_text_camera(&mut self, text: impl Into<String>) {
        self.text_camera.set_text(text);
    }
}

impl Element for Ui {
    type OutputEvent = Option<UiEvent>;

    fn layout(&mut self, mut constraint: AABB) -> AABB {
        let border = 16.0;
        let space = 8.0;
        constraint.min += Vec2::splat(border);
        let aabb_title = self.text_title.layout(constraint);
        let aabb_camera = self.text_camera.layout({
            let mut aabb = aabb_title;
            aabb.min.y = aabb_title.max.y + space;
            aabb.max = constraint.max;
            aabb
        });
        let aabb_btn_debug_normal = self.btn_debug_normal.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_camera.max.y + space;
            aabb
        });
        aabb_title.union(aabb_camera).union(aabb_btn_debug_normal)
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        self.text_title
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.text_camera
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_normal
            .paint(device, queue, render_pass, default_texture_bindgroup);
    }

    fn event(&mut self, event: renderling_gpui::Event) -> Self::OutputEvent {
        use renderling_gpui::ButtonEvent;
        match self.btn_debug_normal.event(event) {
            Some(ev) => match ev {
                ButtonEvent::Down => Some(UiEvent::ToggleDebugNormals),
                _ => None
            }
            None => None
        }
    }
}
