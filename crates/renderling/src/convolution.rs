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

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod prefilter_environment_cubemap {
    //! Shader side of convolution.

    use crabslab_wgsl::{slab_read, Id, SlabItem};
    use wgsl_rs::glm::*;

    use crate::{
        convolution::brdf::{hammersley, importance_sample_ggx, BRDF_SAMPLE_COUNT},
        shader::{
            camera::CameraDescriptor,
            constants::{F32_EPSILON, FRAC_1_PI},
            math::cube,
        },
    };

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
        // TODO: figure out if this is still needed after port to wgsl-rs since we aren't outputting SPIR-V
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

pub mod shader {
    //! Shader side of convolution.
    use crabslab::{Id, Slab, SlabItem};
    use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
    use spirv_std::{
        image::{Cubemap, Image2d},
        num_traits::Zero,
        spirv, Sampler,
    };

    #[allow(unused_imports)]
    use spirv_std::num_traits::Float;

    use crate::{camera::shader::CameraDescriptor, math::IsVector};

    // Allow manual bit rotation because this code is `no_std`.
    #[allow(clippy::manual_rotate)]
    fn radical_inverse_vdc(mut bits: u32) -> f32 {
        bits = (bits << 16u32) | (bits >> 16u32);
        bits = ((bits & 0x55555555u32) << 1u32) | ((bits & 0xAAAAAAAAu32) >> 1u32);
        bits = ((bits & 0x33333333u32) << 2u32) | ((bits & 0xCCCCCCCCu32) >> 2u32);
        bits = ((bits & 0x0F0F0F0Fu32) << 4u32) | ((bits & 0xF0F0F0F0u32) >> 4u32);
        bits = ((bits & 0x00FF00FFu32) << 8u32) | ((bits & 0xFF00FF00u32) >> 8u32);
        (bits as f32) * 2.328_306_4e-10 // / 0x100000000
    }

    fn hammersley(i: u32, n: u32) -> Vec2 {
        Vec2::new(i as f32 / n as f32, radical_inverse_vdc(i))
    }

    fn importance_sample_ggx(xi: Vec2, n: Vec3, roughness: f32) -> Vec3 {
        let a = roughness * roughness;

        let phi = 2.0 * core::f32::consts::PI * xi.x;
        let cos_theta = f32::sqrt((1.0 - xi.y) / (1.0 + (a * a - 1.0) * xi.y));
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        // Convert spherical to cartesian coordinates
        let h = Vec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta);

        // Convert tangent-space vector to world-space vector
        let up = if n.z.abs() < 0.999 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let tangent = up.cross(n).alt_norm_or_zero();
        let bitangent = n.cross(tangent);

        let result = tangent * h.x + bitangent * h.y + n * h.z;
        result.alt_norm_or_zero()
    }

    fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
        let r = roughness;
        let k = (r * r) / 2.0;

        let nom = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;

        if denom.is_zero() {
            0.0
        } else {
            nom / denom
        }
    }

    fn geometry_smith(normal: Vec3, view_dir: Vec3, light_dir: Vec3, roughness: f32) -> f32 {
        let n_dot_v = normal.dot(view_dir).max(0.0);
        let n_dot_l = normal.dot(light_dir).max(0.0);
        let ggx1 = geometry_schlick_ggx(n_dot_v, roughness);
        let ggx2 = geometry_schlick_ggx(n_dot_l, roughness);

        ggx1 * ggx2
    }

    const SAMPLE_COUNT: u32 = 1024;

    pub fn integrate_brdf(mut n_dot_v: f32, roughness: f32) -> Vec2 {
        n_dot_v = n_dot_v.max(f32::EPSILON);
        let v = Vec3::new(f32::sqrt(1.0 - n_dot_v * n_dot_v), 0.0, n_dot_v);

        let mut a = 0.0f32;
        let mut b = 0.0f32;

        let n = Vec3::Z;

        for i in 1..SAMPLE_COUNT {
            let xi = hammersley(i, SAMPLE_COUNT);
            let h = importance_sample_ggx(xi, n, roughness);
            let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

            let n_dot_l = l.z.max(0.0);
            let n_dot_h = h.z.max(0.0);
            let v_dot_h = v.dot(h).max(0.0);

            if n_dot_l > 0.0 {
                let g = geometry_smith(n, v, l, roughness);
                let denom = n_dot_h * n_dot_v;
                let g_vis = (g * v_dot_h) / denom;
                let f_c = (1.0 - v_dot_h).powf(5.0);

                a += (1.0 - f_c) * g_vis;
                b += f_c * g_vis;
            }
        }

        a /= SAMPLE_COUNT as f32;
        b /= SAMPLE_COUNT as f32;

        Vec2::new(a, b)
    }

    /// This function doesn't work on rust-gpu, presumably because of the loop.
    pub fn integrate_brdf_doesnt_work(mut n_dot_v: f32, roughness: f32) -> Vec2 {
        n_dot_v = n_dot_v.max(f32::EPSILON);
        let v = Vec3::new(f32::sqrt(1.0 - n_dot_v * n_dot_v), 0.0, n_dot_v);

        let mut a = 0.0f32;
        let mut b = 0.0f32;

        let n = Vec3::Z;

        let mut i = 0u32;
        while i < SAMPLE_COUNT {
            i += 1;

            let xi = hammersley(i, SAMPLE_COUNT);
            let h = importance_sample_ggx(xi, n, roughness);
            let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

            let n_dot_l = l.z.max(0.0);
            let n_dot_h = h.z.max(0.0);
            let v_dot_h = v.dot(h).max(0.0);

            if n_dot_l > 0.0 {
                let g = geometry_smith(n, v, l, roughness);
                let denom = n_dot_h * n_dot_v;
                let g_vis = (g * v_dot_h) / denom;
                let f_c = (1.0 - v_dot_h).powf(5.0);

                a += (1.0 - f_c) * g_vis;
                b += f_c * g_vis;
            }
        }

        a /= SAMPLE_COUNT as f32;
        b /= SAMPLE_COUNT as f32;

        Vec2::new(a, b)
    }

    /// Used by [`prefilter_environment_cubemap_vertex`] to read the camera and
    /// roughness values from the slab.
    #[derive(Clone, Copy, Default, SlabItem)]
    pub struct VertexPrefilterEnvironmentCubemapIds {
        pub camera: Id<CameraDescriptor>,
        // TODO: does this have to be an Id? Pretty sure it can be inline
        pub roughness: Id<f32>,
    }

    /// Vertex shader for rendering a "prefilter environment" cubemap.
    #[spirv(vertex)]
    pub fn prefilter_environment_cubemap_vertex(
        #[spirv(instance_index)] prefilter_id: Id<VertexPrefilterEnvironmentCubemapIds>,
        #[spirv(vertex_index)] vertex_id: u32,
        #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
        out_pos: &mut Vec3,
        out_roughness: &mut f32,
        #[spirv(position)] gl_pos: &mut Vec4,
    ) {
        let in_pos = crate::math::CUBE[vertex_id as usize];
        let VertexPrefilterEnvironmentCubemapIds { camera, roughness } = slab.read(prefilter_id);
        let camera = slab.read(camera);
        *out_roughness = slab.read(roughness);
        *out_pos = in_pos;
        *gl_pos = camera.view_projection() * in_pos.extend(1.0);
    }

    /// Fragment shader for rendering a "prefilter environment" cubemap.
    ///
    /// Lambertian prefilter.
    #[spirv(fragment)]
    pub fn prefilter_environment_cubemap_fragment(
        #[spirv(descriptor_set = 0, binding = 1)] environment_cubemap: &Cubemap,
        #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
        in_pos: Vec3,
        in_roughness: f32,
        frag_color: &mut Vec4,
    ) {
        let mut n = in_pos.alt_norm_or_zero();
        // `wgpu` and vulkan's y coords are flipped from opengl
        n.y *= -1.0;
        let r = n;
        let v = r;

        let mut total_weight = 0.0f32;
        let mut prefiltered_color = Vec3::ZERO;

        for i in 0..SAMPLE_COUNT {
            let xi = hammersley(i, SAMPLE_COUNT);
            let h = importance_sample_ggx(xi, n, in_roughness);
            let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

            let n_dot_l = n.dot(l).max(0.0);
            if n_dot_l > 0.0 {
                let mip_level = if in_roughness == 0.0 {
                    0.0
                } else {
                    calc_lod(n_dot_l)
                };
                prefiltered_color += environment_cubemap
                    .sample_by_lod(*sampler, l, mip_level)
                    .xyz()
                    * n_dot_l;
                total_weight += n_dot_l;
            }
        }

        prefiltered_color /= total_weight;
        *frag_color = prefiltered_color.extend(1.0);
    }

    pub fn calc_lod_old(n: Vec3, v: Vec3, h: Vec3, roughness: f32) -> f32 {
        // sample from the environment's mip level based on roughness/pdf
        let d = crate::pbr::shader::normal_distribution_ggx(n, h, roughness);
        let n_dot_h = n.dot(h).max(0.0);
        let h_dot_v = h.dot(v).max(0.0);
        let pdf = (d * n_dot_h / (4.0 * h_dot_v)).max(f32::EPSILON);

        let resolution = 512.0; // resolution of source cubemap (per face)
        let sa_texel = 4.0 * core::f32::consts::PI / (6.0 * resolution * resolution);
        let sa_sample = 1.0 / (SAMPLE_COUNT as f32 * pdf + f32::EPSILON);

        0.5 * (sa_sample / sa_texel).log2()
    }

    pub fn calc_lod(n_dot_l: f32) -> f32 {
        let cube_width = 512.0;
        let pdf = (n_dot_l * core::f32::consts::FRAC_1_PI).max(0.0);
        0.5 * (6.0 * cube_width * cube_width / (SAMPLE_COUNT as f32 * pdf).max(f32::EPSILON)).log2()
    }

    #[spirv(vertex)]
    /// Vertex shader for generating texture mips.
    pub fn generate_mipmap_vertex(
        #[spirv(vertex_index)] vertex_id: u32,
        out_uv: &mut Vec2,
        #[spirv(position)] gl_pos: &mut Vec4,
    ) {
        let i = vertex_id as usize;
        *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
        *gl_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
    }

    #[spirv(fragment)]
    /// Fragment shader for generating texture mips.
    pub fn generate_mipmap_fragment(
        #[spirv(descriptor_set = 0, binding = 0)] texture: &Image2d,
        #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
        in_uv: Vec2,
        frag_color: &mut Vec4,
    ) {
        *frag_color = texture.sample(*sampler, in_uv);
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Vert {
        pos: [f32; 3],
        uv: [f32; 2],
    }

    /// A screen-space quad.
    const BRDF_VERTS: [Vert; 6] = {
        let bl = Vert {
            pos: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
        };
        let br = Vert {
            pos: [1.0, -1.0, 0.0],
            uv: [1.0, 1.0],
        };
        let tl = Vert {
            pos: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
        };
        let tr = Vert {
            pos: [1.0, 1.0, 0.0],
            uv: [1.0, 0.0],
        };

        [bl, br, tr, bl, tr, tl]
    };

    #[spirv(vertex)]
    /// Vertex shader for creating a BRDF LUT.
    pub fn brdf_lut_convolution_vertex(
        #[spirv(vertex_index)] vertex_id: u32,
        out_uv: &mut glam::Vec2,
        #[spirv(position)] gl_pos: &mut glam::Vec4,
    ) {
        let Vert { pos, uv } = BRDF_VERTS[vertex_id as usize];
        *out_uv = Vec2::from(uv);
        *gl_pos = Vec3::from(pos).extend(1.0);
    }

    #[spirv(fragment)]
    /// Fragment shader for creating a BRDF LUT.
    pub fn brdf_lut_convolution_fragment(in_uv: glam::Vec2, out_color: &mut glam::Vec2) {
        *out_color = integrate_brdf(in_uv.x, in_uv.y);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integrate_brdf_sanity() {
        let points = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
        for (x, y) in points.into_iter() {
            assert!(
                !shader::integrate_brdf(x, y).is_nan(),
                "brdf is NaN at {x},{y}"
            );
        }
        let size = 32;
        let mut img = image::RgbaImage::new(size, size);
        for (x, y, image::Rgba([r, g, _, a])) in img.enumerate_pixels_mut() {
            let u = x as f32 / size as f32;
            let v = y as f32 / size as f32;
            let brdf = shader::integrate_brdf(u, v);
            *r = (brdf.x * 255.0) as u8;
            *g = (brdf.y * 255.0) as u8;
            *a = 255;
        }
        img_diff::assert_img_eq("skybox/brdf_cpu.png", img);
    }
}
