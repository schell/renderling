//! Material types and utilities.
use std::{
    any::Any,
    hash::{Hash, Hasher},
    sync::Arc,
};

use rustc_hash::FxHasher;

/// Defines things any material uniform is capable of.
pub trait MaterialUniform: Any + Send + Sync + 'static {
    fn get_bindgroup(&self) -> &wgpu::BindGroup;
}

/// Type-erased material uniform.
///
/// These are shader resources for a material object.
pub struct AnyMaterialUniform {
    inner: Arc<dyn MaterialUniform>,
}

impl AnyMaterialUniform {
    pub fn new<T: MaterialUniform>(material: impl Into<Arc<T>>) -> AnyMaterialUniform {
        AnyMaterialUniform {
            inner: material.into(),
        }
    }

    pub fn get_bindgroup(&self) -> &wgpu::BindGroup {
        self.inner.get_bindgroup()
    }
}

/// Defines operations for creating material uniforms from an initial value.
pub trait Material: Any + Send + Sync + 'static {
    fn create_material_uniform(&self, device: &wgpu::Device) -> AnyMaterialUniform;
}

/// Type-erased object material.
#[derive(Clone)]
pub struct AnyMaterial {
    inner: Arc<dyn Material>,
}

impl PartialOrd for AnyMaterial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let here = self.hash_id();
        let there = other.hash_id();
        here.partial_cmp(&there)
    }
}

impl Ord for AnyMaterial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let here = self.hash_id();
        let there = other.hash_id();
        here.cmp(&there)
    }
}

/// Compares materials based on pointers, not values.
impl PartialEq for AnyMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.is_same(other)
    }
}

impl Eq for AnyMaterial {}

impl Hash for AnyMaterial {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.inner), state)
    }
}

impl std::fmt::Debug for AnyMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyMaterial").field("inner", &"_").finish()
    }
}

impl AnyMaterial {
    pub fn new<T: Material>(inner: impl Into<Arc<T>>) -> Self {
        Self {
            inner: inner.into(),
        }
    }

    pub fn create_material_uniform(&self, device: &wgpu::Device) -> AnyMaterialUniform {
        self.inner.create_material_uniform(device)
    }

    pub fn is_same(&self, other: &AnyMaterial) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    pub fn hash_id(&self) -> u64 {
        let mut hasher = FxHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
