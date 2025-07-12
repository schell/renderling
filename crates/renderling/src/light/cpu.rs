//! CPU-only lighting and shadows.

use core::ops::Deref;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
    value::{HybridArray, HybridContainer, IsContainer, WeakContainer, WeakHybrid},
};
use crabslab::{Id, SlabItem};
use glam::{Mat4, UVec2};
use snafu::prelude::*;

use crate::{
    atlas::{Atlas, AtlasBlitter, AtlasError},
    geometry::Geometry,
    stage::NestedTransform,
};

use super::{
    DirectionalLightDescriptor, Light, LightStyle, LightingDescriptor, PointLightDescriptor,
    SpotLightDescriptor,
};

pub use super::shadow_map::ShadowMap;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum LightingError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("AnalyticalLightBundle attached to this ShadowMap was dropped"))]
    DroppedAnalyticalLightBundle,
}

impl From<AtlasError> for LightingError {
    fn from(source: AtlasError) -> Self {
        LightingError::Atlas { source }
    }
}

/// A wrapper around all types of analytical lighting.
#[derive(Clone, Debug)]
pub enum LightDetails<C: IsContainer = HybridContainer> {
    Directional(C::Container<DirectionalLightDescriptor>),
    Point(C::Container<PointLightDescriptor>),
    Spot(C::Container<SpotLightDescriptor>),
}

impl From<Hybrid<DirectionalLightDescriptor>> for LightDetails {
    fn from(value: Hybrid<DirectionalLightDescriptor>) -> Self {
        LightDetails::Directional(value)
    }
}

impl From<Hybrid<SpotLightDescriptor>> for LightDetails {
    fn from(value: Hybrid<SpotLightDescriptor>) -> Self {
        LightDetails::Spot(value)
    }
}

impl From<Hybrid<PointLightDescriptor>> for LightDetails {
    fn from(value: Hybrid<PointLightDescriptor>) -> Self {
        LightDetails::Point(value)
    }
}

impl<C: IsContainer> LightDetails<C> {
    pub fn as_directional(&self) -> Option<&C::Container<DirectionalLightDescriptor>> {
        if let LightDetails::Directional(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_spot(&self) -> Option<&C::Container<SpotLightDescriptor>> {
        if let LightDetails::Spot(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_point(&self) -> Option<&C::Container<PointLightDescriptor>> {
        if let LightDetails::Point(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn style(&self) -> LightStyle {
        match self {
            LightDetails::Directional(_) => LightStyle::Directional,
            LightDetails::Point(_) => LightStyle::Point,
            LightDetails::Spot(_) => LightStyle::Spot,
        }
    }
}

impl LightDetails<WeakContainer> {
    pub(crate) fn from_hybrid(hybrid: &LightDetails<HybridContainer>) -> Self {
        match hybrid {
            LightDetails::Directional(d) => LightDetails::Directional(WeakHybrid::from_hybrid(d)),
            LightDetails::Point(p) => LightDetails::Point(WeakHybrid::from_hybrid(p)),
            LightDetails::Spot(s) => LightDetails::Spot(WeakHybrid::from_hybrid(s)),
        }
    }

    pub(crate) fn upgrade(&self) -> Option<LightDetails> {
        Some(match self {
            LightDetails::Directional(d) => LightDetails::Directional(d.upgrade()?),
            LightDetails::Point(p) => LightDetails::Point(p.upgrade()?),
            LightDetails::Spot(s) => LightDetails::Spot(s.upgrade()?),
        })
    }
}

/// A bundle of lighting resources representing one analytical light in a scene.
///
/// Create an `AnalyticalLightBundle` with the `Lighting::new_analytical_light`,
/// or from `Stage::new_analytical_light`.
pub struct AnalyticalLightBundle<Ct: IsContainer = HybridContainer> {
    pub light: Ct::Container<super::Light>,
    pub light_details: LightDetails<Ct>,
    pub transform: NestedTransform<Ct>,
}

impl core::fmt::Display for AnalyticalLightBundle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "AnalyticalLightBundle type={} light-id={:?}",
            self.light_details.style(),
            self.light.id(),
        ))
    }
}

impl<Ct: IsContainer> Clone for AnalyticalLightBundle<Ct>
where
    Ct::Container<Light>: Clone,
    LightDetails<Ct>: Clone,
    NestedTransform<Ct>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            light: self.light.clone(),
            light_details: self.light_details.clone(),
            transform: self.transform.clone(),
        }
    }
}

impl AnalyticalLightBundle<WeakContainer> {
    pub(crate) fn from_hybrid(light: &AnalyticalLightBundle) -> Self {
        AnalyticalLightBundle {
            light: WeakHybrid::from_hybrid(&light.light),
            light_details: LightDetails::from_hybrid(&light.light_details),
            transform: NestedTransform::from_hybrid(&light.transform),
        }
    }

    pub(crate) fn upgrade(&self) -> Option<AnalyticalLightBundle> {
        Some(AnalyticalLightBundle {
            light: self.light.upgrade()?,
            light_details: self.light_details.upgrade()?,
            transform: self.transform.upgrade()?,
        })
    }
}

impl AnalyticalLightBundle {
    pub fn weak(&self) -> AnalyticalLightBundle<WeakContainer> {
        AnalyticalLightBundle::from_hybrid(self)
    }

    pub fn light_space_transforms(&self, z_near: f32, z_far: f32) -> Vec<Mat4> {
        let t = self.transform.get();
        let m = Mat4::from(t);
        match &self.light_details {
            LightDetails::Directional(d) => vec![{
                let (p, v) = d
                    .get()
                    .shadow_mapping_projection_and_view(&m, z_near, z_far);
                p * v
            }],
            LightDetails::Point(point) => {
                let (p, vs) = point
                    .get()
                    .shadow_mapping_projection_and_view_matrices(&m, z_near, z_far);
                vs.into_iter().map(|v| p * v).collect()
            }
            LightDetails::Spot(spot) => vec![{
                let (p, v) = spot
                    .get()
                    .shadow_mapping_projection_and_view(&m, z_near, z_far);
                p * v
            }],
        }
    }
}

struct AnalyticalLightIterator<'a> {
    inner: RwLockReadGuard<'a, Vec<AnalyticalLightBundle<WeakContainer>>>,
    index: usize,
}

impl Iterator for AnalyticalLightIterator<'_> {
    type Item = AnalyticalLightBundle;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.get(self.index)?;
        item.upgrade()
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
    pub(crate) analytical_lights: Arc<RwLock<Vec<AnalyticalLightBundle<WeakContainer>>>>,
    pub(crate) analytical_lights_array: Arc<RwLock<Option<HybridArray<Id<super::Light>>>>>,
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

    /// Add an [`AnalyticalLightBundle`] to the internal list of lights.
    ///
    /// This is called implicitly by [`Lighting::new_analytical_light`].
    ///
    /// This can be used to add the light back to the scene after using
    /// [`Lighting::remove_light`].
    pub fn add_light(&self, bundle: &AnalyticalLightBundle) {
        log::trace!(
            "adding light {:?} ({})",
            bundle.light.id(),
            bundle.light_details.style()
        );
        // Update our list of weakly ref'd light bundles
        self.analytical_lights
            .write()
            .unwrap()
            .push(AnalyticalLightBundle::<WeakContainer>::from_hybrid(bundle));
        // Invalidate the array of lights
        *self.analytical_lights_array.write().unwrap() = None;
    }

    /// Remove an [`AnalyticalLightBundle`] from the internal list of lights.
    ///
    /// Use this to exclude a light from rendering, without dropping the light.
    ///
    /// After calling this function you can include the light again using [`Lighting::add_light`].
    pub fn remove_light(&self, bundle: &AnalyticalLightBundle) {
        log::trace!(
            "removing light {:?} ({})",
            bundle.light.id(),
            bundle.light_details.style()
        );
        // Remove the light from the list of weakly ref'd light bundles
        let mut guard = self.analytical_lights.write().unwrap();
        guard.retain(|stored_light| stored_light.light.id() != bundle.light.id());
        *self.analytical_lights_array.write().unwrap() = None;
    }

    /// Return an iterator over all lights.
    pub fn lights(&self) -> impl Iterator<Item = AnalyticalLightBundle> + '_ {
        let inner = self.analytical_lights.read().unwrap();
        AnalyticalLightIterator { inner, index: 0 }
    }

    /// Create a new [`AnalyticalLightBundle`] for the given descriptor `T`.
    ///
    /// `T` must be one of:
    /// - [`DirectionalLightDescriptor`]
    /// - [`SpotLightDescriptor`]
    /// - [`PointLightDescriptor`]
    pub fn new_analytical_light<T>(
        &self,
        light_descriptor: T,
        nested_transform: Option<NestedTransform>,
    ) -> AnalyticalLightBundle
    where
        T: Clone + Copy + SlabItem + Send + Sync,
        Light: From<Id<T>>,
        LightDetails: From<Hybrid<T>>,
    {
        let transform = nested_transform.unwrap_or_else(|| NestedTransform::new(&self.light_slab));
        let light_inner = self.light_slab.new_value(light_descriptor);
        let light = self.light_slab.new_value({
            let mut light = Light::from(light_inner.id());
            light.transform_id = transform.global_transform_id();
            light
        });
        let light_details = LightDetails::from(light_inner);
        let bundle = AnalyticalLightBundle {
            light,
            light_details,
            transform,
        };
        log::trace!(
            "created light {:?} ({})",
            bundle.light.id(),
            bundle.light_details.style()
        );

        self.add_light(&bundle);

        bundle
    }

    /// Enable shadow mapping for the given [`AnalyticalLightBundle`], creating
    /// a new [`ShadowMap`].
    pub fn new_shadow_map(
        &self,
        analytical_light_bundle: &AnalyticalLightBundle,
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
    ) -> Result<ShadowMap, LightingError> {
        ShadowMap::new(self, analytical_light_bundle, size, z_near, z_far)
    }

    #[must_use]
    pub fn commit(&self) -> SlabBuffer<wgpu::Buffer> {
        log::trace!("committing lights");
        let lights_array = {
            let mut lights_guard = self.analytical_lights.write().unwrap();
            // Update the list of analytical lights to only reference lights that are still
            // held somewhere in the outside program.
            let mut analytical_lights_dropped = false;
            lights_guard.retain(|light_bundle| {
                let has_refs = light_bundle.light.has_external_references();
                if !has_refs {
                    log::trace!(
                        "  light {:?} ({}) was dropped",
                        light_bundle.light.id(),
                        light_bundle.light_details.style()
                    );
                }
                analytical_lights_dropped = analytical_lights_dropped || !has_refs;
                has_refs
            });

            // If lights have been dropped, invalidate the array
            let mut array_guard = self.analytical_lights_array.write().unwrap();
            if analytical_lights_dropped {
                array_guard.take();
            }

            // If lights have been invalidated (either by some being dropped or if
            // it was previously invalidated by `Lighting::add_light` or `Lighting::remove_light`),
            // create a new array
            array_guard
                .get_or_insert_with(|| {
                    log::trace!("  analytical lights array was invalidated");
                    let new_lights = lights_guard
                        .iter()
                        .map(|bundle| bundle.light.id())
                        .collect::<Vec<_>>();
                    log::trace!("  lights are now: {new_lights:?}");
                    let array = self.light_slab.new_array(new_lights);
                    log::trace!("  lights array is now: {array:?}");
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
