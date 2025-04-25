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
/// Create an `AnalyticalLightBundle` with the `Lighting::new_analytical_light`,
/// or from `Stage::new_analytical_light`.
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
            analytical_lights_array: Arc::new(Mutex::new(light_slab.new_array([]))),
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
        self.lighting_descriptor.modify(
            |LightingDescriptor {
                 analytical_lights_array,
                 shadow_map_atlas_descriptor_id,
                 update_shadow_map_id,
                 update_shadow_map_texture_index,
                 light_tiling_descriptor: _,
             }| {
                *analytical_lights_array = self.analytical_lights_array.lock().unwrap().array();
                *shadow_map_atlas_descriptor_id = self.shadow_map_atlas.descriptor_id();
                *update_shadow_map_id = Id::NONE;
                *update_shadow_map_texture_index = 0;
            },
        );
        self.light_slab.commit()
    }
}

#[cfg(test)]
mod test {

    use glam::{Vec3, Vec4};
    use spirv_std::num_traits::Zero;

    use crate::{
        bvol::BoundingBox,
        camera::Camera,
        light::SpotLightCalculation,
        math::GpuRng,
        pbr::Material,
        prelude::Transform,
        stage::{Renderlet, Stage, Vertex},
    };

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
        let stage = ctx.new_stage().with_msaa_sample_count(4);
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
        let stage = ctx
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

    #[test]
    fn light_tiling_light_bounds() {
        let magnification = 8;
        let w = 16.0 * 2.0f32.powi(magnification);
        let h = 9.0 * 2.0f32.powi(magnification);
        let ctx = crate::Context::headless(w as u32, h as u32);
        let stage = ctx.new_stage().with_msaa_sample_count(4);
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("light_tiling_test.glb"),
            )
            .unwrap();
        let camera = doc.cameras.first().unwrap();

        stage.use_camera(camera);

        let _lights = crate::test::make_two_directional_light_setup(&stage);

        // Here we only want to render the bounding boxes of the renderlets,
        // so mark the renderlets themeselves invisible
        doc.renderlets_iter().for_each(|hy_rend| {
            hy_rend.modify(|r| {
                r.visible = false;
            });
        });

        let colors = [0x6DE1D2FF, 0xFFD63AFF, 0x6DE1D2FF, 0xF75A5AFF].map(|albedo_factor| {
            stage.new_material(Material {
                albedo_factor: {
                    let mut color = crate::math::hex_to_vec4(albedo_factor);
                    crate::color::linear_xfer_vec4(&mut color);
                    color
                },
                ..Default::default()
            })
        });
        let mut resources = vec![];
        for (i, node) in doc.nodes.iter().enumerate() {
            if node.mesh.is_none() {
                continue;
            }
            let transform = Mat4::from(node.transform.get_global_transform());
            if let Some(mesh_index) = node.mesh {
                log::info!("mesh: {}", node.name.as_deref().unwrap_or("unknown"));
                let mesh = &doc.meshes[mesh_index];
                for prim in mesh.primitives.iter() {
                    let (min, max) = prim.bounding_box;
                    let min = transform.transform_point3(min);
                    let max = transform.transform_point3(max);
                    let bb = BoundingBox::from_min_max(min, max);
                    if bb.half_extent.min_element().is_zero() {
                        log::warn!("bounding box is not a volume, skipping");
                        continue;
                    }
                    log::info!("min: {min}, max: {max}");
                    resources.push(
                        stage
                            .builder()
                            .with_vertices({
                                bb.get_mesh()
                                    .map(|(p, n)| Vertex::default().with_position(p).with_normal(n))
                            })
                            .with_material_id(colors[i % colors.len()].id())
                            .build(),
                    );
                }
            }
        }

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("lights/tiling/bounds.png", img);
        frame.present();
    }

    fn gen_vec3(prng: &mut GpuRng) -> Vec3 {
        let x = prng.gen_f32(-50.0, 50.0);
        let y = prng.gen_f32(-50.0, 50.0);
        let z = prng.gen_f32(-50.0, 50.0);
        Vec3::new(x, y, z)
    }

    fn gen_light(
        stage: &Stage,
        prng: &mut GpuRng,
    ) -> (HybridArray<Vertex>, Hybrid<Material>, Hybrid<Renderlet>) {
        let position = gen_vec3(prng);
        // while bounding_boxes.iter().any(|bb| bb.contains_point(position)) {
        //     position = gen_vec3(&mut prng);
        // }

        // let nested_transform = stage.new_nested_transform();
        // nested_transform.modify(|t| {
        //     t.translation = position;
        // });
        let color = Vec4::new(
            prng.gen_f32(0.0, 1.0),
            prng.gen_f32(0.0, 1.0),
            prng.gen_f32(0.0, 1.0),
            1.0,
        );
        let scale = prng.gen_f32(0.1, 1.0);
        let light_bb = BoundingBox {
            center: position,
            half_extent: Vec3::new(scale, scale, scale) * 10.0,
        };
        // let intensity = 50.0 * scale;
        // let light_descriptor = PointLightDescriptor {
        //     color,
        //     intensity,
        //     ..Default::default()
        // };
        // let light = stage.new_analytical_light(light_descriptor, Some(nested_transform));

        // Also make a renderlet for the light, so we can see where it is.
        log::info!("position: {position}, scale: {scale}, color: {color}");
        let rez = stage
            .builder()
            .with_vertices(
                light_bb
                    .get_mesh()
                    .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
            )
            .with_material(Material {
                albedo_factor: color,
                ..Default::default()
            })
            .build();
        log::info!(
            "built bb renderlet {:?} {}",
            rez.2.id(),
            rez.2.notifier_index()
        );
        rez
    }

    fn size() -> UVec2 {
        UVec2::new(
            (16.0 * 2.0f32.powi(8)) as u32,
            (9.0 * 2.0f32.powi(8)) as u32,
        )
    }

    fn camera() -> Camera {
        let size = size();
        Camera::new(
            Mat4::perspective_rh(
                std::f32::consts::FRAC_PI_4,
                size.x as f32 / size.y as f32,
                0.1,
                1000.0,
            ),
            Mat4::look_at_rh(Vec3::new(250.0, 100.0, 250.0), Vec3::ZERO, Vec3::Y),
        )
    }

    // let doc = stage
    //     .load_gltf_document_from_path(
    //         crate::test::workspace_dir()
    //             .join("gltf")
    //             .join("light_tiling_test.glb"),
    //     )
    //     .unwrap();
    // let camera = doc.cameras.first().unwrap();

    // for r in doc.renderlets_iter() {
    //     r.modify(|r| r.visible = false);
    // }

    // let frame = ctx.get_next_frame().unwrap();
    // let start = std::time::Instant::now();
    // stage.render(&frame.view());
    // let elapsed = start.elapsed();
    // log::info!("rendering w/o lights: {}s", elapsed.as_secs_f32());
    // let img = frame.read_image().unwrap();
    // img_diff::save("lights/tiling/before-lights.png", img);
    // frame.present();

    // let mut bounding_boxes = vec![];
    // for node in doc.nodes.iter() {
    //     if node.mesh.is_none() {
    //         continue;
    //     }
    //     let transform = Mat4::from(node.transform.get_global_transform());
    //     if let Some(mesh_index) = node.mesh {
    //         let mesh = &doc.meshes[mesh_index];
    //         for prim in mesh.primitives.iter() {
    //             let (min, max) = prim.bounding_box;
    //             let min = transform.transform_point3(min);
    //             let max = transform.transform_point3(max);
    //             let bb = BoundingBox::from_min_max(min, max);
    //             if bb.half_extent.min_element().is_zero() {
    //                 continue;
    //             }
    //             bounding_boxes.push(bb);
    //         }
    //     }
    // }
    // log::info!("have {} bounding boxes", bounding_boxes.len());

    #[test]
    /// Test the light tiling feature.
    fn light_tiling_sanity() {
        let _ = env_logger::builder().is_test(true).try_init();
        let size = size();
        let ctx = crate::Context::headless(size.x, size.y).with_use_direct_draw(true);
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_frustum_culling(false)
            .with_occlusion_culling(false);

        let camera = stage.new_camera(camera());
        let camera_id = stage.geometry_descriptor().get().camera_id;
        stage.use_camera(camera);
        let new_camera_id = stage.geometry_descriptor().get().camera_id;
        assert_eq!(camera_id, new_camera_id);

        let mut prng = crate::math::GpuRng::new(666);

        const NUM_LIGHTS: u32 = 100;
        let mut lights = vec![];
        for _ in 0..NUM_LIGHTS {
            lights.push(gen_light(&stage, &mut prng));
        }

        let frame = ctx.get_next_frame().unwrap();
        let start = std::time::Instant::now();
        stage.render(&frame.view());
        let elapsed = start.elapsed();
        log::info!("rendering with lights: {}s", elapsed.as_secs_f32());
        let img = frame.read_image().unwrap();
        img_diff::save("lights/tiling/after-lights.png", img);
        frame.present();
    }
}
