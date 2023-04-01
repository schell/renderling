//! Physically based renderer shader code.
//!
//! This is actually just blinn-phong lighting, but is a placeholder for PBR.
use glam::{mat3, mat4, vec4, Mat3, Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles, vec3};
use spirv_std::{image::Image2d, Sampler};

use crate::{CameraRaw, light::*, math::Vec3ColorSwizzles, mesh::*};

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
        // the scene is unlit, so we should provide some default
        let desaturated_norm = in_norm.abs().dot(vec3(0.2126, 0.7152, 0.0722));
        *frag_color = (diffuse_color.rgb() * desaturated_norm).extend(1.0);
        return;
    }

    let norm: Vec3 = in_norm.normalize_or_zero();
    let camera_to_frag_dir: Vec3 = (-in_pos).normalize_or_zero();

    let mut color: Vec3 = Vec3::ZERO;

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
