//! Renderling's SDF shaders.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

use crabslab::{Array, Id, Slab, SlabItem};
use glam::{vec2, vec3, BVec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use renderling_shader_core::math::Float;

use renderling_shader_core::{
    math::{self, IsVector},
    Camera, Transform,
};
use spirv_std::spirv;

#[cfg(test)]
mod helper;
//mod winding;

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
    pub filled: bool,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            items: Default::default(),
            thickness: 0.2,
            filled: false,
        }
    }
}

/// DeCasteljau's bezier splitting algorithm.
pub fn split_bezier(
    bezier: (Vec2, Vec2, Vec2),
    t: f32,
) -> ((Vec2, Vec2, Vec2), (Vec2, Vec2, Vec2)) {
    let (p0, p1, p2) = bezier;

    let p01 = p0.lerp(p1, t);
    let p12 = p1.lerp(p2, t);
    let p0112 = p01.lerp(p12, t);

    ((p0, p01, p0112), (p0112, p12, p2))
}

pub fn area_of_triangle(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    let ab = b - a;
    let ac = c - a;
    let cross = ab.x * ac.y - ab.y * ac.x;
    cross.abs() * 0.5
}

impl Path {
    fn wind_sign(pos: Vec2, start: Vec2, end: Vec2) -> f32 {
        let e = end - start;
        let w = pos - start;
        let cond = BVec3::new(pos.y >= start.y, pos.y < end.y, e.x * w.y > e.y * w.x);
        if cond.all() || !cond.any() {
            -1.0
        } else {
            1.0
        }
    }

    pub fn bez_is_colinear_enough(dxy: Vec2, (a, b, c): (Vec2, Vec2, Vec2)) -> bool {
        let area = area_of_triangle(a, b, c);
        let one_pixel_area = dxy.x * dxy.y;
        let straight_line_area = (a - b).length() * one_pixel_area;
        area <= straight_line_area || area <= f32::EPSILON
    }

    pub fn distance(&self, pos: Vec2, slab: &[u32]) -> f32 {
        let mut distance = f32::MAX;
        let mut sign = 1.0;
        let pos_change = Vec2::new(
            spirv_std::arch::ddx(pos.x).abs(),
            spirv_std::arch::ddy(pos.y).abs(),
        );
        for item_id in self.items.iter() {
            let item_distance = match slab.read(item_id) {
                PathItem::None => {
                    continue;
                }
                PathItem::Bezier(bez_id) => {
                    let bez = slab.read(bez_id);
                    let distance = bez.distance(pos);
                    if self.filled {
                        let mut remaining_bez = (bez.start, bez.control, bez.end);
                        loop {
                            if Self::bez_is_colinear_enough(pos_change, remaining_bez) {
                                break;
                            }
                            let mut t = 1.0;
                            let mut bez_a;
                            let mut bez_b;
                            loop {
                                // Cut `t` in half, split the remaining bezier at `t` and check the
                                // area of the triangle.
                                t *= 0.5;
                                let (a, b) = split_bezier(remaining_bez, t);
                                bez_a = a;
                                bez_b = b;
                                if Self::bez_is_colinear_enough(pos_change, bez_a) {
                                    break;
                                }
                            }
                            // Now we know `bez_a` has an area smaller than a pixel, we can update
                            // our winding
                            sign *= Self::wind_sign(pos, bez_a.0, bez_a.2);
                            remaining_bez = bez_b;
                        }
                        sign *= Self::wind_sign(pos, remaining_bez.0, remaining_bez.2);
                    }
                    distance
                }
                PathItem::Line(line_id) => {
                    let line = slab.read(line_id);
                    let distance = line.distance(pos);
                    if self.filled {
                        sign *= Self::wind_sign(pos, line.start, line.end);
                    }
                    distance
                }
            };
            distance = distance.min(item_distance);
        }

        let thickness = self.thickness * 0.5;
        sign * distance - thickness
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
    pub debug_point: Vec2,
}

impl Default for ShapeLegend {
    fn default() -> Self {
        Self {
            line_distance: 0.1,
            line_thickness: 0.0075,
            inside_color: math::hex_to_vec4(0x4e83b1ff),
            outside_color: math::hex_to_vec4(0x52b14eff),
            shape: Id::default(),
            debug_point: Vec2::MAX,
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

pub fn color_distance(
    inside_color: Vec4,
    outside_color: Vec4,
    line_thickness: f32,
    line_distance: f32,
    distance: f32,
) -> Vec4 {
    let color = inside_color.lerp(outside_color, math::step(0.0, distance));
    let line_distance =
        (spirv_std::num_traits::Float::fract(distance.abs() / line_distance + 0.5) - 0.5).abs()
            * line_distance;
    let half_distance_change = spirv_std::arch::fwidth(distance) * 0.5;
    let lines = math::smoothstep(
        line_thickness - half_distance_change,
        line_thickness + half_distance_change,
        line_distance,
    );
    (color.xyz() * lines).extend(1.0)
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
    let shape_color = color_distance(
        legend.inside_color,
        legend.outside_color,
        legend.line_thickness,
        legend.line_distance,
        distance,
    );
    *output = shape_color;
    if legend.debug_point != Vec2::MAX {
        let adist = distance.abs();
        if adist <= 0.1 {
            *output *= Vec3::splat(adist / 0.1).extend(1.0);
        }
        let radius = 0.05;
        let distance = in_local_pos.xy().distance(legend.debug_point);
        let distance = distance - radius;
        if distance <= 0.0 {
            *output *= Vec4::new(1.0, 0.5, 0.5, 1.0);
        }
    }
}

#[cfg(test)]
mod test {
    use crabslab::GrowableSlab;

    use super::{helper::SdfRenderer, *};

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
        let a = Vec2::new(-0.6384547, 0.6263999) * percent;
        let b = Vec2::new(0.9223702, 0.878696) * percent;
        let c = Vec2::new(0.26539552, -0.87759334) * percent;

        let mut r = SdfRenderer::new(256, 256);
        let ab_id = r.slab.append(&Line {
            start: a,
            end: b,
            thickness: 0.0,
        });
        let bc_id = r.slab.append(&Line {
            start: b,
            end: c,
            thickness: 0.0,
        });
        let ca_id = r.slab.append(&Line {
            start: c,
            end: a,
            thickness: 0.0,
        });

        let items = r.slab.append_array(&[
            PathItem::Line(ab_id),
            PathItem::Line(bc_id),
            PathItem::Line(ca_id),
        ]);
        let mut path = Path {
            items,
            thickness: 0.0,
            filled: true,
        };
        let path_id = r.slab.append(&path);
        let _ = r.set_shape(SdfShape::Path(path_id));

        let position = Vec2::new(166.0, 73.0);
        let position = position / 256.0; // now x and y are between [0, 1]
        let position = position * 2.0 - 1.0; // now [-1, 1]
        let position = Vec2::new(position.x, -position.y); // flip y
        r.set_debug_point(position);
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_path_thickness_0.png", img);

        path.thickness = 0.2;
        r.slab
            .write(path_id + Path::offset_of_thickness(), &path.thickness);
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_path_thickness_0.2.png", img);
    }

    #[test]
    fn sdf_bez_path() {
        let percent = 0.6;
        let a = Vec2::new(-0.6384547, 0.6263999) * percent;
        let b = Vec2::new(0.9223702, 0.878696) * percent;
        let c = Vec2::new(0.26539552, -0.87759334) * percent;

        let mut r = SdfRenderer::new(256, 256);
        let bez_id = r.slab.append(&Bezier {
            start: a,
            control: b,
            end: c,
            thickness: 0.0,
        });
        let ca_id = r.slab.append(&Line {
            start: c,
            end: a,
            thickness: 0.0,
        });

        let items = r
            .slab
            .append_array(&[PathItem::Bezier(bez_id), PathItem::Line(ca_id)]);
        let mut path = Path {
            items,
            thickness: 0.0,
            filled: true,
        };
        let path_id = r.slab.append(&path);
        let _ = r.set_shape(SdfShape::Path(path_id));

        let position = Vec2::new(166.0, 73.0);
        let position = position / 256.0; // now x and y are between [0, 1]
        let position = position * 2.0 - 1.0; // now [-1, 1]
        let position = Vec2::new(position.x, -position.y); // flip y
        r.set_debug_point(position);
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_bez_path_thickness_0.png", img);

        path.thickness = 0.2;
        r.slab
            .write(path_id + Path::offset_of_thickness(), &path.thickness);
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_bez_path_thickness_0.2.png", img);
    }

    #[test]
    fn sdf_bez_path_holes() {
        fn get_items(r: &mut SdfRenderer, percent: f32) -> [PathItem; 2] {
            let a = Vec2::new(-0.6384547, 0.6263999) * percent;
            let b = Vec2::new(0.9223702, 0.878696) * percent;
            let c = Vec2::new(0.26539552, -0.87759334) * percent;
            let bez_id = r.slab.append(&Bezier {
                start: a,
                control: b,
                end: c,
                thickness: 0.0,
            });
            let ca_id = r.slab.append(&Line {
                start: c,
                end: a,
                thickness: 0.0,
            });
            [PathItem::Bezier(bez_id), PathItem::Line(ca_id)]
        }

        let mut r = SdfRenderer::new(256, 256);
        r.set_debug_point(Vec2::ZERO);
        let outer = get_items(&mut r, 1.0);
        let inner = get_items(&mut r, 0.4);

        let items = r
            .slab
            .append_array(&[outer[0], outer[1], inner[0], inner[1]]);

        let mut path = Path {
            items,
            thickness: 0.0,
            filled: true,
        };
        let path_id = r.slab.append(&path);
        let _ = r.set_shape(SdfShape::Path(path_id));

        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_bez_path_holes_thickness_0.png", img);

        path.thickness = 0.2;
        r.slab
            .write(path_id + Path::offset_of_thickness(), &path.thickness);
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/filled_bez_path_holes_thickness_0.2.png", img);
    }

    #[test]
    fn sdf_bez_path_font_face() {
        use ttf_parser::OutlineBuilder;

        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Warn)
            .filter_module("crabslab", log::LevelFilter::Trace)
            .is_test(true)
            .try_init();

        #[derive(Debug, Clone, Copy)]
        enum Outline {
            MoveTo(Vec2),
            LineTo(Vec2),
            QuadTo(Vec2, Vec2),
            CubicTo(Vec2, Vec2, Vec2),
            Close,
        }

        struct FaceOutline {
            path_id: Id<Path>,
            shapes: Vec<SdfShape>,
        }

        struct Builder {
            items: Vec<Outline>,
            global_bbox: (Vec2, Vec2),
        }

        impl Builder {
            pub fn new(face: &ttf_parser::Face) -> Self {
                let ttf_parser::Rect {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                } = face.global_bounding_box();
                Self {
                    items: Vec::new(),
                    global_bbox: (
                        Vec2::new(x_min as f32, y_min as f32),
                        Vec2::new(x_max as f32, y_max as f32),
                    ),
                }
            }

            /// Converts a point from the font's coordinate system to the origin
            /// of the glyph.
            pub fn to_origin(&self, p: Vec2) -> Vec2 {
                let (min, max) = self.global_bbox;
                ((p - min) / (max - min)) * 2.0 - 1.0
            }

            pub fn build(self, r: &mut SdfRenderer) -> FaceOutline {
                let mut start = None;
                let mut last = Vec2::ZERO;
                let mut items = vec![];

                fn add_line(
                    r: &mut SdfRenderer,
                    p: Vec2,
                    last: &mut Vec2,
                    items: &mut Vec<PathItem>,
                ) -> Option<SdfShape> {
                    let line_id = r.slab.append(&Line {
                        start: *last,
                        end: p,
                        thickness: 0.0,
                    });
                    *last = p;
                    items.push(PathItem::Line(line_id));
                    Some(SdfShape::Line(line_id))
                };


                let shapes = self
                    .items
                    .iter()
                    .filter_map(|item| {
                        let i = items.len();
                        let shape = match item {
                            Outline::MoveTo(p) => {
                                last = *p;
                                None
                            }
                            Outline::LineTo(p) => add_line(r, *p, &mut last, &mut items),
                            Outline::QuadTo(b, c) => {
                                let a = last;
                                let b = *b;
                                let c = *c;
                                if area_of_triangle(a, b, c) <= f32::EPSILON {
                                    add_line(r, c, &mut last, &mut items)
                                } else {
                                    let bez = Bezier {
                                        start: a,
                                        control: b,
                                        end: c,
                                        thickness: 0.0,
                                    };
                                    let bez_id = r.slab.append(&bez);
                                    last = c;
                                    items.push(PathItem::Bezier(bez_id));
                                    Some(SdfShape::Bezier(bez_id))
                                }
                            }
                            Outline::CubicTo(_, _, _) => {
                                panic!("CubicTo not supported");
                            }
                            Outline::Close => {
                                let line_id = r.slab.append(&Line {
                                    start: last,
                                    end: start.unwrap(),
                                    thickness: 0.0,
                                });
                                start = None;
                                items.push(PathItem::Line(line_id));
                                Some(SdfShape::Line(line_id))
                            }
                        };
                        if start.is_none() && !matches!(item, Outline::Close) {
                            start = Some(last);
                        }
                        shape
                    })
                    .collect();

                let items = r.slab.append_array(&items);
                let path = Path {
                    items,
                    thickness: 0.0,
                    filled: true,
                };
                let path_id = r.slab.append(&path);
                FaceOutline { path_id, shapes }
            }
        }

        impl OutlineBuilder for Builder {
            fn move_to(&mut self, x: f32, y: f32) {
                self.items
                    .push(Outline::MoveTo(self.to_origin(Vec2::new(x, y))));
            }

            fn line_to(&mut self, x: f32, y: f32) {
                self.items
                    .push(Outline::LineTo(self.to_origin(Vec2::new(x, y))));
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.items.push(Outline::QuadTo(
                    self.to_origin(Vec2::new(x1, y1)),
                    self.to_origin(Vec2::new(x, y)),
                ));
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.items.push(Outline::CubicTo(
                    self.to_origin(Vec2::new(x1, y1)),
                    self.to_origin(Vec2::new(x2, y2)),
                    self.to_origin(Vec2::new(x, y)),
                ));
            }

            fn close(&mut self) {
                self.items.push(Outline::Close);
            }
        }
        let data =
            std::fs::read("../../fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        println!("face_global_bbox: {:?}", face.global_bounding_box());
        let mut r = SdfRenderer::new_with_capacity(256, 256, 32768);

        let chars = "a@%}!".chars();
        for c in chars {
            println!("char: {c}");
            let glyph_index = face.glyph_index(c).unwrap();
            let mut builder = Builder::new(&face);
            if let Some(_) = face.outline_glyph(glyph_index, &mut builder) {
                let outline = builder.build(&mut r);
                r.set_shape(SdfShape::Path(outline.path_id));

                let img = r.render_image();
                let filename = format!("{c}")
                    .replace("/", "forward_slash")
                    .replace("\\", "back_slash")
                    .replace("%", "percent")
                    .replace("}", "close_brace");
                img_diff::assert_img_eq(
                    &format!("sdf/filled_bez_path_font_face/{filename}.png"),
                    img,
                );
                //if c == 'x' {
                //    for (i, shape) in outline.shapes.iter().enumerate() {
                //        r.set_shape(*shape);
                //        let img = r.render_image();
                //        img_diff::save(&format!("sdf/
                // filled_bez_path_font_face/{c}_{i}.png"), img);
                //    }
                //}
            }
        }
    }
}
