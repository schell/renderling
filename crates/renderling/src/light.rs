//! Different kinds of scene lighting.
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::{linkage::pbr::LightsUniform, resources::Shared, Device, Queue};
use glam::{vec3, vec4, Vec3, Vec4};
use moongraph::{Read, Write};
use renderling_shader::light::{
    DirectionalLightRaw, DirectionalLights, PointLightRaw, PointLights, SpotLightRaw, SpotLights,
    DIRECTIONAL_LIGHTS_MAX, POINT_LIGHTS_MAX, SPOT_LIGHTS_MAX,
};

#[derive(snafu::Snafu, Debug)]
pub enum LightsError {
    #[snafu(display("none"))]
    None
}

/// Illuminates geometry in all directions surrounding a point, with
/// attenuation.
///
/// This is like a light bulb, a match or a firefly.
pub struct PointLight {
    inner: Shared<PointLightRaw>,
    dirty: Arc<AtomicBool>,
}

impl PointLight {
    pub fn new(lights: &mut Lights) -> Self {
        let inner = Shared::new(PointLightRaw::default());
        lights.point_lights.push(inner.clone());
        let light = PointLight {
            inner,
            dirty: lights.dirty.point.clone(),
        };
        let white = Vec4::splat(1.0);
        light.set_attenuation(1.0, 0.14, 0.07);
        light.set_ambient_color(white);
        light.set_diffuse_color(white);
        light.set_specular_color(white);
        light
    }

    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vec3) {
        self.inner.write().position_ = position.extend(0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: impl Into<Vec4>) {
        self.inner.write().ambient_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: impl Into<Vec4>) {
        self.inner.write().diffuse_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: impl Into<Vec4>) {
        self.inner.write().specular_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the attenuation coefficients for the point light.
    pub fn set_attenuation(&self, constant: f32, linear: f32, quadratic: f32) {
        self.inner.write().attenuation_ = vec4(constant, linear, quadratic, 0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct DirtyLights {
    pub point: Arc<AtomicBool>,
    pub spot: Arc<AtomicBool>,
    pub directional: Arc<AtomicBool>,
}

impl Default for DirtyLights {
    fn default() -> Self {
        Self {
            point: Arc::new(AtomicBool::new(true)),
            spot: Arc::new(AtomicBool::new(true)),
            directional: Arc::new(AtomicBool::new(true)),
        }
    }
}

pub struct PointLightBuilder {
    inner: PointLight,
}

impl PointLightBuilder {
    pub(crate) fn new(lights: &mut Lights) -> PointLightBuilder {
        let inner = PointLight::new(lights);
        PointLightBuilder { inner }
    }

    pub fn with_position(self, position: Vec3) -> Self {
        self.inner.set_position(position);
        self
    }

    pub fn with_attenuation(self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.set_attenuation(constant, linear, quadratic);
        self
    }

    pub fn with_ambient_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_ambient_color(color);
        self
    }

    pub fn with_diffuse_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_diffuse_color(color);
        self
    }

    pub fn with_specular_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_specular_color(color);
        self
    }

    pub fn build(self) -> PointLight {
        self.inner
    }
}

/// Illuminates geometry in a cone in front of a point, within a certain cutoff
/// boundary.
///
/// This is like a street light or a flashlight.
pub struct SpotLight {
    inner: Shared<SpotLightRaw>,
    dirty: Arc<AtomicBool>,
}

impl SpotLight {
    pub fn new(lights: &Lights) -> Self {
        let white = Vec4::splat(1.0);
        let light = Self {
            inner: Default::default(),
            dirty: lights.dirty.spot.clone(),
        };
        light.set_inner_cutoff(std::f32::consts::PI / 3.0);
        light.set_outer_cutoff(std::f32::consts::PI / 2.0);
        light.set_attenuation(Vec3::new(1.0, 0.014, 0.007));
        light.set_direction(Vec3::new(0.0, -1.0, 0.0));
        light.set_ambient_color(white);
        light.set_diffuse_color(white);
        light.set_specular_color(white);
        light
    }
    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vec3) {
        self.inner.write().position_ = position.extend(0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the direction the light is pointing in.
    pub fn set_direction(&self, direction: Vec3) {
        self.inner.write().direction_ = direction.extend(0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the inner angular cutoff.
    pub fn set_inner_cutoff(&self, cutoff: f32) {
        self.inner.write().cutoff_.x = cutoff;
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the constant, linear, and quadratic terms of attenuation.
    pub fn set_attenuation(&self, attenuation: Vec3) {
        self.inner.write().attenuation_ = attenuation.extend(0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the outer angular cutoff.
    pub fn set_outer_cutoff(&self, cutoff: f32) {
        self.inner.write().cutoff_.y = cutoff;
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: impl Into<Vec4>) {
        self.inner.write().ambient_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: impl Into<Vec4>) {
        self.inner.write().diffuse_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: impl Into<Vec4>) {
        self.inner.write().specular_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct SpotLightBuilder {
    inner: SpotLight,
}

impl SpotLightBuilder {
    pub(crate) fn new(lights: &mut Lights) -> SpotLightBuilder {
        Self {
            inner: SpotLight::new(lights),
        }
    }

    pub fn with_position(self, position: Vec3) -> Self {
        self.inner.set_position(position);
        self
    }

    pub fn with_direction(self, direction: Vec3) -> Self {
        self.inner.set_direction(direction);
        self
    }

    pub fn with_attenuation(self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner
            .set_attenuation(vec3(constant, linear, quadratic));
        self
    }

    pub fn with_cutoff(self, inner: f32, outer: f32) -> Self {
        self.inner.set_inner_cutoff(inner);
        self.inner.set_outer_cutoff(outer);
        self
    }

    pub fn with_ambient_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_ambient_color(color);
        self
    }

    pub fn with_diffuse_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_diffuse_color(color);
        self
    }

    pub fn with_specular_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_specular_color(color);
        self
    }

    pub fn build(self) -> SpotLight {
        self.inner
    }
}

/// Illuminates all geometry from a certain direction, without attenuation.
///
/// This is like the sun, or the moon.
pub struct DirectionalLight {
    inner: Shared<DirectionalLightRaw>,
    dirty: Arc<AtomicBool>,
}

impl DirectionalLight {
    pub fn new(lights: &mut Lights) -> Self {
        let inner = Shared::new(DirectionalLightRaw::default());
        lights.directional_lights.push(inner.clone());
        let light = Self {
            inner,
            dirty: lights.dirty.directional.clone(),
        };
        light.set_direction(Vec3::new(0.0, -1.0, 0.0));
        light.set_ambient_color(Vec4::splat(1.0));
        light.set_diffuse_color(Vec4::splat(1.0));
        light.set_specular_color(Vec4::splat(1.0));
        light
    }
    /// Sets the direction this light points in.
    pub fn set_direction(&self, direction: Vec3) {
        self.inner.write().direction_ = direction.extend(0.0);
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: impl Into<Vec4>) {
        self.inner.write().ambient_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: impl Into<Vec4>) {
        self.inner.write().diffuse_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: impl Into<Vec4>) {
        self.inner.write().specular_color = color.into();
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct DirectionalLightBuilder {
    inner: DirectionalLight,
}

impl DirectionalLightBuilder {
    pub(crate) fn new(lights: &mut Lights) -> DirectionalLightBuilder {
        let inner = DirectionalLight::new(lights);
        Self { inner }
    }

    pub fn with_direction(self, direction: Vec3) -> Self {
        self.inner.set_direction(direction);
        self
    }

    pub fn with_ambient_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_ambient_color(color);
        self
    }

    pub fn with_diffuse_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_diffuse_color(color);
        self
    }

    pub fn with_specular_color(self, color: impl Into<Vec4>) -> Self {
        self.inner.set_specular_color(color);
        self
    }

    pub fn build(self) -> DirectionalLight {
        self.inner
    }
}

/// All lighting on the stage.
#[derive(Default)]
pub struct Lights {
    point_lights: Vec<Shared<PointLightRaw>>,
    spot_lights: Vec<Shared<SpotLightRaw>>,
    directional_lights: Vec<Shared<DirectionalLightRaw>>,
    dirty: DirtyLights,
    uniform: Option<LightsUniform>,
}

impl Lights {
    /// Get a reference to the lights uniform.
    pub fn uniform(&self) -> Option<&LightsUniform> {
        self.uniform.as_ref()
    }

    fn get_point_lights(&self) -> PointLights {
        let mut lights = [PointLightRaw::default(); POINT_LIGHTS_MAX];
        for (light, i) in self
            .point_lights
            .iter()
            .map(|l| *l.read())
            .zip(0..POINT_LIGHTS_MAX)
        {
            lights[i] = light;
        }
        PointLights {
            length: self.point_lights.len() as u32,
            lights,
        }
    }

    fn get_spot_lights(&self) -> SpotLights {
        let mut lights = [SpotLightRaw::default(); SPOT_LIGHTS_MAX];
        for (light, i) in self
            .spot_lights
            .iter()
            .map(|l| *l.read())
            .zip(0..SPOT_LIGHTS_MAX)
        {
            lights[i] = light;
        }
        SpotLights {
            length: self.spot_lights.len() as u32,
            lights,
        }
    }

    fn get_directional_lights(&self) -> DirectionalLights {
        let mut lights = [DirectionalLightRaw::default(); DIRECTIONAL_LIGHTS_MAX];
        for (light, i) in self
            .directional_lights
            .iter()
            .map(|l| *l.read())
            .zip(0..DIRECTIONAL_LIGHTS_MAX)
        {
            lights[i] = light;
        }
        DirectionalLights {
            length: self.directional_lights.len() as u32,
            lights,
        }
    }

    /// Update lights
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let update_point_lights = self
            .dirty
            .point
            .swap(false, std::sync::atomic::Ordering::Relaxed);
        let update_spot_lights = self
            .dirty
            .spot
            .swap(false, std::sync::atomic::Ordering::Relaxed);
        let update_directional_lights = self
            .dirty
            .directional
            .swap(false, std::sync::atomic::Ordering::Relaxed);

        if let Some(lights_uniform) = self.uniform.as_ref() {
            if update_point_lights {
                log::trace!("updating point lights");
                lights_uniform.update_point_lights(queue, &self.get_point_lights());
            }
            if update_spot_lights {
                log::trace!("updating spot lights");
                lights_uniform.update_spot_lights(queue, &self.get_spot_lights());
            }
            if update_directional_lights {
                log::trace!("updating directional lights");
                lights_uniform.update_directional_lights(queue, &self.get_directional_lights());
            }
        } else if update_point_lights || update_spot_lights || update_directional_lights {
            log::trace!("creating initial lights uniform");
            // create our lights uniform
            self.uniform = Some(LightsUniform::new(
                &device,
                &self.get_point_lights(),
                &self.get_spot_lights(),
                &self.get_directional_lights(),
            ));
        }
    }
}

#[derive(moongraph::Edges)]
pub struct LightUpdate {
    pub device: Read<Device>,
    pub queue: Read<Queue>,
    pub lights: Write<Lights>
}

impl LightUpdate {
    /// A render graph node that updates lights on-demand.
    pub fn run(mut self) -> Result<(), LightsError> {
        self.lights.update(&self.device, &self.queue);
        Ok(())
    }
}
