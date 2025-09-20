//! Mesh primitives.

use core::ops::Deref;
use std::sync::{Arc, Mutex};

use craballoc::value::Hybrid;
use crabslab::{Array, Id};

use crate::{
    bvol::BoundingSphere,
    geometry::{Indices, MorphTargetWeights, MorphTargets, Skin, Vertices},
    material::Material,
    primitive::shader::PrimitiveDescriptor,
    stage::Stage,
    transform::Transform,
    types::GpuOnlyArray,
};

/// A unit of rendering.
///
/// A `Primitive` represents one draw call, or one mesh primitive.
pub struct Primitive {
    pub(crate) descriptor: Hybrid<PrimitiveDescriptor>,

    vertices: Arc<Mutex<Option<Vertices<GpuOnlyArray>>>>,
    indices: Arc<Mutex<Option<Indices<GpuOnlyArray>>>>,

    pub(crate) transform: Arc<Mutex<Option<Transform>>>,
    pub(crate) material: Arc<Mutex<Option<Material>>>,
    skin: Arc<Mutex<Option<Skin>>>,
    morph_targets: Arc<Mutex<Option<(MorphTargets, MorphTargetWeights)>>>,
}

impl Primitive {
    /// Create a new [`Primitive`], automatically adding it to the
    /// [`Stage`](crate::stage::Stage) to be drawn.
    ///
    /// The returned [`Primitive`] will have the stage's default [`Vertices`],
    /// which is an all-white unit cube.
    pub fn new(stage: &Stage) -> Self {
        let descriptor = stage
            .geometry
            .slab_allocator()
            .new_value(PrimitiveDescriptor::default());
        let primitive = Primitive {
            descriptor,
            vertices: Default::default(),
            indices: Default::default(),
            transform: Default::default(),
            material: Default::default(),
            skin: Default::default(),
            morph_targets: Default::default(),
        }
        .with_vertices(stage.default_vertices());
        stage.add_primitive(&primitive);
        primitive
    }
}

impl Clone for Primitive {
    fn clone(&self) -> Self {
        Self {
            descriptor: self.descriptor.clone(),
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            transform: self.transform.clone(),
            material: self.material.clone(),
            skin: self.skin.clone(),
            morph_targets: self.morph_targets.clone(),
        }
    }
}

// Vertices impls
impl Primitive {
    /// Set the vertex data of this primitive.
    pub fn set_vertices(&self, vertices: impl Into<Vertices<GpuOnlyArray>>) -> &Self {
        let vertices = vertices.into();
        let array = vertices.array();
        self.descriptor.modify(|d| d.vertices_array = array);
        *self.vertices.lock().unwrap() = Some(vertices.clone());
        self
    }

    /// Set the vertex data of this primitive and return the primitive.
    pub fn with_vertices(self, vertices: impl Into<Vertices<GpuOnlyArray>>) -> Self {
        self.set_vertices(vertices);
        self
    }
}

// Indices impls
impl Primitive {
    /// Set the index data of this primitive.
    pub fn set_indices(&self, indices: impl Into<Indices<GpuOnlyArray>>) -> &Self {
        let indices = indices.into();
        let array = indices.array();
        self.descriptor.modify(|d| d.indices_array = array);
        *self.indices.lock().unwrap() = Some(indices.clone());
        self
    }

    /// Set the index data of this primitive and return the primitive.
    pub fn with_indices(self, indices: impl Into<Indices<GpuOnlyArray>>) -> Self {
        self.set_indices(indices);
        self
    }

    /// Remove the indices from this primitive.
    pub fn remove_indices(&self) -> &Self {
        *self.indices.lock().unwrap() = None;
        self.descriptor.modify(|d| d.indices_array = Array::NONE);
        self
    }
}

// PrimitiveDescriptor impls
impl Primitive {
    /// Return a pointer to the underlying descriptor on the GPU.
    pub fn id(&self) -> Id<PrimitiveDescriptor> {
        self.descriptor.id()
    }

    /// Return the underlying descriptor.
    pub fn descriptor(&self) -> PrimitiveDescriptor {
        self.descriptor.get()
    }

    /// Set the bounds of this primitive.
    pub fn set_bounds(&self, bounds: BoundingSphere) -> &Self {
        self.descriptor.modify(|d| d.bounds = bounds);
        self
    }

    /// Set the bounds and return the primitive.
    pub fn with_bounds(self, bounds: BoundingSphere) -> Self {
        self.set_bounds(bounds);
        self
    }

    /// Get the bounds.
    ///
    /// Returns the current [`BoundingSphere`].
    pub fn bounds(&self) -> BoundingSphere {
        self.descriptor.get().bounds
    }

    /// Modify the bounds of the primitive.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to the
    ///   [`BoundingSphere`] and returns a value of type `T`.
    pub fn modify_bounds<T: 'static>(&self, f: impl FnOnce(&mut BoundingSphere) -> T) -> T {
        self.descriptor.modify(|d| f(&mut d.bounds))
    }

    /// Set the visibility of this primitive.
    pub fn set_visible(&self, visible: bool) -> &Self {
        self.descriptor.modify(|d| d.visible = visible);
        self
    }

    /// Set the visibility and return the primitive.
    pub fn with_visible(self, visible: bool) -> Self {
        self.set_visible(visible);
        self
    }

    /// Return the primitive's visibility.
    pub fn visible(&self) -> bool {
        self.descriptor.get().visible
    }

    /// Modify the visible of the primitive.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to the visibility and
    ///   returns a value of type `T`.
    pub fn modify_visible<T: 'static>(&self, f: impl FnOnce(&mut bool) -> T) -> T {
        self.descriptor.modify(|d| f(&mut d.visible))
    }
}

// Transform functions
impl Primitive {
    /// Set the transform.
    ///
    /// # Note
    /// This can be set with [`Transform`] or
    /// [`NestedTransform`](crate::transform::NestedTransform).
    pub fn set_transform(&self, transform: impl Into<Transform>) -> &Self {
        let transform = transform.into();
        self.descriptor.modify(|d| d.transform_id = transform.id());
        *self.transform.lock().unwrap() = Some(transform.clone());
        self
    }

    /// Set the transform and return the `Primitive`.
    ///
    /// # Note
    /// This can be set with [`Transform`] or
    /// [`NestedTransform`](crate::transform::NestedTransform).
    pub fn with_transform(self, transform: impl Into<Transform>) -> Self {
        self.set_transform(transform);
        self
    }

    /// Get the transform.
    ///
    /// Returns a reference to the current `Transform`, if any.
    pub fn transform(&self) -> impl Deref<Target = Option<Transform>> + '_ {
        self.transform.lock().unwrap()
    }

    /// Remove the transform from this primitive.
    ///
    /// This effectively makes the transform the identity.
    pub fn remove_transform(&self) -> &Self {
        self.descriptor.modify(|d| d.transform_id = Id::NONE);
        *self.transform.lock().unwrap() = None;
        self
    }
}

// Material impls
impl Primitive {
    /// Set the material of this primitive.
    pub fn set_material(&self, material: impl Into<Material>) -> &Self {
        let material = material.into();
        self.descriptor.modify(|d| d.material_id = material.id());
        *self.material.lock().unwrap() = Some(material);
        self
    }

    /// Set the material and return the primitive.
    pub fn with_material(self, material: impl Into<Material>) -> Self {
        self.set_material(material);
        self
    }

    /// Get the material.
    ///
    /// Returns a reference to the current `Material`, if any.
    pub fn material(&self) -> impl Deref<Target = Option<Material>> + '_ {
        self.material.lock().unwrap()
    }

    /// Remove the material from this primitive.
    pub fn remove_material(&self) -> &Self {
        self.descriptor.modify(|d| d.material_id = Id::NONE);
        *self.material.lock().unwrap() = None;
        self
    }
}

// Skin impls
impl Primitive {
    /// Set the skin of this primitive.
    pub fn set_skin(&self, skin: impl Into<Skin>) -> &Self {
        let skin = skin.into();
        self.descriptor.modify(|d| d.skin_id = skin.id());
        *self.skin.lock().unwrap() = Some(skin.clone());
        self
    }

    /// Set the skin and return the primitive.
    pub fn with_skin(self, skin: impl Into<Skin>) -> Self {
        self.set_skin(skin);
        self
    }

    /// Get the skin.
    ///
    /// Returns a reference to the current `Skin`, if any.
    pub fn skin(&self) -> impl Deref<Target = Option<Skin>> + '_ {
        self.skin.lock().unwrap()
    }

    /// Remove the skin from this primitive.
    pub fn remove_skin(&self) -> &Self {
        self.descriptor.modify(|d| d.skin_id = Id::NONE);
        *self.skin.lock().unwrap() = None;
        self
    }
}

// (MorphTargets, MorphTargetsWeights) impls
impl Primitive {
    /// Set the morph targets and weights of this primitive.
    pub fn set_morph_targets(
        &self,
        morph_targets: impl Into<MorphTargets>,
        weights: impl Into<MorphTargetWeights>,
    ) -> &Self {
        let morph_targets = morph_targets.into();
        let weights = weights.into();
        self.descriptor.modify(|d| {
            d.morph_targets = morph_targets.array();
            d.morph_weights = weights.array();
        });
        *self.morph_targets.lock().unwrap() = Some((morph_targets.clone(), weights.clone()));
        self
    }

    /// Set the morph targets and weights and return the primitive.
    pub fn with_morph_targets(
        self,
        morph_targets: impl Into<MorphTargets>,
        weights: impl Into<MorphTargetWeights>,
    ) -> Self {
        self.set_morph_targets(morph_targets, weights);
        self
    }

    /// Get the morph targets and weights.
    ///
    /// Returns a reference to the current `MorphTargets` and `MorphTargetsWeights`, if any.
    pub fn morph_targets(
        &self,
    ) -> impl Deref<Target = Option<(MorphTargets, MorphTargetWeights)>> + '_ {
        self.morph_targets.lock().unwrap()
    }

    /// Remove the morph targets and weights from this primitive.
    pub fn remove_morph_targets(&self) -> &Self {
        self.descriptor.modify(|d| {
            d.morph_targets = Array::NONE;
            d.morph_weights = Array::NONE;
        });
        *self.morph_targets.lock().unwrap() = None;
        self
    }
}
