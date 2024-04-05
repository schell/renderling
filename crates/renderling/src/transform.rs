use crabslab::SlabItem;
use glam::{Quat, Vec3};

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
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
