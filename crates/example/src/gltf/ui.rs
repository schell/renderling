//! User interface for the gltf viewer.
use renderling::{debug::DebugChannel, FontArc};
use renderling_gpui::{Dropdown, DropdownEvent, Element, Gpui, Paint, Text, Vec2, Vec4, AABB};

pub enum UiEvent {
    ToggleDebugChannel(DebugChannel),
}

pub struct Ui {
    text_title: Text,
    text_camera: Text,
    dropdown_debug: Dropdown<DebugChannel>,
}

impl Ui {
    pub async fn new(gpui: &mut Gpui, font_dir: impl AsRef<std::path::Path>) -> Self {
        // get the fonts for the UI
        let fonts = [
            "Recursive Mn Lnr St Med Nerd Font Complete.ttf",
            "Font Awesome 6 Free-Regular-400.otf",
        ];
        for path in fonts {
            let path = font_dir.as_ref().join(path);
            let path = format!("{}", path.display());
            let bytes: Vec<u8> = loading_bytes::load(&path).await.unwrap();
            let font = FontArc::try_from_vec(bytes).unwrap();
            gpui.add_font(font);
        }

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
            dropdown_debug: gpui
                .new_dropdown()
                .with_scale(32.0)
                .with_selections(DebugChannel::all().map(|c| (format!("{c:?}"), c)))
                .with_label_builder(|label| label.set_color(Vec4::ONE)),
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
        let aabb_dropdown = self.dropdown_debug.layout({
            let mut aabb = constraint;
            aabb.min.y = aabb_camera.max.y;
            aabb
        });
        aabb_title.union(aabb_camera).union(aabb_dropdown)
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Vec<Paint<'a, 'b>> {
        let mut ps = self.text_title.paint(device, queue);
        ps.extend(self.text_camera.paint(device, queue));
        ps.extend(self.dropdown_debug.paint(device, queue));
        ps
    }

    fn event(&mut self, event: renderling_gpui::Event) -> Self::OutputEvent {
        if let Some(DropdownEvent::Selected(channel)) = self.dropdown_debug.event(event) {
            Some(UiEvent::ToggleDebugChannel(channel))
        } else {
            None
        }
    }
}
