//! Light builders for the stage.
use renderling_shader::id::Id;
use renderling_shader::stage::{GpuLight, LightType};
use glam::{Vec3, Vec4};

use crate::StageSlab;

#[cfg(feature = "gltf")]
pub fn from_gltf_light_kind(kind: gltf::khr_lights_punctual::Kind) -> LightType {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => LightType::DIRECTIONAL_LIGHT,
        gltf::khr_lights_punctual::Kind::Point => LightType::POINT_LIGHT,
        gltf::khr_lights_punctual::Kind::Spot { .. } => LightType::SPOT_LIGHT,
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

/// A builder for a spot light.
pub struct GpuSpotLightBuilder<'a> {
    inner: GpuLight,
    stage: &'a mut StageSlab,
}

impl<'a> GpuSpotLightBuilder<'a> {
    pub fn new(stage: &'a mut StageSlab) -> GpuSpotLightBuilder<'a> {
        let inner = GpuLight {
            light_type: LightType::SPOT_LIGHT,
            ..Default::default()
        };
        let white = Vec4::splat(1.0);
        Self {
            inner,
            stage,
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_color(white)
        .with_intensity(1.0)
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

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.color = color.into();
        self
    }

    pub fn with_intensity(mut self, i: f32) -> Self {
        self.inner.intensity = i;
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.stage.append(&self.inner)
    }
}

/// A builder for a directional light.
///
/// Directional lights illuminate all geometry from a certain direction,
/// without attenuation.
///
/// This is like the sun, or the moon.
pub struct GpuDirectionalLightBuilder<'a> {
    inner: GpuLight,
    stage: &'a mut StageSlab,
}

impl<'a> GpuDirectionalLightBuilder<'a> {
    pub fn new(stage: &'a mut StageSlab) -> GpuDirectionalLightBuilder<'a> {
        let inner = GpuLight {
            light_type: LightType::DIRECTIONAL_LIGHT,
            ..Default::default()
        };
        Self {
            inner,
            stage,
        }
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_color(Vec4::splat(1.0))
        .with_intensity(1.0)
    }

    pub fn with_direction(mut self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.color = color.into();
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.inner.intensity = intensity;
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.stage.append(&self.inner)
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
        .with_color(white)
        .with_intensity(1.0)
    }

    pub fn with_position(self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(0.0);
        self
    }

    pub fn with_attenuation(self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.color = color.into();
        self
    }

    pub fn with_intensity(self, i: f32) -> Self {
        self.inner.intensity = i;
        self
    }

    pub fn build(self) -> Id<GpuLight> {
        self.id
    }
}
