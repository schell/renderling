//! Stage lighting.
use crate::{self as renderling_shader, id::Id, slab::SlabItem};
use glam::{Vec3, Vec4};

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, SlabItem)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub attenuation: Vec3,
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
        let attenuation = Vec3::new(1.0, 0.014, 0.007);
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let color = white;
        let intensity = 1.0;

        Self {
            position: Default::default(),
            direction,
            attenuation,
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
    pub attenuation: Vec3,
    pub color: Vec4,
    pub intensity: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        let attenuation = Vec3::new(1.0, 0.14, 0.07);
        let color = Vec4::splat(1.0);
        let intensity = 1.0;

        Self {
            position: Default::default(),
            attenuation,
            color,
            intensity,
        }
    }
}

#[cfg(feature = "gltf")]
pub fn from_gltf_light_kind(kind: gltf::khr_lights_punctual::Kind) -> LightStyle {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => LightStyle::Directional,
        gltf::khr_lights_punctual::Kind::Point => LightStyle::Point,
        gltf::khr_lights_punctual::Kind::Spot { .. } => LightStyle::Spot,
    }
}

#[cfg(feature = "gltf")]
pub fn gltf_light_intensity_units(kind: gltf::khr_lights_punctual::Kind) -> &'static str {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => "lux (lm/m^2)",
        // sr is "steradian"
        _ => "candelas (lm/sr)",
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
    fn slab_size() -> usize {
        1
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let mut proxy = 0u32;
        let index = proxy.read_slab(index, slab);
        match proxy {
            0 => *self = LightStyle::Directional,
            1 => *self = LightStyle::Point,
            2 => *self = LightStyle::Spot,
            _ => *self = LightStyle::Directional,
        }
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        let proxy = *self as u32;
        proxy.write_slab(index, slab)
    }
}

/// A type-erased linked-list-of-lights that is used as a slab pointer to any light type.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, PartialEq, SlabItem)]
pub struct Light {
    // The type of the light
    pub light_type: LightStyle,
    // The index of the light in the slab
    pub index: u32,
    //// The id of the next light
    //pub next: Id<Light>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: Id::<()>::NONE.inner(),
            //next: Id::NONE,
        }
    }
}

impl From<Id<DirectionalLight>> for Light {
    fn from(id: Id<DirectionalLight>) -> Self {
        Self {
            light_type: LightStyle::Directional,
            index: id.inner(),
            //next: Id::NONE,
        }
    }
}

impl From<Id<SpotLight>> for Light {
    fn from(id: Id<SpotLight>) -> Self {
        Self {
            light_type: LightStyle::Spot,
            index: id.inner(),
            //next: Id::NONE,
        }
    }
}

impl From<Id<PointLight>> for Light {
    fn from(id: Id<PointLight>) -> Self {
        Self {
            light_type: LightStyle::Point,
            index: id.inner(),
            //next: Id::NONE,
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
