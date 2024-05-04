//! Stage lighting.
use crabslab::{Id, SlabItem};
use glam::{Vec3, Vec4};

use crate::Transform;

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for SpotLight {
    fn default() -> Self {
        let white = Vec4::splat(1.0);
        let inner_cutoff = core::f32::consts::PI / 3.0;
        let outer_cutoff = core::f32::consts::PI / 2.0;
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let color = white;
        let intensity = 1.0;

        Self {
            position: Default::default(),
            direction,
            inner_cutoff,
            outer_cutoff,
            color,
            intensity,
        }
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let color = Vec4::splat(1.0);
        let intensity = 1.0;

        Self {
            direction,
            color,
            intensity,
        }
    }
}

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        let color = Vec4::splat(1.0);
        let intensity = 1.0;

        Self {
            position: Default::default(),
            color,
            intensity,
        }
    }
}

#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq)]
pub enum LightStyle {
    Directional = 0,
    Point = 1,
    Spot = 2,
}

impl SlabItem for LightStyle {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let proxy = u32::read_slab(index, slab);
        match proxy {
            0 => LightStyle::Directional,
            1 => LightStyle::Point,
            2 => LightStyle::Spot,
            _ => LightStyle::Directional,
        }
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        let proxy = *self as u32;
        proxy.write_slab(index, slab)
    }
}

/// A type-erased linked-list-of-lights that is used as a slab pointer to any
/// light type.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq, SlabItem)]
pub struct Light {
    // The type of the light
    pub light_type: LightStyle,
    // The index of the light in the slab
    pub index: u32,
    // The id of a transform to apply to the position and direction of the light.
    pub transform: Id<Transform>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: Id::<()>::NONE.inner(),
            transform: Id::NONE,
        }
    }
}

impl From<Id<DirectionalLight>> for Light {
    fn from(id: Id<DirectionalLight>) -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: id.inner(),
            transform: Id::NONE,
        }
    }
}

impl From<Id<SpotLight>> for Light {
    fn from(id: Id<SpotLight>) -> Self {
        Self {
            light_type: LightStyle::Spot,
            index: id.inner(),
            transform: Id::NONE,
        }
    }
}

impl From<Id<PointLight>> for Light {
    fn from(id: Id<PointLight>) -> Self {
        Self {
            light_type: LightStyle::Point,
            index: id.inner(),
            transform: Id::NONE,
        }
    }
}

impl Light {
    pub fn into_directional_id(self) -> Id<DirectionalLight> {
        Id::from(self.index)
    }

    pub fn into_spot_id(self) -> Id<SpotLight> {
        Id::from(self.index)
    }

    pub fn into_point_id(self) -> Id<PointLight> {
        Id::from(self.index)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "gltf")]
    #[test]
    fn position_direction_sanity() {
        // With GLTF, the direction of a light is given by the light's node's transform.
        // Specifically we get the node's transform and use the rotation quaternion to
        // rotate the vector Vec3::NEG_Z - the result is our direction.

        use glam::{Mat4, Quat};
        println!("{:#?}", std::env::current_dir());
        let (document, _buffers, _images) = gltf::import("../../gltf/four_spotlights.glb").unwrap();
        for node in document.nodes() {
            println!("node: {} {:?}", node.index(), node.name());

            let gltf_transform = node.transform();
            let (translation, rotation, _scale) = gltf_transform.decomposed();
            let position = Vec3::from_array(translation);
            let direction =
                Mat4::from_quat(Quat::from_array(rotation)).transform_vector3(Vec3::NEG_Z);
            println!("position: {position}");
            println!("direction: {direction}");

            // In Blender, our lights are sitting at (0, 0, 1) pointing at -Z, +Z, +X and +Y.
            // But alas, it is a bit more complicated than that because this file is exported with
            // UP being +Y, so Z and Y have been flipped...
            assert_eq!(Vec3::Y, position);
            let expected_direction = match node.name() {
                Some("light_negative_z") => Vec3::NEG_Y,
                Some("light_positive_z") => Vec3::Y,
                Some("light_positive_x") => Vec3::X,
                Some("light_positive_y") => Vec3::NEG_Z,
                n => panic!("unexpected node '{n:?}'"),
            };
            // And also there are rounding ... imprecisions...
            assert_approx_eq::assert_approx_eq!(expected_direction.x, direction.x);
            assert_approx_eq::assert_approx_eq!(expected_direction.y, direction.y);
            assert_approx_eq::assert_approx_eq!(expected_direction.z, direction.z);
        }
    }
}
