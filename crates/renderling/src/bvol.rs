//! Bounding volumes and culling primitives.
//!
// The initial implementation here is taken from `treeculler`.
use crabslab::SlabItem;
use glam::{Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::camera::Camera;

/// Normalize a plane.
pub fn normalize_plane(mut plane: Vec4) -> Vec4 {
    let normal_magnitude = (plane.x.powi(2) + plane.y.powi(2) + plane.z.powi(2)).sqrt();
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
#[derive(Clone, Copy, SlabItem)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn is_outside_frustum(&self, frustum: Frustum) -> bool {
        let (inside, _) = self.coherent_test_against_frustum(&frustum, 0);
        !inside
    }
}

/// Six planes of a view frustum.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct Frustum {
    pub planes: [Vec4; 6],
    pub points: [Vec3; 8],
}

impl Frustum {
    pub fn from_camera(camera: &Camera) -> Self {
        let viewprojection = camera.projection * camera.view;
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

/// Bounding volume trait.
pub trait BVol {
    /// Returns an AABB that contains the bounding volume.
    fn get_aabb(&self) -> Aabb;

    /// Checks if bounding volume intersects with a plane.
    ///
    /// Returns true if it does, false otherwise.
    fn test_against_plane(&self, plane: &Vec4) -> bool;

    /// Checks if bounding volume is outside the frustum.
    ///
    /// Sets a bit if the bounding volume is outside that bit's plane. Returns `1` if it is fully outside.
    fn test_against_frustum(&self, frustum: &Frustum, mut mask: u32) -> u32 {
        for i in 0..6 {
            if self.test_against_plane(&frustum.planes[i as usize]) {
                return u32::MAX;
            } else {
                // This piece of code first shifts the hardcoded byte by i, which is used as an
                // index, and then OR is used to set the index bit to one.
                mask |= 0b1000_0000u32 >> i;
            }
        }

        if !frustum.test_against_aabb(&self.get_aabb()) {
            return u32::MAX;
        }

        mask
    }

    /// Checks if bounding volume is outside the frustum coherently.
    /// Coherence is provided by the `lpindex` argument, which should be the last plane
    /// this volume got culled.
    ///
    /// Returns false if the volume is outside, true otherwise. Returns the last plane
    /// this volume has been culled as an `u8`, to save it and use it later again.
    fn coherent_test_against_frustum(&self, frustum: &Frustum, lpindex: u32) -> (bool, u32) {
        if self.test_against_plane(&frustum.planes[lpindex as usize]) {
            return (false, lpindex);
        }

        for i in 0..6 {
            if (i != lpindex) && self.test_against_plane(&frustum.planes[i as usize]) {
                return (false, i);
            }
        }

        if !frustum.test_against_aabb(&self.get_aabb()) {
            return (false, lpindex);
        }

        (true, lpindex)
    }
}

impl BVol for Aabb {
    fn get_aabb(&self) -> Aabb {
        *self
    }

    fn test_against_plane(&self, plane: &Vec4) -> bool {
        dist_bpp(plane, mi_vertex(plane, self)) < 0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bvol_frustum_is_in_world_space_sanity() {
        let (p, v) = crate::camera::default_perspective(800.0, 600.0);
        let camera = Camera::new(p, v);
        let aabb_outside = Aabb {
            min: Vec3::new(-10.0, -12.0, 20.0),
            max: Vec3::new(10.0, 12.0, 40.0),
        };
        assert!(aabb_outside.is_outside_frustum(camera.frustum));

        let aabb_inside = Aabb {
            min: Vec3::new(-3.0, -3.0, -3.0),
            max: Vec3::new(3.0, 3.0, 3.0),
        };
        assert!(!aabb_inside.is_outside_frustum(camera.frustum));
    }
}
