//! Physically based renderer shader code.
//!
//! This is actually just blinne-phong lighting, but is a placeholder for PBR.
use glam::{mat3, mat4, vec4, Mat3, Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math::Vec3ColorSwizzles, Camera};

pub trait IsLight: Default {
    fn num_lights(&self) -> u32;
    fn set_num_lights(&mut self, num_lights: usize);
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PointLight {
    pub position: Vec3,
    pub num_lights: u32,

    pub attenuation: Vec3,
    _padding: u32,

    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl IsLight for PointLight {
    fn num_lights(&self) -> u32 {
        self.num_lights
    }

    fn set_num_lights(&mut self, num_lights: usize) {
        self.num_lights = num_lights as u32;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct SpotLight {
    pub position: Vec3,
    pub num_lights: u32,

    pub direction: Vec3,
    pub inner_cutoff: f32,

    pub attenuation: Vec3,
    pub outer_cutoff: f32,

    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl IsLight for SpotLight {
    fn num_lights(&self) -> u32 {
        self.num_lights
    }

    fn set_num_lights(&mut self, num_lights: usize) {
        self.num_lights = num_lights as u32;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub num_lights: u32,

    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl IsLight for DirectionalLight {
    fn num_lights(&self) -> u32 {
        self.num_lights
    }


    fn set_num_lights(&mut self, num_lights: usize) {
        self.num_lights = num_lights as u32;
    }
}

#[repr(transparent)]
pub struct LightArray<T, const S: usize>(pub [T; S]); //


impl<T: IsLight + Default + Copy, const S: usize> Default for LightArray<T, S> {
    fn default() -> Self {
        let inner = [T::default(); S];
        Self(inner)
    }
}

impl<T: IsLight + Default + Copy, const S: usize> LightArray<T, S> {
    pub fn len(&self) -> usize {
        self.0[0].num_lights() as usize
    }

    #[cfg(feature = "cpu")]
    pub fn into_array(lights: Vec<T>) -> Self {
        let len = lights.len();
        let mut array: [T; S] = [T::default(); S];
        for (mut light, i) in lights.into_iter().zip(0..S) {
            light.set_num_lights(len);
            array[i] = light;
        }
        LightArray(array)
    }
}

pub const POINT_LIGHTS_MAX: usize = 64;
pub type PointLights = LightArray<PointLight, POINT_LIGHTS_MAX>;

pub const SPOT_LIGHTS_MAX: usize = 32;
pub type SpotLights = LightArray<SpotLight, SPOT_LIGHTS_MAX>;

pub const DIRECTIONAL_LIGHTS_MAX: usize = 8;
pub type DirectionalLights = LightArray<DirectionalLight, DIRECTIONAL_LIGHTS_MAX>;

/// Calculate attenuation
///
/// attenuation.x: constant
/// attenuation.y: linear
/// attenuation.z: quadratic
pub fn attenuate(attenuation: Vec3, distance: f32) -> f32 {
    1.0 / (attenuation.x + attenuation.y * distance + attenuation.z * (distance * distance))
}

impl PointLight {
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
        let light_pos: Vec3 = (view * self.position.extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation, distance);
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

impl SpotLight {
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
        let light_pos: Vec3 = (view * self.position.extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation, distance);
        // spotlight intensity
        let direction: Vec3 = (-(view * self.direction.extend(0.0)).xyz()).normalize();
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
}

impl DirectionalLight {
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
        let light_dir: Vec3 = (-(view * self.direction.extend(0.0)).xyz()).normalize();
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

pub fn main_vertex(
    camera: &Camera,
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
    camera: &Camera,
    diffuse_texture: &Image2d,
    diffuse_sampler: &Sampler,
    specular_texture: &Image2d,
    specular_sampler: &Sampler,
    material_shininess: &Vec4,
    point_lights: &PointLights,
    spot_lights: &SpotLights,
    directional_lights: &DirectionalLights,
    in_pos: Vec3,
    in_uv: Vec2,
    in_norm: Vec3,
    frag_color: &mut Vec4,
) {
    let norm: Vec3 = in_norm.normalize();
    let camera_to_frag_dir: Vec3 = (-in_pos).normalize();
    let diffuse_color: Vec4 = diffuse_texture.sample(*diffuse_sampler, in_uv);
    let specular_color: Vec4 = specular_texture.sample(*specular_sampler, in_uv);

    let mut color: Vec3 = Vec3::default();

    for i in 0..directional_lights.len() {
        color += directional_lights.0[i].color(
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            material_shininess.x,
        );
    }

    for i in 0..point_lights.len() {
        color += point_lights.0[i].color(
            in_pos,
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            material_shininess.x,
        );
    }

    for i in 0..spot_lights.len() {
        color += spot_lights.0[i].color(
            in_pos,
            camera.view,
            norm,
            camera_to_frag_dir,
            diffuse_color,
            specular_color,
            material_shininess.x,
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
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), Vec3::default());
    }

    #[test]
    fn size_of_lights_sanity() {
        assert_eq!(7168 / 64, std::mem::size_of_val(&PointLight::default()));
    }
}
