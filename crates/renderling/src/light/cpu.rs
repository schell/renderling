//! CPU-only lighting and shadows.
use std::sync::{Arc, RwLock};

#[cfg(doc)]
use crate::stage::Stage;
use craballoc::{
    prelude::{Hybrid, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
    value::HybridArray,
};
use crabslab::Id;
use glam::{Mat4, UVec2, Vec3, Vec4};
use snafu::prelude::*;

use crate::{
    atlas::{Atlas, AtlasBlitter, AtlasError},
    geometry::Geometry,
    transform::{shader::TransformDescriptor, NestedTransform, Transform},
};

use super::shader::{
    DirectionalLightDescriptor, LightDescriptor, LightStyle, LightingDescriptor,
    PointLightDescriptor, SpotLightDescriptor,
};

pub use super::shadow_map::ShadowMap;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum LightingError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("Driver poll error: {source}"))]
    Poll { source: wgpu::PollError },
}

impl From<AtlasError> for LightingError {
    fn from(source: AtlasError) -> Self {
        LightingError::Atlas { source }
    }
}

/// Describes shared behaviour between all analytical lights.
pub trait IsLight: Clone {
    /// Return the style of this light.
    fn style(&self) -> LightStyle;

    fn light_space_transforms(
        &self,
        // Another transform applied to the light.
        parent_transform: &TransformDescriptor,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        // TODO: in `DirectionalLightDescriptor::shadow_mapping_projection_and_view`, take Frustum
        // as a parameter and then figure out the minimal view projection that includes that frustum
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> Vec<Mat4>;
}

/// A directional light.
///
/// An analitical light that casts light in parallel, infinitely.
#[derive(Clone, Debug)]
pub struct DirectionalLight {
    descriptor: Hybrid<DirectionalLightDescriptor>,
}

impl IsLight for DirectionalLight {
    fn style(&self) -> LightStyle {
        LightStyle::Directional
    }

    fn light_space_transforms(
        &self,
        parent_transform: &TransformDescriptor,
        z_near: f32,
        z_far: f32,
    ) -> Vec<Mat4> {
        let m = Mat4::from(*parent_transform);
        vec![{
            let (p, v) = self
                .descriptor()
                .shadow_mapping_projection_and_view(&m, z_near, z_far);
            p * v
        }]
    }
}

impl DirectionalLight {
    /// Returns a pointer to the descriptor data on the GPU slab.
    pub fn id(&self) -> Id<DirectionalLightDescriptor> {
        self.descriptor.id()
    }

    /// Returns the a copy of the descriptor.
    pub fn descriptor(&self) -> DirectionalLightDescriptor {
        self.descriptor.get()
    }
}

/// A [`DirectionalLight`] comes wrapped in [`AnalyticalLight`], giving the
/// [`AnalyticalLight`] the ability to simulate sunlight or other lights that
/// are "infinitely" far away.
impl AnalyticalLight<DirectionalLight> {
    /// Set the direction of the directional light.
    pub fn set_direction(&self, direction: Vec3) -> &Self {
        self.inner.descriptor.modify(|d| d.direction = direction);
        self
    }

    /// Set the direction and return the directional light.
    pub fn with_direction(self, direction: Vec3) -> Self {
        self.set_direction(direction);
        self
    }

    /// Modify the direction of the directional light.
    pub fn modify_direction<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.direction))
    }

    /// Get the direction of the directional light.
    pub fn direction(&self) -> Vec3 {
        self.inner.descriptor.get().direction
    }

    /// Set the color of the directional light.
    pub fn set_color(&self, color: Vec4) -> &Self {
        self.inner.descriptor.modify(|d| d.color = color);
        self
    }

    /// Set the color and return the directional light.
    pub fn with_color(self, color: Vec4) -> Self {
        self.set_color(color);
        self
    }

    /// Modify the color of the directional light.
    pub fn modify_color<T: 'static>(&self, f: impl FnOnce(&mut Vec4) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.color))
    }

    /// Get the color of the directional light.
    pub fn color(&self) -> Vec4 {
        self.inner.descriptor.get().color
    }

    /// Set the intensity of the directional light.
    pub fn set_intensity(&self, intensity: f32) -> &Self {
        self.inner.descriptor.modify(|d| d.intensity = intensity);
        self
    }

    /// Set the intensity and return the directional light.
    pub fn with_intensity(self, intensity: f32) -> Self {
        self.set_intensity(intensity);
        self
    }

    /// Modify the intensity of the directional light.
    pub fn modify_intensity<T: 'static>(&self, f: impl FnOnce(&mut f32) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.intensity))
    }

    /// Get the intensity of the directional light.
    pub fn intensity(&self) -> f32 {
        self.inner.descriptor.get().intensity
    }
}

/// A point light.
///
/// An analytical light that emits light in all directions from a single point.
#[derive(Clone, Debug)]
pub struct PointLight {
    descriptor: Hybrid<PointLightDescriptor>,
}

impl IsLight for PointLight {
    fn style(&self) -> LightStyle {
        LightStyle::Point
    }

    fn light_space_transforms(
        &self,
        t: &TransformDescriptor,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> Vec<Mat4> {
        let m = Mat4::from(*t);
        let (p, vs) = self
            .descriptor()
            .shadow_mapping_projection_and_view_matrices(&m, z_near, z_far);
        vs.into_iter().map(|v| p * v).collect()
    }
}

impl PointLight {
    /// Returns a pointer to the descriptor data on the GPU slab.
    pub fn id(&self) -> Id<PointLightDescriptor> {
        self.descriptor.id()
    }

    /// Returns a copy of the descriptor.
    pub fn descriptor(&self) -> PointLightDescriptor {
        self.descriptor.get()
    }
}

/// A [`PointLight`] comes wrapped in [`AnalyticalLight`], giving the
/// [`AnalyticalLight`] the ability to simulate lights that
/// emit from a single point in space and attenuate exponentially with
/// distance.
impl AnalyticalLight<PointLight> {
    /// Set the position of the point light.
    pub fn set_position(&self, position: Vec3) -> &Self {
        self.inner.descriptor.modify(|d| d.position = position);
        self
    }

    /// Set the position and return the point light.
    pub fn with_position(self, position: Vec3) -> Self {
        self.set_position(position);
        self
    }

    /// Modify the position of the point light.
    pub fn modify_position<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.position))
    }

    /// Get the position of the point light.
    pub fn position(&self) -> Vec3 {
        self.inner.descriptor.get().position
    }

    /// Set the color of the point light.
    pub fn set_color(&self, color: Vec4) -> &Self {
        self.inner.descriptor.modify(|d| d.color = color);
        self
    }

    /// Set the color and return the point light.
    pub fn with_color(self, color: Vec4) -> Self {
        self.set_color(color);
        self
    }

    /// Modify the color of the point light.
    pub fn modify_color<T: 'static>(&self, f: impl FnOnce(&mut Vec4) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.color))
    }

    /// Get the color of the point light.
    pub fn color(&self) -> Vec4 {
        self.inner.descriptor.get().color
    }

    /// Set the intensity of the point light.
    pub fn set_intensity(&self, intensity: f32) -> &Self {
        self.inner.descriptor.modify(|d| d.intensity = intensity);
        self
    }

    /// Set the intensity and return the point light.
    pub fn with_intensity(self, intensity: f32) -> Self {
        self.set_intensity(intensity);
        self
    }

    /// Modify the intensity of the point light.
    pub fn modify_intensity<T: 'static>(&self, f: impl FnOnce(&mut f32) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.intensity))
    }

    /// Get the intensity of the point light.
    pub fn intensity(&self) -> f32 {
        self.inner.descriptor.get().intensity
    }
}

/// A spot light.
///
/// An analytical light that emits light in a cone shape.
#[derive(Clone, Debug)]
pub struct SpotLight {
    descriptor: Hybrid<SpotLightDescriptor>,
}

impl IsLight for SpotLight {
    fn style(&self) -> LightStyle {
        LightStyle::Spot
    }

    fn light_space_transforms(
        &self,
        t: &TransformDescriptor,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> Vec<Mat4> {
        let m = Mat4::from(*t);
        vec![{
            let (p, v) = self
                .descriptor()
                .shadow_mapping_projection_and_view(&m, z_near, z_far);
            p * v
        }]
    }
}

impl SpotLight {
    /// Returns a pointer to the descriptor data on the GPU slab.
    pub fn id(&self) -> Id<SpotLightDescriptor> {
        self.descriptor.id()
    }

    /// Returns a copy of the descriptor.
    pub fn descriptor(&self) -> SpotLightDescriptor {
        self.descriptor.get()
    }
}

/// A [`SpotLight`] comes wrapped in [`AnalyticalLight`], giving the
/// [`AnalyticalLight`] the ability to simulate lights that
/// emit from a single point in space in a specific direction, with
/// a specific spread.
impl AnalyticalLight<SpotLight> {
    /// Set the position of the spot light.
    pub fn set_position(&self, position: Vec3) -> &Self {
        self.inner.descriptor.modify(|d| d.position = position);
        self
    }

    /// Set the position and return the spot light.
    pub fn with_position(self, position: Vec3) -> Self {
        self.set_position(position);
        self
    }

    /// Modify the position of the spot light.
    pub fn modify_position<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.position))
    }

    /// Get the position of the spot light.
    pub fn position(&self) -> Vec3 {
        self.inner.descriptor.get().position
    }

    /// Set the direction of the spot light.
    pub fn set_direction(&self, direction: Vec3) -> &Self {
        self.inner.descriptor.modify(|d| d.direction = direction);
        self
    }

    /// Set the direction and return the spot light.
    pub fn with_direction(self, direction: Vec3) -> Self {
        self.set_direction(direction);
        self
    }

    /// Modify the direction of the spot light.
    pub fn modify_direction<T: 'static>(&self, f: impl FnOnce(&mut Vec3) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.direction))
    }

    /// Get the direction of the spot light.
    pub fn direction(&self) -> Vec3 {
        self.inner.descriptor.get().direction
    }

    /// Set the inner cutoff of the spot light.
    pub fn set_inner_cutoff(&self, inner_cutoff: f32) -> &Self {
        self.inner
            .descriptor
            .modify(|d| d.inner_cutoff = inner_cutoff);
        self
    }

    /// Set the inner cutoff and return the spot light.
    pub fn with_inner_cutoff(self, inner_cutoff: f32) -> Self {
        self.set_inner_cutoff(inner_cutoff);
        self
    }

    /// Modify the inner cutoff of the spot light.
    pub fn modify_inner_cutoff<T: 'static>(&self, f: impl FnOnce(&mut f32) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.inner_cutoff))
    }

    /// Get the inner cutoff of the spot light.
    pub fn inner_cutoff(&self) -> f32 {
        self.inner.descriptor.get().inner_cutoff
    }

    /// Set the outer cutoff of the spot light.
    pub fn set_outer_cutoff(&self, outer_cutoff: f32) -> &Self {
        self.inner
            .descriptor
            .modify(|d| d.outer_cutoff = outer_cutoff);
        self
    }

    /// Set the outer cutoff and return the spot light.
    pub fn with_outer_cutoff(self, outer_cutoff: f32) -> Self {
        self.set_outer_cutoff(outer_cutoff);
        self
    }

    /// Modify the outer cutoff of the spot light.
    pub fn modify_outer_cutoff<T: 'static>(&self, f: impl FnOnce(&mut f32) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.outer_cutoff))
    }

    /// Get the outer cutoff of the spot light.
    pub fn outer_cutoff(&self) -> f32 {
        self.inner.descriptor.get().outer_cutoff
    }

    /// Set the color of the spot light.
    pub fn set_color(&self, color: Vec4) -> &Self {
        self.inner.descriptor.modify(|d| d.color = color);
        self
    }

    /// Set the color and return the spot light.
    pub fn with_color(self, color: Vec4) -> Self {
        self.set_color(color);
        self
    }

    /// Modify the color of the spot light.
    pub fn modify_color<T: 'static>(&self, f: impl FnOnce(&mut Vec4) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.color))
    }

    /// Get the color of the spot light.
    pub fn color(&self) -> Vec4 {
        self.inner.descriptor.get().color
    }

    /// Set the intensity of the spot light.
    pub fn set_intensity(&self, intensity: f32) -> &Self {
        self.inner.descriptor.modify(|d| d.intensity = intensity);
        self
    }

    /// Set the intensity and return the spot light.
    pub fn with_intensity(self, intensity: f32) -> Self {
        self.set_intensity(intensity);
        self
    }

    /// Modify the intensity of the spot light.
    pub fn modify_intensity<T: 'static>(&self, f: impl FnOnce(&mut f32) -> T) -> T {
        self.inner.descriptor.modify(|d| f(&mut d.intensity))
    }

    /// Get the intensity of the spot light.
    pub fn intensity(&self) -> f32 {
        self.inner.descriptor.get().intensity
    }
}

#[derive(Clone)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

impl From<DirectionalLight> for Light {
    fn from(light: DirectionalLight) -> Self {
        Light::Directional(light)
    }
}

impl From<PointLight> for Light {
    fn from(light: PointLight) -> Self {
        Light::Point(light)
    }
}

impl From<SpotLight> for Light {
    fn from(light: SpotLight) -> Self {
        Light::Spot(light)
    }
}

impl IsLight for Light {
    fn style(&self) -> LightStyle {
        match self {
            Light::Directional(light) => light.style(),
            Light::Point(light) => light.style(),
            Light::Spot(light) => light.style(),
        }
    }

    fn light_space_transforms(
        &self,
        // Another transform applied to the light.
        parent_transform: &TransformDescriptor,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> Vec<Mat4> {
        match self {
            Light::Directional(light) => {
                light.light_space_transforms(parent_transform, z_near, z_far)
            }
            Light::Point(light) => light.light_space_transforms(parent_transform, z_near, z_far),
            Light::Spot(light) => light.light_space_transforms(parent_transform, z_near, z_far),
        }
    }
}

/// A bundle of lighting resources representing one analytical light in a scene.
///
/// Create an [`AnalyticalLight`] with:
/// * [`Stage::new_directional_light`]
/// * [`Stage::new_point_light`]
/// * [`Stage::new_spot_light`].
///
/// Lights may be added and removed from rendering with [`Stage::add_light`] and
/// [`Stage::remove_light`].
/// The GPU resources a light uses will not be released until [`Stage::remove_light`]
/// is called _and_ the light is dropped.
#[derive(Clone)]
pub struct AnalyticalLight<T = Light> {
    /// The generic light descriptor.
    pub(crate) light_descriptor: Hybrid<LightDescriptor>,
    /// The specific light.
    inner: T,
    /// The light's global transform.
    ///
    /// This value lives in the lighting slab.
    transform: Transform,
    /// The light's nested transform.
    ///
    /// This value comes from the light's node, if it belongs to one.
    /// This may have been set if this light originated from a GLTF file.
    /// This value lives on the geometry slab and must be referenced here
    /// to keep the two in sync, which is required to animate lights.
    node_transform: Arc<RwLock<Option<NestedTransform>>>,
}

impl<T: IsLight> core::fmt::Display for AnalyticalLight<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "AnalyticalLight type={} light-id={:?} node-nested-transform-global-id:{:?}",
            self.inner.style(),
            self.light_descriptor.id(),
            self.node_transform
                .read()
                .unwrap()
                .as_ref()
                .map(|h| h.global_id())
        ))
    }
}

impl<T: IsLight> IsLight for AnalyticalLight<T> {
    fn style(&self) -> LightStyle {
        self.inner.style()
    }

    fn light_space_transforms(
        &self,
        // Another transform applied to the light.
        parent_transform: &TransformDescriptor,
        // Near limits of the light's reach
        //
        // The maximum should be the `Camera`'s `Frustum::depth()`.
        // TODO: in `DirectionalLightDescriptor::shadow_mapping_projection_and_view`, take Frustum
        // as a parameter and then figure out the minimal view projection that includes that frustum
        z_near: f32,
        // Far limits of the light's reach
        z_far: f32,
    ) -> Vec<Mat4> {
        self.inner
            .light_space_transforms(parent_transform, z_near, z_far)
    }
}

impl AnalyticalLight {
    /// Returns a reference to the inner `DirectionalLight`, if this light is directional.
    pub fn as_directional(&self) -> Option<&DirectionalLight> {
        match &self.inner {
            Light::Directional(light) => Some(light),
            _ => None,
        }
    }

    /// Returns a reference to the inner `PointLight`, if this light is a point light.
    pub fn as_point(&self) -> Option<&PointLight> {
        match &self.inner {
            Light::Point(light) => Some(light),
            _ => None,
        }
    }

    /// Returns a reference to the inner `SpotLight`, if this light is a spot light.
    pub fn as_spot(&self) -> Option<&SpotLight> {
        match &self.inner {
            Light::Spot(light) => Some(light),
            _ => None,
        }
    }
}

impl<T: IsLight> AnalyticalLight<T> {
    /// Returns a pointer to this light on the GPU
    pub fn id(&self) -> Id<LightDescriptor> {
        self.light_descriptor.id()
    }

    /// Returns a copy of the descriptor on the GPU.
    pub fn descriptor(&self) -> LightDescriptor {
        self.light_descriptor.get()
    }

    /// Link this light to a node's `NestedTransform`.
    pub fn link_node_transform(&self, transform: &NestedTransform) {
        *self.node_transform.write().unwrap() = Some(transform.clone());
    }

    /// Get a reference to the inner light.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Get a reference to the light's global transform.
    ///
    /// This value lives in the lighting slab.
    ///
    /// ## Note
    /// If a [`NestedTransform`] has been linked to this light by using [`Self::link_node_transform`],
    /// the transform returned by this function may be overwritten at any point by the given
    /// [`NestedTransform`].
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Get a reference to the light's linked global node transform.
    ///
    /// ## Note
    /// The returned transform, if any, is the global transform of a linked `NestedTransform`.
    /// To change this value, you should do so through the `NestedTransform`, which is likely
    /// held in the
    pub fn linked_node_transform(&self) -> Option<NestedTransform> {
        self.node_transform
            .read()
            .unwrap()
            .as_ref()
            .map(|t| t.clone())
    }

    /// Convert this light into a generic light, hiding the specific light type that it is.
    ///
    /// This is useful if you want to store your lights together.
    pub fn into_generic(self) -> AnalyticalLight
    where
        Light: From<T>,
    {
        let AnalyticalLight {
            light_descriptor,
            inner,
            transform,
            node_transform,
        } = self;
        let inner = Light::from(inner);
        AnalyticalLight {
            light_descriptor,
            inner,
            transform,
            node_transform,
        }
    }
}

/// Manages lighting for an entire scene.
#[derive(Clone)]
pub struct Lighting {
    pub(crate) geometry_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) light_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    pub(crate) geometry_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    pub(crate) lighting_descriptor: Hybrid<LightingDescriptor>,
    pub(crate) analytical_lights: Arc<RwLock<Vec<AnalyticalLight>>>,
    pub(crate) analytical_lights_array: Arc<RwLock<Option<HybridArray<Id<LightDescriptor>>>>>,
    pub(crate) shadow_map_update_pipeline: Arc<wgpu::RenderPipeline>,
    pub(crate) shadow_map_update_bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    pub(crate) shadow_map_update_blitter: AtlasBlitter,
    pub(crate) shadow_map_atlas: Atlas,
}

pub struct LightingBindGroupLayoutEntries {
    pub light_slab: wgpu::BindGroupLayoutEntry,
    pub shadow_map_image: wgpu::BindGroupLayoutEntry,
    pub shadow_map_sampler: wgpu::BindGroupLayoutEntry,
}

impl LightingBindGroupLayoutEntries {
    pub fn new(starting_binding: u32) -> Self {
        Self {
            light_slab: wgpu::BindGroupLayoutEntry {
                binding: starting_binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            shadow_map_image: wgpu::BindGroupLayoutEntry {
                binding: starting_binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
                count: None,
            },
            shadow_map_sampler: wgpu::BindGroupLayoutEntry {
                binding: starting_binding + 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        }
    }
}

impl Lighting {
    /// Create the atlas used to store all shadow maps.
    fn create_shadow_map_atlas(
        light_slab: &SlabAllocator<WgpuRuntime>,
        size: wgpu::Extent3d,
    ) -> Atlas {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC;
        Atlas::new(
            light_slab,
            size,
            Some(wgpu::TextureFormat::R32Float),
            Some("shadow-map-atlas"),
            Some(usage),
        )
    }

    /// Create a new [`Lighting`] manager.
    pub fn new(atlas_size: wgpu::Extent3d, geometry: &Geometry) -> Self {
        let runtime = geometry.runtime();
        let light_slab = SlabAllocator::new(runtime, "light-slab", wgpu::BufferUsages::empty());
        let lighting_descriptor = light_slab.new_value(LightingDescriptor::default());
        let light_slab_buffer = light_slab.commit();
        let shadow_map_update_bindgroup_layout: Arc<_> =
            ShadowMap::create_update_bindgroup_layout(&runtime.device).into();
        let shadow_map_update_pipeline =
            ShadowMap::create_update_pipeline(&runtime.device, &shadow_map_update_bindgroup_layout)
                .into();
        Self {
            shadow_map_atlas: Self::create_shadow_map_atlas(&light_slab, atlas_size),
            analytical_lights: Default::default(),
            analytical_lights_array: Default::default(),
            geometry_slab: geometry.slab_allocator().clone(),
            light_slab,
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            lighting_descriptor,
            geometry_slab_buffer: Arc::new(RwLock::new(geometry.slab_allocator().commit())),
            shadow_map_update_pipeline,
            shadow_map_update_bindgroup_layout,
            shadow_map_update_blitter: AtlasBlitter::new(
                &runtime.device,
                wgpu::TextureFormat::R32Float,
                wgpu::FilterMode::Nearest,
            ),
        }
    }

    pub fn slab_allocator(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.light_slab
    }

    /// Add an [`AnalyticalLight`] to the internal list of lights.
    ///
    /// This is called implicitly by:
    ///
    /// * [`Lighting::new_directional_light`].
    /// * [`Lighting::new_point_light`].
    /// * [`Lighting::new_spot_light`].
    ///
    /// This can be used to add the light back to the scene after using
    /// [`Lighting::remove_light`].
    pub fn add_light<T>(&self, bundle: &AnalyticalLight<T>)
    where
        T: IsLight,
        Light: From<T>,
    {
        log::trace!(
            "adding light {:?} ({})",
            bundle.light_descriptor.id(),
            bundle.inner.style()
        );
        // Update our list of weakly ref'd light bundles
        self.analytical_lights
            .write()
            .unwrap()
            .push(bundle.clone().into_generic());
        // Invalidate the array of lights
        *self.analytical_lights_array.write().unwrap() = None;
    }

    /// Remove an [`AnalyticalLight`] from the internal list of lights.
    ///
    /// Use this to exclude a light from rendering, without dropping the light.
    ///
    /// After calling this function you can include the light again using [`Lighting::add_light`].
    pub fn remove_light<T: IsLight>(&self, bundle: &AnalyticalLight<T>) {
        log::trace!(
            "removing light {:?} ({})",
            bundle.light_descriptor.id(),
            bundle.inner.style()
        );
        // Remove the light from the list of weakly ref'd light bundles
        let mut guard = self.analytical_lights.write().unwrap();
        guard.retain(|stored_light| {
            stored_light.light_descriptor.id() != bundle.light_descriptor.id()
        });
        *self.analytical_lights_array.write().unwrap() = None;
    }

    /// Return an iterator over all lights.
    pub fn lights(&self) -> Vec<AnalyticalLight> {
        self.analytical_lights.read().unwrap().clone()
    }

    /// Create a new [`AnalyticalLight<DirectionalLight>`].
    ///
    /// The light is automatically added with [`Lighting::add_light`].
    pub fn new_directional_light(&self) -> AnalyticalLight<DirectionalLight> {
        let descriptor = self
            .light_slab
            .new_value(DirectionalLightDescriptor::default());
        let transform = Transform::new(&self.light_slab);
        let light_descriptor = self.light_slab.new_value({
            let mut light = LightDescriptor::from(descriptor.id());
            light.transform_id = transform.id();
            light
        });

        let bundle = AnalyticalLight {
            light_descriptor,
            inner: DirectionalLight { descriptor },
            transform,
            node_transform: Default::default(),
        };
        self.add_light(&bundle);

        bundle
    }

    /// Create a new [`AnalyticalLight<PointLight>`].
    ///
    /// The light is automatically added with [`Lighting::add_light`].
    pub fn new_point_light(&self) -> AnalyticalLight<PointLight> {
        let descriptor = self.light_slab.new_value(PointLightDescriptor::default());
        let transform = Transform::new(&self.light_slab);
        let light_descriptor = self.light_slab.new_value({
            let mut light = LightDescriptor::from(descriptor.id());
            light.transform_id = transform.id();
            light
        });

        let bundle = AnalyticalLight {
            light_descriptor,
            inner: PointLight { descriptor },
            transform,
            node_transform: Default::default(),
        };
        self.add_light(&bundle);

        bundle
    }

    /// Create a new [`AnalyticalLight<SpotLight>`].
    ///
    /// The light is automatically added with [`Lighting::add_light`].
    pub fn new_spot_light(&self) -> AnalyticalLight<SpotLight> {
        let descriptor = self.light_slab.new_value(SpotLightDescriptor::default());
        let transform = Transform::new(&self.light_slab);
        let light_descriptor = self.light_slab.new_value({
            let mut light = LightDescriptor::from(descriptor.id());
            light.transform_id = transform.id();
            light
        });

        let bundle = AnalyticalLight {
            light_descriptor,
            inner: SpotLight { descriptor },
            transform,
            node_transform: Default::default(),
        };
        self.add_light(&bundle);

        bundle
    }

    /// Enable shadow mapping for the given [`AnalyticalLight`], creating
    /// a new [`ShadowMap`].
    pub fn new_shadow_map<T>(
        &self,
        analytical_light_bundle: &AnalyticalLight<T>,
        // Size of the shadow map
        size: UVec2,
        // Distance to the near plane of the shadow map's frustum.
        //
        // Only objects within the shadow map's frustum will cast shadows.
        z_near: f32,
        // Distance to the far plane of the shadow map's frustum
        //
        // Only objects within the shadow map's frustum will cast shadows.
        z_far: f32,
    ) -> Result<ShadowMap, LightingError>
    where
        T: IsLight,
        Light: From<T>,
    {
        ShadowMap::new(self, analytical_light_bundle, size, z_near, z_far)
    }

    #[must_use]
    pub fn commit(&self) -> SlabBuffer<wgpu::Buffer> {
        log::trace!("committing lights");

        // Sync any lights whose node transforms have changed
        for light in self.analytical_lights.read().unwrap().iter() {
            if let Some(node_transform) = light.node_transform.read().unwrap().as_ref() {
                let global_node_transform = node_transform.global_descriptor();
                if node_transform.global_descriptor() != light.transform.descriptor() {
                    light.transform.set_descriptor(global_node_transform);
                }
            }
        }

        let lights_array = {
            let mut array_guard = self.analytical_lights_array.write().unwrap();

            // Create a new array if lights have been invalidated by
            // `Lighting::add_light` or `Lighting::remove_light`
            array_guard
                .get_or_insert_with(|| {
                    log::trace!("  analytical lights array was invalidated");
                    let lights_guard = self.analytical_lights.read().unwrap();
                    let new_lights = lights_guard
                        .iter()
                        .map(|bundle| bundle.light_descriptor.id());
                    let array = self.light_slab.new_array(new_lights);
                    log::trace!("  lights array is now: {:?}", array.array());
                    array
                })
                .array()
        };

        self.lighting_descriptor.modify(
            |LightingDescriptor {
                 analytical_lights_array,
                 shadow_map_atlas_descriptor_id,
                 update_shadow_map_id,
                 update_shadow_map_texture_index,
                 // Don't change the tiling descriptor
                 light_tiling_descriptor_id: _,
             }| {
                *analytical_lights_array = lights_array;
                *shadow_map_atlas_descriptor_id = self.shadow_map_atlas.descriptor_id();
                *update_shadow_map_id = Id::NONE;
                *update_shadow_map_texture_index = 0;
            },
        );

        let buffer = self.light_slab.commit();
        log::trace!("  light slab creation time: {}", buffer.creation_time());
        buffer
    }
}

#[cfg(test)]
mod test;
