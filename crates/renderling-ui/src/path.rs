//! Path and builder.
//!
//! Path colors are sRGB.
use lyon::{
    path::traits::PathBuilder,
    tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
        StrokeVertex, VertexBuffers,
    },
};
use renderling::{
    math::{Vec2, Vec3, Vec4},
    slab::{GpuArray, Hybrid},
    stage::{Renderlet, Vertex},
};

use crate::{Ui, UiTransform};

pub struct UiPath {
    pub vertices: GpuArray<Vertex>,
    pub indices: GpuArray<u32>,
    pub transform: UiTransform,
    pub renderlet: Hybrid<Renderlet>,
}

#[derive(Clone, Copy)]
struct PathAttributes {
    stroke_color: Vec4,
    fill_color: Vec4,
}

impl Default for PathAttributes {
    fn default() -> Self {
        Self {
            stroke_color: Vec4::ONE,
            fill_color: Vec4::new(0.2, 0.2, 0.2, 1.0),
        }
    }
}

impl PathAttributes {
    const NUM_ATTRIBUTES: usize = 8;

    fn to_array(&self) -> [f32; Self::NUM_ATTRIBUTES] {
        [
            self.stroke_color.x,
            self.stroke_color.y,
            self.stroke_color.z,
            self.stroke_color.w,
            self.fill_color.x,
            self.fill_color.y,
            self.fill_color.z,
            self.fill_color.w,
        ]
    }

    fn from_slice(s: &[f32]) -> Self {
        Self {
            stroke_color: Vec4::new(s[0], s[1], s[2], s[3]),
            fill_color: Vec4::new(s[4], s[5], s[6], s[7]),
        }
    }
}

#[derive(Clone)]
pub struct UiPathBuilder {
    ui: Ui,
    attributes: PathAttributes,
    inner: lyon::path::BuilderWithAttributes,
}

impl lyon::path::builder::PathBuilder for UiPathBuilder {
    fn num_attributes(&self) -> usize {
        PathAttributes::NUM_ATTRIBUTES
    }

    fn begin(
        &mut self,
        at: lyon::math::Point,
        _: lyon::path::Attributes,
    ) -> lyon::path::EndpointId {
        self.inner.begin(at, &self.attributes.to_array())
    }

    fn end(&mut self, close: bool) {
        self.inner.end(close)
    }

    fn line_to(
        &mut self,
        to: lyon::math::Point,
        _: lyon::path::Attributes,
    ) -> lyon::path::EndpointId {
        self.inner.line_to(to, &self.attributes.to_array())
    }

    fn quadratic_bezier_to(
        &mut self,
        ctrl: lyon::math::Point,
        to: lyon::math::Point,
        _: lyon::path::Attributes,
    ) -> lyon::path::EndpointId {
        self.inner
            .quadratic_bezier_to(ctrl, to, &self.attributes.to_array())
    }

    fn cubic_bezier_to(
        &mut self,
        ctrl1: lyon::math::Point,
        ctrl2: lyon::math::Point,
        to: lyon::math::Point,
        _: lyon::path::Attributes,
    ) -> lyon::path::EndpointId {
        self.inner
            .cubic_bezier_to(ctrl1, ctrl2, to, &self.attributes.to_array())
    }
}

fn vec2_to_point(v: impl Into<Vec2>) -> lyon::geom::Point<f32> {
    let Vec2 { x, y } = v.into();
    lyon::geom::point(x, y)
}

fn vec2_to_vec(v: impl Into<Vec2>) -> lyon::geom::Vector<f32> {
    let Vec2 { x, y } = v.into();
    lyon::geom::Vector::new(x, y)
}

impl UiPathBuilder {
    pub fn new(ui: &Ui) -> Self {
        Self {
            ui: ui.clone(),
            attributes: PathAttributes::default(),
            inner: lyon::path::Path::builder_with_attributes(PathAttributes::NUM_ATTRIBUTES),
        }
    }

    pub fn add_rectangle(
        &mut self,
        box_min: impl Into<Vec2>,
        box_max: impl Into<Vec2>,
    ) -> &mut Self {
        let bx = lyon::geom::Box2D::new(vec2_to_point(box_min), vec2_to_point(box_max));
        self.inner.add_rectangle(
            &bx,
            lyon::path::Winding::Positive,
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_rectangle(mut self, box_min: impl Into<Vec2>, box_max: impl Into<Vec2>) -> Self {
        self.add_rectangle(box_min, box_max);
        self
    }

    pub fn add_ellipse(
        &mut self,
        center: impl Into<Vec2>,
        radii: impl Into<Vec2>,
        rotation: f32,
    ) -> &mut Self {
        self.inner.add_ellipse(
            vec2_to_point(center),
            vec2_to_vec(radii),
            lyon::path::math::Angle { radians: rotation },
            lyon::path::Winding::Positive,
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_ellipse(
        mut self,
        center: impl Into<Vec2>,
        radii: impl Into<Vec2>,
        rotation: f32,
    ) -> Self {
        self.add_ellipse(center, radii, rotation);
        self
    }

    pub fn add_circle(&mut self, center: impl Into<Vec2>, radius: f32) -> &mut Self {
        self.inner.add_circle(
            vec2_to_point(center),
            radius,
            lyon::path::Winding::Positive,
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_circle(mut self, center: impl Into<Vec2>, radius: f32) -> Self {
        self.add_circle(center, radius);
        self
    }

    pub fn set_fill_color(&mut self, color: impl Into<Vec4>) -> &mut Self {
        let mut color = color.into();
        renderling::color::linear_xfer_vec4(&mut color);
        self.attributes.fill_color = color;
        self
    }

    pub fn with_fill_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_fill_color(color);
        self
    }

    pub fn set_stroke_color(&mut self, color: impl Into<Vec4>) -> &mut Self {
        let mut color = color.into();
        renderling::color::linear_xfer_vec4(&mut color);
        self.attributes.stroke_color = color;
        self
    }

    pub fn with_stroke_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_stroke_color(color);
        self
    }

    pub fn fill_with_options(mut self, options: FillOptions) -> UiPath {
        let l_path = self.inner.build();
        let mut geometry = VertexBuffers::<Vertex, u16>::new();
        let mut tesselator = FillTessellator::new();
        tesselator
            .tessellate_path(
                l_path.as_slice(),
                &options,
                &mut BuffersBuilder::new(&mut geometry, |mut vertex: FillVertex| {
                    let p = vertex.position();
                    let PathAttributes {
                        stroke_color: _,
                        fill_color,
                    } = PathAttributes::from_slice(vertex.interpolated_attributes());
                    Vertex {
                        position: Vec3::new(p.x, p.y, 0.0),
                        color: fill_color,
                        ..Default::default()
                    }
                }),
            )
            .unwrap();
        let vertices = self
            .ui
            .stage
            .new_array(std::mem::take(&mut geometry.vertices));
        let indices = self.ui.stage.new_array(
            std::mem::take(&mut geometry.indices)
                .into_iter()
                .map(|u| u as u32),
        );
        let renderlet = self.ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.ui.camera.id(),
            ..Default::default()
        });

        self.ui.stage.add_renderlet(&renderlet);

        let transform = self.ui.new_transform(vec![renderlet.id()]);
        renderlet.modify(|r| r.transform_id = transform.id());

        UiPath {
            vertices: vertices.into_gpu_only(),
            indices: indices.into_gpu_only(),
            transform,
            renderlet,
        }
    }

    pub fn fill(self) -> UiPath {
        self.fill_with_options(Default::default())
    }

    pub fn stroke_with_options(mut self, options: StrokeOptions) -> UiPath {
        let l_path = self.inner.build();
        let mut geometry = VertexBuffers::<Vertex, u16>::new();
        let mut tesselator = StrokeTessellator::new();
        tesselator
            .tessellate_path(
                l_path.as_slice(),
                &options,
                &mut BuffersBuilder::new(&mut geometry, |mut vertex: StrokeVertex| {
                    let p = vertex.position();
                    let PathAttributes {
                        stroke_color,
                        fill_color: _,
                    } = PathAttributes::from_slice(vertex.interpolated_attributes());
                    Vertex {
                        position: Vec3::new(p.x, p.y, 0.0),
                        color: stroke_color,
                        ..Default::default()
                    }
                }),
            )
            .unwrap();
        let vertices = self
            .ui
            .stage
            .new_array(std::mem::take(&mut geometry.vertices));
        let indices = self.ui.stage.new_array(
            std::mem::take(&mut geometry.indices)
                .into_iter()
                .map(|u| u as u32),
        );
        let renderlet = self.ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.ui.camera.id(),
            ..Default::default()
        });

        self.ui.stage.add_renderlet(&renderlet);

        let transform = self.ui.new_transform(vec![renderlet.id()]);
        renderlet.modify(|r| r.transform_id = transform.id());

        UiPath {
            vertices: vertices.into_gpu_only(),
            indices: indices.into_gpu_only(),
            transform,
            renderlet,
        }
    }

    pub fn stroke(self) -> UiPath {
        let options = StrokeOptions::default().with_line_width(2.0);
        self.stroke_with_options(options)
    }

    pub fn fill_and_stroke_with_options(
        self,
        fill_options: FillOptions,
        stroke_options: StrokeOptions,
    ) -> (UiPath, UiPath) {
        (
            self.clone().fill_with_options(fill_options),
            self.stroke_with_options(stroke_options),
        )
    }

    pub fn fill_and_stroke(self) -> (UiPath, UiPath) {
        self.fill_and_stroke_with_options(
            Default::default(),
            StrokeOptions::default().with_line_width(2.0),
        )
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use itertools::Itertools;
    use renderling::{
        color::rgb_hex_color,
        math::{Vec2, Vec4},
        Context,
    };

    use crate::{
        test::{cute_beach_palette, Colors},
        Ui,
    };

    #[test]
    fn can_build_path_sanity() {
        let ctx = Context::headless(100, 100);
        let mut ui = Ui::new(&ctx);
        let builder = ui
            .new_path()
            .with_fill_color([1.0, 1.0, 0.0, 1.0])
            .with_stroke_color([0.0, 1.0, 1.0, 1.0])
            .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
            .with_circle(Vec2::splat(100.0), 20.0);

        let _fill = builder.clone().fill();
        let _stroke = builder.stroke();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("ui/path/sanity.png", img);
    }

    #[test]
    fn can_draw_shapes() {
        let ctx = Context::headless(256, 256);
        let mut ui = Ui::new(&ctx);
        let mut colors = Colors::from_array(cute_beach_palette());

        // rectangle
        let (stroke, fill) = colors.next_color();
        let _rect = ui
            .new_path()
            .with_stroke_color(stroke)
            .with_fill_color(fill)
            .with_rectangle(Vec2::splat(2.0), Vec2::splat(42.0))
            .fill_and_stroke();

        // circle
        let (stroke, fill) = colors.next_color();
        let _circ = ui
            .new_path()
            .with_stroke_color(stroke)
            .with_fill_color(fill)
            .with_circle([64.0, 22.0], 20.0)
            .fill_and_stroke();

        // ellipse
        let (stroke, fill) = colors.next_color();
        let _elli = ui
            .new_path()
            .with_stroke_color(stroke)
            .with_fill_color(fill)
            .with_ellipse([104.0, 22.0], [20.0, 15.0], std::f32::consts::FRAC_PI_4)
            .fill_and_stroke();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("ui/path/shapes.png", img);
    }
}
