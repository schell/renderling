//! Geometric transformations.
use nalgebra::{Matrix4, Point3, Transform3, Translation3, UnitQuaternion, Vector3};

/// A global transform with separate translation, scale and rotation.
///
/// This is like [`Transform3`], but is invertable. It is analogous
/// to a 4x4 matrix that has been translated, rotated and then scaled.
#[derive(Clone, Debug)]
pub struct WorldTransform {
    pub translate: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotate: UnitQuaternion<f32>,
}

impl Default for WorldTransform {
    fn default() -> Self {
        WorldTransform {
            translate: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotate: UnitQuaternion::identity(),
        }
    }
}

impl From<&WorldTransform> for Matrix4<f32> {
    fn from(trns: &WorldTransform) -> Self {
        let t: Matrix4<f32> =
            Translation3::new(trns.translate.x, trns.translate.y, trns.translate.z)
                .to_homogeneous();
        let r: Matrix4<f32> = trns.rotate.to_homogeneous();
        let s: Matrix4<f32> = Matrix4::new_nonuniform_scaling(&trns.scale);
        t * r * s
    }
}

impl From<&WorldTransform> for Transform3<f32> {
    fn from(t: &WorldTransform) -> Self {
        let mat4 = Matrix4::from(t);
        Transform3::from_matrix_unchecked(mat4)
    }
}

impl WorldTransform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut t = WorldTransform::default();
        t.translate.x = x;
        t.translate.y = y;
        t.translate.z = z;
        t
    }

    pub fn from_translate(v: Vector3<f32>) -> Self {
        WorldTransform {
            translate: v,
            ..Default::default()
        }
    }

    pub fn with_position(mut self, p: Point3<f32>) -> Self {
        self.translate = Vector3::new(p.x, p.y, p.z);
        self
    }

    pub fn with_rotation(mut self, r: UnitQuaternion<f32>) -> Self {
        self.rotate = r;
        self
    }

    pub fn with_scale(mut self, s: Vector3<f32>) -> Self {
        self.scale = s;
        self
    }

    pub fn append(&self, other: &WorldTransform) -> WorldTransform {
        WorldTransform {
            translate: self.translate + other.translate,
            scale: Vector3::new(
                self.scale.x * other.scale.x,
                self.scale.y * other.scale.y,
                self.scale.z * other.scale.z,
            ),
            rotate: self.rotate * other.rotate,
        }
    }

    pub fn transform_point(&self, p: &Point3<f32>) -> Point3<f32> {
        let m = Matrix4::from(self);
        m.transform_point(p)
    }
}
