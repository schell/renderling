//! Different kinds of scene lighting.
use std::sync::mpsc::Sender;

use crate::resources::Shared;
use nalgebra::{Point3, Vector3};
use renderling_core::light::{
    DirectionalLight as ShaderDirectionalLight, PointLight as ShaderPointLight,
    SpotLight as ShaderSpotLight,
};

pub(crate) struct PointLightInner(pub(crate) ShaderPointLight);
pub(crate) struct SpotLightInner(pub(crate) ShaderSpotLight);
pub(crate) struct DirectionalLightInner(pub(crate) ShaderDirectionalLight);

#[derive(PartialEq)]
pub(crate) enum LightUpdateCmd {
    PointLights,
    SpotLights,
    DirectionalLights,
}

/// Illuminates geometry in all directions surrounding a point, with attenuation.
///
/// This is like a light bulb, a match or a firefly.
pub struct PointLight {
    inner: Shared<PointLightInner>,
    cmd: Sender<LightUpdateCmd>,
}

impl PointLight {
    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vector3<f32>) {
        self.inner.write().0.position = position.into();
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: wgpu::Color) {
        self.inner.write().0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: wgpu::Color) {
        self.inner.write().0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: wgpu::Color) {
        self.inner.write().0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }

    /// Sets the attenuation coefficients for the point light.
    pub fn set_attenuation(&self, constant: f32, linear: f32, quadratic: f32) {
        self.inner.write().0.attenuation = [constant, linear, quadratic];
        self.cmd.send(LightUpdateCmd::PointLights).unwrap();
    }
}

pub struct PointLightBuilder<'a> {
    inner: PointLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Scene,
}

impl<'a> PointLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Scene,
        cmd: Sender<LightUpdateCmd>,
    ) -> PointLightBuilder<'a> {
        Self {
            inner: PointLightInner(Default::default()),
            cmd,
            scene,
        }
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
        .with_diffuse_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
        .with_specular_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
    }

    pub fn with_position(mut self, position: Point3<f32>) -> Self {
        self.inner.0.position = position.into();
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.0.attenuation = [constant, linear, quadratic];
        self
    }

    pub fn with_ambient_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_diffuse_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_specular_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
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

/// Illuminates geometry in a cone in front of a point, within a certain cutoff boundary.
///
/// This is like a street light or a flashlight.
pub struct SpotLight {
    inner: Shared<SpotLightInner>,
    cmd: Sender<LightUpdateCmd>,
}

impl SpotLight {
    /// Sets the position of the light in world space.
    pub fn set_position(&self, position: Vector3<f32>) {
        self.inner.write().0.position = position.into();
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the direction the light is pointing in.
    pub fn set_direction(&self, direction: Vector3<f32>) {
        self.inner.write().0.direction = direction.into();
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the inner angular cutoff.
    pub fn set_inner_cutoff(&self, cutoff: f32) {
        self.inner.write().0.inner_cutoff = cutoff;
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the constant, linear, and quadratic terms of attenuation.
    pub fn set_attenuation(&self, attenuation: [f32; 3]) {
        self.inner.write().0.attenuation = attenuation;
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the outer angular cutoff.
    pub fn set_outer_cutoff(&self, cutoff: f32) {
        self.inner.write().0.outer_cutoff = cutoff;
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: wgpu::Color) {
        self.inner.write().0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: wgpu::Color) {
        self.inner.write().0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: wgpu::Color) {
        self.inner.write().0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::SpotLights).unwrap();
    }
}

pub struct SpotLightBuilder<'a> {
    inner: SpotLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Scene,
}

impl<'a> SpotLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Scene,
        cmd: Sender<LightUpdateCmd>,
    ) -> SpotLightBuilder<'a> {
        Self {
            cmd,
            inner: SpotLightInner(Default::default()),
            scene,
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vector3::new(0.0, -1.0, 0.0))
        .with_ambient_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
        .with_diffuse_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
        .with_specular_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
    }

    pub fn with_position(mut self, position: Point3<f32>) -> Self {
        self.inner.0.position = position.into();
        self
    }

    pub fn with_direction(mut self, direction: Vector3<f32>) -> Self {
        self.inner.0.direction = direction.into();
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.0.attenuation = [constant, linear, quadratic];
        self
    }

    pub fn with_cutoff(mut self, inner: f32, outer: f32) -> Self {
        self.inner.0.inner_cutoff = inner;
        self.inner.0.outer_cutoff = outer;
        self
    }

    pub fn with_ambient_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_diffuse_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_specular_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
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
    pub fn set_direction(&self, direction: Vector3<f32>) {
        self.inner.write().0.direction = direction.into();
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the ambient color.
    pub fn set_ambient_color(&self, color: wgpu::Color) {
        self.inner.write().0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the diffuse color.
    pub fn set_diffuse_color(&self, color: wgpu::Color) {
        self.inner.write().0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }

    /// Sets the specular color.
    pub fn set_specular_color(&self, color: wgpu::Color) {
        self.inner.write().0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self.cmd.send(LightUpdateCmd::DirectionalLights).unwrap();
    }
}

pub struct DirectionalLightBuilder<'a> {
    inner: DirectionalLightInner,
    cmd: Sender<LightUpdateCmd>,
    scene: &'a mut crate::Scene,
}

impl<'a> DirectionalLightBuilder<'a> {
    pub(crate) fn new(
        scene: &'a mut crate::Scene,
        cmd: Sender<LightUpdateCmd>,
    ) -> DirectionalLightBuilder<'a> {
        Self {
            inner: DirectionalLightInner(Default::default()),
            cmd,
            scene,
        }
        .with_direction(Vector3::new(0.0, -1.0, 0.0))
        .with_ambient_color(wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        })
            .with_diffuse_color(wgpu::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            })
            .with_specular_color(wgpu::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            })
    }

    pub fn with_direction(mut self, direction: Vector3<f32>) -> Self {
        self.inner.0.direction = direction.into();
        self
    }

    pub fn with_ambient_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.ambient_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_diffuse_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.diffuse_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        self
    }

    pub fn with_specular_color(mut self, color: wgpu::Color) -> Self {
        self.inner.0.specular_color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
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
