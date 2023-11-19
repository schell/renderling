//! Light builders for the stage.
use renderling_shader::id::Id;
use renderling_shader::stage::{GpuLight, LightType};
use glam::{Vec3, Vec4};

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
            id,
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_color(white)
        .with_intensity(1.0)
    }

    pub fn with_position(self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(1.0);
        self
    }

    pub fn with_direction(self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_attenuation(self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_cutoff(self, inner: f32, outer: f32) -> Self {
        self.inner.inner_cutoff = inner;
        self.inner.outer_cutoff = outer;
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
