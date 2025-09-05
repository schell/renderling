//! Bounding volumes and culling primitives.
//!
//! The initial implementation here was gleaned from `treeculler`, which
//! unfortunately cannot compile to SPIR-V because of its use of `u8`.
//!
//! Also, here we use `glam`, whereas `treeculler` uses its own internal
//! primitives.
//!
//! More resources:
//! * <https://fgiesen.wordpress.com/2010/10/17/view-frustum-culling/>
//! * <http://old.cescg.org/CESCG-2002/DSykoraJJelinek/>
//! * <https://iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm>

use crabslab::SlabItem;
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
#[cfg(gpu)]
use spirv_std::num_traits::Float;

use crate::{camera::CameraDescriptor, transform::TransformDescriptor};

/// Normalize a plane.
pub fn normalize_plane(mut plane: Vec4) -> Vec4 {
    let normal_magnitude = (plane.x.powi(2) + plane.y.powi(2) + plane.z.powi(2))
        .sqrt()
        .max(f32::EPSILON);
    plane.x /= normal_magnitude;
    plane.y /= normal_magnitude;
    plane.z /= normal_magnitude;
    plane.w /= normal_magnitude;
    plane
}

/// Find the intersection point of three planes.
///
/// # Notes
/// This assumes that the planes will not intersect in a line.
pub fn intersect_planes(p0: &Vec4, p1: &Vec4, p2: &Vec4) -> Vec3 {
    let bxc = p1.xyz().cross(p2.xyz());
    let cxa = p2.xyz().cross(p0.xyz());
    let axb = p0.xyz().cross(p1.xyz());
    let r = -bxc * p0.w - cxa * p1.w - axb * p2.w;
    r * (1.0 / bxc.dot(p0.xyz()))
}

/// Calculates distance between plane and point
pub fn dist_bpp(plane: &Vec4, point: Vec3) -> f32 {
    plane.x * point.x + plane.y * point.y + plane.z * point.z + plane.w
}

/// Calculates the most inside vertex of an AABB.
pub fn mi_vertex(plane: &Vec4, aabb: &Aabb) -> Vec3 {
    Vec3::new(
        if plane.x >= 0.0 {
            aabb.max.x
        } else {
            aabb.min.x
        },
        if plane.y >= 0.0 {
            aabb.max.y
        } else {
            aabb.min.y
        },
        if plane.z >= 0.0 {
            aabb.max.z
        } else {
            aabb.min.z
        },
    )
}

/// Calculates the most outside vertex of an AABB.
pub fn mo_vertex(plane: &Vec4, aabb: &Aabb) -> Vec3 {
    Vec3::new(
        if plane.x >= 0.0 {
            aabb.min.x
        } else {
            aabb.max.x
        },
        if plane.y >= 0.0 {
            aabb.min.y
        } else {
            aabb.max.y
        },
        if plane.z >= 0.0 {
            aabb.min.z
        } else {
            aabb.max.z
        },
    )
}

/// Axis aligned bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq, SlabItem)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl From<(Vec3, Vec3)> for Aabb {
    fn from((a, b): (Vec3, Vec3)) -> Self {
        Aabb::new(a, b)
    }
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Return the length along the x axis.
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Return the length along the y axis.
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Return the length along the z axis.
    pub fn depth(&self) -> f32 {
        self.max.z - self.min.z
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn extents(&self) -> Vec3 {
        self.max - self.center()
    }

    pub fn diagonal_length(&self) -> f32 {
        self.min.distance(self.max)
    }

    pub fn is_zero(&self) -> bool {
        self.min == self.max
    }

    /// Returns the union of the two [`Aabbs`].
    pub fn union(a: Self, b: Self) -> Self {
        Aabb {
            min: a.min.min(a.max).min(b.min).min(b.max),
            max: a.max.max(a.min).max(b.max).max(b.min),
        }
    }

    /// Determines whether this `Aabb` can be seen by `camera` after being
    /// transformed by `transform`.
    pub fn is_outside_camera_view(
        &self,
        camera: &CameraDescriptor,
        transform: TransformDescriptor,
    ) -> bool {
        let transform = Mat4::from(transform);
        let min = transform.transform_point3(self.min);
        let max = transform.transform_point3(self.max);
        Aabb::new(min, max).is_inside_frustum(camera.frustum())
    }

    #[cfg(not(target_arch = "spirv"))]
    /// Return a triangle mesh connecting this `Aabb`'s corners.
    ///
    /// ```ignore
    ///    y           1_____2     _____
    ///    |           /    /|    /|    |  (same box, left and front sides removed)
    ///    |___x     0/___3/ |   /7|____|6
    ///   /           |    | /   | /    /
    /// z/            |____|/   4|/____/5
    ///
    /// 7 is min
    /// 3 is max
    /// ```
    pub fn get_mesh(&self) -> Vec<(Vec3, Vec3)> {
        let p0 = Vec3::new(self.min.x, self.max.y, self.max.z);
        let p1 = Vec3::new(self.min.x, self.max.y, self.min.z);
        let p2 = Vec3::new(self.max.x, self.max.y, self.min.z);
        let p3 = Vec3::new(self.max.x, self.max.y, self.max.z);
        let p4 = Vec3::new(self.min.x, self.min.y, self.max.z);
        let p7 = Vec3::new(self.min.x, self.min.y, self.min.z);
        let p6 = Vec3::new(self.max.x, self.min.y, self.min.z);
        let p5 = Vec3::new(self.max.x, self.min.y, self.max.z);

        let positions = crate::math::convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7]);
        positions
            .chunks_exact(3)
            .flat_map(|chunk| match chunk {
                [a, b, c] => {
                    let n = crate::math::triangle_face_normal(*a, *b, *c);
                    [(*a, n), (*b, n), (*c, n)]
                }
                _ => unreachable!(),
            })
            .collect()
    }

    /// Returns whether this `Aabb` intersects another `Aabb`.
    ///
    /// Returns `false` if the two are touching, but not overlapping.
    pub fn intersects_aabb(&self, other: &Aabb) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }
}

/// Six planes of a view frustum.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct Frustum {
    /// Planes constructing the sides of the frustum,
    /// each expressed as a normal vector (xyz) and the distance (w)
    /// from the origin along that vector.
    pub planes: [Vec4; 6],
    /// Points representing the corners of the frustum
    pub points: [Vec3; 8],
    /// Centroid of the corners of the frustum
    pub center: Vec3,
}

impl Frustum {
    /// Compute a frustum in world space from the given [`Camera`].
    pub fn from_camera(camera: &CameraDescriptor) -> Self {
        let viewprojection = camera.view_projection();
        let mvp = viewprojection.to_cols_array_2d();

        let left = normalize_plane(Vec4::new(
            mvp[0][0] + mvp[0][3],
            mvp[1][0] + mvp[1][3],
            mvp[2][0] + mvp[2][3],
            mvp[3][0] + mvp[3][3],
        ));
        let right = normalize_plane(Vec4::new(
            -mvp[0][0] + mvp[0][3],
            -mvp[1][0] + mvp[1][3],
            -mvp[2][0] + mvp[2][3],
            -mvp[3][0] + mvp[3][3],
        ));
        let bottom = normalize_plane(Vec4::new(
            mvp[0][1] + mvp[0][3],
            mvp[1][1] + mvp[1][3],
            mvp[2][1] + mvp[2][3],
            mvp[3][1] + mvp[3][3],
        ));
        let top = normalize_plane(Vec4::new(
            -mvp[0][1] + mvp[0][3],
            -mvp[1][1] + mvp[1][3],
            -mvp[2][1] + mvp[2][3],
            -mvp[3][1] + mvp[3][3],
        ));
        let near = normalize_plane(Vec4::new(
            mvp[0][2] + mvp[0][3],
            mvp[1][2] + mvp[1][3],
            mvp[2][2] + mvp[2][3],
            mvp[3][2] + mvp[3][3],
        ));
        let far = normalize_plane(Vec4::new(
            -mvp[0][2] + mvp[0][3],
            -mvp[1][2] + mvp[1][3],
            -mvp[2][2] + mvp[2][3],
            -mvp[3][2] + mvp[3][3],
        ));

        // Account for the possibility that the projection is infinite.
        //
        // See <https://renderling.xyz/devlog/index.html#actual_frustum_culling>
        // for more details.
        let far = (-1.0 * near.xyz()).extend(far.w);

        let flt = intersect_planes(&far, &left, &top);
        let frt = intersect_planes(&far, &right, &top);
        let flb = intersect_planes(&far, &left, &bottom);
        let frb = intersect_planes(&far, &right, &bottom);
        let nlt = intersect_planes(&near, &left, &top);
        let nrt = intersect_planes(&near, &right, &top);
        let nlb = intersect_planes(&near, &left, &bottom);
        let nrb = intersect_planes(&near, &right, &bottom);

        Self {
            center: (nlt + nrt + nlb + nrb) / 4.0,
            planes: [near, left, right, bottom, top, far],
            points: [nlt, nrt, nlb, nrb, flt, frt, flb, frb],
        }
    }

    #[cfg(not(target_arch = "spirv"))]
    /// Return a triangle mesh connecting this `Frustum`'s corners.
    pub fn get_mesh(&self) -> Vec<(Vec3, Vec3)> {
        let [nlt, nrt, nlb, nrb, flt, frt, flb, frb] = self.points;
        let p0 = nlt;
        let p1 = flt;
        let p2 = frt;
        let p3 = nrt;
        let p4 = nlb;
        let p5 = nrb;
        let p6 = frb;
        let p7 = flb;
        crate::math::convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7])
            .chunks_exact(3)
            .flat_map(|chunk| match chunk {
                [a, b, c] => {
                    let n = crate::math::triangle_face_normal(*a, *b, *c);
                    [(*a, n), (*b, n), (*c, n)]
                }
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn test_against_aabb(&self, aabb: &Aabb) -> bool {
        for i in 0..3 {
            let mut out = 0;
            for j in 0..8 {
                if self.points[j].to_array()[i] < aabb.min.to_array()[i] {
                    out += 1;
                }
            }
            if out == 8 {
                return false;
            }
            out = 0;
            for j in 0..8 {
                if self.points[j].to_array()[i] > aabb.max.to_array()[i] {
                    out += 1;
                }
            }
            if out == 8 {
                return false;
            }
        }
        true
    }

    /// Returns the depth of the frustum.
    pub fn depth(&self) -> f32 {
        (self.planes[0].w - self.planes[5].w).abs()
    }
}

/// Bounding box consisting of a center and three half extents.
///
/// Essentially a point at the center and a vector pointing from
/// the center to the corner.
///
/// This is _not_ an axis aligned bounding box.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct BoundingBox {
    pub center: Vec3,
    pub half_extent: Vec3,
}

impl BoundingBox {
    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        let center = (min + max) / 2.0;
        let half_extent = max - center;
        Self {
            center,
            half_extent,
        }
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        let p = point - self.center;
        let component_edge_distance = p.abs() - self.half_extent;
        let outside = component_edge_distance.max(Vec3::ZERO).length();
        let inside = component_edge_distance
            .x
            .max(component_edge_distance.y)
            .min(0.0);
        inside + outside
    }

    #[cfg(cpu)]
    /// Return a triangle mesh connecting this `Aabb`'s corners.
    ///
    /// ```ignore
    ///    y           1_____2     _____
    ///    |           /    /|    /|    |  (same box, left and front sides removed)
    ///    |___x     0/___3/ |   /7|____|6
    ///   /           |    | /   | /    /
    /// z/            |____|/   4|/____/5
    ///
    /// 7 is min
    /// 3 is max
    /// ```
    pub fn get_mesh(&self) -> [(Vec3, Vec3); 36] {
        // Deriving the corner positions from centre and half-extent,

        let p0 = Vec3::new(-self.half_extent.x, self.half_extent.y, self.half_extent.z);
        let p1 = Vec3::new(-self.half_extent.x, self.half_extent.y, -self.half_extent.z);
        let p2 = Vec3::new(self.half_extent.x, self.half_extent.y, -self.half_extent.z);
        let p3 = self.half_extent;
        let p4 = Vec3::new(-self.half_extent.x, -self.half_extent.y, self.half_extent.z);
        let p5 = Vec3::new(self.half_extent.x, -self.half_extent.y, self.half_extent.z);
        let p6 = Vec3::new(self.half_extent.x, -self.half_extent.y, -self.half_extent.z);
        // min
        let p7 = -self.half_extent;

        let positions =
            crate::math::convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7].map(|p| p + self.center));

        // Attach per-triangle face normals.
        let vertices: Vec<(Vec3, Vec3)> = positions
            .chunks_exact(3)
            .flat_map(|chunk| match chunk {
                [a, b, c] => {
                    let n = crate::math::triangle_face_normal(*a, *b, *c);
                    [(*a, n), (*b, n), (*c, n)]
                }
                _ => unreachable!(),
            })
            .collect();

        // Convert into fixed-size array (12 triangles × 3 vertices).
        vertices
            .try_into()
            .unwrap_or_else(|v: Vec<(Vec3, Vec3)>| panic!("expected 36 vertices, got {}", v.len()))
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        let delta = (point - self.center).abs();
        let extent = self.half_extent.abs();
        delta.x <= extent.x && delta.y <= extent.y && delta.z <= extent.z
    }
}

/// Bounding sphere consisting of a center and radius.
#[derive(Clone, Copy, Debug, Default, PartialEq, SlabItem)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl From<(Vec3, Vec3)> for BoundingSphere {
    fn from((min, max): (Vec3, Vec3)) -> Self {
        let center = (min + max) * 0.5;
        let radius = center.distance(max);
        BoundingSphere { center, radius }
    }
}

impl From<Aabb> for BoundingSphere {
    fn from(value: Aabb) -> Self {
        (value.min, value.max).into()
    }
}

impl BoundingSphere {
    /// Creates a new bounding sphere.
    pub fn new(center: impl Into<Vec3>, radius: f32) -> BoundingSphere {
        BoundingSphere {
            center: center.into(),
            radius,
        }
    }

    /// Determine whether this sphere is inside the camera's view frustum after
    /// being transformed by `transform`.  
    pub fn is_inside_camera_view(
        &self,
        camera: &CameraDescriptor,
        transform: TransformDescriptor,
    ) -> (bool, BoundingSphere) {
        let center = Mat4::from(transform).transform_point3(self.center);
        let scale = Vec3::splat(transform.scale.max_element());
        let radius = Mat4::from_scale(scale)
            .transform_point3(Vec3::new(self.radius, 0.0, 0.0))
            .distance(Vec3::ZERO);
        let sphere = BoundingSphere::new(center, radius);
        (sphere.is_inside_frustum(camera.frustum()), sphere)
    }

    /// Transform this `BoundingSphere` by the given view projection matrix.
    pub fn project_by(&self, view_projection: &Mat4) -> Self {
        let center = self.center;
        // Pick any direction to find a point on the surface.
        let surface_point = self.center + self.radius * Vec3::Z;
        let new_center = view_projection.project_point3(center);
        let new_surface_point = view_projection.project_point3(surface_point);
        let new_radius = new_center.distance(new_surface_point);
        Self {
            center: new_center,
            radius: new_radius,
        }
    }

    /// Returns an [`Aabb`] with x and y coordinates in viewport pixels and z coordinate
    /// in NDC depth.
    pub fn project_onto_viewport(&self, camera: &CameraDescriptor, viewport: Vec2) -> Aabb {
        fn ndc_to_pixel(viewport: Vec2, ndc: Vec3) -> Vec2 {
            let screen = Vec3::new((ndc.x + 1.0) * 0.5, 1.0 - (ndc.y + 1.0) * 0.5, ndc.z);
            (screen * viewport.extend(1.0)).xy()
        }

        let viewproj = camera.view_projection();
        let frustum = camera.frustum();

        // Find the center and radius of the bounding sphere in pixel space.
        // By pixel space, I mean where (0, 0) is the top-left of the screen
        // and (w, h) is is the bottom-left.
        let center_clip = viewproj * self.center.extend(1.0);
        let front_center_ndc =
            viewproj.project_point3(self.center + self.radius * frustum.planes[5].xyz());
        let back_center_ndc =
            viewproj.project_point3(self.center + self.radius * frustum.planes[0].xyz());
        let center_ndc = center_clip.xyz() / center_clip.w;
        let center_pixels = ndc_to_pixel(viewport, center_ndc);
        let radius_pixels = viewport.x * (self.radius / center_clip.w);
        Aabb::new(
            (center_pixels - radius_pixels).extend(front_center_ndc.z),
            (center_pixels + radius_pixels).extend(back_center_ndc.z),
        )
    }
}

impl BVol for BoundingSphere {
    fn get_aabb(&self) -> Aabb {
        Aabb {
            min: self.center - Vec3::splat(self.radius),
            max: self.center + Vec3::splat(self.radius),
        }
    }

    fn culls_this_plane(&self, plane: &Vec4) -> bool {
        dist_bpp(plane, self.center) < -self.radius
    }
}

/// Bounding volume trait.
pub trait BVol {
    /// Returns an AABB that contains the bounding volume.
    fn get_aabb(&self) -> Aabb;

    /// Checks if the given bounding volume is culled by this plane.
    ///
    /// Returns true if it does, false otherwise.
    fn culls_this_plane(&self, plane: &Vec4) -> bool;

    fn is_inside_frustum(&self, frustum: Frustum) -> bool {
        let (inside, _) = self.coherent_test_is_volume_outside_frustum(&frustum, 0);
        !inside
    }

    /// Checks if bounding volume is outside the frustum "coherently".
    ///
    /// In order for a bounding volume to be inside the frustum, it must not be
    /// culled by any plane.
    ///
    /// Coherence is provided by the `lpindex` argument, which should be the
    /// index of the first plane found that culls this volume, given as part
    /// of the return value of this function.
    ///
    /// Returns `true` if the volume is outside the frustum, `false` otherwise.
    ///
    /// Returns the index of first plane found that culls this volume, to cache
    /// and use later as a short circuit.
    fn coherent_test_is_volume_outside_frustum(
        &self,
        frustum: &Frustum,
        lpindex: u32,
    ) -> (bool, u32) {
        if self.culls_this_plane(&frustum.planes[lpindex as usize]) {
            return (true, lpindex);
        }

        for i in 0..6 {
            if (i != lpindex) && self.culls_this_plane(&frustum.planes[i as usize]) {
                return (true, i);
            }
        }

        if !frustum.test_against_aabb(&self.get_aabb()) {
            return (true, lpindex);
        }

        (false, lpindex)
    }
}

impl BVol for Aabb {
    fn get_aabb(&self) -> Aabb {
        *self
    }

    fn culls_this_plane(&self, plane: &Vec4) -> bool {
        dist_bpp(plane, mi_vertex(plane, self)) < 0.0
    }
}

#[cfg(test)]
mod test {
    use glam::{Mat4, Quat};

    use crate::{geometry::Vertex, test::BlockOnFuture, Context};

    use super::*;

    #[test]
    fn bvol_frustum_is_in_world_space_sanity() {
        let (p, v) = crate::camera::default_perspective(800.0, 600.0);
        let camera = CameraDescriptor::new(p, v);
        let aabb_outside = Aabb {
            min: Vec3::new(-10.0, -12.0, 20.0),
            max: Vec3::new(10.0, 12.0, 40.0),
        };
        assert!(!aabb_outside.is_inside_frustum(camera.frustum()));

        let aabb_inside = Aabb {
            min: Vec3::new(-3.0, -3.0, -3.0),
            max: Vec3::new(3.0, 3.0, 3.0),
        };
        assert!(aabb_inside.is_inside_frustum(camera.frustum()));
    }

    #[test]
    fn frustum_culling_debug_corner_case() {
        // https://github.com/schell/renderling/issues/131
        // https://renderling.xyz/devlog/index.html#frustum_culling_last_debugging__aabb_vs_frustum_corner_case
        let camera = {
            let aspect = 1.0;
            let fovy = core::f32::consts::FRAC_PI_4;
            let znear = 4.0;
            let zfar = 1000.0;
            let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
            let eye = Vec3::new(0.0, 0.0, 10.0);
            let target = Vec3::ZERO;
            let up = Vec3::Y;
            let view = Mat4::look_at_rh(eye, target, up);
            CameraDescriptor::new(projection, view)
        };
        let aabb = Aabb {
            min: Vec3::new(-3.2869213, -3.0652206, -3.8715153),
            max: Vec3::new(3.2869213, 3.0652206, 3.8715153),
        };
        let transform = TransformDescriptor {
            translation: Vec3::new(7.5131035, -9.947085, -5.001645),
            rotation: Quat::from_xyzw(0.4700742, 0.34307128, 0.6853008, -0.43783003),
            scale: Vec3::new(1.0, 1.0, 1.0),
        };
        assert!(
            !aabb.is_outside_camera_view(&camera, transform),
            "aabb should be inside the frustum"
        );
    }

    #[test]
    fn bounding_box_from_min_max() {
        let ctx = Context::headless(256, 256).block();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::ZERO)
            .with_msaa_sample_count(4)
            .with_lighting(true);
        let _camera = stage.new_camera().with_projection_and_view(
            // TODO: BUG - using orthographic here renderes nothing
            // Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 10.0, -10.0),
            crate::camera::perspective(256.0, 256.0),
            Mat4::look_at_rh(Vec3::new(-3.0, 3.0, 5.0) * 0.5, Vec3::ZERO, Vec3::Y),
        );
        let _lights = crate::test::make_two_directional_light_setup(&stage);

        let white = stage.new_material();
        let red = stage
            .new_material()
            .with_albedo_factor(Vec4::new(1.0, 0.0, 0.0, 1.0));

        let _w = stage.new_renderlet().with_material(&white).with_vertices(
            stage.new_vertices(
                crate::math::unit_cube()
                    .into_iter()
                    .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
            ),
        );

        let mut corners = vec![];
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    corners.push(Vec3::new(x, y, z));
                }
            }
        }
        let mut rs = vec![];
        for corner in corners.iter() {
            let bb = BoundingBox {
                center: Vec3::new(0.5, 0.5, 0.5) * corner,
                half_extent: Vec3::splat(0.25),
            };
            assert!(
                bb.contains_point(bb.center),
                "BoundingBox {bb:?} does not contain center"
            );

            rs.push(
                stage.new_renderlet().with_material(&red).with_vertices(
                    stage.new_vertices(
                        bb.get_mesh()
                            .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
                    ),
                ),
            );
        }

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("bvol/bounding_box/get_mesh.png", img);
    }

    #[test]
    fn aabb_intersection() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(0.9), Vec3::splat(1.9));
        assert!(a.intersects_aabb(&b));
        assert!(b.intersects_aabb(&a));
    }

    #[test]
    fn aabb_union() {
        let a = Aabb::new(Vec3::splat(4.0), Vec3::splat(5.0));
        let b = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let c = Aabb::union(a, b);
        assert_eq!(
            Aabb {
                min: Vec3::ZERO,
                max: Vec3::splat(5.0)
            },
            c
        );
    }
}
