//! Geometric transformations.
use std::marker::PhantomData;

use nalgebra::{Matrix4, Point3, Transform3, Translation3, UnitQuaternion, Vector3};

/// A transformation with separate translation, scale and rotation.
///
/// This is like [`nalgebra::Transform3`], but is invertable. It is analogous
/// to a 4x4 matrix that has been translated, rotated and then scaled.
///
/// M = T * R * S
pub struct Transform<T> {
    pub translate: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotate: UnitQuaternion<f32>,
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for Transform<T> {
    fn eq(&self, other: &Self) -> bool {
        self.translate == other.translate
            && self.scale == other.scale
            && self.rotate == other.rotate
    }
}

impl<T> Eq for Transform<T> {}

impl<T> std::fmt::Debug for Transform<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transform")
            .field("translate", &self.translate)
            .field("scale", &self.scale)
            .field("rotate", &self.rotate)
            .finish()
    }
}

impl<T> Clone for Transform<T> {
    fn clone(&self) -> Self {
        Self {
            translate: self.translate.clone(),
            scale: self.scale.clone(),
            rotate: self.rotate.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Transform<T> {
    fn default() -> Self {
        Transform {
            translate: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotate: UnitQuaternion::identity(),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<&Transform<T>> for Matrix4<f32> {
    fn from(trns: &Transform<T>) -> Self {
        let t: Matrix4<f32> =
            Translation3::new(trns.translate.x, trns.translate.y, trns.translate.z)
                .to_homogeneous();
        let r: Matrix4<f32> = trns.rotate.to_homogeneous();
        let s: Matrix4<f32> = Matrix4::new_nonuniform_scaling(&trns.scale);
        t * r * s
    }
}

impl<T> From<&Transform<T>> for Transform3<f32> {
    fn from(t: &Transform<T>) -> Self {
        let mat4 = Matrix4::from(t);
        Transform3::from_matrix_unchecked(mat4)
    }
}

impl<T> Transform<T> {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut t = Transform::default();
        t.translate.x = x;
        t.translate.y = y;
        t.translate.z = z;
        t
    }

    pub fn from_translate(v: Vector3<f32>) -> Self {
        Transform {
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

    /// Append another transformation.
    ///
    /// M = (T + T) * (R * R) * (S * S)
    pub fn append(&self, other: &Transform<T>) -> Transform<T> {
        Transform {
            translate: self.translate + other.translate,
            scale: Vector3::new(
                self.scale.x * other.scale.x,
                self.scale.y * other.scale.y,
                self.scale.z * other.scale.z,
            ),
            rotate: self.rotate * other.rotate,
            _phantom: PhantomData,
        }
    }

    pub fn transform_point(&self, p: &Point3<f32>) -> Point3<f32> {
        let m = Matrix4::from(self);
        m.transform_point(p)
    }

    pub fn as_global(&self) -> Transform<Global> {
        Transform {
            translate: self.translate,
            scale: self.scale,
            rotate: self.rotate,
            _phantom: PhantomData,
        }
    }
}

/// Valueless type used to parameterize a local transformation.
pub struct Local;

/// Valueless type used to parameterize a global transformation.
pub struct Global;

/// An alias for a transformation in a local coordinate space.
pub type LocalTransform = Transform<Local>;

/// An alias for a transformation in the world coordinate space.
pub type WorldTransform = Transform<Global>;
