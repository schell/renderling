//! Path and builder.
use lyon::{
    path::traits::PathBuilder,
    tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
};
use renderling::{
    math::{Vec2, Vec3, Vec4},
    slab::{GpuArray, Hybrid},
    stage::{Renderlet, Vertex},
};

use crate::{Ui, UiTransform};

pub struct UiPath {
    pub fill_vertices: GpuArray<Vertex>,
    pub fill_indices: GpuArray<u32>,
    pub transform: UiTransform,
    pub fill_renderlet: Option<Hybrid<Renderlet>>,
}

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

fn vec2_to_point(Vec2 { x, y }: Vec2) -> lyon::geom::Point<f32> {
    lyon::geom::point(x, y)
}

impl UiPathBuilder {
    pub fn new(ui: &Ui) -> Self {
        Self {
            ui: ui.clone(),
            attributes: PathAttributes::default(),
            inner: lyon::path::Path::builder_with_attributes(PathAttributes::NUM_ATTRIBUTES),
        }
    }

    pub fn add_rectangle(&mut self, box_min: Vec2, box_max: Vec2) {
        let bx = lyon::geom::Box2D::new(vec2_to_point(box_min), vec2_to_point(box_max));
        self.inner.add_rectangle(
            &bx,
            lyon::path::Winding::Positive,
            &self.attributes.to_array(),
        );
    }

    pub fn with_rectangle(mut self, box_min: Vec2, box_max: Vec2) -> Self {
        self.add_rectangle(box_min, box_max);
        self
    }

    pub fn set_fill_color(&mut self, color: impl Into<Vec4>) {
        self.attributes.fill_color = color.into();
    }

    pub fn with_fill_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_fill_color(color);
        self
    }

    pub fn build(mut self) -> UiPath {
        let l_path = self.inner.build();
        let mut geometry = VertexBuffers::<Vertex, u16>::new();
        let options = FillOptions::tolerance(0.1);
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
        let fill_renderlet = self.ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.ui.camera.id(),
            ..Default::default()
        });

        self.ui.stage.add_renderlet(&fill_renderlet);

        let transform = self.ui.new_transform(vec![fill_renderlet.id()]);
        fill_renderlet.modify(|r| r.transform_id = transform.id());

        UiPath {
            fill_vertices: vertices.into_gpu_only(),
            fill_indices: indices.into_gpu_only(),
            transform,
            fill_renderlet: Some(fill_renderlet),
        }
    }
}
