//! Path and builder.
//!
//! Path colors are sRGB.
use crabslab::Id;
use lyon::{
    path::traits::PathBuilder,
    tessellation::{
        BuffersBuilder, FillTessellator, FillVertex, StrokeTessellator, StrokeVertex, VertexBuffers,
    },
};
use renderling::{
    math::{Vec2, Vec3, Vec3Swizzles, Vec4},
    pbr::Material,
    slab::{GpuArray, Hybrid},
    stage::{Renderlet, Vertex},
};

use crate::{ImageId, Ui, UiTransform};
pub use lyon::tessellation::{LineCap, LineJoin};

pub struct UiPath {
    pub vertices: GpuArray<Vertex>,
    pub indices: GpuArray<u32>,
    pub transform: UiTransform,
    pub material: Hybrid<Material>,
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

    fn to_array(self) -> [f32; Self::NUM_ATTRIBUTES] {
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

#[derive(Clone, Copy, Debug)]
pub struct StrokeOptions {
    pub line_width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub image_id: Option<ImageId>,
}

impl Default for StrokeOptions {
    fn default() -> Self {
        StrokeOptions {
            line_width: 2.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            image_id: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FillOptions {
    pub image_id: Option<ImageId>,
}

#[derive(Clone)]
pub struct UiPathBuilder {
    ui: Ui,
    attributes: PathAttributes,
    inner: lyon::path::BuilderWithAttributes,
    default_stroke_options: StrokeOptions,
    default_fill_options: FillOptions,
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
            default_stroke_options: *ui.default_stroke_options.read().unwrap(),
            default_fill_options: *ui.default_fill_options.read().unwrap(),
        }
    }

    pub fn begin(&mut self, at: impl Into<Vec2>) -> &mut Self {
        self.inner
            .begin(vec2_to_point(at), &self.attributes.to_array());
        self
    }

    pub fn with_begin(mut self, at: impl Into<Vec2>) -> Self {
        self.begin(at);
        self
    }

    pub fn end(&mut self, close: bool) -> &mut Self {
        self.inner.end(close);
        self
    }

    pub fn with_end(mut self, close: bool) -> Self {
        self.end(close);
        self
    }

    pub fn line_to(&mut self, to: impl Into<Vec2>) -> &mut Self {
        self.inner
            .line_to(vec2_to_point(to), &self.attributes.to_array());
        self
    }

    pub fn with_line_to(mut self, to: impl Into<Vec2>) -> Self {
        self.line_to(to);
        self
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: impl Into<Vec2>, to: impl Into<Vec2>) -> &mut Self {
        self.inner.quadratic_bezier_to(
            vec2_to_point(ctrl),
            vec2_to_point(to),
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_quadratic_bezier_to(mut self, ctrl: impl Into<Vec2>, to: impl Into<Vec2>) -> Self {
        self.quadratic_bezier_to(ctrl, to);
        self
    }

    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: impl Into<Vec2>,
        ctrl2: impl Into<Vec2>,
        to: impl Into<Vec2>,
    ) -> &mut Self {
        self.inner.cubic_bezier_to(
            vec2_to_point(ctrl1),
            vec2_to_point(ctrl2),
            vec2_to_point(to),
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_cubic_bezier_to(
        mut self,
        ctrl1: impl Into<Vec2>,
        ctrl2: impl Into<Vec2>,
        to: impl Into<Vec2>,
    ) -> Self {
        self.cubic_bezier_to(ctrl1, ctrl2, to);
        self
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

    pub fn add_rounded_rectangle(
        &mut self,
        box_min: impl Into<Vec2>,
        box_max: impl Into<Vec2>,
        top_left_radius: f32,
        top_right_radius: f32,
        bottom_left_radius: f32,
        bottom_right_radius: f32,
    ) -> &mut Self {
        let rect = lyon::geom::Box2D {
            min: vec2_to_point(box_min),
            max: vec2_to_point(box_max),
        };
        let radii = lyon::path::builder::BorderRadii {
            top_left: top_left_radius,
            top_right: top_right_radius,
            bottom_left: bottom_left_radius,
            bottom_right: bottom_right_radius,
        };
        self.inner.add_rounded_rectangle(
            &rect,
            &radii,
            lyon::path::Winding::Positive,
            &self.attributes.to_array(),
        );
        self
    }

    pub fn with_rounded_rectangle(
        mut self,
        box_min: impl Into<Vec2>,
        box_max: impl Into<Vec2>,
        top_left_radius: f32,
        top_right_radius: f32,
        bottom_left_radius: f32,
        bottom_right_radius: f32,
    ) -> Self {
        self.add_rounded_rectangle(
            box_min,
            box_max,
            top_left_radius,
            top_right_radius,
            bottom_left_radius,
            bottom_right_radius,
        );
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

    pub fn add_polygon(
        &mut self,
        is_closed: bool,
        polygon: impl IntoIterator<Item = Vec2>,
    ) -> &mut Self {
        let points = polygon.into_iter().map(vec2_to_point).collect::<Vec<_>>();
        let polygon = lyon::path::Polygon {
            points: points.as_slice(),
            closed: is_closed,
        };
        self.inner.add_polygon(polygon, &self.attributes.to_array());
        self
    }

    pub fn with_polygon(
        mut self,
        is_closed: bool,
        polygon: impl IntoIterator<Item = Vec2>,
    ) -> Self {
        self.add_polygon(is_closed, polygon);
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

    pub fn fill_with_options(self, options: FillOptions) -> UiPath {
        let l_path = self.inner.build();
        let mut geometry = VertexBuffers::<Vertex, u16>::new();
        let mut tesselator = FillTessellator::new();

        let mut size = Vec2::ONE;
        let albedo_texture_id = if let Some(ImageId(index)) = options.image_id {
            if let Some(image) = self.ui.get_image(index) {
                let frame = image.0.frame().get();
                log::debug!("size: {}", frame.size_px);
                size.x = frame.size_px.x as f32;
                size.y = frame.size_px.y as f32;
                image.0.texture().id()
            } else {
                Id::NONE
            }
        } else {
            log::debug!("no image");
            Id::NONE
        };

        tesselator
            .tessellate_path(
                l_path.as_slice(),
                &Default::default(),
                &mut BuffersBuilder::new(&mut geometry, |mut vertex: FillVertex| {
                    let p = vertex.position();
                    let PathAttributes {
                        stroke_color: _,
                        fill_color,
                    } = PathAttributes::from_slice(vertex.interpolated_attributes());
                    let position = Vec3::new(p.x, p.y, 0.0);
                    Vertex {
                        position,
                        uv0: position.xy() / size,
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

        let material = self.ui.stage.new_value(Material {
            albedo_texture_id,
            ..Default::default()
        });
        let renderlet = self.ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.ui.camera.id(),
            material_id: material.id(),
            ..Default::default()
        });

        self.ui.stage.add_renderlet(&renderlet);

        let transform = self.ui.new_transform(vec![renderlet.id()]);
        renderlet.modify(|r| r.transform_id = transform.id());

        UiPath {
            vertices: vertices.into_gpu_only(),
            indices: indices.into_gpu_only(),
            transform,
            material,
            renderlet,
        }
    }

    pub fn fill(self) -> UiPath {
        let options = self.default_fill_options;
        self.fill_with_options(options)
    }

    pub fn stroke_with_options(self, options: StrokeOptions) -> UiPath {
        let l_path = self.inner.build();
        let mut geometry = VertexBuffers::<Vertex, u16>::new();
        let mut tesselator = StrokeTessellator::new();
        let StrokeOptions {
            line_width,
            line_cap,
            line_join,
            image_id,
        } = options;
        let tesselator_options = lyon::tessellation::StrokeOptions::default()
            .with_line_cap(line_cap)
            .with_line_join(line_join)
            .with_line_width(line_width);

        let mut size = Vec2::ONE;
        let albedo_texture_id = if let Some(ImageId(index)) = image_id {
            if let Some(image) = self.ui.get_image(index) {
                let frame = image.0.frame().get();
                log::debug!("size: {}", frame.size_px);
                size.x = frame.size_px.x as f32;
                size.y = frame.size_px.y as f32;
                image.0.texture().id()
            } else {
                Id::NONE
            }
        } else {
            log::debug!("no image");
            Id::NONE
        };

        tesselator
            .tessellate_path(
                l_path.as_slice(),
                &tesselator_options,
                &mut BuffersBuilder::new(&mut geometry, |mut vertex: StrokeVertex| {
                    let p = vertex.position();
                    let PathAttributes {
                        stroke_color,
                        fill_color: _,
                    } = PathAttributes::from_slice(vertex.interpolated_attributes());
                    let position = Vec3::new(p.x, p.y, 0.0);
                    Vertex {
                        position,
                        uv0: position.xy() / size,
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
        let material = self.ui.stage.new_value(Material {
            albedo_texture_id,
            ..Default::default()
        });
        let renderlet = self.ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.ui.camera.id(),
            material_id: material.id(),
            ..Default::default()
        });

        self.ui.stage.add_renderlet(&renderlet);

        let transform = self.ui.new_transform(vec![renderlet.id()]);
        renderlet.modify(|r| r.transform_id = transform.id());

        UiPath {
            vertices: vertices.into_gpu_only(),
            indices: indices.into_gpu_only(),
            transform,
            material,
            renderlet,
        }
    }

    pub fn stroke(self) -> UiPath {
        let options = self.default_stroke_options;
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
        let fill_options = self.default_fill_options;
        let stroke_options = self.default_stroke_options;
        self.fill_and_stroke_with_options(fill_options, stroke_options)
    }
}

#[cfg(test)]
mod test {
    use renderling::{
        math::{hex_to_vec4, Vec2},
        Context,
    };

    use crate::{
        test::{cute_beach_palette, Colors},
        Ui,
    };

    use super::*;

    /// Generates points for a star shape.
    /// `num_points` specifies the number of points (tips) the star will
    /// have. `radius` specifies the radius of the circle in which
    /// the star is inscribed.
    fn star_points(num_points: usize, outer_radius: f32, inner_radius: f32) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(num_points * 2);
        let angle_step = std::f32::consts::PI / num_points as f32;
        for i in 0..num_points * 2 {
            let angle = angle_step * i as f32;
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };
            points.push(Vec2::new(radius * angle.cos(), radius * angle.sin()));
        }
        points
    }

    #[test]
    fn can_build_path_sanity() {
        let ctx = Context::headless(100, 100);
        let ui = Ui::new(&ctx).with_antialiasing(false);
        let builder = ui
            .new_path()
            .with_fill_color([1.0, 1.0, 0.0, 1.0])
            .with_stroke_color([0.0, 1.0, 1.0, 1.0])
            .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
            .with_circle(Vec2::splat(100.0), 20.0);
        {
            let _fill = builder.clone().fill();
            let _stroke = builder.clone().stroke();

            let frame = ctx.get_next_frame().unwrap();
            ui.render(&frame.view());
            let img = frame.read_image().unwrap();
            img_diff::assert_img_eq("ui/path/sanity.png", img);
        }

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        frame.present();

        {
            let _resources = builder.fill_and_stroke();
            let frame = ctx.get_next_frame().unwrap();
            ui.render(&frame.view());
            let img = frame.read_image().unwrap();
            img_diff::assert_img_eq_cfg(
                "ui/path/sanity.png",
                img,
                img_diff::DiffCfg {
                    test_name: Some("ui/path/sanity - separate path and stroke same as together"),
                    ..Default::default()
                },
            );
        }
    }

    #[test]
    fn can_draw_shapes() {
        let ctx = Context::headless(256, 48);
        let ui = Ui::new(&ctx).with_default_stroke_options(StrokeOptions {
            line_width: 4.0,
            ..Default::default()
        });
        let mut colors = Colors::from_array(cute_beach_palette());

        // rectangle
        let fill = colors.next_color();
        let _rect = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_rectangle(Vec2::splat(2.0), Vec2::splat(42.0))
            .fill_and_stroke();

        // circle
        let fill = colors.next_color();
        let _circ = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_circle([64.0, 22.0], 20.0)
            .fill_and_stroke();

        // ellipse
        let fill = colors.next_color();
        let _elli = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_ellipse([104.0, 22.0], [20.0, 15.0], std::f32::consts::FRAC_PI_4)
            .fill_and_stroke();

        // various polygons
        fn circle_points(num_points: usize, radius: f32) -> Vec<Vec2> {
            let mut points = Vec::with_capacity(num_points);
            for i in 0..num_points {
                let angle = 2.0 * std::f32::consts::PI * i as f32 / num_points as f32;
                points.push(Vec2::new(radius * angle.cos(), radius * angle.sin()));
            }
            points
        }

        let fill = colors.next_color();
        let center = Vec2::new(144.0, 22.0);
        let _penta = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_polygon(true, circle_points(5, 20.0).into_iter().map(|p| p + center))
            .fill_and_stroke();

        let fill = colors.next_color();
        let center = Vec2::new(184.0, 22.0);
        let _star = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_polygon(
                true,
                star_points(5, 20.0, 10.0).into_iter().map(|p| p + center),
            )
            .fill_and_stroke();

        let fill = colors.next_color();
        let tl = Vec2::new(210.0, 4.0);
        let _rrect = ui
            .new_path()
            .with_fill_color(fill)
            .with_stroke_color(hex_to_vec4(0x333333FF))
            .with_rounded_rectangle(tl, tl + Vec2::new(40.0, 40.0), 5.0, 0.0, 0.0, 10.0)
            .fill_and_stroke();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("ui/path/shapes.png", img);
    }

    #[test]
    fn can_fill_image() {
        let w = 150.0;
        let ctx = Context::headless(w as u32, w as u32);
        let ui = Ui::new(&ctx);
        let image_id = futures_lite::future::block_on(ui.load_image("../../img/dirt.jpg")).unwrap();
        let center = Vec2::splat(w / 2.0);
        let _path = ui
            .new_path()
            .with_polygon(
                true,
                star_points(7, w / 2.0, w / 3.0)
                    .into_iter()
                    .map(|p| center + p),
            )
            .with_fill_color([1.0, 1.0, 1.0, 1.0])
            .with_stroke_color([1.0, 0.0, 0.0, 1.0])
            .fill_and_stroke_with_options(
                FillOptions {
                    image_id: Some(image_id),
                },
                StrokeOptions {
                    line_width: 5.0,
                    image_id: Some(image_id),
                    ..Default::default()
                },
            );

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let mut img = frame.read_srgb_image().unwrap();
        img.pixels_mut().for_each(|p| {
            renderling::color::opto_xfer_u8(&mut p.0[0]);
            renderling::color::opto_xfer_u8(&mut p.0[1]);
            renderling::color::opto_xfer_u8(&mut p.0[2]);
        });
        img_diff::assert_img_eq("ui/path/fill_image.png", img);
    }
}
