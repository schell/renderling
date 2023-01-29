//! Material types and utilities.
use std::{any::Any, sync::Arc};

/// Type-erased object material.
#[derive(Clone, Debug)]
pub struct AnyMaterial {
    inner: Arc<dyn Any + Send + Sync + 'static>,
}

impl AnyMaterial {
    pub fn new<T: Any + Send + Sync + 'static>(inner: impl Into<Arc<T>>) -> Self {
        Self { inner: inner.into() }
    }

    pub fn downcast<T: Any + Send + Sync + 'static>(self) -> Result<Arc<T>, AnyMaterial> {
        match self.inner.downcast() {
            Ok(t) => Ok(t),
            Err(inner) => Err(AnyMaterial { inner }),
        }
    }
}
