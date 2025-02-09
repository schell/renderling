//! CPU-only lighting and shadows.

use core::{ops::Deref, sync::atomic::AtomicUsize};
use std::sync::{Arc, Mutex, RwLock};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
    value::{
        HybridArray, HybridContainer, HybridWriteGuard, IsContainer, WeakContainer, WeakHybrid,
    },
};
use crabslab::Id;
use glam::{Mat4, UVec2};
use snafu::prelude::*;

use crate::{
    atlas::{Atlas, AtlasBlitter, AtlasBlittingOperation, AtlasError, AtlasImage, AtlasTexture},
    bindgroup::ManagedBindGroup,
    bvol::Frustum,
    camera::Camera,
    stage::{NestedTransform, Renderlet},
};

use super::{
    DirectionalLightDescriptor, LightStyle, LightingDescriptor, PointLightDescriptor,
    ShadowMapDescriptor, SpotLightDescriptor,
};

#[derive(Debug, Snafu)]
pub enum LightingError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },
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

impl<C: IsContainer> LightDetails<C> {
    pub fn as_directional(&self) -> Option<&C::Container<DirectionalLightDescriptor>> {
        if let LightDetails::Directional(d) = self {
            Some(d)
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
    pub fn from_hybrid(hybrid: &LightDetails<HybridContainer>) -> Self {
        match hybrid {
            LightDetails::Directional(d) => LightDetails::Directional(WeakHybrid::from_hybrid(d)),
            LightDetails::Point(p) => LightDetails::Point(WeakHybrid::from_hybrid(p)),
            LightDetails::Spot(s) => LightDetails::Spot(WeakHybrid::from_hybrid(s)),
        }
    }
}

/// A depth map rendering of the scene from a light's point of view.
///
/// Used to project shadows from one light source for specific objects.
#[derive(Clone)]
pub struct ShadowMap {
    /// Last time the stage slab was bound.
    stage_slab_buffer_creation_time: Arc<AtomicUsize>,
    /// Last time the light slab was bound.
    light_slab_buffer_creation_time: Arc<AtomicUsize>,
    /// This shadow map's light transform,
    shadowmap_descriptor: Hybrid<ShadowMapDescriptor>,
    /// This shadow map's transforms.
    ///
    /// Directional and spot lights have 1, point lights
    /// have 6.
    #[allow(dead_code)]
    light_space_transforms: HybridArray<Mat4>,
    /// Bindgroup for the shadow map update shader
    update_bindgroup: ManagedBindGroup,
    atlas_textures: Vec<Hybrid<AtlasTexture>>,
    #[allow(dead_code)]
    atlas_textures_array: HybridArray<Id<AtlasTexture>>,
    update_texture: crate::texture::Texture,
    blitting_op: AtlasBlittingOperation,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    /// Create the bindgroup for the shadow map update shader.
    fn create_update_bindgroup(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        geometry_slab_buffer: &wgpu::Buffer,
        light_slab_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout: bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        geometry_slab_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        light_slab_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    /// Returns the [`Id`] of the inner [`ShadowMapDescriptor`].
    pub fn descriptor_id(&self) -> Id<ShadowMapDescriptor> {
        self.shadowmap_descriptor.id()
    }

    /// Returns a guard on the inner [`ShadowMapDescriptor`].
    ///
    /// Use this to update descriptor values before calling `ShadowMap::update`.
    pub fn descriptor_lock(&self) -> HybridWriteGuard<'_, ShadowMapDescriptor> {
        self.shadowmap_descriptor.lock()
    }

    /// Update the shadow map, rendering the given [`Renderlet`]s to the map as shadow casters.
    // TODO: pass `AnalyticalLightBundle` to `ShadowMap::update`
    pub fn update<'a>(
        &self,
        lighting: &Lighting,
        renderlets: impl IntoIterator<Item = &'a Hybrid<Renderlet>>,
    ) {
        let renderlets = renderlets.into_iter().collect::<Vec<_>>();

        let device = lighting.light_slab.device();
        let queue = lighting.light_slab.queue();
        let mut light_slab_buffer = lighting.light_slab_buffer.write().unwrap();
        let mut stage_slab_buffer = lighting.stage_slab_buffer.write().unwrap();

        let bindgroup = {
            light_slab_buffer.update_if_invalid();
            stage_slab_buffer.update_if_invalid();
            let stored_light_buffer_creation_time = self.light_slab_buffer_creation_time.swap(
                light_slab_buffer.creation_time(),
                std::sync::atomic::Ordering::Relaxed,
            );
            let stored_stage_buffer_creation_time = self.stage_slab_buffer_creation_time.swap(
                stage_slab_buffer.creation_time(),
                std::sync::atomic::Ordering::Relaxed,
            );
            let should_invalidate = light_slab_buffer.creation_time()
                > stored_light_buffer_creation_time
                || stage_slab_buffer.creation_time() > stored_stage_buffer_creation_time;
            self.update_bindgroup.get(should_invalidate, || {
                log::trace!("recreating shadow mapping bindgroup");
                Self::create_update_bindgroup(
                    device,
                    &lighting.shadow_map_update_bindgroup_layout,
                    &stage_slab_buffer,
                    &light_slab_buffer,
                )
            })
        };
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });

        for (i, atlas_texture) in self.atlas_textures.iter().enumerate() {
            // Update the lighting descriptor to point to this shadow map, which tells the
            // vertex shader which shadow map we're updating.
            lighting.lighting_descriptor.modify(|ld| {
                let id = self.shadowmap_descriptor.id();
                log::trace!("updating the shadow map {id:?} {i}");
                ld.update_shadow_map_id = id;
                ld.update_shadow_map_texture_index = i as u32;
            });
            // Sync those changes
            let _ = lighting.light_slab.commit();
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Self::LABEL,
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.update_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                render_pass.set_pipeline(&lighting.shadow_map_update_pipeline);
                render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
                let mut count = 0;
                for rlet in renderlets.iter() {
                    let id = rlet.id();
                    let rlet = rlet.get();
                    let vertex_range = 0..rlet.get_vertex_count();
                    let instance_range = id.inner()..id.inner() + 1;
                    render_pass.draw(vertex_range, instance_range);
                    count += 1;
                }
                log::trace!("rendered {count} renderlets to the shadow map");
            }
            // Then copy the depth texture to our shadow map atlas in the lighting struct
            self.blitting_op.run(
                lighting.light_slab.runtime(),
                &mut encoder,
                &self.update_texture,
                &lighting.shadow_map_atlas,
                atlas_texture,
            );
        }
        let submission = queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::wait_for(submission));
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

impl AnalyticalLightBundle<WeakContainer> {
    fn from_hybrid(light: &AnalyticalLightBundle) -> Self {
        AnalyticalLightBundle {
            light: WeakHybrid::from_hybrid(&light.light),
            light_details: LightDetails::from_hybrid(&light.light_details),
            transform: NestedTransform::from_hybrid(&light.transform),
        }
    }
}

impl AnalyticalLightBundle {
    pub fn light_space_transforms(&self, frustum: Frustum) -> Vec<Mat4> {
        let t = self.transform.get();
        let m = Mat4::from(t);
        match &self.light_details {
            LightDetails::Directional(d) => vec![{
                let (p, j) = d.get().shadow_mapping_projection_and_view(&m, frustum);
                p * j
            }],
            LightDetails::Point(_) => todo!(),
            LightDetails::Spot(_) => todo!(),
        }
    }
}

/// Manages lighting for an entire scene.
#[derive(Clone)]
pub struct Lighting {
    light_slab: SlabAllocator<WgpuRuntime>,
    light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    lighting_descriptor: Hybrid<LightingDescriptor>,
    analytical_lights: Arc<Mutex<Vec<AnalyticalLightBundle<WeakContainer>>>>,
    analytical_lights_array: Arc<Mutex<HybridArray<Id<super::Light>>>>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    shadow_map_update_pipeline: Arc<wgpu::RenderPipeline>,
    shadow_map_update_bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    shadow_map_update_blitter: AtlasBlitter,
    shadow_map_atlas: Atlas,
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
    pub fn new(
        runtime: impl AsRef<WgpuRuntime>,
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
    ) -> Self {
        let runtime = runtime.as_ref();
        let light_slab =
            SlabAllocator::new_with_label(runtime, wgpu::BufferUsages::empty(), Some("light-slab"));
        let lighting_descriptor = light_slab.new_value(LightingDescriptor::default());
        let light_slab_buffer = light_slab.commit();
        let bindgroup_layout = Self::create_bindgroup_layout(&runtime.device);

        let shadow_map_update_vertex =
            crate::linkage::shadow_mapping_vertex::linkage(&runtime.device);
        let shadow_map_update_bindgroup_layout: Arc<_> = runtime
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: ShadowMap::LABEL,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            })
            .into();
        let shadow_map_update_layout =
            runtime
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: ShadowMap::LABEL,
                    bind_group_layouts: &[&shadow_map_update_bindgroup_layout],
                    push_constant_ranges: &[],
                });
        let shadow_map_update_pipeline = runtime
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Self::LABEL,
                layout: Some(&shadow_map_update_layout),
                vertex: wgpu::VertexState {
                    module: &shadow_map_update_vertex.module,
                    entry_point: Some(shadow_map_update_vertex.entry_point),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Front),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                fragment: None,
                multiview: None,
                cache: None,
            })
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
            light_slab,
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            lighting_descriptor,
            stage_slab_buffer: Arc::new(RwLock::new(stage_slab_buffer.clone())),
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

    pub fn slab(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.light_slab
    }

    fn add_light_bundle(&self, bundle: &AnalyticalLightBundle) {
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

    /// Create a new [`AnalyticalLightBundle`] for the given [`DirectionalLightDescriptor`].
    pub fn new_directional_light(
        &self,
        directional_light: DirectionalLightDescriptor,
        nested_transform: Option<NestedTransform>,
    ) -> AnalyticalLightBundle {
        let transform = nested_transform.unwrap_or_else(|| NestedTransform::new(&self.light_slab));
        let light_inner = self.light_slab.new_value(directional_light);
        let light = self.light_slab.new_value({
            let mut light: super::Light = light_inner.id().into();
            light.transform_id = transform.global_transform_id();
            light
        });
        let light_details = LightDetails::Directional(light_inner);
        let bundle = AnalyticalLightBundle {
            light,
            light_details,
            transform,
        };

        self.add_light_bundle(&bundle);

        bundle
    }

    /// Enable shadow mapping for the given [`AnalyticalLightBundle`], creating
    /// a new [`ShadowMap`].
    pub fn new_shadow_map(
        &self,
        analytical_light_bundle: &AnalyticalLightBundle,
        size: UVec2,
        camera: &Camera,
    ) -> Result<ShadowMap, LightingError> {
        let stage_slab_buffer = self.stage_slab_buffer.read().unwrap();
        let is_point_light =
            analytical_light_bundle.light_details.style() == super::LightStyle::Point;
        let count = if is_point_light { 6 } else { 1 };
        let atlas = &self.shadow_map_atlas;
        let image = AtlasImage::new(size, crate::atlas::AtlasImageFormat::R32FLOAT);
        // UNWRAP: safe because we know there's one in here
        let atlas_textures = atlas.add_images(vec![&image; count])?;
        // Regardless of light type, we only create one depth texture.
        let update_texture =
            crate::texture::Texture::create_depth_texture(atlas.device(), size.x, size.y, 1);
        let atlas_textures_array = self
            .light_slab
            .new_array(atlas_textures.iter().map(|t| t.id()));
        let blitting_op = self.shadow_map_update_blitter.new_blitting_operation(atlas);
        let light_space_transforms = self
            .light_slab
            .new_array(analytical_light_bundle.light_space_transforms(camera.frustum()));
        let shadowmap_descriptor = self.light_slab.new_value(ShadowMapDescriptor {
            light_space_transforms_array: light_space_transforms.array(),
            atlas_textures_array: atlas_textures_array.array(),
            bias_min: 0.005,
            bias_max: 0.05,
        });
        // Set the descriptor in the light, so the shader knows to use it
        analytical_light_bundle.light.modify(|light| {
            light.shadow_map_desc_id = shadowmap_descriptor.id();
        });
        let light_slab_buffer = self.light_slab.commit();
        let update_bindgroup = ManagedBindGroup::from(ShadowMap::create_update_bindgroup(
            self.light_slab.device(),
            &self.shadow_map_update_bindgroup_layout,
            stage_slab_buffer.deref(),
            &light_slab_buffer,
        ));

        Ok(ShadowMap {
            stage_slab_buffer_creation_time: Arc::new(stage_slab_buffer.creation_time().into()),
            light_slab_buffer_creation_time: Arc::new(light_slab_buffer.creation_time().into()),
            shadowmap_descriptor,
            light_space_transforms,
            update_bindgroup,
            atlas_textures,
            atlas_textures_array,
            update_texture,
            blitting_op,
        })
    }

    pub fn upkeep(&self) {
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
    use crabslab::Slab;
    use glam::{Mat4, Vec2, Vec3, Vec4};
    use image::Luma;

    use crate::{
        camera::Camera,
        light::{ShadowCalculation, ShadowMappingVertexInfo},
        math::{ConstTexture, CpuTexture2dArray, IsVector},
        pbr::PbrConfig,
        stage::RenderletPbrVertexInfo,
        texture::DepthTexture,
    };

    use super::*;

    #[test]
    fn shadow_mapping_sanity() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx.new_stage().with_lighting(true);

        // let hdr_path =
        //     std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR")).join("img/hdr/night.hdr");
        // let hdr_img = AtlasImage::from_hdr_path(hdr_path).unwrap();

        let camera = stage.new_value(Camera::default());
        // let skybox = Skybox::new(&ctx, hdr_img, camera.id());
        // stage.set_skybox(skybox);
        log::info!("camera_id: {:?}", camera.id());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_sanity.gltf"),
                camera.id(),
            )
            .unwrap();
        let gltf_camera = doc.cameras.first().unwrap();
        let mut c = gltf_camera.get_camera();
        c.set_projection(crate::camera::perspective(w, h));
        camera.set(c);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        // Rendering the scene without shadows as a sanity check
        img_diff::save("shadows/shadow_mapping_sanity/scene_before.png", img);

        let gltf_light = doc.lights.first().unwrap();
        assert_eq!(
            gltf_light.light.get().transform_id,
            gltf_light.transform.global_transform_id(),
            "light's global transform id is different from its transform_id"
        );

        // 1. Get the light transform
        // 2. Create a camera to view the scene from the light's POV
        // 3. Render the scene to an image to sanity check the setup
        // 4. Extract the depth texture as a a sanity check
        // 5. Extract ComparisonInfo for each vertex
        let (light_projection, light_view) = {
            let parent_light_transform = Mat4::from(gltf_light.transform.get_global_transform());
            let camera = camera.get();
            match &gltf_light.light_details {
                LightDetails::Directional(d) => {
                    let directional_light = d.get();
                    let (projection, view) = directional_light.shadow_mapping_projection_and_view(
                        &parent_light_transform,
                        camera.frustum(),
                    );
                    (projection, view)
                }
                _ => panic!("wasn't supposed to be anything but directional"),
                // LightStyle::Spot => {
                //     let spot_light = slab.read_unchecked(Id::<SpotLight>::new(light.index));
                //     let projection =
                //         Mat4::perspective_rh(spot_light.outer_cutoff, 1.0, 0.01, 100.0);
                //     let direction = parent_light_transform.transform_vector3(spot_light.direction);
                //     let position = parent_light_transform.transform_point3(spot_light.position);
                //     let view = Mat4::look_to_rh(position, direction, Vec3::Y);
                //     projection * view
                // }
                // LightStyle::Point => Mat4::default(),
            }
        };
        let light_transform = light_projection * light_view;

        let mut renderlet_vertex_info = vec![];
        {
            let light_camera_struct = Camera::new(light_projection, light_view);
            let light_camera = stage.new_value(light_camera_struct);
            assert_eq!(light_transform, light_camera.get().view_projection());

            for renderlet in doc.renderlets_iter() {
                renderlet.modify(|r| {
                    r.camera_id = light_camera.id();
                });
            }

            let frame = ctx.get_next_frame().unwrap();
            stage.render(&frame.view());
            let img = frame.read_image().unwrap();
            frame.present();
            img_diff::save("shadows/shadow_mapping_sanity/scene_light_pov.png", img);

            let mut depth_img = stage.get_depth_texture().read_image().unwrap();
            // Normalize the value
            img_diff::normalize_gray_img(&mut depth_img);
            img_diff::save(
                "shadows/shadow_mapping_sanity/light_pov_depth.png",
                depth_img,
            );

            let slab = futures_lite::future::block_on(stage.read(..)).unwrap();
            for hybrid in doc.renderlets_iter() {
                let renderlet = hybrid.get();
                for vertex_index in 0..renderlet.get_vertex_count() {
                    let mut info = RenderletPbrVertexInfo::default();
                    crate::stage::renderlet_vertex(
                        hybrid.id(),
                        vertex_index,
                        &slab,
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut Default::default(),
                        &mut info,
                    );
                    renderlet_vertex_info.push(info);
                }
            }
            // Make sure to reset the renderlet's cameras and then update the
            // stage, which is easiest by rendering a frame...
            doc.renderlets_iter().for_each(|renderlet| {
                renderlet.modify(|r| {
                    r.camera_id = camera.id();
                });
            });
            let frame = ctx.get_next_frame().unwrap();
            stage.render(&frame.view());
            frame.present();
        }

        let shadows = stage
            .lighting()
            .new_shadow_map(gltf_light, UVec2::new(w as u32, h as u32), &camera.get())
            .unwrap();

        crate::test::capture_gpu_frame(
            &ctx,
            "shadows/shadow_mapping_sanity/shadow_map_update.gputrace",
            || shadows.update(stage.lighting(), doc.renderlets_iter()),
        );

        let geometry_slab = futures_lite::future::block_on(stage.read(..)).unwrap();
        let light_slab = futures_lite::future::block_on(stage.lighting().slab().read(..)).unwrap();
        {
            // Inspect the blitting vertex
            #[derive(Default, Debug)]
            struct AtlasVertexOutput {
                out_uv: Vec2,
                out_clip_pos: Vec4,
            }
            let mut output = vec![];
            for i in 0..6 {
                let mut out = AtlasVertexOutput::default();
                crate::atlas::atlas_blit_vertex(
                    i,
                    shadows.blitting_op.desc.id(),
                    &light_slab,
                    &mut out.out_uv,
                    &mut out.out_clip_pos,
                );
                output.push(out);
            }
            log::info!(
                "clip_pos: {:#?}",
                output
                    .into_iter()
                    .map(|out| out.out_clip_pos)
                    .collect::<Vec<_>>()
            );
        }

        let mut shadow_vertex_info = vec![];
        for hybrid in doc.renderlets_iter() {
            let renderlet = hybrid.get();
            for vertex_index in 0..renderlet.get_vertex_count() {
                let mut info = ShadowMappingVertexInfo::default();
                crate::light::shadow_mapping_vertex(
                    hybrid.id(),
                    vertex_index,
                    &geometry_slab,
                    &light_slab,
                    &mut Default::default(),
                    &mut info,
                );
                shadow_vertex_info.push(info);
            }
        }

        {
            // Ensure the state of the "update texture", which receives the depth of the scene on update
            let shadow_map_update_texture =
                DepthTexture::try_new_from(&ctx, shadows.update_texture.clone()).unwrap();
            let mut shadow_map_update_img = shadow_map_update_texture.read_image().unwrap();
            img_diff::normalize_gray_img(&mut shadow_map_update_img);
            img_diff::save(
                "shadows/shadow_mapping_sanity/shadows_update_texture.png",
                shadow_map_update_img,
            );
        }

        let shadow_depth_buffer = stage.lighting().shadow_map_atlas.atlas_img_buffer(&ctx, 0);
        let shadow_depth_img = shadow_depth_buffer
            .into_image::<f32, Luma<f32>>(ctx.get_device())
            .unwrap();
        let shadow_depth_img = shadow_depth_img.into_luma8();
        let mut depth_img = shadow_depth_img.clone();
        img_diff::normalize_gray_img(&mut depth_img);
        img_diff::save("shadows/shadow_mapping_sanity/depth.png", depth_img);

        assert_eq!(renderlet_vertex_info.len(), shadow_vertex_info.len());

        // Get the green sphere to use for testing what should be in shadow
        let sphere_001 = doc
            .nodes
            .iter()
            .find(|n| n.name.as_deref() == Some("Sphere.001"))
            .unwrap();
        let mut found_vertex_output_for_sphere_001 = None;

        for (pbr_info, shadow_info) in renderlet_vertex_info
            .into_iter()
            .zip(shadow_vertex_info.into_iter())
        {
            if found_vertex_output_for_sphere_001.is_none() {
                let distance_to_sphere_origin = shadow_info
                    .world_pos
                    .distance(sphere_001.global_transform().translation);
                if distance_to_sphere_origin < 0.1 {
                    // Save the point for further renders
                    log::info!("found it: distance={distance_to_sphere_origin} {shadow_info:#?}");
                    found_vertex_output_for_sphere_001 = Some(pbr_info);
                }
            }
        }

        let found_vertex_output_for_sphere_001 = found_vertex_output_for_sphere_001.unwrap();

        fn luma_8_to_vec4(Luma([a]): &Luma<u8>) -> Vec4 {
            Vec4::splat(*a as f32 / 255.0)
        }

        let top_of_green_sphere_pos = {
            let sphere_001 = doc
                .nodes
                .iter()
                .find(|n| n.name.as_deref() == Some("Sphere.001"))
                .unwrap();
            // Fragment position in world space
            sphere_001.global_transform().translation
        };

        {
            // Ensure the light slab has the correct light transform
            let light_slab =
                futures_lite::future::block_on(stage.lighting().slab().read(..)).unwrap();
            let light_space_transform = light_slab.read(shadows.light_space_transforms.get_id(0));

            let dx = light_transform
                .x_axis
                .distance(light_space_transform.x_axis);
            let dy = light_transform
                .y_axis
                .distance(light_space_transform.y_axis);
            let dz = light_transform
                .z_axis
                .distance(light_space_transform.z_axis);
            let dw = light_transform
                .w_axis
                .distance(light_space_transform.w_axis);
            if [dx, dy, dz, dw].iter().any(|d| {
                let log = d.log10();
                log::info!("log: {log}");
                log > -7.0
            }) {
                pretty_assertions::assert_eq!(
                    light_transform,
                    light_space_transform,
                    "light space transforms are not equal"
                );
            }

            let frag_pos_in_light_space =
                light_space_transform.project_point3(top_of_green_sphere_pos);
            log::info!("frag_pos_in_light_space: {frag_pos_in_light_space}");
        }

        {
            // Run the fragment shader on that one point
            let shadow_map = CpuTexture2dArray::from_images(Some(shadow_depth_img), luma_8_to_vec4);
            let geometry_slab = futures_lite::future::block_on(stage.read(..)).unwrap();
            let light_slab =
                futures_lite::future::block_on(stage.lighting().slab().read(..)).unwrap();
            let vertex_info = found_vertex_output_for_sphere_001;
            {
                // Check that this point actually _is in shadow_
                let light = gltf_light.light.get();
                let transform = light_slab.read(light.transform_id);
                let transform = Mat4::from(transform);
                let DirectionalLightDescriptor { direction, .. } =
                    light_slab.read(light.into_directional_id());
                let is_in_shadow = ShadowCalculation::new(
                    &light_slab,
                    light,
                    vertex_info.out_pos,
                    vertex_info.out_norm.alt_norm_or_zero(),
                    {
                        let direction = transform.transform_vector3(direction);
                        -direction.alt_norm_or_zero()
                    },
                )
                .run(&shadow_map, &());
                assert_eq!(1.0, is_in_shadow, "point should be in shadow");
            }
            let mut output = Vec4::ZERO;
            crate::pbr::fragment_impl(
                &ConstTexture::new(Vec4::ONE),
                &(),
                &ConstTexture::new(Vec4::ONE),
                &(),
                &ConstTexture::new(Vec4::ONE),
                &(),
                &ConstTexture::new(Vec4::ONE),
                &(),
                &shadow_map,
                &(),
                &geometry_slab,
                &light_slab,
                vertex_info.renderlet_id,
                vertex_info.out_color,
                vertex_info.out_uv0,
                vertex_info.out_uv1,
                vertex_info.out_norm,
                vertex_info.out_tangent,
                vertex_info.out_bitangent,
                vertex_info.out_pos,
                &mut output,
            );
            log::info!("color: {output}");
        }

        let _ = stage.lighting().slab().commit();

        // Now do the rendering *with the shadow map* to see if it works.
        let frame = ctx.get_next_frame().unwrap();
        crate::test::capture_gpu_frame(
            &ctx,
            "shadows/shadow_mapping_sanity/render.gputrace",
            || stage.render(&frame.view()),
        );

        let img = frame.read_image().unwrap();
        frame.present();
        img_diff::save("shadows/shadow_mapping_sanity/stage_render.png", img);
    }

    #[test]
    fn light_transform_depth_sanity() {
        let camera = Camera::default_perspective(800.0, 800.0);
        let directional_light = DirectionalLightDescriptor {
            direction: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        };
        let (light_projection, light_view) =
            directional_light.shadow_mapping_projection_and_view(&Mat4::IDENTITY, camera.frustum());
        let light_camera = Camera::new(light_projection, light_view);
        log::info!("z_near: {}", light_camera.z_near());
        log::info!("z_far: {}", light_camera.z_far());
        log::info!("depth: {}", light_camera.depth());
        log::info!("near_plane: {}", light_camera.near_plane());
        log::info!("far_plane: {}", light_camera.far_plane());
        log::info!("position: {}", light_camera.position());
    }
}
