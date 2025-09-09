//! CPU side of the [super::geometry](geometry) module.
use std::sync::{Arc, Mutex};

use craballoc::{
    runtime::{IsRuntime, WgpuRuntime},
    slab::{SlabAllocator, SlabBuffer},
    value::{GpuArray, Hybrid, HybridArray, IsContainer},
};
use crabslab::{Array, Id};
use glam::{Mat4, UVec2, Vec4};

use crate::{
    camera::Camera,
    geometry::{GeometryDescriptor, MorphTarget, SkinDescriptor, Vertex},
    transform::{NestedTransform, Transform, TransformDescriptor},
    types::{GpuCpuArray, GpuOnlyArray},
};

/// A contiguous array of vertices, staged on the GPU.
///
/// The type variable `Ct` denotes whether the dta lives on the GPU only, or on
/// the CPU and the GPU.
pub struct Vertices<Ct: IsContainer = GpuCpuArray> {
    inner: Ct::Container<Vertex>,
}

impl<Ct> Clone for Vertices<Ct>
where
    Ct: IsContainer,
    Ct::Container<Vertex>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Ct> std::fmt::Debug for Vertices<Ct>
where
    Ct: IsContainer<Pointer<Vertex> = Array<Vertex>>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vertices")
            .field("array", &Ct::get_pointer(&self.inner))
            .finish()
    }
}

impl From<Vertices> for Vertices<GpuOnlyArray> {
    fn from(value: Vertices) -> Self {
        value.into_gpu_only()
    }
}

impl From<&Vertices> for Vertices<GpuOnlyArray> {
    fn from(value: &Vertices) -> Self {
        value.clone().into_gpu_only()
    }
}

impl From<&Vertices<GpuOnlyArray>> for Vertices<GpuOnlyArray> {
    fn from(value: &Vertices<GpuOnlyArray>) -> Self {
        value.clone()
    }
}

impl Vertices {
    /// Stage a new array of vertex data on the GPU.
    ///
    /// The resulting `Vertices` will live on the GPU and CPU, allowing for modification.
    /// If you would like to unload the CPU side, use [`Vertices::into_gpu_only`].
    pub(crate) fn new(
        slab: &SlabAllocator<WgpuRuntime>,
        vertices: impl IntoIterator<Item = Vertex>,
    ) -> Self {
        Vertices {
            inner: slab.new_array(vertices),
        }
    }

    /// Unload the CPU side of vertex data.
    ///
    /// After this operation the data can still be updated using [`Vertices::set_item`],
    /// but it cannot be modified in-place.
    pub fn into_gpu_only(self) -> Vertices<GpuOnlyArray> {
        Vertices {
            inner: self.inner.into_gpu_only(),
        }
    }

    /// Returns a [`Vertex`] at a specific index, if any.
    pub fn get(&self, index: usize) -> Option<Vertex> {
        self.inner.get(index)
    }

    /// Return all vertices as a vector.
    pub fn get_vec(&self) -> Vec<Vertex> {
        self.inner.get_vec()
    }

    /// Set the [`Vertex`] at the given index to the given value, if the item at
    /// the index exists.
    ///
    /// Returns the previous value, if any.
    pub fn set_item(&self, index: usize, value: Vertex) -> Option<Vertex> {
        self.inner.set_item(index, value)
    }
}

impl<T> Vertices<T>
where
    T: IsContainer<Pointer<Vertex> = Array<Vertex>>,
{
    /// Returns a pointer to the underlying data on the GPU.
    pub fn array(&self) -> Array<Vertex> {
        T::get_pointer(&self.inner)
    }
}

/// A contiguous array of indices, staged on the GPU.
/// The type variable `Ct` denotes whether the data lives on the GPU only, or on
/// the CPU and the GPU.
pub struct Indices<Ct: IsContainer = GpuCpuArray> {
    inner: Ct::Container<u32>,
}

impl<Ct> std::fmt::Debug for Indices<Ct>
where
    Ct: IsContainer<Pointer<u32> = Array<u32>>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Indices")
            .field("array", &Ct::get_pointer(&self.inner))
            .finish()
    }
}

impl<Ct> Clone for Indices<Ct>
where
    Ct: IsContainer,
    Ct::Container<u32>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl From<Indices> for Indices<GpuOnlyArray> {
    fn from(value: Indices) -> Self {
        value.into_gpu_only()
    }
}

impl From<&Indices> for Indices<GpuOnlyArray> {
    fn from(value: &Indices) -> Self {
        value.clone().into_gpu_only()
    }
}

impl From<&Indices<GpuOnlyArray>> for Indices<GpuOnlyArray> {
    fn from(value: &Indices<GpuOnlyArray>) -> Self {
        value.clone()
    }
}

impl<T> Indices<T>
where
    T: IsContainer<Pointer<u32> = Array<u32>>,
{
    /// Returns a pointer to the underlying data on the GPU.
    pub fn array(&self) -> Array<u32> {
        T::get_pointer(&self.inner)
    }
}

impl Indices {
    /// Stage a new array of index data on the GPU.
    ///
    /// The resulting `Indices` will live on the GPU and CPU, allowing for modification.
    /// If you would like to unload the CPU side, use [`Indices::into_gpu_only`].
    pub fn new(geometry: &Geometry, indices: impl IntoIterator<Item = u32>) -> Self {
        Indices {
            inner: geometry.slab.new_array(indices),
        }
    }

    /// Unload the CPU side of this index data.
    pub fn into_gpu_only(self) -> Indices<GpuOnlyArray> {
        Indices {
            inner: self.inner.into_gpu_only(),
        }
    }
}

/// Holds morph targets for animated nodes.
#[derive(Clone)]
pub struct MorphTargets {
    // Held onto so the values don't drop from under us
    _targets: Arc<Vec<GpuArray<MorphTarget>>>,
    arrays: HybridArray<Array<MorphTarget>>,
}

impl From<&MorphTargets> for MorphTargets {
    fn from(value: &MorphTargets) -> Self {
        value.clone()
    }
}

impl std::fmt::Debug for MorphTargets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MorphTargets")
            .field("arrays", &self.arrays)
            .field("targets", &"...")
            .finish()
    }
}

impl MorphTargets {
    pub(crate) fn new(
        slab: &SlabAllocator<impl IsRuntime>,
        morph_targets: impl IntoIterator<Item = impl IntoIterator<Item = MorphTarget>>,
    ) -> Self {
        let targets = morph_targets
            .into_iter()
            .map(|verts| slab.new_array(verts).into_gpu_only())
            .collect::<Vec<_>>();
        let arrays = slab.new_array(targets.iter().map(|ts| ts.array()));
        Self {
            _targets: targets.into(),
            arrays,
        }
    }
    /// Returns a pointer to the underlying morph targets data on the GPU.                    
    pub fn array(&self) -> Array<Array<MorphTarget>> {
        self.arrays.array()
    }
}

/// Holds morph targets weights for animated nodes.
#[derive(Clone, Debug)]
pub struct MorphTargetWeights {
    inner: HybridArray<f32>,
}

impl From<&MorphTargetWeights> for MorphTargetWeights {
    fn from(value: &MorphTargetWeights) -> Self {
        value.clone()
    }
}

impl MorphTargetWeights {
    pub(crate) fn new(
        slab: &SlabAllocator<impl IsRuntime>,
        data: impl IntoIterator<Item = f32>,
    ) -> Self {
        Self {
            inner: slab.new_array(data),
        }
    }

    /// Returns a pointer to the underlying morph targets weights data on the GPU.            
    pub fn array(&self) -> Array<f32> {
        self.inner.array()
    }

    /// Return the weight at the given index, if any.
    pub fn get_item(&self, index: usize) -> Option<f32> {
        self.inner.get(index)
    }

    /// Update the weight at the given index.
    pub fn set_item(&self, index: usize, weight: f32) {
        self.inner.set_item(index, weight);
    }
}

/// Wrapper around the geometry slab, which holds mesh data and more.
#[derive(Clone)]
pub struct Geometry {
    slab: SlabAllocator<WgpuRuntime>,
    descriptor: Hybrid<GeometryDescriptor>,
    /// A plain white cube to use as default geometry.
    default_vertices: Vertices,
    /// Holds the current camera just in case the user drops it,
    /// this way we never lose a camera that is in use. Dropping
    /// the camera would cause a blank screen, which is very confusing
    /// =(
    camera: Arc<Mutex<Option<Camera>>>,
}

impl AsRef<WgpuRuntime> for Geometry {
    fn as_ref(&self) -> &WgpuRuntime {
        self.slab.runtime()
    }
}

impl AsRef<SlabAllocator<WgpuRuntime>> for Geometry {
    fn as_ref(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.slab
    }
}

impl Geometry {
    pub fn new(runtime: impl AsRef<WgpuRuntime>, resolution: UVec2, atlas_size: UVec2) -> Self {
        let runtime = runtime.as_ref();
        let slab = SlabAllocator::new(runtime, "geometry", wgpu::BufferUsages::empty());
        let descriptor = slab.new_value(GeometryDescriptor {
            atlas_size,
            resolution,
            ..Default::default()
        });
        let default_vertices = Vertices::new(
            &slab,
            crate::math::unit_cube().into_iter().map(|(p, n)| {
                Vertex::default()
                    .with_position(p)
                    .with_normal(n)
                    .with_color(Vec4::ONE)
            }),
        );
        Self {
            slab,
            descriptor,
            default_vertices,
            camera: Default::default(),
        }
    }

    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    pub fn slab_allocator(&self) -> &SlabAllocator<WgpuRuntime> {
        self.as_ref()
    }

    pub fn descriptor(&self) -> &Hybrid<GeometryDescriptor> {
        &self.descriptor
    }

    /// Returns the vertices of a white unit cube.
    pub fn default_vertices(&self) -> &Vertices {
        &self.default_vertices
    }

    #[must_use]
    pub fn commit(&self) -> SlabBuffer<wgpu::Buffer> {
        self.slab.commit()
    }

    /// Create a new camera.
    ///
    /// If this is the first camera created, it will be automatically used.
    pub fn new_camera(&self) -> Camera {
        let c = Camera::new(&self.slab);
        if self.descriptor.get().camera_id.is_none() {
            self.use_camera(&c);
        }
        c
    }

    /// Set all geometry to use the given camera.
    pub fn use_camera(&self, camera: &Camera) {
        let id = camera.id();
        log::info!("using camera: {id:?}");
        // Save a clone so we never lose the active camera, even if the user drops it
        self.descriptor.modify(|cfg| cfg.camera_id = id);
        *self.camera.lock().unwrap() = Some(camera.clone());
    }

    /// Stage a new transform.
    pub fn new_transform(&self) -> Transform {
        Transform::new(&self.slab)
    }

    /// Stage vertex geometry data on the GPU.
    pub fn new_vertices(&self, vertices: impl IntoIterator<Item = Vertex>) -> Vertices {
        Vertices::new(self.slab_allocator(), vertices)
    }

    /// Stage indices that point to offsets of an array of vertices.
    pub fn new_indices(&self, indices: impl IntoIterator<Item = u32>) -> Indices {
        Indices::new(self, indices)
    }

    /// Stage new morph targets on the GPU.
    pub fn new_morph_targets(
        &self,
        data: impl IntoIterator<Item = impl IntoIterator<Item = MorphTarget>>,
    ) -> MorphTargets {
        MorphTargets::new(&self.slab, data)
    }

    /// Create new morph target weights.
    pub fn new_morph_target_weights(
        &self,
        data: impl IntoIterator<Item = f32>,
    ) -> MorphTargetWeights {
        MorphTargetWeights::new(&self.slab, data)
    }

    /// Create a new array of matrices.
    pub fn new_matrices(&self, data: impl IntoIterator<Item = Mat4>) -> HybridArray<Mat4> {
        self.slab.new_array(data)
    }

    pub fn new_skin(
        &self,
        joints: impl IntoIterator<Item = impl Into<SkinJoint>>,
        inverse_bind_matrices: impl IntoIterator<Item = impl Into<Mat4>>,
    ) -> Skin {
        Skin::new(self.slab_allocator(), joints, inverse_bind_matrices)
    }
}

/// A vertex skin.
///
/// For more info on vertex skinning, see
/// <https://github.khronos.org/glTF-Tutorials/gltfTutorial/gltfTutorial_019_SimpleSkin.html>
#[derive(Clone)]
pub struct Skin {
    descriptor: Hybrid<SkinDescriptor>,
    joints: HybridArray<Id<TransformDescriptor>>,
    // Held onto so the transforms don't drop from under us
    _skin_joints: Arc<Mutex<Vec<SkinJoint>>>,
    // Contains the 4x4 inverse-bind matrices.
    //
    // When None, each matrix is assumed to be the 4x4 identity matrix which implies that the
    // inverse-bind matrices were pre-applied.
    _inverse_bind_matrices: Arc<Mutex<Option<GpuArray<Mat4>>>>,
}

impl From<&Skin> for Skin {
    fn from(value: &Skin) -> Self {
        value.clone()
    }
}

impl core::fmt::Debug for Skin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Skin")
            .field("descriptor", &self.descriptor)
            .field("joints", &self.joints)
            .field("joint_transforms", &"...")
            .field("inverse_bind_matrices", &"...")
            .finish()
    }
}

impl Skin {
    /// Stage a new skin on the GPU.
    pub fn new(
        slab: &SlabAllocator<impl IsRuntime>,
        joints: impl IntoIterator<Item = impl Into<SkinJoint>>,
        inverse_bind_matrices: impl IntoIterator<Item = impl Into<Mat4>>,
    ) -> Self {
        let descriptor = slab.new_value(SkinDescriptor::default());
        let skin_joints = joints.into_iter().map(|t| t.into()).collect::<Vec<_>>();
        let joints = skin_joints.iter().map(|sj| sj.0.id()).collect::<Vec<_>>();
        let inverse_bind_matrices = inverse_bind_matrices
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        let inverse_bind_matrices = if inverse_bind_matrices.is_empty() {
            None
        } else {
            Some(slab.new_array(inverse_bind_matrices).into_gpu_only())
        };

        Skin {
            descriptor,
            joints: slab.new_array(joints),
            // We hold on to the transforms so they don't get dropped if the user drops them.
            _skin_joints: Arc::new(Mutex::new(skin_joints)),
            _inverse_bind_matrices: Arc::new(Mutex::new(inverse_bind_matrices)),
        }
    }

    /// Return a pointer to the underlying descriptor data on the GPU.
    pub fn id(&self) -> Id<SkinDescriptor> {
        self.descriptor.id()
    }

    /// Return a copy of the underlying descriptor.
    pub fn descriptor(&self) -> SkinDescriptor {
        self.descriptor.get()
    }
}

/// A joint in a skinned rigging.
///
/// This is a thin wrapper over [`Transform`] and
/// [`NestedTransform`]. You can create a [`SkinJoint`]
/// from either of those types using the [`From`] trait.
///
/// You can also pass an iterator of either [`Transform`] or [`NestedTransform`]
/// to [`Stage::new_skin`](crate::stage::Stage::new_skin).
pub struct SkinJoint(pub(crate) Transform);

impl From<Transform> for SkinJoint {
    fn from(transform: Transform) -> Self {
        SkinJoint(transform)
    }
}

impl From<&Transform> for SkinJoint {
    fn from(transform: &Transform) -> Self {
        transform.clone().into()
    }
}

impl From<NestedTransform> for SkinJoint {
    fn from(value: NestedTransform) -> Self {
        SkinJoint(value.global_transform)
    }
}

impl From<&NestedTransform> for SkinJoint {
    fn from(value: &NestedTransform) -> Self {
        value.clone().into()
    }
}
