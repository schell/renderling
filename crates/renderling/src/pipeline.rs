//! Pipeline types and utilities.
use std::{any::Any, sync::Arc};

/// Defines the operations a pipeline can do within a `Renderling`.
pub trait Pipeline: Any + Send + Sync + 'static {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline;
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
}
