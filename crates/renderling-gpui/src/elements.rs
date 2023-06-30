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
    ) -> Vec<Paint<'a, 'b>> {
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
        vec![Paint::drawing(|render_pass, default_texture_bindgroup| {
            draw_obj.draw(render_pass, default_texture_bindgroup);
        })]
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
    pub fn new(font: &FontArc) -> Self {
        let cache = GlyphCache::new(vec![font.clone()]);
        Text {
            section: OwnedSection::default().add_text(OwnedText::new("").with_scale(12.0)),
            cache,
            aabb: AABB::default(),
            updated: true,
            draw_object: None,
        }
    }

    pub fn set_font(&mut self, font: FontArc) {
        self.cache = GlyphCache::new(vec![font]);
        self.updated = true;
    }

    pub fn with_font(mut self, font: FontArc) -> Self {
        self.set_font(font);
        self
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
    ) -> Vec<Paint<'a, 'b>> {
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

        let (may_vertices, may_texture) = self.cache.get_updated(device, queue);
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
        vec![Paint::drawing(|render_pass, default_texture_bindgroup| {
            draw_obj.draw(render_pass, default_texture_bindgroup);
        })]
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

    pub fn new(font: &FontArc) -> Self {
        let mut btn = Button {
            foreground: Rectangle::new(),
            background: Rectangle::new().with_color(Vec4::new(0.0, 0.0, 0.0, 0.5)),
            text: Text::new(font)
                .with_text("Button")
                .with_scale(16.0)
                .with_color(Self::TEXT_COLOR_NORMAL),
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

    pub fn get_text_field(&self) -> &Text {
        &self.text
    }

    pub fn get_text_field_mut(&mut self) -> &mut Text {
        &mut self.text
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
    ) -> Vec<Paint<'a, 'b>> {
        let mut ps = vec![];
        ps.extend(self.background.paint(device, queue));
        ps.extend(self.foreground.paint(device, queue));
        ps.extend(self.text.paint(device, queue));
        ps
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

pub struct Dropdown<T> {
    selections: Vec<(String, T)>,
    label_font: FontArc,
    scale: f32,
    is_open: bool,
    text_label: Text,
    button_open: Button,
    buttons: Vec<Button>,
}

impl<T> Dropdown<T> {
    pub fn new(label_font: &FontArc, icon_font: &FontArc) -> Self {
        let mut dropdown = Self {
            selections: vec![],
            is_open: false,
            scale: 32.0,
            button_open: Button::new(icon_font).with_scale(32.0).with_text("  "),
            text_label: Text::new(label_font)
                .with_scale(32.0)
                .with_text("No selection"),
            buttons: vec![],
            label_font: label_font.clone(),
        };
        dropdown.set_selected_index(None);
        dropdown
    }

    pub fn get_label(&self) -> &Text {
        &self.text_label
    }

    pub fn get_label_mut(&mut self) -> &mut Text {
        &mut self.text_label
    }

    pub fn with_label_builder(mut self, f: impl FnOnce(&mut Text)) -> Self {
        f(&mut self.text_label);
        self
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.button_open.set_scale(scale);
        self.text_label.set_scale(scale);
        for button in self.buttons.iter_mut() {
            button.set_scale(scale);
        }
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.set_scale(scale);
        self
    }

    pub fn set_selections(&mut self, selections: impl IntoIterator<Item = (String, T)>) {
        self.selections = selections.into_iter().collect();
        self.buttons = self
            .selections
            .iter()
            .map(|(name, _)| {
                Button::new(&self.label_font)
                    .with_scale(self.scale)
                    .with_text(name)
            })
            .collect();
        self.text_label.set_text("");
    }

    pub fn with_selections(mut self, selections: impl IntoIterator<Item = (String, T)>) -> Self {
        self.set_selections(selections);
        self
    }

    pub fn set_selected_index(&mut self, may_index: Option<usize>) {
        if let Some(index) = may_index {
            if let Some((name, _)) = self.selections.get(index) {
                self.text_label.set_text(name);
            }
        } else {
            self.text_label.set_text("No selection");
        }
    }

    pub fn with_selected_index(mut self, may_index: Option<usize>) -> Self {
        self.set_selected_index(may_index);
        self
    }
}

pub enum DropdownEvent<T> {
    Selected(T),
}

impl<T: Clone> Element for Dropdown<T> {
    type OutputEvent = Option<DropdownEvent<T>>;

    fn layout(&mut self, mut constraint: AABB) -> AABB {
        let spacing = 4.0;

        let btn_aabb = self.button_open.layout(constraint);

        constraint.min.x += btn_aabb.max.x + spacing;
        let label_aabb = self.text_label.layout(constraint);

        let mut out = btn_aabb.union(label_aabb);
        if self.is_open {
            constraint.min.y = out.max.y + spacing;
            for button in self.buttons.iter_mut() {
                out.add(button.layout(constraint));
                constraint.min.y = out.max.y + spacing;
            }
        }
        out
    }

    fn paint<'a, 'b: 'a>(
        &'b mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Vec<Paint<'a, 'b>> {
        let mut ps = self.button_open.paint(device, queue);
        ps.extend(self.text_label.paint(device, queue));
        if self.is_open {
            for button in self.buttons.iter_mut() {
                ps.extend(button.paint(device, queue));
            }
        }
        ps
    }

    fn event(&mut self, event: Event) -> Self::OutputEvent {
        match self.button_open.event(event) {
            Some(ev) => match ev {
                ButtonEvent::Over => {}
                ButtonEvent::Out => {}
                ButtonEvent::Down => {
                    self.is_open = !self.is_open;
                }
                ButtonEvent::Up => {}
            },
            None => {}
        }

        if self.is_open {
            let may_selected: Option<(usize, T)> = self
                .buttons
                .iter_mut()
                .zip(self.selections.iter())
                .enumerate()
                .find_map(|(i, (button, (_, t)))| {
                    button.event(event).map(|ev| {
                        if let ButtonEvent::Down = ev {
                            Some((i, t.clone()))
                        } else {
                            None
                        }
                    })?
                });
            if let Some((i, t)) = may_selected {
                self.set_selected_index(Some(i));
                self.is_open = false;
                return Some(DropdownEvent::Selected(t));
            }
        }
        None
    }
}
