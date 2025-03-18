//! CPU-only lighting and shadows.

use std::sync::{Arc, Mutex, RwLock};

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
/// Create an `AnalyticalLightBundle` with the `Lighting::new_*_light` functions:
/// - [`Lighting::new_directional_light`]
pub struct AnalyticalLightBundle<Ct: IsContainer = HybridContainer> {
    pub light: Ct::Container<super::Light>,
    pub light_details: LightDetails<Ct>,
    pub transform: NestedTransform<Ct>,
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

/// Manages lighting for an entire scene.
#[derive(Clone)]
pub struct Lighting {
    pub(crate) geometry_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) light_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    pub(crate) geometry_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    pub(crate) lighting_descriptor: Hybrid<LightingDescriptor>,
    pub(crate) analytical_lights: Arc<Mutex<Vec<AnalyticalLightBundle<WeakContainer>>>>,
    pub(crate) analytical_lights_array: Arc<Mutex<HybridArray<Id<super::Light>>>>,
    pub(crate) bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    pub(crate) shadow_map_update_pipeline: Arc<wgpu::RenderPipeline>,
    pub(crate) shadow_map_update_bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    pub(crate) shadow_map_update_blitter: AtlasBlitter,
    pub(crate) shadow_map_atlas: Atlas,
}

impl Lighting {
    const LABEL: Option<&str> = Some("lighting");

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

    pub(crate) fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // light slab
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // shadow map texture view
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                // shadow map texture sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }

    pub fn create_bindgroup(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        light_slab_buffer: &wgpu::Buffer,
        shadow_map_depth_texture: &crate::texture::Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout: bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_slab_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&shadow_map_depth_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&shadow_map_depth_texture.sampler),
                },
            ],
        })
    }

    /// Returns the lighting bindgroup.
    pub fn get_bindgroup(&self) -> wgpu::BindGroup {
        let mut light_slab_buffer = self.light_slab_buffer.write().unwrap();
        // TODO: invalidate
        let _should_invalidate = light_slab_buffer.update_if_invalid();

        Self::create_bindgroup(
            self.light_slab.device(),
            &self.bindgroup_layout,
            &light_slab_buffer,
            &self.shadow_map_atlas.get_texture(),
        )
    }

    /// Create a new [`Lighting`] manager.
    pub fn new(geometry: &Geometry) -> Self {
        let runtime = geometry.runtime();
        let light_slab =
            SlabAllocator::new_with_label(runtime, wgpu::BufferUsages::empty(), Some("light-slab"));
        let lighting_descriptor = light_slab.new_value(LightingDescriptor::default());
        let light_slab_buffer = light_slab.commit();
        let bindgroup_layout = Self::create_bindgroup_layout(&runtime.device);

        let shadow_map_update_bindgroup_layout: Arc<_> =
            ShadowMap::create_update_bindgroup_layout(&runtime.device).into();
        let shadow_map_update_pipeline =
            ShadowMap::create_update_pipeline(&runtime.device, &shadow_map_update_bindgroup_layout)
                .into();
        Self {
            shadow_map_atlas: Self::create_shadow_map_atlas(
                &light_slab,
                // TODO: make the shadow map atlas size configurable
                wgpu::Extent3d {
                    width: 1024,
                    height: 1024,
                    depth_or_array_layers: 4,
                },
            ),
            analytical_lights: Default::default(),
            analytical_lights_array: Arc::new(Mutex::new(light_slab.new_array([]))),
            geometry_slab: geometry.slab_allocator().clone(),
            light_slab,
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            lighting_descriptor,
            geometry_slab_buffer: Arc::new(RwLock::new(geometry.slab_allocator().commit())),
            bindgroup_layout: bindgroup_layout.into(),
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
        {
            // Update the array of light ids
            // UNWRAP: POP
            let mut analytical_lights_array_guard = self.analytical_lights_array.lock().unwrap();
            let mut analytical_light_ids_vec = analytical_lights_array_guard.get_vec();
            analytical_light_ids_vec.push(bundle.light.id());
            *analytical_lights_array_guard = self.light_slab.new_array(analytical_light_ids_vec);
        }
        {
            // Update our list of weakly ref'd light bundles
            self.analytical_lights
                .lock()
                .unwrap()
                .push(AnalyticalLightBundle::<WeakContainer>::from_hybrid(bundle));
        }
    }

    /// Remove an [`AnalyticalLightBundle`] from the internal list of lights.
    ///
    /// Use this to exclude a light from rendering, without dropping the light.
    ///
    /// After calling this function you can include the light again using [`Lighting::add_light`].
    pub fn remove_light(&self, bundle: &AnalyticalLightBundle) {
        let ids = {
            let mut guard = self.analytical_lights.lock().unwrap();
            guard.retain(|stored_light| stored_light.light.id() != bundle.light.id());
            guard
                .iter()
                .map(|stored_light| stored_light.light.id())
                .collect::<Vec<_>>()
        };
        *self.analytical_lights_array.lock().unwrap() = self.light_slab.new_array(ids);
    }

    /// Create a new [`AnalyticalLightBundle`] for the given descriptor `T`.
    ///
    /// `T` must be one of:
    /// - [`DirectionalLightDescriptor`]
    /// - [`SpotLightDescriptor`]
    /// - [`PointLightDescirptor`]
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

    pub fn upkeep(&self) {
        {
            // Drop any analytical lights that don't have external references,
            // and update our lights array.
            let mut guard = self.analytical_lights.lock().unwrap();
            let mut changed = false;
            guard.retain(|light_bundle| {
                let has_refs = light_bundle.light.has_external_references();
                changed = changed || !has_refs;
                has_refs
            });
            if changed {
                *self.analytical_lights_array.lock().unwrap() = self
                    .light_slab
                    .new_array(guard.iter().map(|bundle| bundle.light.id()));
            }
        }
        self.lighting_descriptor.set(LightingDescriptor {
            analytical_lights_array: self.analytical_lights_array.lock().unwrap().array(),
            shadow_map_atlas_descriptor_id: self.shadow_map_atlas.descriptor_id(),
            update_shadow_map_id: Id::NONE,
            update_shadow_map_texture_index: 0,
        });
        self.light_slab.commit();
    }
}

#[cfg(test)]
mod test {

    use glam::Vec3;

    use crate::{light::SpotLightCalculation, prelude::Transform};

    use super::*;

    #[test]
    /// Ensures that a spot light can determine if a point lies inside or outside its cone
    /// of emission.
    fn spot_one_calc() {
        let (doc, _, _) = gltf::import(
            crate::test::workspace_dir()
                .join("gltf")
                .join("spot_one.glb"),
        )
        .unwrap();
        let light = doc.lights().unwrap().next().unwrap();
        let spot = if let gltf::khr_lights_punctual::Kind::Spot {
            inner_cone_angle,
            outer_cone_angle,
        } = light.kind()
        {
            (inner_cone_angle, outer_cone_angle)
        } else {
            panic!("not a spot light");
        };
        log::info!("spot: {spot:#?}");

        let light_node = doc.nodes().find(|node| node.light().is_some()).unwrap();
        let parent_transform = Transform::from(light_node.transform());
        log::info!("parent_transform: {parent_transform:#?}");

        let spot_descriptor = SpotLightDescriptor {
            position: Vec3::ZERO,
            direction: Vec3::NEG_Z,
            inner_cutoff: spot.0,
            outer_cutoff: spot.1,
            color: Vec3::from(light.color()).extend(1.0),
            intensity: light.intensity(),
        };

        let specific_points = [
            (Vec3::ZERO, true, true, Some(1.0)),
            (Vec3::new(0.5, 0.0, 0.0), false, true, None),
            (Vec3::new(0.5, 0.0, 0.5), false, false, None),
            (Vec3::new(1.0, 0.0, 0.0), false, false, Some(0.0)),
        ];
        for (i, (point, inside_inner, inside_outer, maybe_contribution)) in
            specific_points.into_iter().enumerate()
        {
            log::info!("{i} descriptor: {spot_descriptor:#?}");
            let spot_calc =
                SpotLightCalculation::new(spot_descriptor, parent_transform.into(), point);
            log::info!("{i} spot_calc@{point}:\n{spot_calc:#?}");
            assert_eq!(
                (inside_inner, inside_outer),
                (
                    spot_calc.fragment_is_inside_inner_cone,
                    spot_calc.fragment_is_inside_outer_cone
                ),
            );
            if let Some(expected_contribution) = maybe_contribution {
                assert_eq!(expected_contribution, spot_calc.contribution);
            }
        }
    }

    #[test]
    /// Ensures that a spot light illuminates only the objects within its cone of
    /// emission.
    fn spot_one_frame() {
        let m = 32.0;
        let (w, h) = (16.0f32 * m, 9.0 * m);
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx.new_stage().with_msaa_sample_count(4);
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("spot_one.glb"),
            )
            .unwrap();
        let camera = doc.cameras.first().unwrap();
        camera
            .as_ref()
            .modify(|cam| cam.set_projection(crate::camera::perspective(w, h)));
        stage.use_camera(camera);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("lights/spot_lights/one.png", img);
        frame.present();
    }

    #[test]
    /// Test the spot lights.
    ///
    /// This should render a cube with two spot lights illuminating a spot on two
    /// of its sides.
    fn spot_lights() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_msaa_sample_count(4);

        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("spot_lights.glb"),
            )
            .unwrap();
        let camera = doc.cameras.first().unwrap();
        // TODO: investigate using the camera's aspect for any frame size.
        // A `TextureView` of the frame could be created that renders to the frame
        // within the camera's expected aspect ratio.
        //
        // We'd probably need to constrain rendering to one camera, though.
        camera
            .as_ref()
            .modify(|cam| cam.set_projection(crate::camera::perspective(w, h)));
        stage.use_camera(camera);

        let down_light = doc.lights.first().unwrap();
        log::info!(
            "down_light: {:#?}",
            down_light.light_details.as_spot().unwrap().get()
        );

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("lights/spot_lights/frame.png", img);
        frame.present();
    }
}
