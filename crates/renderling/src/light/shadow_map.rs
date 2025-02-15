//! Shadow mapping.

use core::{ops::Deref, sync::atomic::AtomicUsize};
use std::sync::{Arc, Mutex, RwLock};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
    value::{
        HybridArray, HybridContainer, HybridWriteGuard, IsContainer, WeakContainer, WeakHybrid,
    },
};
use crabslab::{Id, SlabItem};
use glam::{Mat4, UVec2};
use snafu::prelude::*;

use crate::{
    atlas::{Atlas, AtlasBlitter, AtlasBlittingOperation, AtlasError, AtlasImage, AtlasTexture},
    bindgroup::ManagedBindGroup,
    stage::{NestedTransform, Renderlet},
};

use super::{
    AnalyticalLightBundle, DirectionalLightDescriptor, Light, LightStyle, Lighting,
    LightingDescriptor, LightingError, PointLightDescriptor, ShadowMapDescriptor,
    SpotLightDescriptor,
};

/// A depth map rendering of the scene from a light's point of view.
///
/// Used to project shadows from one light source for specific objects.
#[derive(Clone)]
pub struct ShadowMap {
    /// Last time the stage slab was bound.
    pub(crate) stage_slab_buffer_creation_time: Arc<AtomicUsize>,
    /// Last time the light slab was bound.
    pub(crate) light_slab_buffer_creation_time: Arc<AtomicUsize>,
    /// This shadow map's light transform,
    pub(crate) shadowmap_descriptor: Hybrid<ShadowMapDescriptor>,
    /// This shadow map's transforms.
    ///
    /// Directional and spot lights have 1, point lights
    /// have 6.
    pub(crate) light_space_transforms: HybridArray<Mat4>,
    /// Bindgroup for the shadow map update shader
    pub(crate) update_bindgroup: ManagedBindGroup,
    pub(crate) atlas_textures: Vec<Hybrid<AtlasTexture>>,
    pub(crate) atlas_textures_array: HybridArray<Id<AtlasTexture>>,
    pub(crate) update_texture: crate::texture::Texture,
    pub(crate) blitting_op: AtlasBlittingOperation,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    pub fn create_update_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
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
    }

    pub fn create_update_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shadow_map_update_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: ShadowMap::LABEL,
                bind_group_layouts: &[&bindgroup_layout],
                push_constant_ranges: &[],
            });
        let shadow_map_update_vertex = crate::linkage::shadow_mapping_vertex::linkage(device);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
    }

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
        if lighting.geometry_slab.has_queued_updates() {
            lighting.geometry_slab.commit();
        }
        let renderlets = renderlets.into_iter().collect::<Vec<_>>();

        let device = lighting.light_slab.device();
        let queue = lighting.light_slab.queue();
        let mut light_slab_buffer = lighting.light_slab_buffer.write().unwrap();
        let mut stage_slab_buffer = lighting.geometry_slab_buffer.write().unwrap();

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

    /// Enable shadow mapping for the given [`AnalyticalLightBundle`], creating
    /// a new [`ShadowMap`].
    pub fn new(
        lighting: &Lighting,
        analytical_light_bundle: &AnalyticalLightBundle,
        // Size of the shadow map
        size: UVec2,
        // Diameter of the area to cover with the shadow map, in world coordinates
        depth: f32,
    ) -> Result<Self, LightingError> {
        let stage_slab_buffer = lighting.geometry_slab_buffer.read().unwrap();
        let is_point_light =
            analytical_light_bundle.light_details.style() == super::LightStyle::Point;
        let count = if is_point_light { 6 } else { 1 };
        let atlas = &lighting.shadow_map_atlas;
        let image = AtlasImage::new(size, crate::atlas::AtlasImageFormat::R32FLOAT);
        // UNWRAP: safe because we know there's one in here
        let atlas_textures = atlas.add_images(vec![&image; count])?;
        let atlas_textures_len = atlas_textures.len();
        // Regardless of light type, we only create one depth texture.
        let label = format!("shadow-map-{atlas_textures_len}");
        let update_texture = crate::texture::Texture::create_depth_texture(
            atlas.device(),
            size.x,
            size.y,
            1,
            Some(&label),
        );
        let atlas_textures_array = lighting
            .light_slab
            .new_array(atlas_textures.iter().map(|t| t.id()));
        let blitting_op = lighting
            .shadow_map_update_blitter
            .new_blitting_operation(atlas);
        let light_space_transforms = lighting
            .light_slab
            .new_array(analytical_light_bundle.light_space_transforms(depth));
        let shadowmap_descriptor = lighting.light_slab.new_value(ShadowMapDescriptor {
            light_space_transforms_array: light_space_transforms.array(),
            atlas_textures_array: atlas_textures_array.array(),
            bias_min: 0.0005,
            bias_max: 0.005,
            pcf_samples: 4,
        });
        // Set the descriptor in the light, so the shader knows to use it
        analytical_light_bundle.light.modify(|light| {
            light.shadow_map_desc_id = shadowmap_descriptor.id();
        });
        let light_slab_buffer = lighting.light_slab.commit();
        let update_bindgroup = ManagedBindGroup::from(ShadowMap::create_update_bindgroup(
            lighting.light_slab.device(),
            &lighting.shadow_map_update_bindgroup_layout,
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
}
