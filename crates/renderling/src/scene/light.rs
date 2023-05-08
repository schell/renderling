//! Light builders.

use glam::{Vec4, Vec3};
use renderling_shader::scene::{GpuLight, LightType, Id};

#[cfg(feature = "gltf")]
pub fn from_gltf_light_kind(kind: gltf::khr_lights_punctual::Kind) -> LightType {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => LightType::DIRECTIONAL_LIGHT,
        gltf::khr_lights_punctual::Kind::Point => LightType::POINT_LIGHT,
        gltf::khr_lights_punctual::Kind::Spot { .. } => LightType::SPOT_LIGHT,
    }
}

/// A builder for a spot light.
pub struct GpuSpotLightBuilder<'a> {
    id: Id<GpuLight>,
    inner: &'a mut GpuLight,
}

impl<'a> GpuSpotLightBuilder<'a> {
    pub fn new(lights: &'a mut Vec<GpuLight>) -> GpuSpotLightBuilder<'a> {
        let inner = GpuLight {
            light_type: LightType::SPOT_LIGHT,
            ..Default::default()
        };
        let id = Id::new(lights.len() as u32);
        lights.push(inner);
        let white = Vec4::splat(1.0);
        Self {
            inner: &mut lights[id.index()],
            id
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(1.0);
        self
    }

    pub fn with_direction(mut self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_cutoff(mut self, inner: f32, outer: f32) -> Self {
        self.inner.inner_cutoff = inner;
        self.inner.outer_cutoff = outer;
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.id
    }
}

/// A builder for a directional light.
///
/// Directional lights illuminate all geometry from a certain direction,
/// without attenuation.
///
/// This is like the sun, or the moon.
pub struct GpuDirectionalLightBuilder<'a> {
    id: Id<GpuLight>,
    inner: &'a mut GpuLight,
}

impl<'a> GpuDirectionalLightBuilder<'a> {
    pub fn new(lights: &'a mut Vec<GpuLight>) -> GpuDirectionalLightBuilder<'a> {
        let inner = GpuLight {
            light_type: LightType::DIRECTIONAL_LIGHT,
            ..Default::default()
        };
        let id = Id::new(lights.len() as u32);
        lights.push(inner);
        Self {
            id,
            inner: &mut lights[id.index()],
        }
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(Vec4::splat(1.0))
        .with_diffuse_color(Vec4::splat(1.0))
        .with_specular_color(Vec4::splat(1.0))
    }

    pub fn with_direction(mut self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.id
    }
}

pub struct GpuPointLightBuilder<'a> {
    id: Id<GpuLight>,
    inner: &'a mut GpuLight,
}

impl<'a> GpuPointLightBuilder<'a> {
    pub fn new(lights: &mut Vec<GpuLight>) -> GpuPointLightBuilder<'_> {
        let inner = GpuLight {
            light_type: LightType::POINT_LIGHT,
            ..Default::default()
        };
        let id = Id::new(lights.len() as u32);
        lights.push(inner);
        let white = Vec4::splat(1.0);
        GpuPointLightBuilder {
            id,
            inner: &mut lights[id.index()],
        }
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(0.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.id
    }
}
