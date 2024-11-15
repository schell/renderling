//! Bounding volumes and culling primitives.
//!
//! The initial implementation here was gleaned from `treeculler`, which
//! unfortunately cannot compile to SPIR-V because of its use of `u8`.
//!
//! Also, here we use `glam`, whereas `treeculler` uses its own internal
//! primitives.
//!
//! More resources:
//! * https://fgiesen.wordpress.com/2010/10/17/view-frustum-culling/
//! * http://old.cescg.org/CESCG-2002/DSykoraJJelinek/
//! * https://iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm

use crabslab::SlabItem;
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{camera::Camera, transform::Transform};

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
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
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

    pub fn diagonal_length(&self) -> f32 {
        self.min.distance(self.max)
    }

    pub fn is_zero(&self) -> bool {
        self.min == self.max
    }

    /// Determines whether this `Aabb` can be seen by `camera` after being
    /// transformed by `transform`.
    pub fn is_outside_camera_view(&self, camera: &Camera, transform: Transform) -> bool {
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
}

/// Six planes of a view frustum.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct Frustum {
    /// Planes constructing the sides of the frustum,
    /// each expressed as a normal vector (xyz) and the distance (w)
    /// from the origin along that vector.
    pub planes: [Vec4; 6],
    pub points: [Vec3; 8],
}

impl Frustum {
    /// Compute a frustum in world space from the given [`Camera`].
    pub fn from_camera(camera: &Camera) -> Self {
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
}

/// Bounding sphere consisting of a center and radius.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
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
        camera: &Camera,
        transform: Transform,
    ) -> (bool, BoundingSphere) {
        let center = Mat4::from(transform).transform_point3(self.center);
        let scale = Vec3::splat(transform.scale.max_element());
        let radius = Mat4::from_scale(scale)
            .transform_point3(Vec3::new(self.radius, 0.0, 0.0))
            .distance(Vec3::ZERO);
        let sphere = BoundingSphere::new(center, radius);
        (sphere.is_inside_frustum(camera.frustum()), sphere)
    }

    /// Returns an [`Aabb`] with x and y coordinates in viewport pixels and z coordinate
    /// in NDC depth.
    pub fn project_onto_viewport(&self, camera: &Camera, viewport: Vec2) -> Aabb {
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

    use crate::{
        stage::{Renderlet, Vertex},
        Context,
    };

    use super::*;

    #[test]
    fn bvol_frustum_is_in_world_space_sanity() {
        let (p, v) = crate::camera::default_perspective(800.0, 600.0);
        let camera = Camera::new(p, v);
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
            Camera::new(projection, view)
        };
        let aabb = Aabb {
            min: Vec3::new(-3.2869213, -3.0652206, -3.8715153),
            max: Vec3::new(3.2869213, 3.0652206, 3.8715153),
        };
        let transform = Transform {
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
    fn bounding_sphere_from_min_max() {
        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_background_color(Vec4::ONE)
            .with_debug_overlay(true)
            .with_frustum_culling(false);

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        let vertices = stage.new_array(crate::math::unit_cube().into_iter().map(|(p, n)| {
            min = min.min(p);
            max = max.max(p);
            Vertex::default()
                .with_position(p)
                .with_normal(n)
                .with_color(Vec4::new(1.0, 0.0, 0.0, 1.0))
        }));
        let bounds = BoundingSphere::from((min, max));
        log::info!("bounds: {bounds:?}");

        let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 20.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, Vec3::Y);
        let camera = stage.new_value(Camera::new(projection, view));

        let renderlet = stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            bounds,
            camera_id: camera.id(),
            ..Default::default()
        });
        stage.add_renderlet(&renderlet);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("bvol/bounding_sphere_from_min_max.png", img);
        frame.present();
    }
}
