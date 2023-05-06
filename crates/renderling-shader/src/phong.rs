//! Phong shading helpers.
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math::Vec3ColorSwizzles, scene::{GpuLight, LightType}};
use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

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

impl GpuLight {
    /// Calculate a point light's color contribution to a fragment.
    pub fn color_phong_point(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        let light_pos: Vec3 = (view * self.position.xyz().extend(1.0)).xyz();
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
        let attenuation: f32 = attenuate(self.attenuation.xyz(), distance);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation;
        diffuse *= attenuation;
        specular *= attenuation;

        ambient + diffuse + specular
    }

    // Calculate a spotlight's color contribution to a fragment.
    pub fn color_phong_spot(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction.xyz() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_pos: Vec3 = (view * self.position.xyz().extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation.xyz(), distance);
        // spotlight intensity
        let direction: Vec3 = (-(view * self.direction.xyz().extend(0.0)).xyz()).normalize();
        let theta: f32 = light_dir.dot(direction);
        let epsilon: f32 = self.inner_cutoff - self.outer_cutoff;
        let intensity: f32 = ((theta - self.outer_cutoff) / epsilon).clamp(0.0, 1.0);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation * intensity;
        diffuse *= attenuation * intensity;
        specular *= attenuation * intensity;

        ambient + diffuse + specular
    }

    // Calculate a directional light's color contribution to a fragment.
    pub fn color_phong_directional(
        &self,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction.xyz() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_dir: Vec3 = (-(view * self.direction.xyz().extend(0.0)).xyz()).normalize();
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

pub fn shade_fragment(
    camera_view: &Mat4,
    lights: &[GpuLight],
    diffuse_color: Vec4,
    specular_color: Vec4,
    in_pos: Vec3,
    in_norm: Vec3,
) -> Vec4 {
    if lights.is_empty() || lights[0].light_type == LightType::END_OF_LIGHTS {
        // the scene is unlit, so we should provide some default
        let desaturated_norm = in_norm.abs().dot(Vec3::new(0.2126, 0.7152, 0.0722));
        return (diffuse_color.rgb() * desaturated_norm).extend(1.0);
    }

    let in_pos = camera_view.transform_vector3(in_pos);
    let norm: Vec3 = camera_view.transform_vector3(in_norm).normalize_or_zero();
    let camera_to_frag_dir: Vec3 = (-in_pos).normalize_or_zero();
    let mut color: Vec3 = Vec3::ZERO;
    for i in 0..lights.len() {
        let light = lights[i];
        match light.light_type {
            LightType::END_OF_LIGHTS => {
                break;
            }
            LightType::DIRECTIONAL_LIGHT => {
                color += light.color_phong_directional(
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            LightType::POINT_LIGHT => {
                color += light.color_phong_point(
                    in_pos,
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            LightType::SPOT_LIGHT => {
                color += light.color_phong_spot(
                    in_pos,
                    *camera_view,
                    norm,
                    camera_to_frag_dir,
                    diffuse_color,
                    specular_color,
                    // change this to material shininess when we have materials
                    16.0,
                );
            }
            _ => {}
        }
    }

    color.extend(1.0)
}
