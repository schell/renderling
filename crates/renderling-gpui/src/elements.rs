//! User interface elements.
pub use super::*;

#[derive(Default)]
pub struct Rectangle {
    aabb: AABB,
    color: Vec4,
    draw_object: Option<UiDrawObject>,
}

impl Rectangle {
    // Create a new rectangle.
    pub fn new() -> Self {
        Self {
            aabb: AABB {
                min: Vec2::ZERO,
                max: Vec2::splat(100.0),
            },
            color: Vec4::ONE,
            draw_object: None,
        }
    }

    pub fn set_origin(&mut self, origin: impl Into<Vec2>) {
        let origin = origin.into();
        if self.aabb.min != origin {
            let delta = origin - self.aabb.min;
            self.aabb.min += delta;
            self.aabb.max += delta;
            if let Some(obj) = self.draw_object.as_mut() {
                obj.set_position(self.aabb.min);
            }
        }
    }

    pub fn with_origin(mut self, origin: impl Into<Vec2>) -> Self {
        self.set_origin(origin);
        self
    }

    // Set the size of the rectangle.
    pub fn set_size(&mut self, size: impl Into<Vec2>) {
        let size = size.into();
        if self.aabb.size() != size {
            self.aabb.max = self.aabb.min + size;
            if let Some(obj) = self.draw_object.as_mut() {
                obj.set_scale(size);
            }
        }
    }

    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.set_size(size);
        self
    }

    // Get the color of the rectangle.
    pub fn get_color(&self) -> Vec4 {
        self.color
    }

    // Set the color of the rectangle.
    pub fn set_color(&mut self, color: impl Into<Vec4>) {
        let color = color.into();
        if self.color != color {
            self.color = color;
            let vertices = self.vertices();
            if let Some(obj) = self.draw_object.as_mut() {
                obj.set_vertices(vertices);
            }
        }
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_color(color);
        self
    }

    fn vertices(&self) -> [UiVertex; 6] {
        let tl = UiVertex::default()
            .with_position(Vec2::ZERO)
            .with_color(self.color);
        let tr = UiVertex::default()
            .with_position(Vec2::new(1.0, 0.0))
            .with_color(self.color);
        let bl = UiVertex::default()
            .with_position(Vec2::new(0.0, 1.0))
            .with_color(self.color);
        let br = UiVertex::default()
            .with_position(Vec2::ONE)
            .with_color(self.color);
        [tl, bl, br, tl, br, tr]
    }
}

impl Element for Rectangle {
    type OutputEvent = ();

    fn layout(&mut self, constraint: AABB) -> AABB {
        self.aabb = AABB {
            min: self.aabb.min.max(constraint.min),
            max: self.aabb.max.min(constraint.max),
        };

        self.aabb
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        let origin = self.aabb.min;
        let size = self.aabb.size();
        if self.draw_object.is_none() {
            let obj = UiDrawObjectBuilder::new(device)
                .with_draw_mode(UiMode::DEFAULT)
                .with_position(origin)
                .with_scale(size)
                .with_vertices(self.vertices())
                .build();
            self.draw_object = Some(obj);
        }
        let draw_obj = self.draw_object.as_mut().unwrap();

        if origin != draw_obj.get_position() {
            draw_obj.set_position(origin);
        }
        if size != draw_obj.get_scale() {
            draw_obj.set_scale(size);
        }

        draw_obj.update(device, queue).unwrap();
        draw_obj.draw(render_pass, default_texture_bindgroup);
    }

    fn event(&mut self, _event: Event) {}
}

pub struct Text {
    cache: GlyphCache,
    section: OwnedSection,
    aabb: AABB,
    updated: bool,
    draw_object: Option<UiDrawObject>,
}

impl Text {
    pub fn new(renderling: &Renderling, fonts: impl IntoIterator<Item = FontArc>) -> Self {
        let fonts = fonts.into_iter().collect::<Vec<_>>();
        let cache = GlyphCache::new(renderling, fonts);
        Text {
            section: OwnedSection::default().add_text(OwnedText::new("").with_scale(12.0)),
            cache,
            aabb: AABB::default(),
            updated: true,
            draw_object: None,
        }
    }

    pub fn add_text(
        &mut self,
        text: impl Into<String>,
        scale: f32,
        color: impl Into<Vec4>,
        font_id: Id<FontArc>,
    ) {
        let section = std::mem::take(&mut self.section).add_text(
            OwnedText::new(&text.into())
                .with_scale(scale)
                .with_color(color.into())
                .with_font_id(FontId(font_id.index())),
        );
        self.section = section;
        self.updated = true;
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.section.text.resize_with(1, Default::default);
        self.section.text[0].text = text.into();
        self.updated = true;
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.set_text(text);
        self
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.section
            .text
            .iter_mut()
            .for_each(|t| t.scale = scale.into());
        self.updated = true;
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.set_scale(scale);
        self
    }

    pub fn set_color(&mut self, color: impl Into<Vec4>) {
        let color: Vec4 = color.into();
        self.section
            .text
            .iter_mut()
            .for_each(|t| t.extra.color = color.to_array());
        self.updated = true;
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_color(color);
        self
    }

    /// Return the bounding box for this text.
    ///
    /// This will return `AABB::default` until [`Rectangle::layout`] is called.
    pub fn aabb(&self) -> AABB {
        self.aabb
    }
}

impl Element for Text {
    type OutputEvent = ();

    fn layout(&mut self, constraint: AABB) -> AABB {
        use renderling::GlyphCruncher;
        let max_size = constraint.size();
        let max_size: (f32, f32) = max_size.into();
        if self.section.bounds != max_size {
            self.section.bounds = max_size;
            self.updated = true;
        }

        if self.updated {
            self.aabb = AABB {
                min: constraint.min,
                max: constraint.min,
            };
            if let Some(rect) = self.cache.brush.glyph_bounds(&self.section) {
                self.aabb.max = self.aabb.min + Vec2::new(rect.width(), rect.height());
            }
        }
        self.aabb
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        let origin = self.aabb.min;
        if self.draw_object.is_none() {
            self.draw_object = Some(
                UiDrawObjectBuilder::new(device)
                    .with_draw_mode(UiMode::TEXT)
                    .with_position(origin)
                    .build(),
            );
        }
        let draw_obj = self.draw_object.as_mut().unwrap();
        if self.updated {
            self.updated = false;
            self.cache.queue(&self.section);
        }

        let (may_vertices, may_texture) = self.cache.get_updated();
        if let Some(verts) = may_vertices {
            draw_obj.set_vertices(verts);
        }
        if let Some(texture) = may_texture {
            draw_obj.set_texture(&texture);
        }
        if origin != draw_obj.get_position() {
            draw_obj.set_position(origin);
        }

        draw_obj.update(device, queue).unwrap();
        draw_obj.draw(render_pass, default_texture_bindgroup);
    }

    fn event(&mut self, _: Event) {}
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Normal,
    Over,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonEvent {
    Over,
    Out,
    Down,
    Up,
}

pub struct Button {
    foreground: Rectangle,
    background: Rectangle,
    text: Text,
    aabb: AABB,
    state: ButtonState,
}

impl Button {
    const TEXT_COLOR_NORMAL: Vec4 = Vec4::new(0.1, 0.1, 0.1, 1.0);
    const TEXT_COLOR_OVER: Vec4 = Vec4::new(0.7, 0.13, 0.13, 1.0);
    const TEXT_COLOR_DOWN: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
    const PX_OFFSET: f32 = 8.0;
    const PX_BORDER: f32 = 4.0;

    pub fn new(renderling: &Renderling, fonts: impl IntoIterator<Item = FontArc>) -> Self {
        let mut btn = Button {
            foreground: Rectangle::new(),
            background: Rectangle::new().with_color(Vec4::new(0.0, 0.0, 0.0, 0.5)),
            text: {
                let mut text = Text::new(renderling, fonts);
                text.add_text("Button", 16.0, Self::TEXT_COLOR_NORMAL, Id::new(0));
                text
            },
            aabb: AABB::default(),
            state: ButtonState::default(),
        };
        btn.set_normal();
        btn
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text.set_text(text);
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text.set_text(text);
        self
    }

    pub fn get_text(&self) -> String {
        self.text
            .section
            .text
            .iter()
            .map(|t| t.text.clone())
            .collect::<Vec<_>>()
            .concat()
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.text.set_scale(scale)
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.text.set_scale(scale);
        self
    }

    fn set_over(&mut self) {
        if self.state == ButtonState::Over {
            return;
        }
        self.state = ButtonState::Over;
        self.text.section.text.iter_mut().for_each(|text| {
            *text = std::mem::take(text).with_color(Self::TEXT_COLOR_OVER);
        });
        self.text.updated = true;
    }

    fn set_normal(&mut self) {
        if self.state == ButtonState::Normal {
            return;
        }
        self.state = ButtonState::Normal;
        self.text.section.text.iter_mut().for_each(|text| {
            *text = std::mem::take(text).with_color(Self::TEXT_COLOR_NORMAL);
        });
        self.text.updated = true;
    }

    fn set_down(&mut self) {
        if self.state == ButtonState::Down {
            return;
        }
        self.state = ButtonState::Down;
        self.text.section.text.iter_mut().for_each(|text| {
            *text = std::mem::take(text).with_color(Self::TEXT_COLOR_DOWN);
        });
        self.text.updated = true;
    }
}

impl Element for Button {
    type OutputEvent = Option<ButtonEvent>;

    fn layout(&mut self, constraint: AABB) -> AABB {
        let border = Vec2::splat(Self::PX_BORDER);
        let offset = Vec2::splat(Self::PX_OFFSET);
        let down_offset = if self.state == ButtonState::Down {
            offset * 0.5
        } else {
            Vec2::ZERO
        };
        let text_aabb = self.text.layout(AABB {
            min: constraint.min + border + down_offset,
            max: constraint.max,
        });

        let bg_size = text_aabb.size() + border * 2.0;
        self.foreground.set_origin(constraint.min + down_offset);
        self.foreground.set_size(bg_size);
        self.background
            .set_origin(self.foreground.aabb.min - down_offset + offset);
        self.background.set_size(bg_size);

        let fg_aabb = self.foreground.layout(constraint);
        let bg_aabb = self.background.layout(constraint);
        self.aabb = text_aabb.union(fg_aabb).union(bg_aabb);
        self.aabb
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        self.background
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.foreground
            .paint(device, queue, render_pass, default_texture_bindgroup);
        self.text
            .paint(device, queue, render_pass, default_texture_bindgroup);
    }

    fn event(&mut self, event: Event) -> Option<ButtonEvent> {
        let from_state = self.state;
        match event {
            Event::MouseMoved { position, is_down } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                if self.aabb.contains(position) {
                    if is_down {
                        self.set_down();
                    } else {
                        self.set_over();
                    }
                } else {
                    self.set_normal();
                }
            }
            Event::MouseButton { position, is_down } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                if self.aabb.contains(position) {
                    if is_down {
                        self.set_down();
                    } else {
                        self.set_over();
                    }
                } else {
                    self.set_normal();
                }
            }
            Event::WindowResized { .. } => {}
        };
        match (from_state, self.state) {
            (ButtonState::Normal, ButtonState::Normal) => None,
            (ButtonState::Normal, ButtonState::Over) => Some(ButtonEvent::Over),
            (ButtonState::Normal, ButtonState::Down) => None,
            (ButtonState::Over, ButtonState::Normal) => Some(ButtonEvent::Out),
            (ButtonState::Over, ButtonState::Over) => None,
            (ButtonState::Over, ButtonState::Down) => Some(ButtonEvent::Down),
            (ButtonState::Down, ButtonState::Normal) => None,
            (ButtonState::Down, ButtonState::Over) => Some(ButtonEvent::Up),
            (ButtonState::Down, ButtonState::Down) => None,
        }
    }
}
