//! Different kinds of scene lighting.
use std::sync::mpsc::Sender;

use crate::resources::Shared;
use glam::{vec3, vec4, Vec3, Vec4};
use renderling_shader::light::{DirectionalLightRaw, PointLightRaw, SpotLightRaw};

pub(crate) struct PointLightInner(pub(crate) PointLightRaw);
pub(crate) struct SpotLightInner(pub(crate) SpotLightRaw);
pub(crate) struct DirectionalLightInner(pub(crate) DirectionalLightRaw);

#[derive(PartialEq)]
pub(crate) enum LightUpdateCmd {
    PointLights,
    SpotLights,
    DirectionalLights,
}

/// Illuminates geometry in all directions surrounding a point, with
/// attenuation.
///
/// This is like a light bulb, a match or a firefly.
pub struct PointLight {
    inner: Shared<PointLightInner>,
    cmd: Sender<LightUpdateCmd>,
}

impl PointLight {
    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vec3) {
        self.inner.write().0.position_ = position.extend(0.0);
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: wgpu::Color) {
        self.inner.write().0.ambient_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: wgpu::Color) {
        self.inner.write().0.diffuse_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: wgpu::Color) {
        self.inner.write().0.specular_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the attenuation coefficients for the point light.
    pub fn set_attenuation(&self, constant: f32, linear: f32, quadratic: f32) {
        self.inner.write().0.attenuation_ = vec4(constant, linear, quadratic, 0.0);
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }
}

pub struct PointLightBuilder<'a> {
    inner: PointLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Stage,
}

impl<'a> PointLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Stage,
        cmd: Sender<LightUpdateCmd>,
    ) -> PointLightBuilder<'a> {
        let white = Vec4::splat(1.0);
        Self {
            inner: PointLightInner(Default::default()),
            cmd,
            scene,
        }
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.inner.0.position_ = position.extend(0.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.0.attenuation_ = vec3(constant, linear, quadratic).extend(0.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.specular_color = color.into();
        self
    }

    pub fn build(self) -> PointLight {
        let inner = Shared::new(self.inner);
        self.scene.point_lights.push(inner.clone());
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
        PointLight {
            inner,
            cmd: self.cmd.clone(),
        }
    }
}

/// Illuminates geometry in a cone in front of a point, within a certain cutoff
/// boundary.
///
/// This is like a street light or a flashlight.
pub struct SpotLight {
    inner: Shared<SpotLightInner>,
    cmd: Sender<LightUpdateCmd>,
}

impl SpotLight {
    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vec3) {
        self.inner.write().0.position_ = position.extend(0.0);
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the direction the light is pointing in.
    pub fn set_direction(&self, direction: Vec3) {
        self.inner.write().0.direction_ = direction.extend(0.0);
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the inner angular cutoff.
    pub fn set_inner_cutoff(&self, cutoff: f32) {
        self.inner.write().0.cutoff_.x = cutoff;
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the constant, linear, and quadratic terms of attenuation.
    pub fn set_attenuation(&self, attenuation: Vec3) {
        self.inner.write().0.attenuation_ = attenuation.extend(0.0);
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the outer angular cutoff.
    pub fn set_outer_cutoff(&self, cutoff: f32) {
        self.inner.write().0.cutoff_.y = cutoff;
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: wgpu::Color) {
        self.inner.write().0.ambient_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: wgpu::Color) {
        self.inner.write().0.diffuse_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: wgpu::Color) {
        self.inner.write().0.specular_color = vec4(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        );
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }
}

pub struct SpotLightBuilder<'a> {
    inner: SpotLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Stage,
}

impl<'a> SpotLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Stage,
        cmd: Sender<LightUpdateCmd>,
    ) -> SpotLightBuilder<'a> {
        let white = Vec4::splat(1.0);
        Self {
            cmd,
            inner: SpotLightInner(Default::default()),
            scene,
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.inner.0.position_ = position.extend(0.0);
        self
    }

    pub fn with_direction(mut self, direction: Vec3) -> Self {
        self.inner.0.direction_ = direction.extend(0.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.0.attenuation_ = vec4(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_cutoff(mut self, inner: f32, outer: f32) -> Self {
        self.inner.0.cutoff_.x = inner;
        self.inner.0.cutoff_.y = outer;
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.specular_color = color.into();
        self
    }

    pub fn build(self) -> SpotLight {
        let inner = Shared::new(self.inner);
        self.scene.spot_lights.push(inner.clone());
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
        SpotLight {
            inner,
            cmd: self.cmd,
        }
    }
}

/// Illuminates all geometry from a certain direction, without attenuation.
///
/// This is like the sun, or the moon.
pub struct DirectionalLight {
    inner: Shared<DirectionalLightInner>,
    cmd: Sender<LightUpdateCmd>,
}

impl DirectionalLight {
    /// Sets the direction this light points in.
    pub fn set_direction(&self, direction: Vec3) {
        self.inner.write().0.direction_ = direction.extend(0.0);
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: impl Into<Vec4>) {
        self.inner.write().0.ambient_color = color.into();
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: impl Into<Vec4>) {
        self.inner.write().0.diffuse_color = color.into();
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: impl Into<Vec4>) {
        self.inner.write().0.specular_color = color.into();
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }
}

pub struct DirectionalLightBuilder<'a> {
    inner: DirectionalLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Stage,
}

impl<'a> DirectionalLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Stage,
        cmd: Sender<LightUpdateCmd>,
    ) -> DirectionalLightBuilder<'a> {
        Self {
            inner: DirectionalLightInner(Default::default()),
            cmd,
            scene,
        }
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(Vec4::splat(1.0))
        .with_diffuse_color(Vec4::splat(1.0))
        .with_specular_color(Vec4::splat(1.0))
    }

    pub fn with_direction(mut self, direction: Vec3) -> Self {
        self.inner.0.direction_ = direction.extend(0.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.0.specular_color = color.into();
        self
    }

    pub fn build(self) -> DirectionalLight {
        let inner = Shared::new(self.inner);
        self.scene.directional_lights.push(inner.clone());
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
        DirectionalLight {
            inner,
            cmd: self.cmd,
        }
    }
}
