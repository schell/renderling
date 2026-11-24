//! Compositor for alpha-blending a source texture onto a target framebuffer.
//!
//! This is used by the `renderling-ui` crate to overlay MSAA-resolved UI
//! content onto a 3D scene without overwriting existing framebuffer content.

#[wgsl_rs::wgsl]
pub mod shader {
    //! Kernels for compositor for alpha-blending a source texture onto a target
    //! framebuffer.
    use crate::shader::constants::*;
    use wgsl_rs::std::*;

    pub struct CompositorInterStage {
        #[location(0)]
        uv: Vec2f,
        #[builtin(position)]
        #[allow(dead_code)]
        clip_pos: Vec4f,
    }
    /// Fullscreen quad vertex shader for compositing.
    ///
    /// Generates 6 vertices (two triangles) covering the full clip-space quad
    /// and passes through UV coordinates for texture sampling.
    #[vertex]
    pub fn compositor_vertex(#[builtin(vertex_index)] vertex_id: u32) -> CompositorInterStage {
        let i = vertex_id as usize;
        CompositorInterStage {
            uv: UV_COORD_QUAD_CCW[i],
            clip_pos: CLIP_SPACE_COORD_QUAD_CCW[i],
        }
    }

    texture!(group(0), binding(0), TEXTURE: Texture2D<f32>);
    sampler!(group(0), binding(1), SAMPLER: Sampler);

    /// Passthrough fragment shader for compositing.
    ///
    /// Samples the source texture at the given UV and outputs the color.
    /// Alpha blending is handled by the pipeline's blend state, not the shader.
    #[fragment]
    pub fn compositor_fragment(input: CompositorInterStage) -> Vec4f {
        texture_sample(TEXTURE, SAMPLER, input.uv)
    }
}

mod cpu;
pub use cpu::*;
