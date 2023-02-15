//! Physically based renderer shader code.
//!
//! This is actually just blinne-phong lighting, but is a placeholder for PBR.
use glam::{mat3, mat4, vec4, Mat3, Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math::Vec3ColorSwizzles, CameraRaw};

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

pub fn main_vertex(
    camera: &CameraRaw,
    in_pos: Vec3,
    in_uv: Vec2,
    in_norm: Vec3,
    in_model_matrix_0: Vec4,
    in_model_matrix_1: Vec4,
    in_model_matrix_2: Vec4,
    in_model_matrix_3: Vec4,
    in_norm_matrix_0: Vec3,
    in_norm_matrix_1: Vec3,
    in_norm_matrix_2: Vec3,
    out_pos: &mut Vec3,
    out_uv: &mut Vec2,
    out_norm: &mut Vec3,
    gl_pos: &mut Vec4,
) {
    let model: Mat4 = mat4(
        in_model_matrix_0,
        in_model_matrix_1,
        in_model_matrix_2,
        in_model_matrix_3,
    );
    let norm: Mat3 = mat3(
        in_norm_matrix_0.xyz(),
        in_norm_matrix_1.xyz(),
        in_norm_matrix_2.xyz(),
    );
    let view: Mat3 = mat3(
        camera.view.x_axis.xyz(),
        camera.view.y_axis.xyz(),
        camera.view.z_axis.xyz(),
    );
    let view_pos: Vec4 = camera.view * model * vec4(in_pos.x, in_pos.y, in_pos.z, 1.0);

    *out_pos = view_pos.xyz();
    *out_uv = in_uv;
    *out_norm = view * norm * in_norm;

    *gl_pos = camera.projection * view_pos;
}

pub fn main_fragment(
    camera: &CameraRaw,
    diffuse_texture: &Image2d,
    diffuse_sampler: &Sampler,
    specular_texture: &Image2d,
    specular_sampler: &Sampler,
    material_shininess: &f32,
    point_lights: &PointLights,
    spot_lights: &SpotLights,
    directional_lights: &DirectionalLights,
    in_pos: Vec3,
    in_uv: Vec2,
    in_norm: Vec3,
    frag_color: &mut Vec4,
) {
    let diffuse_color: Vec4 = diffuse_texture.sample(*diffuse_sampler, in_uv);
    let specular_color: Vec4 = specular_texture.sample(*specular_sampler, in_uv);

    if directional_lights.length == 0 && point_lights.length == 0 && spot_lights.length == 0 {
        *frag_color = in_norm.extend(1.0);
        return;
    }

    let norm: Vec3 = in_norm.normalize_or_zero();
    let camera_to_frag_dir: Vec3 = (-in_pos).normalize_or_zero();

    let mut color: Vec3 = Vec3::default();

    for i in 0..directional_lights.length as usize {
        color += directional_lights.lights[i].color(
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            *material_shininess,
        );
    }

    for i in 0..point_lights.length as usize {
        color += point_lights.lights[i].color(
            in_pos,
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            *material_shininess,
        );
    }

    for i in 0..spot_lights.length as usize {
        color += spot_lights.lights[i].color(
            in_pos,
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            *material_shininess,
        );
    }

    *frag_color = color.extend(1.0);
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::Vec3;

    #[test]
    fn vec3_default_sanity() {
        assert_eq!(Vec3::ZERO, Vec3::default());
    }

    #[test]
    /// Tests that a light's `color` function returns positive elements.
    ///
    /// Important because lights never "remove" light from the scene.
    fn light_color_always_positive() {
        let vertex_pos = Vec3::new(0.0, 0.5, 0.0);
        let view = Mat4::look_at_rh(Vec3::new(1.8, 1.8, 1.8), Vec3::ZERO, Vec3::Y);
        let normal = Vec3::Y;
        let camera_to_frag_dir = (-vertex_pos).normalize();
        let diffuse_color = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let specular_color = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let shininess = 16.0;

        let point = PointLightRaw::default();
        let point_color = point.color(
            vertex_pos,
            view,
            normal,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            shininess,
        );
        assert!(point_color.x >= 0.0, "'{point_color:?}' x is negative");
        assert!(point_color.y >= 0.0, "'{point_color:?}' y is negative");
        assert!(point_color.z >= 0.0, "'{point_color:?}' z is negative");

        let spot = SpotLightRaw::default();
        let spot_color = spot.color(
            vertex_pos,
            view,
            normal,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            shininess,
        );
        assert!(spot_color.x >= 0.0, "'{spot_color:?}' x is negative");
        assert!(spot_color.y >= 0.0, "'{spot_color:?}' y is negative");
        assert!(spot_color.z >= 0.0, "'{spot_color:?}' z is negative");

        let directional = DirectionalLightRaw::default();
        let directional_color = directional.color(
            view,
            normal,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            shininess,
        );
        assert!(
            directional_color.x >= 0.0,
            "'{directional_color:?}' x is negative"
        );
        assert!(
            directional_color.y >= 0.0,
            "'{directional_color:?}' y is negative"
        );
        assert!(
            directional_color.z >= 0.0,
            "'{directional_color:?}' z is negative"
        );
    }

    //    #[test]
    //    #[should_panic]
    //    /// Tests that glam assertions are on while running tests.
    //    fn glam_assert() {
    //        let _ = Vec3::ZERO.normalize();
    //    }
}
