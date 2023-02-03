//! Pipeline types and utilities.
use std::{any::Any, sync::Arc};

/// Defines the operations a pipeline can do within a `Renderling`.
pub trait Pipeline: Any + Send + Sync + 'static {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline;
    //fn begin_render_pass<'a, 'b: 'a>(
    //    &'b self,
    //    encoder: &'a mut wgpu::CommandEncoder,
    //    frame_texture_view: &'a wgpu::TextureView,
    //    depth_texture_view: &'a wgpu::TextureView,
    //) -> wgpu::RenderPass<'a>;

    //fn render_object<'a, 'b: 'a>(
    //    &'b self,
    //    render_pass: &'a mut wgpu::RenderPass<'b>,
    //    object: renderling_core::ShaderObject<'b>,
    //    default_material_bindgroup: &'b wgpu::BindGroup,
    //);
}

/// A type-erased shader pipeline.
#[derive(Clone)]
pub struct AnyPipeline {
    inner: Arc<dyn Pipeline>,
}

impl std::fmt::Debug for AnyPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyPipeline").field("inner", &"_").finish()
    }
}

impl AnyPipeline {
    pub fn new<T: Pipeline>(inner: impl Into<Arc<T>>) -> Self {
        Self {
            inner: inner.into(),
        }
    }

    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        self.inner.get_render_pipeline()
    }
    ///// Begin a new render pass.
    //pub fn begin_render_pass<'a, 'b: 'a>(
    //    &'b self,
    //    encoder: &'a mut wgpu::CommandEncoder,
    //    frame_texture_view: &'a wgpu::TextureView,
    //    depth_texture_view: &'a wgpu::TextureView,
    //) -> wgpu::RenderPass<'a> {

    //    self.inner.begin_render_pass(encoder, frame_texture_view, depth_texture_view)
    //}

    ///// Render a single shader object.
    //pub fn render_object<'a, 'b: 'a>(
    //    &'b self,
    //    render_pass: &'a mut wgpu::RenderPass<'b>,
    //    object: renderling_core::ShaderObject<'b>,
    //    default_material_bindgroup: &'b wgpu::BindGroup,
    //) {
    //    self.inner.render_object(render_pass, object, default_material_bindgroup)
    //}
}
