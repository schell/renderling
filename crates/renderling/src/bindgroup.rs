//! A helper wrapper around `Arc<RwLock<Arc<wgpu::BindGroup>>>` that provides invalidation.

use std::sync::{Arc, RwLock};

/// A [`wgpu::BindGroup`] with invalidation.
///
/// This struct exists to simplify the common pattern of invalidating and
/// re-creating bindgroups.
#[derive(Clone)]
pub struct ManagedBindGroup {
    bindgroup: Arc<RwLock<Arc<wgpu::BindGroup>>>,
}

impl ManagedBindGroup {
    pub fn new(buffer: wgpu::BindGroup) -> Self {
        Self {
            bindgroup: Arc::new(RwLock::new(buffer.into())),
        }
    }

    pub fn get(
        &self,
        should_invalidate: bool,
        fn_recreate: impl FnOnce() -> wgpu::BindGroup,
    ) -> Arc<wgpu::BindGroup> {
        if should_invalidate {
            let mut guard = self.bindgroup.write().unwrap();

            let bg = Arc::new(fn_recreate());
            *guard = bg.clone();
            bg
        } else {
            self.bindgroup.read().unwrap().clone()
        }
    }
}
