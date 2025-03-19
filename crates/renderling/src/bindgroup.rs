//! A helper wrapper around `Arc<RwLock<Arc<wgpu::BindGroup>>>` that provides invalidation.

use std::sync::{Arc, RwLock};

/// A [`wgpu::BindGroup`] with invalidation.
///
/// This struct exists to simplify the common pattern of invalidating and
/// re-creating bindgroups.
#[derive(Clone)]
pub struct ManagedBindGroup {
    bindgroup: Arc<RwLock<Option<Arc<wgpu::BindGroup>>>>,
}

impl Default for ManagedBindGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl From<wgpu::BindGroup> for ManagedBindGroup {
    fn from(value: wgpu::BindGroup) -> Self {
        let mbg = ManagedBindGroup::new();
        // UNWRAP: POP
        *mbg.bindgroup.write().unwrap() = Some(value.into());
        mbg
    }
}

impl ManagedBindGroup {
    pub fn new() -> Self {
        Self {
            bindgroup: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get(
        &self,
        should_invalidate: bool,
        fn_recreate: impl FnOnce() -> wgpu::BindGroup,
    ) -> Arc<wgpu::BindGroup> {
        let recreate = || {
            let mut guard = self.bindgroup.write().unwrap();

            let bg = Arc::new(fn_recreate());
            *guard = Some(bg.clone());
            bg
        };
        if should_invalidate {
            recreate()
        } else {
            let maybe_buffer = self.bindgroup.read().unwrap().clone();
            if let Some(buffer) = maybe_buffer {
                buffer
            } else {
                recreate()
            }
        }
    }

    /// Invalidate the [`wgpu::BindGroup`], destroying it if it exists.
    pub fn invalidate(&self) {
        *self.bindgroup.write().unwrap() = None;
    }
}
