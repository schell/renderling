use crabslab::Id;
use lyon::{
    path::traits::PathBuilder,
    tessellation::{
        geometry_builder::simple_builder, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
        FillVertexConstructor, VertexBuffers,
    },
};
use renderling::{
    camera::Camera,
    math::{UVec2, Vec2, Vec3, Vec4},
    slab::{Gpu, GpuArray, Hybrid},
    stage::{Renderlet, Stage, Vertex},
    transform::Transform,
    Context,
};

pub struct UiPath {
    pub fill_vertices: GpuArray<Vertex>,
    pub fill_indices: GpuArray<u32>,
    pub transform: Hybrid<Transform>,
    pub renderlets: Vec<Hybrid<Renderlet>>,
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
    stage: Stage,
    camera_id: Id<Camera>,
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
            stage: ui.stage.clone(),
            camera_id: ui.camera.id(),
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
        let vertices = self.stage.new_array(std::mem::take(&mut geometry.vertices));
        let indices = self.stage.new_array(
            std::mem::take(&mut geometry.indices)
                .into_iter()
                .map(|u| u as u32),
        );
        let transform = self.stage.new_value(Transform::default());
        let fill_renderlet = self.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: self.camera_id,
            transform_id: transform.id(),
            ..Default::default()
        });

        self.stage.add_renderlet(&fill_renderlet);

        UiPath {
            fill_vertices: vertices.into_gpu_only(),
            fill_indices: indices.into_gpu_only(),
            transform,
            renderlets: vec![fill_renderlet],
        }
    }
}

pub struct Ui {
    pub camera: Hybrid<Camera>,
    pub stage: Stage,
}

impl Ui {
    pub fn new(ctx: &Context) -> Self {
        let UVec2 { x, y } = ctx.get_size();
        let mut stage = ctx
            .new_stage()
            .with_background_color(Vec4::ONE)
            .with_lighting(false)
            .with_bloom(false);
        let camera = stage.new_value(Camera::default_ortho2d(x as f32, y as f32));
        Ui { camera, stage }
    }

    pub fn new_path(&self) -> UiPathBuilder {
        UiPathBuilder::new(self)
    }

    pub fn render(&mut self, view: &wgpu::TextureView) {
        self.stage.render(view);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ctor::ctor]
    fn init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Warn)
            .filter_module("renderling", log::LevelFilter::Info)
            .filter_module("crabslab", log::LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn can_build_path_sanity() {
        let ctx = Context::headless(100, 100);
        let mut ui = Ui::new(&ctx);
        let _path = ui
            .new_path()
            .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
            .build();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("renderling-ui/path_sanity.png", img);
    }
}
