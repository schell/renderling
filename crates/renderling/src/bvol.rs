//! Bounding volumes and culling primitives.

use crabslab::SlabItem;
use glam::{Vec3, Vec4};
use treeculler::BVol;

use crate::camera::Camera;

/// Axis aligned bounding box.
#[derive(Clone, Copy, SlabItem)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn is_outside_frustum(&self, frustum: Frustum) -> bool {
        let aabb = treeculler::AABB::new(vec3_to_tc(self.min), vec3_to_tc(self.max));
        let frustum = treeculler::Frustum::from(frustum);
        let (inside, _) = aabb.coherent_test_against_frustum(&frustum, 0);
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

fn vec4_from_tc(v: treeculler::Vec4<f32>) -> Vec4 {
    Vec4::new(v.x, v.y, v.z, v.w)
}

fn vec3_from_tc(v: treeculler::Vec3<f32>) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

fn vec3_to_tc(v: Vec3) -> treeculler::Vec3<f32> {
    v.to_array().into()
}

fn vec4_to_tc(v: Vec4) -> treeculler::Vec4<f32> {
    v.to_array().into()
}

impl From<treeculler::frustum::Frustum<f32>> for Frustum {
    fn from(tcf: treeculler::frustum::Frustum<f32>) -> Self {
        Frustum {
            planes: tcf.planes.map(vec4_from_tc),
            points: tcf.points.map(vec3_from_tc),
        }
    }
}

impl From<Frustum> for treeculler::frustum::Frustum<f32> {
    fn from(f: Frustum) -> Self {
        treeculler::frustum::Frustum {
            planes: f.planes.map(vec4_to_tc),
            points: f.points.map(vec3_to_tc),
        }
    }
}

impl Frustum {
    pub fn from_camera(camera: &Camera) -> Self {
        let viewprojection = camera.projection * camera.view;
        Self::from(treeculler::frustum::Frustum::from_modelview_projection(
            viewprojection.to_cols_array_2d(),
        ))
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
