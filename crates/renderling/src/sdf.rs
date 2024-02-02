//! Renderling's SDF shaders.
#![deny(clippy::disallowed_methods)]

use crabslab::{Array, Id, Slab, SlabItem};
use glam::{vec2, vec3, BVec3, Vec2, Vec3, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use crate::math::Float;

use crate::{
    math::{self, IsVector},
    Camera, Transform,
};
use spirv_std::spirv;

#[cfg(test)]
mod helper;
pub mod raymarch;

/// Returns the implicit position of the given index in clip space
/// when being rendered in 2d.
pub const fn get_2d_clip_position(index: usize) -> Vec3 {
    math::CLIP_QUAD_CCW[index % 6]
}

/// Returns the implicit uv coords of the given index
/// when being rendered in 2d.
pub const fn get_2d_uvs(index: usize) -> Vec2 {
    math::UV_COORD_QUAD_CCW[index % 6]
}

/// A sphere shape in three dimensions or a circle in two dimensions.
#[derive(Debug, Clone, Copy, SlabItem)]
pub struct Sphere {
    pub radius: f32,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

impl Sphere {
    pub fn distance(&self, position: Vec3) -> f32 {
        position.length() - self.radius
    }

    pub fn closest_point(&self, position: Vec3) -> Vec3 {
        position.normalize() * self.radius
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Vec3::new(-1.0, 0.0, 0.0),
            end: Vec3::new(1.0, 0.0, 0.0),
            thickness: 0.2,
        }
    }
}

impl Line {
    pub fn distance(&self, position: Vec3) -> f32 {
        let p = position;
        let a = self.start;
        let b = self.end;
        let pa = p - a;
        let ba = b - a;
        let h = math::clamp(pa.dot(ba) / ba.dot(ba), 0.0, 1.0);
        (pa - ba * h).length() - self.thickness * 0.5
    }

    /// Returns the closest point on the line to the given position.
    pub fn closest_point(&self, position: Vec3) -> Vec3 {
        let p = position;
        let a = self.start;
        let b = self.end;
        let ap = p - a;
        let ab = b - a;
        let h = math::clamp(ap.dot(ab) / ab.dot(ab), 0.0, 1.0);
        // c = point on line closest to p
        let c = a + ab * h;
        let d = (p - c).alt_norm_or_zero();
        c + d * self.thickness * 0.5
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct Bezier {
    pub start: Vec3,
    pub control: Vec3,
    pub end: Vec3,
    pub thickness: f32,
}

impl Default for Bezier {
    fn default() -> Self {
        Self {
            start: Vec3::new(-1.0, 0.0, 0.0),
            control: Vec3::new(0.0, 1.0, 0.0),
            end: Vec3::new(1.0, 0.0, 0.0),
            thickness: 0.2,
        }
    }
}

impl Bezier {
    fn distance(&self, pos: Vec3) -> f32 {
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
    bezier: (Vec3, Vec3, Vec3),
    t: f32,
) -> ((Vec3, Vec3, Vec3), (Vec3, Vec3, Vec3)) {
    let (p0, p1, p2) = bezier;

    let p01 = p0.lerp(p1, t);
    let p12 = p1.lerp(p2, t);
    let p0112 = p01.lerp(p12, t);

    ((p0, p01, p0112), (p0112, p12, p2))
}

pub fn area_of_triangle(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b - a;
    let ac = c - a;
    let cross = ab.cross(ac);
    cross.length() * 0.5
}

impl Path {
    fn wind_sign(pos: Vec3, start: Vec3, end: Vec3) -> f32 {
        let e = end - start;
        let w = pos - start;
        let cond = BVec3::new(pos.y >= start.y, pos.y < end.y, e.x * w.y > e.y * w.x);
        if cond.all() || !cond.any() {
            -1.0
        } else {
            1.0
        }
    }

    pub fn bez_is_colinear_enough(dxy: Vec2, (a, b, c): (Vec3, Vec3, Vec3)) -> bool {
        let area = area_of_triangle(a, b, c);
        let one_pixel_area = dxy.x * dxy.y;
        let straight_line_area = (a - b).length() * one_pixel_area;
        area <= straight_line_area || area <= f32::EPSILON
    }

    pub fn distance(&self, pos: Vec3, slab: &[u32]) -> f32 {
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

#[derive(Clone, Copy, SlabItem)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct Cuboid {
    pub size: Vec3,
}

impl Default for Cuboid {
    fn default() -> Self {
        Self {
            size: Vec3::splat(2.0),
        }
    }
}

impl Cuboid {
    pub fn half_size(&self) -> Vec3 {
        self.size * 0.5
    }

    pub fn distance(&self, position: Vec3) -> f32 {
        let q = position.abs() - self.half_size();
        q.max(Vec3::ZERO).length() + q.x.max(q.y.max(q.z)).min(0.0)
    }
}

/// Translates an [`SdfShape`].
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct Transformed {
    pub shape: Id<SdfShape>,
    pub transform: Transform,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
#[repr(u32)]
pub enum ShapeType {
    #[default]
    None,
    Sphere,
    Cuboid,
    Line,
    Bezier,
    Path,
    Transformed,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct SdfShape {
    pub shape_type: ShapeType,
    pub shape_id: u32,
}

impl SdfShape {
    pub fn from_sphere(id: Id<Sphere>) -> Self {
        Self {
            shape_type: ShapeType::Sphere,
            shape_id: id.inner(),
        }
    }

    pub fn from_cuboid(id: Id<Cuboid>) -> Self {
        Self {
            shape_type: ShapeType::Cuboid,
            shape_id: id.inner(),
        }
    }

    pub fn from_line(id: Id<Line>) -> Self {
        Self {
            shape_type: ShapeType::Line,
            shape_id: id.inner(),
        }
    }

    pub fn from_bezier(id: Id<Bezier>) -> Self {
        Self {
            shape_type: ShapeType::Bezier,
            shape_id: id.inner(),
        }
    }

    pub fn from_path(id: Id<Path>) -> Self {
        Self {
            shape_type: ShapeType::Path,
            shape_id: id.inner(),
        }
    }

    pub fn from_transformed(id: Id<Transformed>) -> Self {
        Self {
            shape_type: ShapeType::Transformed,
            shape_id: id.inner(),
        }
    }

    pub fn distance(&self, mut position: Vec3, slab: &[u32]) -> f32 {
        let mut shape = *self;
        let mut adjustment = 1.0;
        let distance;
        loop {
            match shape.shape_type {
                ShapeType::None => return 0.0,
                ShapeType::Sphere => {
                    let circle = slab.read(Id::<Sphere>::from(shape.shape_id));
                    distance = circle.distance(position);
                    break;
                }
                ShapeType::Line => {
                    let id = Id::<Line>::from(shape.shape_id);
                    let line = slab.read(id);
                    distance = line.distance(position);
                    break;
                }
                ShapeType::Bezier => {
                    let id = Id::<Bezier>::from(shape.shape_id);
                    let bez = slab.read(id);
                    distance = bez.distance(position);
                    break;
                }
                ShapeType::Cuboid => {
                    let id = Id::<Cuboid>::from(shape.shape_id);
                    let rectangle = slab.read(id);
                    distance = rectangle.distance(position);
                    break;
                }
                ShapeType::Path => {
                    let id = Id::<Path>::from(shape.shape_id);
                    let path = slab.read(id);
                    distance = path.distance(position, slab);
                    break;
                }
                ShapeType::Transformed => {
                    let id = Id::<Transformed>::from(shape.shape_id);
                    let Transformed {
                        shape: shape_id,
                        transform,
                    } = slab.read(id);
                    shape = slab.read(shape_id);
                    let Transform {
                        translation,
                        rotation,
                        scale,
                    } = transform;
                    position = position / scale;
                    adjustment *= scale.x.min(scale.y).min(scale.z);
                    position = rotation.inverse().mul_vec3(position);
                    position -= translation;
                    continue;
                }
            };
        }
        distance * adjustment
    }
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Scene {
    pub shapes: Array<SdfShape>,
    pub camera: Id<Camera>,
}

impl Scene {
    pub const fn vertex_count(&self) -> u32 {
        6
    }

    pub fn distance(&self, position: Vec3, slab: &[u32]) -> f32 {
        let mut distance = f32::MAX;
        for id in self.shapes.iter() {
            let shape = slab.read(id);
            distance = shape.distance(position, slab).min(distance);
        }
        distance
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct ShapeLegend {
    pub line_distance: f32,
    pub line_thickness: f32,
    pub inside_color: Vec4,
    pub outside_color: Vec4,
    pub shape: Id<SdfShape>,
    pub debug_points: Array<Vec3>,
}

impl Default for ShapeLegend {
    fn default() -> Self {
        Self {
            line_distance: 0.1,
            line_thickness: 0.0075,
            inside_color: math::hex_to_vec4(0x4e83b1ff),
            outside_color: math::hex_to_vec4(0x52b14eff),
            shape: Id::default(),
            debug_points: Array::default(),
        }
    }
}

/// Vertex shader used to inspect and test SDF shapes.
#[spirv(vertex)]
pub fn sdf_shape_vertex(
    #[spirv(instance_index)] legend_id: Id<ShapeLegend>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] _slab: &[u32],
    #[spirv(flat)] out_legend_id: &mut Id<ShapeLegend>,
    local_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    *out_legend_id = legend_id;
    let position = get_2d_clip_position(vertex_index as usize);
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
    let distance = shape.distance(in_local_pos, slab);
    let shape_color = color_distance(
        legend.inside_color,
        legend.outside_color,
        legend.line_thickness,
        legend.line_distance,
        distance,
    );
    *output = shape_color;
    for point_id in legend.debug_points.iter() {
        let point = slab.read(point_id);
        let adist = distance.abs();
        if adist <= 0.1 {
            *output *= Vec3::splat(adist / 0.1).extend(1.0);
        }
        let radius = 0.05;
        let distance = in_local_pos.distance(point);
        let distance = distance - radius;
        if distance <= 0.0 {
            *output *= Vec4::new(1.0, 0.5, 0.5, 1.0);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crabslab::GrowableSlab;

    use super::{helper::SdfRenderer, *};

    #[test]
    fn circle_sanity() {
        let circle = Sphere { radius: 1.0 };
        assert_eq!(-circle.radius, circle.distance(Vec3::ZERO));
        assert_eq!(0.0, circle.distance(Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(
            0.0,
            circle.distance(Vec3::new(
                f32::cos(std::f32::consts::FRAC_PI_4),
                f32::sin(std::f32::consts::FRAC_PI_4),
                0.0
            ))
        );
        assert_eq!(1.0, circle.distance(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn sdf_circle() {
        let mut r = SdfRenderer::new(256, 256);
        let circle_id = r.slab.append(&Sphere { radius: 1.0 });
        let _ = r.set_shape(SdfShape::from_sphere(circle_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/circle.png", img);
    }

    #[test]
    fn sdf_line() {
        let mut r = SdfRenderer::new(256, 256);
        let line = Line {
            start: Vec3::new(-0.75, -0.75, 0.0),
            end: Vec3::new(0.75, 0.75, 0.0),
            thickness: 0.2,
        };
        let line_id = r.slab.append(&line);
        let _ = r.set_shape(SdfShape::from_line(line_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/line.png", img);
    }

    #[test]
    fn rect_sanity() {
        let rect = Cuboid {
            size: Vec3::splat(2.0),
        };
        assert_eq!(0.0, rect.distance(Vec3::ONE));
        assert_eq!(0.0, rect.distance(Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(0.0, rect.distance(Vec3::new(0.0, 1.0, 0.0)));
        assert_eq!(1.0, rect.distance(Vec3::new(2.0, 0.0, 0.0)));
        assert_eq!(-1.0, rect.distance(Vec3::ZERO));
    }

    #[test]
    fn sdf_rect() {
        let mut r = SdfRenderer::new(256, 256);
        let rect_id = r.slab.append(&Cuboid {
            size: Vec3::new(1.4, 0.8, 1.0),
        });
        let _ = r.set_shape(SdfShape::from_cuboid(rect_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/rect.png", img);
    }

    #[test]
    fn bez_sanity() {
        assert!((-1.0f32).sqrt().is_nan());
    }

    #[test]
    fn sdf_bez() {
        let v0 = Vec3::new(-0.6384547, 0.6263999, 0.0);
        let v1 = Vec3::new(0.9223702, 0.878696, 0.0);
        let v2 = Vec3::new(0.26539552, -0.87759334, 0.0);
        let mut r = SdfRenderer::new(256, 256);
        let bez_id = r.slab.append(&Bezier {
            start: v0,
            control: v1,
            end: v2,
            thickness: 0.2,
        });
        let _ = r.set_shape(SdfShape::from_bezier(bez_id));
        let img = r.render_image();
        img_diff::assert_img_eq("sdf/bez.png", img);
    }

    #[test]
    fn sdf_path() {
        let percent = 0.6;
        let a = Vec3::new(-0.6384547, 0.6263999, 0.0) * percent;
        let b = Vec3::new(0.9223702, 0.878696, 0.0) * percent;
        let c = Vec3::new(0.26539552, -0.87759334, 0.0) * percent;

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
        let _ = r.set_shape(SdfShape::from_path(path_id));

        let position = Vec2::new(166.0, 73.0);
        let position = position / 256.0; // now x and y are between [0, 1]
        let position = position * 2.0 - 1.0; // now [-1, 1]
        let position = Vec2::new(position.x, -position.y); // flip y
        r.set_debug_points([position.extend(0.0)]);
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
        let a = Vec3::new(-0.6384547, 0.6263999, 0.0) * percent;
        let b = Vec3::new(0.9223702, 0.878696, 0.0) * percent;
        let c = Vec3::new(0.26539552, -0.87759334, 0.0) * percent;

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
        let _ = r.set_shape(SdfShape::from_path(path_id));

        let position = Vec2::new(166.0, 73.0);
        let position = position / 256.0; // now x and y are between [0, 1]
        let position = position * 2.0 - 1.0; // now [-1, 1]
        let position = Vec2::new(position.x, -position.y); // flip y
        r.set_debug_points([position.extend(0.0)]);
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
            let a = Vec3::new(-0.6384547, 0.6263999, 0.0) * percent;
            let b = Vec3::new(0.9223702, 0.878696, 0.0) * percent;
            let c = Vec3::new(0.26539552, -0.87759334, 0.0) * percent;
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
        r.set_debug_points([Vec3::ZERO]);
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
        let _ = r.set_shape(SdfShape::from_path(path_id));

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

        #[allow(dead_code)]
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
                let mut last = Vec3::ZERO;
                let mut items = vec![];

                fn add_line(
                    r: &mut SdfRenderer,
                    p: Vec3,
                    last: &mut Vec3,
                    items: &mut Vec<PathItem>,
                ) -> Option<SdfShape> {
                    let line_id = r.slab.append(&Line {
                        start: *last,
                        end: p,
                        thickness: 0.0,
                    });
                    *last = p;
                    items.push(PathItem::Line(line_id));
                    Some(SdfShape::from_line(line_id))
                }

                let shapes = self
                    .items
                    .iter()
                    .filter_map(|item| {
                        let shape = match item {
                            Outline::MoveTo(p) => {
                                last = p.extend(0.0);
                                None
                            }
                            Outline::LineTo(p) => add_line(r, p.extend(0.0), &mut last, &mut items),
                            Outline::QuadTo(b, c) => {
                                let a = last;
                                let b = b.extend(0.0);
                                let c = c.extend(0.0);
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
                                    Some(SdfShape::from_bezier(bez_id))
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
                                Some(SdfShape::from_line(line_id))
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
                r.set_shape(SdfShape::from_path(outline.path_id));

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

    #[test]
    fn sdf_sanity_closest_point() {
        let sphere = Sphere { radius: 3.2 };
        let point = Vec3::new(1.0, 2.0, 3.0);
        let closest_point = sphere.closest_point(point);
        assert_eq!(0.0, sphere.distance(closest_point));
        let distance = sphere.distance(point);
        assert_approx_eq::assert_approx_eq!((point - closest_point).length(), distance, 1e-6);

        let line = Line {
            start: Vec3::new(-1.0, 0.0, 0.0),
            end: Vec3::new(1.0, 0.0, 0.0),
            thickness: 0.0,
        };
        let point = Vec3::new(0.0, 1.0, 0.0);
        let closest_point = line.closest_point(point);
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), closest_point);

        let line = Line {
            start: Vec3::new(1.0, 2.0, 0.0),
            end: Vec3::new(4.0, 4.0, 0.0),
            thickness: 0.0,
        };
        let point = Vec3::new(2.0, 5.0, 0.0);
        let closest_point = sphere.closest_point(point);
        println!("closest_point: {:?}", closest_point);
        assert_eq!(0.0, line.distance(closest_point));
        let distance = line.distance(point);
        assert_approx_eq::assert_approx_eq!((point - closest_point).length(), distance, 1e-6);
    }
}
