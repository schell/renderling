//! CPU side of transform.

use std::sync::{Arc, RwLock};

use craballoc::{runtime::IsRuntime, slab::SlabAllocator, value::Hybrid};
use crabslab::Id;
use glam::{Mat4, Quat, Vec3};

use super::shader::TransformDescriptor;

/// A decomposed 3d transformation.
#[derive(Clone, Debug)]
pub struct Transform {
    pub(crate) descriptor: Hybrid<TransformDescriptor>,
}

impl From<&Transform> for Transform {
    fn from(value: &Transform) -> Self {
        value.clone()
    }
}

impl Transform {
    /// Stage a new transform on the GPU.
    pub(crate) fn new(slab: &SlabAllocator<impl IsRuntime>) -> Self {
        let descriptor = slab.new_value(TransformDescriptor::default());
        Self { descriptor }
    }

    /// Return a pointer to the underlying descriptor data on the GPU.
    pub fn id(&self) -> Id<TransformDescriptor> {
        self.descriptor.id()
    }

    /// Return the a copy of the underlying descriptor.
    pub fn descriptor(&self) -> TransformDescriptor {
        self.descriptor.get()
    }

    /// Set the descriptor.
    pub fn set_descriptor(&self, descriptor: TransformDescriptor) -> &Self {
        self.descriptor.set(descriptor);
        self
    }

    /// Set the descriptor and return the `Transform`.
    pub fn with_descriptor(self, descriptor: TransformDescriptor) -> Self {
        self.set_descriptor(descriptor);
        self
    }

    /// Return the transform in combined matrix format;
    pub fn as_mat4(&self) -> Mat4 {
        self.descriptor().into()
    }

    /// Get the translation of the transform.
    pub fn translation(&self) -> Vec3 {
        self.descriptor.get().translation
    }

    /// Modify the translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the translation vector and returns a value of type `T`.
    pub fn modify_translation<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.descriptor.modify(|t| f(&mut t.translation))
    }

    /// Set the translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `translation`: A 3d translation vector `Vec3`.
    pub fn set_translation(&self, translation: impl Into<Vec3>) -> &Self {
        self.descriptor
            .modify(|t| t.translation = translation.into());
        self
    }

    /// Set the translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `translation`: A 3d translation vector `Vec3`.
    pub fn with_translation(self, translation: impl Into<Vec3>) -> Self {
        self.set_translation(translation);
        self
    }

    /// Get the rotation of the transform.
    pub fn rotation(&self) -> Quat {
        self.descriptor.get().rotation
    }

    /// Modify the rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the rotation quaternion and returns a value of type `T`.
    pub fn modify_rotation<T: 'static>(&self, f: impl FnOnce(&mut Quat) -> T) -> T {
        self.descriptor.modify(|t| f(&mut t.rotation))
    }

    /// Set the rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `rotation`: A quaternion representing the rotation.
    pub fn set_rotation(&self, rotation: impl Into<Quat>) -> &Self {
        self.descriptor.modify(|t| t.rotation = rotation.into());
        self
    }

    /// Set the rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `rotation`: A quaternion representing the rotation.
    pub fn with_rotation(self, rotation: impl Into<Quat>) -> Self {
        self.set_rotation(rotation);
        self
    }

    /// Get the scale of the transform.
    pub fn scale(&self) -> Vec3 {
        self.descriptor.get().scale
    }

    /// Modify the scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the scale vector and returns a value of type `T`.
    pub fn modify_scale<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.descriptor.modify(|t| f(&mut t.scale))
    }

    /// Set the scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `scale`: A 3d scale vector `Vec3`.
    pub fn set_scale(&self, scale: impl Into<Vec3>) -> &Self {
        self.descriptor.modify(|t| t.scale = scale.into());
        self
    }

    /// Set the scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `scale`: A 3d scale vector `Vec3`.
    pub fn with_scale(self, scale: impl Into<Vec3>) -> Self {
        self.set_scale(scale);
        self
    }
}

/// Manages scene heirarchy on the [`Stage`](crate::stage::Stage).
///
/// Can be created with
/// [`Stage::new_nested_transform`](crate::stage::Stage::new_nested_transform).
///
/// Clones all reference the same nested transform.
#[derive(Clone)]
pub struct NestedTransform {
    pub(crate) global_transform: Transform,
    local_transform: Arc<RwLock<TransformDescriptor>>,
    children: Arc<RwLock<Vec<NestedTransform>>>,
    parent: Arc<RwLock<Option<NestedTransform>>>,
}

impl core::fmt::Debug for NestedTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let children = self
            .children
            .read()
            .unwrap()
            .iter()
            .map(|nt| nt.global_transform.id())
            .collect::<Vec<_>>();
        let parent = self
            .parent
            .read()
            .unwrap()
            .as_ref()
            .map(|nt| nt.global_transform.id());
        f.debug_struct("NestedTransform")
            .field("local_transform", &self.local_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl From<&NestedTransform> for Transform {
    fn from(value: &NestedTransform) -> Self {
        value.global_transform.clone()
    }
}

impl From<NestedTransform> for Transform {
    fn from(value: NestedTransform) -> Self {
        value.global_transform
    }
}

impl NestedTransform {
    /// Stage a new hierarchical transform on the GPU.
    pub(crate) fn new(slab: &SlabAllocator<impl IsRuntime>) -> Self {
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(TransformDescriptor::default())),
            global_transform: Transform::new(slab),
            children: Default::default(),
            parent: Default::default(),
        };
        nested.mark_dirty();
        nested
    }

    /// Get the _local_ translation of the transform.
    pub fn local_translation(&self) -> Vec3 {
        self.local_transform.read().unwrap().translation
    }

    /// Modify the _local_ translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the translation vector and returns a value of type `T`.
    pub fn modify_local_translation<T>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        let t = {
            let mut local_transform = self.local_transform.write().unwrap();
            f(&mut local_transform.translation)
        };
        self.mark_dirty();
        t
    }

    /// Set the _local_ translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `translation`: A 3d translation vector `Vec3`.
    pub fn set_local_translation(&self, translation: impl Into<Vec3>) -> &Self {
        self.local_transform.write().unwrap().translation = translation.into();
        self.mark_dirty();
        self
    }

    /// Set the _local_ translation of the transform.
    ///
    /// # Arguments
    ///
    /// - `translation`: A 3d translation vector `Vec3`.
    pub fn with_local_translation(self, translation: impl Into<Vec3>) -> Self {
        self.set_local_translation(translation);
        self
    }

    /// Get the _local_ rotation of the transform.
    pub fn local_rotation(&self) -> Quat {
        self.local_transform.read().unwrap().rotation
    }

    /// Modify the _local_ rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the rotation quaternion and returns a value of type `T`.
    pub fn modify_local_rotation<T>(&self, f: impl FnOnce(&mut Quat) -> T) -> T {
        let t = {
            let mut local_transform = self.local_transform.write().unwrap();
            f(&mut local_transform.rotation)
        };
        self.mark_dirty();
        t
    }

    /// Set the _local_ rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `rotation`: A quaternion representing the rotation.
    pub fn set_local_rotation(&self, rotation: impl Into<Quat>) -> &Self {
        self.local_transform.write().unwrap().rotation = rotation.into();
        self.mark_dirty();
        self
    }

    /// Set the _local_ rotation of the transform.
    ///
    /// # Arguments
    ///
    /// - `rotation`: A quaternion representing the rotation.
    pub fn with_local_rotation(self, rotation: impl Into<Quat>) -> Self {
        self.set_local_rotation(rotation);
        self
    }

    /// Get the _local_ scale of the transform.
    pub fn local_scale(&self) -> Vec3 {
        self.local_transform.read().unwrap().scale
    }

    /// Modify the _local_ scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `f`: A closure that takes a mutable reference to the scale vector and returns a value of type `T`.
    pub fn modify_local_scale<T>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        let t = {
            let mut local_transform = self.local_transform.write().unwrap();
            f(&mut local_transform.scale)
        };
        self.mark_dirty();
        t
    }

    /// Set the _local_ scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `scale`: A 3d scale vector `Vec3`.
    pub fn set_local_scale(&self, scale: impl Into<Vec3>) -> &Self {
        self.local_transform.write().unwrap().scale = scale.into();
        self.mark_dirty();
        self
    }

    /// Set the _local_ scale of the transform.
    ///
    /// # Arguments
    ///
    /// - `scale`: A 3d scale vector `Vec3`.
    pub fn with_local_scale(self, scale: impl Into<Vec3>) -> Self {
        self.set_local_scale(scale);
        self
    }

    /// Return a pointer to the underlying descriptor data on the GPU.
    ///
    /// The descriptor is the descriptor that describes the _global_ transform.
    pub fn global_id(&self) -> Id<TransformDescriptor> {
        self.global_transform.id()
    }

    /// Return the descriptor of the _global_ transform.
    ///
    /// This traverses the heirarchy and computes the result.
    pub fn global_descriptor(&self) -> TransformDescriptor {
        let maybe_parent_guard = self.parent.read().unwrap();
        let transform = self.local_descriptor();
        let parent_transform = maybe_parent_guard
            .as_ref()
            .map(|parent| parent.global_descriptor())
            .unwrap_or_default();
        TransformDescriptor::from(Mat4::from(parent_transform) * Mat4::from(transform))
    }

    /// Return the descriptor of the _local_ tarnsform.
    pub fn local_descriptor(&self) -> TransformDescriptor {
        *self.local_transform.read().unwrap()
    }

    fn mark_dirty(&self) {
        self.global_transform
            .descriptor
            .set(self.global_descriptor());
        for child in self.children.read().unwrap().iter() {
            child.mark_dirty();
        }
    }

    /// Get a vector containing all the hierarchy's transforms.
    ///
    /// Starts with the root transform and ends with the local transform.
    pub fn hierarchy(&self) -> Vec<TransformDescriptor> {
        let mut transforms = vec![];
        if let Some(parent) = self.parent() {
            transforms.extend(parent.hierarchy());
        }
        transforms.push(self.local_descriptor());
        transforms
    }

    pub fn add_child(&self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        node.mark_dirty();
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform.id() == node.global_transform.id() {
                node.mark_dirty();
                let _ = node.parent.write().unwrap().take();
                false
            } else {
                true
            }
        });
    }

    /// Return a clone of the parent `NestedTransform`, if any.
    pub fn parent(&self) -> Option<NestedTransform> {
        self.parent.read().unwrap().clone()
    }
}
