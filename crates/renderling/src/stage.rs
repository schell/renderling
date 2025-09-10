//! Scene staging.
//!
//! The [`Stage`] is the entrypoint for staging data on the GPU and
//! interacting with lighting.

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

#[cfg(test)]
mod test {
    use craballoc::{prelude::SlabAllocator, runtime::CpuRuntime};
    use glam::{Mat4, Quat, Vec3};

    use crate::{
        math::IsMatrix,
        transform::{shader::TransformDescriptor, NestedTransform},
    };

    #[test]
    fn matrix_hierarchy_sanity() {
        let a: Mat4 = TransformDescriptor {
            translation: Vec3::new(100.0, 100.0, 0.0),
            ..Default::default()
        }
        .into();
        let b: Mat4 = TransformDescriptor {
            scale: Vec3::splat(0.5),
            ..Default::default()
        }
        .into();
        let c1 = a * b;
        let c2 = b * a;
        assert_ne!(c1, c2);
    }

    #[test]
    fn nested_transform_fox_rigging() {
        pub fn legacy_get_world_transform(tfrm: &NestedTransform) -> (Vec3, Quat, Vec3) {
            let mut mat = Mat4::IDENTITY;
            let mut local = Some(tfrm.clone());
            while let Some(t) = local.take() {
                let transform = t.local_descriptor();
                mat = Mat4::from_scale_rotation_translation(
                    transform.scale,
                    transform.rotation,
                    transform.translation,
                ) * mat;
                local = t.parent();
            }
            let (s, r, t) = mat.to_scale_rotation_translation_or_id();
            (t, r, s)
        }

        let slab = SlabAllocator::new(CpuRuntime, "transform", ());
        let a = NestedTransform::new(&slab);
        a.set_local_translation(Vec3::splat(100.0));
        let b = NestedTransform::new(&slab);
        b.set_local_rotation(Quat::from_scaled_axis(Vec3::Z));
        let c = NestedTransform::new(&slab);
        c.set_local_scale(Vec3::splat(2.0));

        a.add_child(&b);
        b.add_child(&c);

        let TransformDescriptor {
            translation,
            rotation,
            scale,
        } = c.global_descriptor();
        let global_transform = (translation, rotation, scale);
        let legacy_transform = legacy_get_world_transform(&c);
        assert_eq!(legacy_transform, global_transform);

        c.set_local_translation(Vec3::ONE);

        let all_updates = slab.get_updated_source_ids();
        assert_eq!(
            std::collections::HashSet::from_iter([
                a.global_transform.descriptor.notifier_index(),
                b.global_transform.descriptor.notifier_index(),
                c.global_transform.descriptor.notifier_index()
            ]),
            all_updates
        );

        let TransformDescriptor {
            translation,
            rotation,
            scale,
        } = c.global_descriptor();
        let global_transform = (translation, rotation, scale);
        let legacy_transform = legacy_get_world_transform(&c);
        assert_eq!(legacy_transform, global_transform);
    }
}
