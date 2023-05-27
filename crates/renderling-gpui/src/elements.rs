//! User interface elements.
pub use super::*;

#[derive(Default)]
pub struct Rectangle {
    size: Size,
    color: Vec4,
    draw_object: Option<UiDrawObject>,
}

impl Rectangle {
    // Create a new rectangle.
    pub fn new(size: Size, color: Vec4) -> Self {
        Self {
            size,
            color,
            draw_object: None,
        }
    }

    // Set the size of the rectangle.
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = Size { width, height };
        if let Some(obj) = self.draw_object.as_mut() {
            obj.set_scale(self.size);
        }
    }

    // Get the color of the rectangle.
    pub fn get_color(&self) -> Vec4 {
        self.color
    }

    // Set the color of the rectangle.
    pub fn set_color(&mut self, color: Vec4) {
        self.color = color;
        let vertices = self.vertices();
        if let Some(obj) = self.draw_object.as_mut() {
            obj.set_vertices(vertices);
        }
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

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        origin: Vec2,
        size: Vec2,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        if self.draw_object.is_none() {
            log::trace!(
                "creating rectangle origin: {origin:?} size: {size:?} color: {:?}",
                self.color
            );
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
        log::trace!("drawing rectangle");
        draw_obj.draw(render_pass, default_texture_bindgroup);
    }

    fn layout(&mut self, constraint: SizeConstraint) -> Size {
        Size {
            width: self
                .size
                .width
                .clamp(constraint.min.width, constraint.max.width),
            height: self
                .size
                .height
                .clamp(constraint.min.height, constraint.max.height),
        }
    }

    fn event(&mut self, _event: Event) -> Option<()> {
        None
    }
}

pub struct Text {
    cache: GlyphCache,
    section: OwnedSection,
    updated: bool,
    draw_object: Option<UiDrawObject>,
}

impl Text {
    pub fn new(renderling: &Renderling, fonts: impl IntoIterator<Item = FontArc>) -> Self {
        let fonts = fonts.into_iter().collect::<Vec<_>>();
        let cache = GlyphCache::new(renderling, fonts);
        Text {
            section: OwnedSection::default(),
            cache,
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
}

impl Element for Text {
    type OutputEvent = ();
    fn layout(&mut self, constraint: SizeConstraint) -> Size {
        use renderling::GlyphCruncher;
        let max_size = (constraint.max.width as f32, constraint.max.height as f32);
        if self.section.bounds != max_size {
            self.section.bounds = max_size;
            self.updated = true;
        }
        if let Some(rect) = self.cache.brush.glyph_bounds(&self.section) {
            Size {
                width: rect.width().ceil() as u32,
                height: rect.height().ceil() as u32,
            }
        } else {
            Size {
                width: 0,
                height: 0,
            }
        }
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        origin: Vec2,
        size: Vec2,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        if self.draw_object.is_none() {
            log::trace!("creating text origin: {origin:?} size: {size:?}");
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

    fn event(&mut self, _: Event) -> Option<()> {
        None
    }
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
    Up
}

pub struct Button {
    foreground: Rectangle,
    background: Rectangle,
    text: Text,
    aabb: AABB,
    state: ButtonState,
}

impl Button {
    const TEXT_COLOR_NORMAL: Vec4 = Vec4::new(0.2, 0.2, 0.2, 1.0);
    const TEXT_COLOR_OVER: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
    const TEXT_COLOR_DOWN: Vec4 = Vec4::new(1.0, 1.0, 0.2, 1.0);
    const PX_OFFSET: f32 = 8.0;
    const PX_BORDER: f32 = 4.0;

    pub fn new(renderling: &Renderling, fonts: impl IntoIterator<Item = FontArc>) -> Self {
        Button {
            foreground: Rectangle::new(
                Size {
                    width: 50,
                    height: 25,
                },
                Vec4::ONE,
            ),
            background: Rectangle::new(
                Size {
                    width: 50,
                    height: 25,
                },
                Vec4::new(0.0, 0.0, 0.0, 0.5),
            ),
            text: Text::new(renderling, fonts),
            aabb: AABB::default(),
            state: ButtonState::default(),
        }
    }

    pub fn add_text(
        &mut self,
        text: impl Into<String>,
        scale: f32,
        font_id: Id<FontArc>,
    ) {
        self.text.add_text(text, scale, Self::TEXT_COLOR_NORMAL, font_id)
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
    type OutputEvent = ButtonEvent;

    fn layout(&mut self, constraint: SizeConstraint) -> Size {
        let border = UVec2::splat(Self::PX_BORDER as u32);
        let offset = UVec2::splat(Self::PX_OFFSET as u32);
        let text_size = self
            .text
            .layout(constraint - (border * 2) - offset);
        let bg_constraint = (text_size + (border * 2)).into();
        let fg_size = self.foreground.layout(bg_constraint);
        let bg_size = self.background.layout(bg_constraint);
        debug_assert_eq!(fg_size, bg_size);
        fg_size + offset
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        origin: Vec2,
        size: Vec2,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) {
        let offset = Vec2::splat(Self::PX_OFFSET);
        self.background.paint(
            origin + offset,
            size - offset,
            device,
            queue,
            render_pass,
            default_texture_bindgroup,
        );
        let down_offset = if self.state == ButtonState::Down {
            offset - 1.0
        } else {
            Vec2::ZERO
        };
        self.foreground.paint(
            origin + down_offset,
            size - offset,
            device,
            queue,
            render_pass,
            default_texture_bindgroup,
        );
        let border = Vec2::splat(Self::PX_BORDER);
        self.text.paint(
            origin + border + down_offset,
            size - (border * 2.0) - offset,
            device,
            queue,
            render_pass,
            default_texture_bindgroup,
        );
        self.aabb.min = origin;
        self.aabb.max = origin + size;
    }

    fn event(&mut self, event: Event) -> Option<ButtonEvent> {
        let from_state = self.state;
        match event {
            Event::MouseMoved { position } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                if self.aabb.contains(position) {
                    self.set_over();
                } else {
                    self.set_normal();
                }
            }
            Event::MouseButton { position, is_down } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                if self.aabb.contains(position) && is_down {
                    self.set_down();
                } else {
                    self.set_over();
                }
            }

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
