//! Geometric transformations.
use std::marker::PhantomData;

use glam::{Mat4, Quat, Vec3};

/// A transformation with separate translation, scale and rotation.
///
/// This is like [`nalgebra::Transform3`], but is invertable. It is analogous
/// to a 4x4 matrix that has been translated, rotated and then scaled.
///
/// M = T * R * S
pub struct Transform<T> {
    pub translate: Vec3,
    pub scale: Vec3,
    pub rotate: Quat,
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
            translate: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotate: Quat::IDENTITY,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<&Transform<T>> for Mat4 {
    fn from(trns: &Transform<T>) -> Self {
        let t = Mat4::from_translation(trns.translate);
        let r = Mat4::from_quat(trns.rotate);
        let s = Mat4::from_scale(trns.scale);
        t * r * s
    }
}

impl<T> From<Transform<T>> for Mat4 {
    fn from(trns: Transform<T>) -> Self {
        Mat4::from(&trns)
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

    pub fn from_translate(v: Vec3) -> Self {
        Transform {
            translate: v,
            ..Default::default()
        }
    }

    pub fn with_position(mut self, p: Vec3) -> Self {
        self.translate = Vec3::new(p.x, p.y, p.z);
        self
    }

    pub fn with_rotation(mut self, r: Quat) -> Self {
        self.rotate = r;
        self
    }

    pub fn with_scale(mut self, s: Vec3) -> Self {
        self.scale = s;
        self
    }

    /// Append another transformation.
    ///
    /// M = (T + T) * (R * R) * (S * S)
    pub fn append(&self, other: &Transform<T>) -> Transform<T> {
        Transform {
            translate: self.translate + other.translate,
            scale: Vec3::new(
                self.scale.x * other.scale.x,
                self.scale.y * other.scale.y,
                self.scale.z * other.scale.z,
            ),
            rotate: self.rotate * other.rotate,
            _phantom: PhantomData,
        }
    }

    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        let m = Mat4::from(self);
        m.project_point3(p)
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
