//! Shadow mapping.

use core::{ops::Deref, sync::atomic::AtomicUsize};
use std::sync::Arc;

use craballoc::{
    prelude::Hybrid,
    value::{HybridArray, HybridWriteGuard},
};
use crabslab::Id;
use glam::{Mat4, UVec2};

use crate::{
    atlas::{AtlasBlittingOperation, AtlasImage, AtlasTexture},
    bindgroup::ManagedBindGroup,
    stage::Renderlet,
};

use super::{AnalyticalLightBundle, Lighting, LightingError, ShadowMapDescriptor};

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
                bind_group_layouts: &[bindgroup_layout],
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
    ) -> Result<(), LightingError> {
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
        for (i, atlas_texture) in self.atlas_textures.iter().enumerate() {
            let mut encoder = device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });

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
            let label = format!("{}-view-{i}", Self::LABEL.unwrap());
            let view = self
                .update_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&label),
                    base_array_layer: i as u32,
                    array_layer_count: Some(1),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    ..Default::default()
                });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Self::LABEL,
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &view,
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
                i as u32,
                &lighting.shadow_map_atlas,
                atlas_texture,
            )?;
            let submission = queue.submit(Some(encoder.finish()));
            device.poll(wgpu::Maintain::wait_for(submission));
        }
        Ok(())
    }

    /// Enable shadow mapping for the given [`AnalyticalLightBundle`], creating
    /// a new [`ShadowMap`].
    pub fn new(
        lighting: &Lighting,
        analytical_light_bundle: &AnalyticalLightBundle,
        // Size of the shadow map
        size: UVec2,
        // Distance to the shadow map frustum's near plane
        z_near: f32,
        // Distance to the shadow map frustum's far plane
        z_far: f32,
    ) -> Result<Self, LightingError> {
        let stage_slab_buffer = lighting.geometry_slab_buffer.read().unwrap();
        let is_point_light =
            analytical_light_bundle.light_details.style() == super::LightStyle::Point;
        let count = if is_point_light { 6 } else { 1 };
        let atlas = &lighting.shadow_map_atlas;
        let image = AtlasImage::new(size, crate::atlas::AtlasImageFormat::R32FLOAT);
        // UNWRAP: safe because we know there's one in here
        let atlas_textures = atlas.add_images(vec![&image; count])?;
        let atlas_len = atlas.len();
        // Regardless of light type, we only create one depth texture,
        // but that texture may be of layer 1 or 6
        let label = format!("shadow-map-{atlas_len}");
        let update_texture = crate::texture::Texture::create_depth_texture_for_shadow_map(
            atlas.device(),
            size.x,
            size.y,
            1,
            Some(&label),
            is_point_light,
        );
        let atlas_textures_array = lighting
            .light_slab
            .new_array(atlas_textures.iter().map(|t| t.id()));
        let blitting_op = lighting
            .shadow_map_update_blitter
            .new_blitting_operation(atlas, if is_point_light { 6 } else { 1 });
        let light_space_transforms = lighting
            .light_slab
            .new_array(analytical_light_bundle.light_space_transforms(z_near, z_far));
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

#[cfg(test)]
mod test {
    use image::Luma;

    use crate::{camera::Camera, texture::DepthTexture};

    use super::super::*;

    #[test]
    fn shadow_mapping_just_cuboid() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_msaa_sample_count(4);

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
                    .join("shadow_mapping_only_cuboid.gltf"),
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
        img_diff::assert_img_eq("shadows/shadow_mapping_just_cuboid/scene_before.png", img);

        let gltf_light = doc.lights.first().unwrap();
        let shadow_map = stage
            .lighting()
            .new_shadow_map(gltf_light, UVec2::splat(256), 0.0, 20.0)
            .unwrap();
        shadow_map.shadowmap_descriptor.modify(|desc| {
            desc.bias_min = 0.00008;
            desc.bias_max = 0.00008;
        });
        shadow_map
            .update(stage.lighting(), doc.renderlets_iter())
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("shadows/shadow_mapping_just_cuboid/scene_after.png", img);
        frame.present();
    }

    #[test]
    fn shadow_mapping_just_cuboid_red_and_blue() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_msaa_sample_count(4);

        let camera = stage.new_value(Camera::default());
        log::info!("camera_id: {:?}", camera.id());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_only_cuboid_red_and_blue.gltf"),
                camera.id(),
            )
            .unwrap();
        let gltf_camera = doc.cameras.first().unwrap();
        let mut c = gltf_camera.get_camera();
        c.set_projection(crate::camera::perspective(w, h));
        camera.set(c);

        let gltf_light_a = doc.lights.first().unwrap();
        let gltf_light_b = doc.lights.get(1).unwrap();
        let shadow_map_a = stage
            .lighting()
            .new_shadow_map(gltf_light_a, UVec2::splat(256), 0.0, 20.0)
            .unwrap();
        shadow_map_a.shadowmap_descriptor.modify(|desc| {
            desc.bias_min = 0.00008;
            desc.bias_max = 0.00008;
        });
        shadow_map_a
            .update(stage.lighting(), doc.renderlets_iter())
            .unwrap();
        let shadow_map_b = stage
            .lighting()
            .new_shadow_map(gltf_light_b, UVec2::splat(256), 0.0, 20.0)
            .unwrap();
        shadow_map_b.shadowmap_descriptor.modify(|desc| {
            desc.bias_min = 0.00008;
            desc.bias_max = 0.00008;
        });
        shadow_map_b
            .update(stage.lighting(), doc.renderlets_iter())
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();

        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq(
            "shadows/shadow_mapping_just_cuboid/red_and_blue/frame.png",
            img,
        );
        frame.present();
    }

    #[test]
    fn shadow_mapping_sanity() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx.new_stage().with_lighting(true);

        let camera = stage.new_value(Camera::default());

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
        img_diff::assert_img_eq("shadows/shadow_mapping_sanity/scene_before.png", img);

        let gltf_light = doc.lights.first().unwrap();
        assert_eq!(
            gltf_light.light.get().transform_id,
            gltf_light.transform.global_transform_id(),
            "light's global transform id is different from its transform_id"
        );

        let shadows = stage
            .lighting()
            .new_shadow_map(gltf_light, UVec2::new(w as u32, h as u32), 0.0, 20.0)
            .unwrap();
        shadows
            .update(stage.lighting(), doc.renderlets_iter())
            .unwrap();

        {
            // Ensure the state of the "update texture", which receives the depth of the scene on update
            let shadow_map_update_texture =
                DepthTexture::try_new_from(&ctx, shadows.update_texture.clone()).unwrap();
            let mut shadow_map_update_img = shadow_map_update_texture.read_image().unwrap();
            img_diff::normalize_gray_img(&mut shadow_map_update_img);
            img_diff::assert_img_eq(
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
        img_diff::assert_img_eq("shadows/shadow_mapping_sanity/depth.png", depth_img);

        // Now do the rendering *with the shadow map* to see if it works.
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());

        let img = frame.read_image().unwrap();
        frame.present();
        img_diff::assert_img_eq("shadows/shadow_mapping_sanity/stage_render.png", img);
    }

    #[test]
    fn shadow_mapping_spot_lights() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_msaa_sample_count(4);

        let camera = stage.new_value(Camera::default());
        log::info!("camera_id: {:?}", camera.id());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_spots.glb"),
                camera.id(),
            )
            .unwrap();
        let gltf_camera = doc.cameras.first().unwrap();
        let mut c = gltf_camera.get_camera();
        c.set_projection(crate::camera::perspective(w, h));
        camera.set(c);

        let mut shadow_maps = vec![];
        let z_near = 0.1;
        let z_far = 100.0;
        for (i, light_bundle) in doc.lights.iter().enumerate() {
            {
                let desc = light_bundle.light_details.as_spot().unwrap().get();
                let (p, v) = desc.shadow_mapping_projection_and_view(
                    &light_bundle.transform.get_global_transform().into(),
                    z_near,
                    z_far,
                );
                camera.set(Camera::new(p, v));
                let frame = ctx.get_next_frame().unwrap();
                stage.render(&frame.view());
                let img = frame.read_image().unwrap();
                img_diff::assert_img_eq(
                    &format!("shadows/shadow_mapping_spots/light_pov_{i}.png"),
                    img,
                );
                frame.present();
            }
            let shadow = stage
                .lighting()
                .new_shadow_map(light_bundle, UVec2::splat(256), z_near, z_far)
                .unwrap();
            shadow.shadowmap_descriptor.modify(|desc| {
                desc.bias_min = f32::EPSILON;
                desc.bias_max = f32::EPSILON;
            });

            shadow
                .update(stage.lighting(), doc.renderlets_iter())
                .unwrap();
            shadow_maps.push(shadow);
        }
        camera.set(c);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("shadows/shadow_mapping_spots/frame.png", img);
        frame.present();
    }

    #[test]
    fn shadow_mapping_point_lights() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_background_color(Vec3::splat(0.05087).extend(1.0))
            .with_msaa_sample_count(4);
        let camera = stage.new_value(Camera::default());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_points.glb"),
                camera.id(),
            )
            .unwrap();
        let gltf_camera = doc.cameras.first().unwrap();
        let mut c = gltf_camera.get_camera();
        c.set_projection(crate::camera::perspective(w, h));

        camera.set(c);

        let mut shadows = vec![];
        let z_near = 0.1;
        let z_far = 100.0;
        for (i, light_bundle) in doc.lights.iter().enumerate() {
            {
                let desc = light_bundle.light_details.as_point().unwrap().get();
                let (p, vs) = desc.shadow_mapping_projection_and_view_matrices(
                    &light_bundle.transform.get_global_transform().into(),
                    z_near,
                    z_far,
                );
                for (j, v) in vs.into_iter().enumerate() {
                    camera.set(Camera::new(p, v));
                    let frame = ctx.get_next_frame().unwrap();
                    stage.render(&frame.view());
                    let img = frame.read_image().unwrap();
                    img_diff::save(
                        &format!("shadows/shadow_mapping_points/light_{i}_pov_{j}.png"),
                        img,
                    );
                    frame.present();
                }
            }
            let shadow = stage
                .lighting()
                .new_shadow_map(light_bundle, UVec2::splat(256), z_near, z_far)
                .unwrap();
            shadow.shadowmap_descriptor.modify(|desc| {
                desc.bias_min = f32::EPSILON;
                desc.bias_max = f32::EPSILON;
            });
            shadow
                .update(stage.lighting(), doc.renderlets_iter())
                .unwrap();
            shadows.push(shadow);
        }

        let frame = ctx.get_next_frame().unwrap();
        crate::test::capture_gpu_frame(
            &ctx,
            "shadows/shadow_mapping_points/frame.gputrace",
            || stage.render(&frame.view()),
        );
        let img = frame.read_image().unwrap();
        img_diff::save("shadows/shadow_mapping_points/frame.png", img);
        frame.present();
    }
}
