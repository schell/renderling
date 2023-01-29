//! Material types and utilities.
use std::{any::Any, sync::Arc};

pub trait Material: Any + Send + Sync + 'static {
    fn create_bindgroup(&self, device: &wgpu::Device) -> wgpu::BindGroup;
}

/// Type-erased object material.
#[derive(Clone)]
pub struct AnyMaterial {
    inner: Arc<dyn Material>,
}

impl std::fmt::Debug for AnyMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyMaterial").field("inner", &"_").finish()
    }
}

impl AnyMaterial {
    pub fn new<T: Material>(inner: impl Into<Arc<T>>) -> Self {
        Self { inner: inner.into() }
    }

    pub fn create_bindgroup(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        self.inner.create_bindgroup(device)
    }
}
