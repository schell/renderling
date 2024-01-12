//! Renderling's SDF shaders.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

use crabslab::{Array, Id, Slab, SlabItem};
use glam::{vec2, vec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use renderling_shader_core::math::Float;

use renderling_shader_core::{
    math::{self, IsVector},
    Camera, Transform,
};
use spirv_std::spirv;

#[cfg(test)]
mod helper;

/// Returns the implicit position of the given index in clip space.
pub fn get_clip_position(index: usize) -> Vec3 {
    math::CLIP_QUAD_CCW[index % 6]
}

/// Returns the implicit uv coords of the given index.
pub fn get_uvs(index: usize) -> Vec2 {
    math::UV_COORD_QUAD_CCW[index % 6]
}

#[derive(Debug, Clone, Copy, SlabItem)]
pub struct Circle {
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

impl Circle {
    pub fn distance(&self, position: Vec2) -> f32 {
        position.length() - self.radius
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Vec2::new(-1.0, 0.0),
            end: Vec2::new(1.0, 0.0),
            thickness: 0.2,
        }
    }
}

impl Line {
    pub fn distance(&self, position: Vec2) -> f32 {
        let p = position;
        let a = self.start;
        let b = self.end;
        let pa = p - a;
        let ba = b - a;
        let h = math::clamp(pa.dot(ba) / ba.dot(ba), 0.0, 1.0);
        (pa - ba * h).length() - self.thickness * 0.5
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct Bezier {
    pub start: Vec2,
    pub control: Vec2,
    pub end: Vec2,
    pub thickness: f32,
}

impl Default for Bezier {
    fn default() -> Self {
        Self {
            start: Vec2::new(-1.0, 0.0),
            control: Vec2::new(0.0, 1.0),
            end: Vec2::new(1.0, 0.0),
            thickness: 0.2,
        }
    }
}

impl Bezier {
    fn distance(&self, pos: Vec2) -> f32 {
        let a = self.control - self.start;
        let b = self.start - 2.0 * self.control + self.end;
        let c = a * 2.0;
        let d = self.start - pos;

        let kk = 1.0 / b.dot(b);
        let kx = kk * a.dot(b);
        let ky = kk * (2.0 * a.dot2() + d.dot(b)) / 3.0;
        let kz = kk * d.dot(a);

        let p = ky - kx * kx;
        let p3 = p * p * p;
        let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
        let q2 = q * q;
        let h = q2 + 4.0 * p3;

        let res = if h >= 0.0 {
            let h = h.sqrt();
            let x = (vec2(h, -h) - q) / 2.0;
            let uv = x.signum_or_zero() * {
                let n = x.abs();
                let e = 1.0 / 3.0;
                n.powf(e)
            };
            let t = {
                let n = uv.x + uv.y - kx;
                math::clamp(n, 0.0, 1.0)
            };
            let r = d + (c + b * t) * t;
            r.dot(r)
        } else {
            let z = {
                let n = -p;
                n.sqrt()
            };
            let v = ({
                let n = q / (p * z * 2.0);
                n.acos()
            }) / 3.0;
            let m = v.cos();
            let n = v.sin() * 1.732050808;
            let t = {
                let n = vec3(m + m, -n - m, n - m) * z - kx;
                n.clamp(Vec3::splat(0.0), Vec3::splat(1.0))
            };
            {
                let a = d + (c + b * t.x) * t.x;
                a.dot(a)
            }
            .min({
                let a = d + (c + b * t.y) * t.y;
                a.dot(a)
            })
            // the third root cannot be the closest
            // res = min(res,dot2(d+(c+b*t.z)*t.z));
        };
        return res.sqrt() - self.thickness * 0.5;
    }
}

#[derive(Default, Clone, Copy, SlabItem)]
pub enum PathItem {
    #[default]
    None,
    Line(Id<Line>),
    Bezier(Id<Bezier>),
}

#[derive(Debug, Clone, Copy, SlabItem)]
pub struct Path {
    pub items: Array<PathItem>,
    pub thickness: f32,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            items: Default::default(),
            thickness: 0.2,
        }
    }
}

impl Path {
    pub fn distance(&self, pos: Vec2, slab: &[u32]) -> f32 {
        let mut distance = f32::MAX;
        for item_id in self.items.iter() {
            match slab.read(item_id) {
                PathItem::None => {}
                PathItem::Bezier(bez_id) => {
                    let bez = slab.read(bez_id);
                    distance = distance.min(bez.distance(pos));
                }
                PathItem::Line(line_id) => {
                    let line = slab.read(line_id);
                    distance = distance.min(line.distance(pos));
                }
            }
        }
        distance
    }
}

#[derive(Debug, Clone, Copy, SlabItem)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 2.0,
            height: 2.0,
        }
    }
}

impl Rectangle {
    pub fn distance(&self, position: Vec2) -> f32 {
        let half_size = Vec2::new(self.width, self.height) * 0.5;
        let componentwise_edge_distance = position.abs() - half_size;
        let outside_distance = componentwise_edge_distance.max(Vec2::ZERO).length();
        let inside_distance = componentwise_edge_distance
            .x
            .max(componentwise_edge_distance.y)
            .min(0.0);
        outside_distance + inside_distance
    }
}

#[derive(Default, Debug, Clone, Copy, SlabItem)]
pub enum SdfShape {
    #[default]
    None,
    Circle(Id<Circle>),
    Rectangle(Id<Rectangle>),
    Line(Id<Line>),
    Bezier(Id<Bezier>),
    Path(Id<Path>),
}

impl SdfShape {
    pub fn distance(&self, position: Vec2, slab: &[u32]) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Circle(id) => {
                let circle = slab.read(*id);
                circle.distance(position)
            }
            Self::Line(id) => {
                let line = slab.read(*id);
                line.distance(position)
            }
            Self::Bezier(id) => {
                let bez = slab.read(*id);
                bez.distance(position)
            }
            Self::Rectangle(id) => {
                let rectangle = slab.read(*id);
                rectangle.distance(position)
            }
            Self::Path(id) => {
                let path = slab.read(*id);
                path.distance(position, slab)
            }
        }
    }

    pub fn get_clip_position(&self, index: usize, _slab: &[u32]) -> Vec3 {
        match self {
            Self::None => Vec3::ZERO,
            _ => get_clip_position(index),
        }
    }

    pub fn get_uvs(&self, index: usize, _slab: &[u32]) -> Vec2 {
        match self {
            Self::None => Vec2::ZERO,
            _ => get_uvs(index),
        }
    }

    pub fn vertex_count(&self) -> u32 {
        match self {
            SdfShape::None => 0,
            _ => 6,
        }
    }
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Sdf {
    pub shape: SdfShape,
    pub transform: Transform,
    pub material: u32, // likely an Id<pbr::Material>
    pub camera: Id<Camera>,
}

impl Sdf {
    pub const fn vertex_count(&self) -> u32 {
        match self.shape {
            SdfShape::None => 0,
            _ => 6,
        }
    }

    pub fn distance(&self, position: Vec2, slab: &[u32]) -> f32 {
        self.shape.distance(position, slab)
    }
}

pub fn antialias_distance(distance: f32) -> f32 {
    let distance_change = spirv_std::arch::fwidth(distance);
    let opacity = math::smoothstep(distance_change, -distance_change, distance);
    opacity
}

#[derive(Clone, Copy, SlabItem)]
pub struct ShapeLegend {
    pub line_distance: f32,
    pub line_thickness: f32,
    pub inside_color: Vec4,
    pub outside_color: Vec4,
    pub shape: Id<SdfShape>,
}

impl Default for ShapeLegend {
    fn default() -> Self {
        Self {
            line_distance: 0.1,
            line_thickness: 0.0075,
            inside_color: math::hex_to_vec4(0x4e83b1ff),
            outside_color: math::hex_to_vec4(0x52b14eff),
            shape: Id::default(),
        }
    }
}

/// Vertex shader used to inspect and test SDF shapes.
#[spirv(vertex)]
pub fn sdf_shape_vertex(
    #[spirv(instance_index)] legend_id: Id<ShapeLegend>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] out_legend_id: &mut Id<ShapeLegend>,
    local_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    *out_legend_id = legend_id;
    let legend = slab.read(legend_id);
    let shape = slab.read(legend.shape);
    let position = shape.get_clip_position(vertex_index as usize, slab);
    *local_pos = position;
    *clip_pos = position.extend(1.0);
}

/// Fragment shader used to inspect and test SDF shapes.
#[spirv(fragment)]
pub fn sdf_shape_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] in_shape_legend: Id<ShapeLegend>,
    in_local_pos: Vec3,
    output: &mut Vec4,
) {
    let legend = slab.read(in_shape_legend);
    let shape = slab.read(legend.shape);
    let distance = shape.distance(in_local_pos.xy(), slab);
    let color = legend
        .inside_color
        .lerp(legend.outside_color, math::step(0.0, distance));
    let line_distance =
        (spirv_std::num_traits::Float::fract(distance.abs() / legend.line_distance + 0.5) - 0.5)
            .abs()
            * legend.line_distance;
    let half_distance_change = spirv_std::arch::fwidth(distance) * 0.5;
    let lines = math::smoothstep(
        legend.line_thickness - half_distance_change,
        legend.line_thickness + half_distance_change,
        line_distance,
    );
    *output = (color.xyz() * lines).extend(1.0);
}

#[cfg(test)]
mod test {
    use crabslab::GrowableSlab;

    use super::helper::SdfRenderer;
    use super::*;

    #[test]
    fn circle_sanity() {
        let circle = Circle { radius: 1.0 };
        assert_eq!(-circle.radius, circle.distance(Vec2::ZERO));
        assert_eq!(0.0, circle.distance(Vec2::new(1.0, 0.0)));
        assert_eq!(
            0.0,
            circle.distance(Vec2::new(
                f32::cos(std::f32::consts::FRAC_PI_4),
                f32::sin(std::f32::consts::FRAC_PI_4)
            ))
        );
        assert_eq!(1.0, circle.distance(Vec2::new(2.0, 0.0)));
        //assert_eq!(2.0, Vec2::ONE.length());
        //assert_eq!(0.0, Circle { radius: 1.0 }.distance(Vec2::ONE));
    }

    #[test]
    fn sdf_circle() {
        let mut r = SdfRenderer::new(256, 256);
        let circle_id = r.slab.append(&Circle { radius: 1.0 });
        let _ = r.set_shape(SdfShape::Circle(circle_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/circle.png", img);
    }

    #[test]
    fn sdf_line() {
        let mut r = SdfRenderer::new(256, 256);
        let line_id = r.slab.append(&Line {
            start: Vec2::new(-0.75, -0.75),
            end: Vec2::new(0.75, 0.75),
            thickness: 0.2,
        });
        let _ = r.set_shape(SdfShape::Line(line_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/line.png", img);
    }

    #[test]
    fn rect_sanity() {
        let rect = Rectangle {
            width: 2.0,
            height: 2.0,
        };
        assert_eq!(0.0, rect.distance(Vec2::ONE));
        assert_eq!(0.0, rect.distance(Vec2::new(1.0, 0.0)));
        assert_eq!(0.0, rect.distance(Vec2::new(0.0, 1.0)));
        assert_eq!(1.0, rect.distance(Vec2::new(2.0, 0.0)));
        assert_eq!(-1.0, rect.distance(Vec2::ZERO));
    }

    #[test]
    fn sdf_rect() {
        let mut r = SdfRenderer::new(256, 256);
        let rect_id = r.slab.append(&Rectangle {
            width: 1.4,
            height: 0.8,
        });
        let _ = r.set_shape(SdfShape::Rectangle(rect_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/rect.png", img);
    }

    #[test]
    fn bez_sanity() {
        assert!((-1.0f32).sqrt().is_nan());
    }

    #[test]
    fn sdf_bez() {
        let v0 = Vec2::new(-0.6384547, 0.6263999);
        let v1 = Vec2::new(0.9223702, 0.878696);
        let v2 = Vec2::new(0.26539552, -0.87759334);
        let mut r = SdfRenderer::new(256, 256);
        let bez_id = r.slab.append(&Bezier {
            start: v0,
            control: v1,
            end: v2,
            thickness: 0.2,
        });
        let _ = r.set_shape(SdfShape::Bezier(bez_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/bez.png", img);
    }

    #[test]
    fn sdf_path() {
        let percent = 0.6;
        let v0 = Vec2::new(-0.6384547, 0.6263999) * percent;
        let v1 = Vec2::new(0.9223702, 0.878696) * percent;
        let v2 = Vec2::new(0.26539552, -0.87759334) * percent;

        let mut r = SdfRenderer::new(256, 256);
        let bez_id = r.slab.append(&Bezier {
            start: v0,
            control: v1,
            end: v2,
            thickness: 0.2,
        });
        let line_id = r.slab.append(&Line {
            start: v0,
            end: v2,
            thickness: 0.2,
        });

        let items = r
            .slab
            .append_array(&[PathItem::Bezier(bez_id), PathItem::Line(line_id)]);
        let path_id = r.slab.append(&Path {
            items,
            thickness: 0.2,
        });
        let _ = r.set_shape(SdfShape::Path(path_id));
        let img = r.render_image();
        img_diff::save("sdf/path.png", img);
    }
}
