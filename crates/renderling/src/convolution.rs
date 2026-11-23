//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.

#[wgsl_rs::wgsl]
pub mod mipmap {
    //! Mipmap generation shader.
    use wgsl_rs::std::*;

    use crate::shader::constants::{CLIP_SPACE_COORD_QUAD_CCW, UV_COORD_QUAD_CCW};

    pub struct MipmapInterStage {
        #[location(0)]
        uv: Vec2f,
        #[builtin(position)]
        #[allow(dead_code)]
        gl_pos: Vec4f,
    }

    #[vertex]
    /// Vertex shader for generating texture mips.
    pub fn generate_mipmap_vertex(#[builtin(vertex_index)] vertex_id: u32) -> MipmapInterStage {
        let i = vertex_id as usize;

        MipmapInterStage {
            uv: UV_COORD_QUAD_CCW[i],
            gl_pos: CLIP_SPACE_COORD_QUAD_CCW[i],
        }
    }

    texture!(group(0), binding(0), TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(1), SAMPLER: Sampler);

    #[fragment]
    /// Fragment shader for generating texture mips.
    pub fn generate_mipmap_fragment(input: MipmapInterStage) -> Vec4f {
        texture_sample(TEXTURE, SAMPLER, input.uv)
    }
}

#[wgsl_rs::wgsl]
pub mod brdf {
    //! BRDF shader.
    #![allow(clippy::manual_rotate)]
    use wgsl_rs::glm::*;

    use crate::shader::{
        constants::{CLIP_SPACE_COORD_QUAD_CCW, F32_EPSILON, PI, UV_COORD_QUAD_CCW},
        math::is_zero,
    };

    pub struct BrdfInterStage {
        #[builtin(position)]
        #[allow(dead_code)]
        pos: Vec4f,
        #[location(0)]
        uv: Vec2f,
    }

    #[vertex]
    /// Vertex shader for creating a BRDF LUT.
    pub fn brdf_lut_convolution_vertex(#[builtin(vertex_index)] vertex_id: u32) -> BrdfInterStage {
        let pos = CLIP_SPACE_COORD_QUAD_CCW[vertex_id as usize];
        let uv = UV_COORD_QUAD_CCW[vertex_id as usize];
        BrdfInterStage { pos, uv }
    }

    pub fn radical_inverse_vdc(bits: u32) -> f32 {
        (reverse_bits(bits) as f32) * 2.3283064e-10 // / 0x100000000
    }

    pub fn hammersley(i: u32, n: u32) -> Vec2f {
        vec2f(i as f32 / n as f32, radical_inverse_vdc(i))
    }

    pub fn importance_sample_ggx(xi: Vec2f, n: Vec3f, roughness: f32) -> Vec3f {
        let a = roughness * roughness;

        let phi = 2.0 * PI * xi.x;
        let cos_theta = sqrt((1.0 - xi.y) / (1.0 + (a * a - 1.0) * xi.y));
        let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

        // Convert spherical to cartesian coordinates
        let h = vec3f(cos(phi) * sin_theta, sin(phi) * sin_theta, cos_theta);

        // Convert tangent-space vector to world-space vector
        let mut up = Vec3f::X;
        if abs(n.z) < 0.999 {
            up = Vec3f::Z;
        }
        let tangent = normalize_or_zero3(cross(up, n));
        let bitangent = cross(n, tangent);

        let result = tangent * h.x + bitangent * h.y + n * h.z;
        normalize_or_zero3(result)
    }

    pub fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
        let r = roughness;
        let k = (r * r) / 2.0;

        let nom = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;

        let mut result = 0.0;
        if !is_zero(denom) {
            result = nom / denom;
        }
        result
    }

    pub fn geometry_smith(normal: Vec3f, view_dir: Vec3f, light_dir: Vec3f, roughness: f32) -> f32 {
        let n_dot_v = max(dot(normal, view_dir), 0.0);
        let n_dot_l = max(dot(normal, light_dir), 0.0);
        let ggx1 = geometry_schlick_ggx(n_dot_v, roughness);
        let ggx2 = geometry_schlick_ggx(n_dot_l, roughness);

        ggx1 * ggx2
    }

    pub const BRDF_SAMPLE_COUNT: u32 = 1024;

    pub fn integrate_brdf(n_dot_v: f32, roughness: f32) -> Vec2f {
        let mut n_dot_v = n_dot_v;
        n_dot_v = max(n_dot_v, F32_EPSILON);
        let v = vec3f(sqrt(1.0 - n_dot_v * n_dot_v), 0.0, n_dot_v);

        let mut a = 0.0f32;
        let mut b = 0.0f32;

        let n = Vec3f::Z;

        let mut i = 0u32;
        while i < BRDF_SAMPLE_COUNT {
            i += 1;

            let xi = hammersley(i, BRDF_SAMPLE_COUNT);
            let h = importance_sample_ggx(xi, n, roughness);
            let l = normalize_or_zero3(2.0 * dot(v, h) * h - v);

            let n_dot_l = max(l.z, 0.0);
            let n_dot_h = max(h.z, 0.0);
            let v_dot_h = max(dot(v, h), 0.0);

            if n_dot_l > 0.0 {
                let g = geometry_smith(n, v, l, roughness);
                let denom = n_dot_h * n_dot_v;
                let g_vis = (g * v_dot_h) / denom;
                let f_c = pow(1.0 - v_dot_h, 5.0);

                a += (1.0 - f_c) * g_vis;
                b += f_c * g_vis;
            }
        }

        a /= BRDF_SAMPLE_COUNT as f32;
        b /= BRDF_SAMPLE_COUNT as f32;

        vec2f(a, b)
    }

    #[fragment]
    /// Fragment shader for creating a BRDF LUT.
    pub fn brdf_lut_convolution_fragment(input: BrdfInterStage) -> Vec4f {
        let brdf = integrate_brdf(input.uv.x, input.uv.y);
        vec4f(brdf.x, brdf.y, 0.0, 1.0)
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod prefilter_environment_cubemap {
    //! Shader side of convolution.

    use crabslab_wgsl::{slab_read, Id, SlabItem};
    use wgsl_rs::glm::*;

    use crate::shader::{
        camera::CameraDescriptor,
        constants::{F32_EPSILON, FRAC_1_PI},
        math::cube,
    };

    use super::brdf::{hammersley, importance_sample_ggx, BRDF_SAMPLE_COUNT};

    /// Used by [`prefilter_environment_cubemap_vertex`] to read the camera and
    /// roughness values from the slab.
    #[derive(Clone, Copy, Default, Wgsl, SlabItem)]
    pub struct VertexPrefilterEnvironmentCubemapIds {
        pub camera: Id<CameraDescriptor>,
        // TODO: does this have to be an Id? Pretty sure it can be inline
        pub roughness: Id<f32>,
    }

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);

    pub struct PrefilterEnvironmentCubemapInput {
        #[builtin(instance_index)]
        prefilter_id: u32,
        #[builtin(vertex_index)]
        vertex_id: u32,
    }

    pub struct PrefilterEnvironmentCubmapInterStage {
        #[location(0)]
        pos: Vec3f,
        #[location(1)]
        roughness: f32,
        #[builtin(position)]
        #[allow(dead_code)]
        gl_pos: Vec4f,
    }

    /// Vertex shader for rendering a "prefilter environment" cubemap.
    #[vertex]
    pub fn prefilter_environment_cubemap_vertex(
        input: PrefilterEnvironmentCubemapInput,
    ) -> PrefilterEnvironmentCubmapInterStage {
        let in_pos = cube()[input.vertex_id as usize];

        let ids: VertexPrefilterEnvironmentCubemapIds;
        slab_read!(
            VertexPrefilterEnvironmentCubemapIds,
            get!(SLAB),
            input.prefilter_id,
            ids
        );

        let camera: CameraDescriptor;
        slab_read!(CameraDescriptor, get!(SLAB), ids.camera.index, camera);

        let roughness: f32;
        slab_read!(f32, get!(SLAB), ids.roughness.index, roughness);

        PrefilterEnvironmentCubmapInterStage {
            pos: in_pos,
            roughness,
            gl_pos: CameraDescriptor::view_projection(camera) * vec4f_from_3(in_pos, 1.0),
        }
    }

    texture!(group(0), binding(1), ENVIRONMENT_CUBEMAP: TextureCube<f32>);
    sampler!(group(0), binding(2), SAMPLER: Sampler);

    /// Fragment shader for rendering a "prefilter environment" cubemap.
    ///
    /// Lambertian prefilter.
    #[fragment]
    pub fn prefilter_environment_cubemap_fragment(
        input: PrefilterEnvironmentCubmapInterStage,
    ) -> Vec4f {
        let mut n = normalize_or_zero3(input.pos);
        // `wgpu` and vulkan's y coords are flipped from opengl
        n.y *= -1.0;
        let r = n;
        let v = r;

        let mut total_weight = 0.0f32;
        let mut prefiltered_color = Vec3f::ZERO;

        #[wgsl_allow(non_literal_loop_bounds)]
        for i in 0..BRDF_SAMPLE_COUNT {
            let xi = hammersley(i, BRDF_SAMPLE_COUNT);
            let h = importance_sample_ggx(xi, n, input.roughness);
            let l = normalize_or_zero3(2.0 * dot(v, h) * h - v);

            let n_dot_l = max(dot(n, l), 0.0);
            if n_dot_l > 0.0 {
                let mut mip_level = 0.0;
                if input.roughness != 0.0 {
                    mip_level = calc_lod(n_dot_l);
                }
                prefiltered_color +=
                    texture_sample_level(ENVIRONMENT_CUBEMAP, SAMPLER, l, mip_level).xyz()
                        * n_dot_l;
                total_weight += n_dot_l;
            }
        }

        prefiltered_color /= total_weight;
        vec4f_from_3(prefiltered_color, 1.0)
    }

    pub fn calc_lod(n_dot_l: f32) -> f32 {
        let cube_width = 512.0;
        let pdf = max(n_dot_l * FRAC_1_PI, 0.0);
        0.5 * log2(6.0 * cube_width * cube_width / max(BRDF_SAMPLE_COUNT as f32 * pdf, F32_EPSILON))
    }
}

pub mod shader {
    //! Shader side of convolution.
    use crabslab::{Id, SlabItem};

    use crate::camera::shader::CameraDescriptor;

    /// Used by [`prefilter_environment_cubemap_vertex`] to read the camera and
    /// roughness values from the slab.
    #[derive(Clone, Copy, Default, SlabItem)]
    pub struct VertexPrefilterEnvironmentCubemapIds {
        pub camera: Id<CameraDescriptor>,
        // TODO: does this have to be an Id? Pretty sure it can be inline
        pub roughness: Id<f32>,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integrate_brdf_sanity() {
        let points = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
        for (x, y) in points.into_iter() {
            let brdf = brdf::integrate_brdf(x, y);
            assert!(
                !(brdf.x.is_nan() || brdf.y.is_nan()),
                "brdf is NaN at {x},{y}"
            );
        }
        let size = 32;
        let mut img = image::RgbaImage::new(size, size);
        for (x, y, image::Rgba([r, g, _, a])) in img.enumerate_pixels_mut() {
            let u = x as f32 / size as f32;
            let v = y as f32 / size as f32;
            let brdf = brdf::integrate_brdf(u, v);
            *r = (brdf.x * 255.0) as u8;
            *g = (brdf.y * 255.0) as u8;
            *a = 255;
        }
        img_diff::assert_img_eq("skybox/brdf_cpu.png", img);
    }
}
