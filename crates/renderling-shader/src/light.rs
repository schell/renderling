//! Light types and calculations.
// TODO: make the internal fields private and then use getter/setters.
use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::math::Vec3ColorSwizzles;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(encase::ShaderType, bytemuck::Pod, bytemuck::Zeroable)
)]
pub struct PointLightRaw {
    pub position_: Vec4,
    pub attenuation_: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(encase::ShaderType, bytemuck::Pod, bytemuck::Zeroable)
)]
pub struct SpotLightRaw {
    pub position_: Vec4,
    pub direction_: Vec4,
    pub attenuation_: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
    pub cutoff_: Vec4,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(encase::ShaderType, bytemuck::Pod, bytemuck::Zeroable)
)]
pub struct DirectionalLightRaw {
    pub direction_: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

/// Calculate attenuation
///
/// attenuation.x: constant
/// attenuation.y: linear
/// attenuation.z: quadratic
pub fn attenuate(attenuation: Vec3, distance: f32) -> f32 {
    let level = attenuation.x + attenuation.y * distance + attenuation.z * (distance * distance);
    if level == 0.0 {
        // no attenuation
        1.0
    } else {
        1.0 / level
    }
}

impl PointLightRaw {
    pub fn position(&self) -> Vec3 {
        self.position_.xyz()
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation_.xyz()
    }

    /// Calculate a point light's color contribution to a fragment.
    pub fn color(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        let light_pos: Vec3 = (view * self.position().extend(1.0)).xyz();
        let vertex_to_light = light_pos - vertex_pos;
        let vertex_to_light_distance = vertex_to_light.length();

        let light_dir: Vec3 = vertex_to_light.normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = vertex_to_light_distance;
        let attenuation: f32 = attenuate(self.attenuation(), distance);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation;
        diffuse *= attenuation;
        specular *= attenuation;

        ambient + diffuse + specular
    }
}

impl SpotLightRaw {
    pub fn position(&self) -> Vec3 {
        self.position_.xyz()
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation_.xyz()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction_.xyz()
    }

    pub fn inner_cutoff(&self) -> f32 {
        self.cutoff_.x
    }

    pub fn outer_cutoff(&self) -> f32 {
        self.cutoff_.y
    }
    // Calculate a spotlight's color contribution to a fragment.
    pub fn color(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_pos: Vec3 = (view * self.position().extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation(), distance);
        // spotlight intensity
        let direction: Vec3 = (-(view * self.direction().extend(0.0)).xyz()).normalize();
        let theta: f32 = light_dir.dot(direction);
        let epsilon: f32 = self.inner_cutoff() - self.outer_cutoff();
        let intensity: f32 = ((theta - self.outer_cutoff()) / epsilon).clamp(0.0, 1.0);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation * intensity;
        diffuse *= attenuation * intensity;
        specular *= attenuation * intensity;

        ambient + diffuse + specular
    }
}

impl DirectionalLightRaw {
    pub fn direction(&self) -> Vec3 {
        self.direction_.xyz()
    }
    // Calculate a directional light's color contribution to a fragment.
    pub fn color(
        &self,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_dir: Vec3 = (-(view * self.direction().extend(0.0)).xyz()).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // combine results
        let ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient + diffuse + specular
    }
}

pub const POINT_LIGHTS_MAX: usize = 64;

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct PointLights {
    pub lights: [PointLightRaw; POINT_LIGHTS_MAX],
    pub length: u32,
}

impl Default for PointLights {
    fn default() -> Self {
        Self {
            length: 0,
            lights: [PointLightRaw::default(); POINT_LIGHTS_MAX],
        }
    }
}

pub const SPOT_LIGHTS_MAX: usize = 32;

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct SpotLights {
    pub lights: [SpotLightRaw; SPOT_LIGHTS_MAX],
    pub length: u32,
}

impl Default for SpotLights {
    fn default() -> Self {
        Self {
            length: 0,
            lights: [SpotLightRaw::default(); SPOT_LIGHTS_MAX],
        }
    }
}

pub const DIRECTIONAL_LIGHTS_MAX: usize = 8;

#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct DirectionalLights {
    pub lights: [DirectionalLightRaw; DIRECTIONAL_LIGHTS_MAX],
    pub length: u32,
}

impl Default for DirectionalLights {
    fn default() -> Self {
        Self {
            length: 0,
            lights: [DirectionalLightRaw::default(); DIRECTIONAL_LIGHTS_MAX],
        }
    }
}
