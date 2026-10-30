//! Rendering skylines at infinite distances.
#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

pub mod shader;

#[wgsl_rs::wgsl]
pub mod common {
    //! Common types and functions used amongst the different skybox shaders.

    use wgsl_rs::std::*;

    use crate::shader::camera::*;
    use crate::shader::constants::*;
    use crate::shader::math::*;

    /// Takes a unit direction and converts it to a uv lookup in an equirectangular
    /// map.
    // TODO: determine if we need this function or if we can inline.
    pub fn direction_to_equirectangular_uv(dir: Vec3f) -> Vec2f {
        let mut uv = vec2f(atan2(dir.z, dir.x), asin(dir.y));
        uv *= INV_ATAN;
        uv += 0.5;
        uv
    }

    pub struct SkyboxVertexInput {
        #[builtin(instance_index)]
        pub camera_index: u32,
        #[builtin(vertex_index)]
        pub vertex_index: u32,
    }

    pub struct SkyboxInterStage {
        #[location(0)]
        pub local_pos: Vec3f,
        #[builtin(position)]
        pub clip_pos: Vec4f,
    }

    impl SkyboxInterStage {
        /// Used to create a cubemap from an equirectangular image as well as cubemap
        /// convolutions.
        pub fn from_unit_cube_vertex_index(vertex_index: u32, camera: CameraDescriptor) -> Self {
            let pos = cube()[vertex_index as usize];
            SkyboxInterStage {
                local_pos: pos,
                clip_pos: CameraDescriptor::view_projection(camera)
                    * vec4f(pos.x, pos.y, pos.z, 1.0),
            }
        }
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod render {
    //! Skybox shader that renders a skybox behind all objects on the stage.

    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    use crate::shader::camera::*;
    use crate::shader::math::*;

    use super::common::*;

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);
    texture!(group(0), binding(1), TEXTURE: TextureCube<f32>);
    sampler!(group(0), binding(2), SAMPLER: Sampler);

    /// Vertex shader for a skybox.
    #[vertex]
    pub fn vertex(input: SkyboxVertexInput) -> SkyboxInterStage {
        let camera: CameraDescriptor;
        slab_read!(CameraDescriptor, get!(SLAB), input.camera_index, camera);

        let p = cube()[input.vertex_index as usize];
        let point = vec4f(p.x, p.y, p.z, 1.0);
        let view = CameraDescriptor::view(camera);
        let rot_view = mat4x4f(
            vec4f(view[0u32].x, view[0u32].y, view[0u32].z, 0.0),
            vec4f(view[1u32].x, view[1u32].y, view[1u32].z, 0.0),
            vec4f(view[2u32].x, view[2u32].y, view[2u32].z, 0.0),
            vec4f(0.0, 0.0, 0.0, 1.0),
        );
        let position = CameraDescriptor::projection(camera) * rot_view * point;

        SkyboxInterStage {
            local_pos: point.xyz(),
            clip_pos: position.xyww(),
        }
    }

    /// Colors a skybox using a cubemap texture.
    #[fragment]
    pub fn fragment(input: SkyboxInterStage) -> Vec4f {
        let env_color: Vec3f =
            texture_sample(TEXTURE, SAMPLER, vec3_norm_or_zero(input.local_pos)).xyz();
        vec4f(env_color.x, env_color.y, env_color.z, 1.0)
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod equirect_to_cubemap {
    //! TODO: better module docs

    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    use crate::shader::camera::*;
    use crate::shader::math::*;

    use super::common::*;

    storage!(group(0), binding(0), SLAB: RuntimeArray<u32>);
    texture!(group(0), binding(1), TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(2), SAMPLER: Sampler);

    /// Vertex shader that draws a cubemap.
    ///
    /// Uses the `instance_index` as the [`Id`] for a [`CameraDescriptor`].
    ///
    /// Used to create a cubemap from an equirectangular image as well as cubemap
    /// convolutions.
    #[vertex]
    pub fn vertex(input: SkyboxVertexInput) -> SkyboxInterStage {
        let camera: CameraDescriptor;
        slab_read!(CameraDescriptor, get!(SLAB), input.camera_index, camera);
        SkyboxInterStage::from_unit_cube_vertex_index(input.vertex_index, camera)
    }

    /// Fragment shader that colors a skybox using an equirectangular texture.
    #[fragment]
    pub fn fragment(input: SkyboxInterStage) -> Vec4f {
        let uv = direction_to_equirectangular_uv(vec3_norm_or_zero(input.local_pos));
        let env_color = texture_sample(TEXTURE, SAMPLER, uv).rgb();
        vec4f(env_color.x, env_color.y, env_color.z, 1.0)
    }
}
