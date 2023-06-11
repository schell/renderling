//! User interface for the gltf viewer.
use renderling::debug::DebugChannel;
use renderling_gpui::{Button, Element, Gpui, Text, Vec2, Vec4, AABB};

pub enum UiEvent {
    ToggleDebugChannel(DebugChannel)
}

pub struct Ui {
    text_title: Text,
    text_camera: Text,

    btn_debug_uv0: Button,
    btn_debug_uv1: Button,
    btn_debug_normal: Button,
    btn_debug_vertex_normals: Button,
    btn_debug_uv_normals: Button,
    btn_debug_tangents: Button,
    btn_debug_bitangents: Button,
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
            btn_debug_uv0: gpui
                .new_button()
                .with_text("Debug UV0")
                .with_scale(32.0),
            btn_debug_uv1: gpui
                .new_button()
                .with_text("Debug UV1")
                .with_scale(32.0),
            btn_debug_normal: gpui
                .new_button()
                .with_text("Debug Normals")
                .with_scale(32.0),
            btn_debug_vertex_normals: gpui
                .new_button()
                .with_text("Debug Vertex Normals")
                .with_scale(32.0),
            btn_debug_uv_normals: gpui
                .new_button()
                .with_text("Debug UV Normals")
                .with_scale(32.0),
            btn_debug_tangents: gpui
                .new_button()
                .with_text("Debug Tangents")
                .with_scale(32.0),
            btn_debug_bitangents: gpui
                .new_button()
                .with_text("Debug Bitangents")
                .with_scale(32.0),
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
        let aabb_btn_debug_uv0 = self.btn_debug_uv0.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_camera.max.y + space;
            aabb
        });
        let aabb_btn_debug_uv1 = self.btn_debug_uv1.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_uv0.max.y + space;
            aabb
        });

        let aabb_btn_debug_normal = self.btn_debug_normal.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_uv1.max.y + space;
            aabb
        });
        let aabb_btn_debug_vertex_normal = self.btn_debug_vertex_normals.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_normal.max.y + space;
            aabb
        });
        let aabb_btn_debug_uv_normal = self.btn_debug_uv_normals.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_vertex_normal.max.y + space;
            aabb
        });
        let aabb_btn_debug_tangents = self.btn_debug_tangents.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_uv_normal.max.y + space;
            aabb
        });
        let aabb_btn_debug_bitangents = self.btn_debug_bitangents.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_btn_debug_tangents.max.y + space;
            aabb
        });
        aabb_title
            .union(aabb_camera)
            .union(aabb_btn_debug_uv0)
            .union(aabb_btn_debug_uv1)
            .union(aabb_btn_debug_normal)
            .union(aabb_btn_debug_uv_normal)
            .union(aabb_btn_debug_vertex_normal)
            .union(aabb_btn_debug_tangents)
            .union(aabb_btn_debug_bitangents)
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
        self.btn_debug_uv0
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_uv1
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_normal
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_uv_normals
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_vertex_normals
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_tangents
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.btn_debug_bitangents
            .paint(device, queue, render_pass, default_texture_bindgroup);
    }

    fn event(&mut self, event: renderling_gpui::Event) -> Self::OutputEvent {
        use renderling_gpui::ButtonEvent;
        let btns = [
            (&mut self.btn_debug_uv0, UiEvent::ToggleDebugChannel(DebugChannel::UvCoords0)),
            (&mut self.btn_debug_uv1, UiEvent::ToggleDebugChannel(DebugChannel::UvCoords1)),
            (&mut self.btn_debug_normal, UiEvent::ToggleDebugChannel(DebugChannel::Normals)),
            (&mut self.btn_debug_vertex_normals, UiEvent::ToggleDebugChannel(DebugChannel::VertexNormals)),
            (&mut self.btn_debug_uv_normals, UiEvent::ToggleDebugChannel(DebugChannel::UvNormals)),
            (&mut self.btn_debug_tangents, UiEvent::ToggleDebugChannel(DebugChannel::Tangents)),
            (&mut self.btn_debug_bitangents, UiEvent::ToggleDebugChannel(DebugChannel::Bitangents)),
        ];
        for (btn, ev) in btns.into_iter() {
            if btn.event(event) == Some(ButtonEvent::Down) {
                log::trace!("Button '{}' down", btn.get_text());
                return Some(ev);
            }
        }
        None
    }
}
