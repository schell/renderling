//! Decomposed 3d transform.
use crabslab::SlabItem;
use glam::{Mat4, Quat, Vec3};

use crate::math::IsMatrix;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
/// A decomposed transformation.
///
/// `Transform` can be converted to/from [`Mat4`].
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl From<Mat4> for Transform {
    fn from(value: Mat4) -> Self {
        let (scale, rotation, translation) = value.to_scale_rotation_translation_or_id();
        Transform {
            translation,
            rotation,
            scale,
        }
    }
}

impl From<Transform> for Mat4 {
    fn from(
        Transform {
            translation,
            rotation,
            scale,
        }: Transform,
    ) -> Self {
        Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crabslab::*;

    #[test]
    fn transform_roundtrip() {
        assert_eq!(3, Vec3::SLAB_SIZE, "unexpected Vec3 slab size");
        assert_eq!(4, Quat::SLAB_SIZE, "unexpected Quat slab size");
        assert_eq!(10, Transform::SLAB_SIZE);
        let t = Transform::default();
        let mut slab = CpuSlab::new(vec![]);
        let t_id = slab.append(&t);
        pretty_assertions::assert_eq!(
            &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            bytemuck::cast_slice::<u32, f32>(slab.as_ref().as_slice())
        );
        pretty_assertions::assert_eq!(t, slab.read(t_id));
    }
}
