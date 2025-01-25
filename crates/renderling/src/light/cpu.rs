//! CPU-only lighting and shadows.

use std::sync::{Arc, RwLock};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
};
use crabslab::Id;
use glam::Mat4;

use crate::{bindgroup::ManagedBindGroup, stage::Renderlet};

use super::{DirectionalLight, LightingDescriptor, PointLight, SpotLight};

/// A wrapper around all types of analytical lighting.
#[derive(Clone, Debug)]
pub enum LightDetails {
    Directional(Hybrid<DirectionalLight>),
    Point(Hybrid<PointLight>),
    Spot(Hybrid<SpotLight>),
}

impl LightDetails {
    pub fn as_directional(&self) -> Option<&Hybrid<DirectionalLight>> {
        if let LightDetails::Directional(d) = self {
            Some(d)
        } else {
            None
        }
    }
}

/// A depth map rendering of the scene from a light's point of view.
///
/// Used to project shadows from one light source for specific objects.
///
/// Based on the
/// [shadow mapping article at learnopengl](https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping).
///
/// Clones of this type all point to the same underlying data.
// TODO: Separate pipeline and bindgroup layout from ShadowMap
// TODO: Ensure that Lighting doesn't need ShadowMap at creation,
// it should instead only reference the light slab.
// ShadowMap
// |_Lighting
// |_Stage
#[derive(Clone)]
pub struct ShadowMap {
    /// A depth texture used to store the scene from the light's POV.
    // TODO: Use an Atlas for shadow maps.
    texture: crate::texture::Texture,
    light_slab: SlabAllocator<WgpuRuntime>,
    stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    /// This shadow map's light transform
    light_transform: Hybrid<Mat4>,
    /// A clone of the [`Lighting`] manager's descriptor, which
    /// gets written to when updating this shadow map.
    lighting_descriptor: Hybrid<LightingDescriptor>,
    pipeline: Arc<wgpu::RenderPipeline>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup: ManagedBindGroup,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    pub fn create_shadow_map_texture(
        device: &wgpu::Device,
        size: wgpu::Extent3d,
    ) -> crate::texture::Texture {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Self::LABEL,
            size,
            mip_level_count: 1,
            sample_count: 1,
            // TODO: what about point lights? Does this need to be D3 then?
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        crate::texture::Texture::from_wgpu_tex(device, tex, None, None)
    }

    /// Create a new [`ShadowMap`] for a single light source.
    // TODO: ShadowMap::new should take a light source instead of
    // a projection+view matrix.
    pub fn new(
        // Required for the shadow map shader to read the light transform
        lighting_manager: &Lighting,
        light_transform: Mat4,
        size: wgpu::Extent3d,
        // Required for the shadow map shader to access geometry
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
    ) -> Self {
        let light_slab = lighting_manager.light_slab.clone();
        let device = &light_slab.device();
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Self::LABEL,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let vertex = crate::linkage::shadow_mapping_vertex::linkage(device);

        let light_transform = lighting_manager.light_slab.new_value(light_transform);
        let light_slab_buffer = lighting_manager.light_slab.upkeep();
        let bindgroup = ManagedBindGroup::new(Self::create_bindgroup(
            device,
            &bindgroup_layout,
            stage_slab_buffer,
            &light_slab_buffer,
        ));

        ShadowMap {
            light_slab: light_slab.clone(),
            stage_slab_buffer: Arc::new(RwLock::new(stage_slab_buffer.clone())),
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            texture: Self::create_shadow_map_texture(device, size),
            lighting_descriptor: lighting_manager.lighting_descriptor.clone(),
            light_transform,
            bindgroup_layout: bindgroup_layout.into(),
            bindgroup,
            pipeline: device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Self::LABEL,
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &vertex.module,
                        entry_point: Some(vertex.entry_point),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
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
                .into(),
        }
    }

    pub fn size(&self) -> wgpu::Extent3d {
        self.texture.texture.size()
    }

    fn create_bindgroup(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        slab_buffer: &wgpu::Buffer,
        desc_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout: bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(desc_buffer.as_entire_buffer_binding()),
                },
            ],
        })
    }

    /// Update the shadow map, rendering the given [`Renderlet`]s to the map as shadow casters.
    // TODO: Add a `light_source: Option<_>` parameter to `ShadowMap::update`.
    // Or something similar that updates the light source's "light space transform".
    pub fn update<'a>(&self, renderlets: impl IntoIterator<Item = &'a Hybrid<Renderlet>>) {
        // Update the lighting descriptor to point to this shadow map, which tells the
        // vertex shader which shadow map we're updating.
        self.lighting_descriptor.modify(|ld| {
            ld.shadow_map_light_transform = self.light_transform.id();
        });
        let _ = self.light_slab.upkeep();

        let device = &self.light_slab.device();
        let queue = &self.light_slab.queue();
        let mut light_slab_buffer = self.light_slab_buffer.write().unwrap();
        let mut stage_slab_buffer = self.stage_slab_buffer.write().unwrap();

        let bindgroup = {
            let should_invalidate_light_slab = light_slab_buffer.synchronize();
            let should_invalidate_stage_slab = stage_slab_buffer.synchronize();
            let should_invalidate = should_invalidate_light_slab || should_invalidate_stage_slab;
            self.bindgroup.get(should_invalidate, || {
                Self::create_bindgroup(
                    device,
                    &self.bindgroup_layout,
                    &stage_slab_buffer,
                    &light_slab_buffer,
                )
            })
        };
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Self::LABEL,
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
            for rlet in renderlets {
                let id = rlet.id();
                let rlet = rlet.get();
                let vertex_range = 0..rlet.get_vertex_count();
                let instance_range = id.inner()..id.inner() + 1;
                render_pass.draw(vertex_range, instance_range);
            }
        }
        let submission = queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::wait_for(submission));
    }
}

/// Manages lighting for an entire scene.
#[derive(Clone)]
pub struct Lighting {
    light_slab: SlabAllocator<WgpuRuntime>,
    light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    lighting_descriptor: Hybrid<LightingDescriptor>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
}

impl Lighting {
    const LABEL: Option<&str> = Some("lighting");

    pub fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // shadow map texture sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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
    pub fn get_bindgroup(
        &self,
        shadow_map_depth_texture: &crate::texture::Texture,
    ) -> wgpu::BindGroup {
        let mut light_slab_buffer = self.light_slab_buffer.write().unwrap();
        let _should_invalidate = light_slab_buffer.synchronize();

        Self::create_bindgroup(
            self.light_slab.device(),
            &self.bindgroup_layout,
            &light_slab_buffer,
            shadow_map_depth_texture,
        )
    }

    /// Create a new [`Lighting`] manager.
    pub fn new(
        runtime: impl AsRef<WgpuRuntime>,
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
    ) -> Self {
        let runtime = runtime.as_ref();
        let light_slab = SlabAllocator::new(runtime, wgpu::BufferUsages::empty());
        let _lighting_descriptor = light_slab.new_value(LightingDescriptor {
            shadow_map_light_transform: Id::NONE,
        });
        let light_slab_buffer = light_slab.upkeep();
        let bindgroup_layout = Self::create_bindgroup_layout(&runtime.device);
        Self {
            light_slab,
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            lighting_descriptor: _lighting_descriptor,
            stage_slab_buffer: Arc::new(RwLock::new(stage_slab_buffer.clone())),
            bindgroup_layout: bindgroup_layout.into(),
        }
    }

    pub fn new_shadow_map(&self, light_transform: Mat4, size: wgpu::Extent3d) -> ShadowMap {
        let stage_slab_buffer = self.stage_slab_buffer.read().unwrap();
        ShadowMap::new(self, light_transform, size, &stage_slab_buffer)
    }
}

#[cfg(test)]
mod test {
    use crabslab::Slab;
    use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

    use crate::{
        atlas::AtlasImage,
        camera::Camera,
        light::{LightStyle, ShadowMappingVertexInfo},
        prelude::Transform,
        skybox::Skybox,
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
        let gltf_light = doc.lights.first().unwrap();
        log::info!("gltf_light: {gltf_light:#?}");
        stage.set_lights([gltf_light.light.id()]);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        // Rendering the scene without shadows as a sanity check
        img_diff::save("shadows/shadow_mapping_sanity_scene_before.png", img);

        assert_eq!(
            gltf_light.light.get().transform_id,
            gltf_light.node_transform.global_transform_id(),
            "light's global transform id is different from its transform_id"
        );

        // 1. Get the light transform
        // 2. Create a camera to view the scene from the light's POV
        // 3. Render the scene to an image to sanity check the setup
        // 4. Extract the depth texture as a a sanity check
        // 5. Extract ComparisonInfo for each vertex
        let (light_projection, light_view) = {
            let parent_light_transform =
                Mat4::from(gltf_light.node_transform.get_global_transform());
            let camera = camera.get();
            match &gltf_light.details {
                LightDetails::Directional(d) => {
                    let directional_light = d.get();
                    let (projection, view) = directional_light
                        .shadow_mapping_projection_and_view(&parent_light_transform, &camera);
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

            img_diff::save("shadows/shadow_mapping_sanity_light_pov.png", img);

            let mut depth_img = stage.get_depth_texture().read_image().unwrap();
            // Normalize the value
            img_diff::normalize_gray_img(&mut depth_img);
            img_diff::save(
                "shadows/shadow_mapping_sanity_light_pov_depth.png",
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

        let (lighting, shadows) = {
            let stage_slab_buffer = stage.stage_slab_buffer.read().unwrap();
            let lighting = Lighting::new(&ctx, &stage_slab_buffer);
            let shadows = lighting.new_shadow_map(
                light_transform,
                wgpu::Extent3d {
                    width: w as u32,
                    height: h as u32,
                    depth_or_array_layers: 1,
                },
            );
            shadows.update(doc.renderlets_iter());
            (lighting, shadows)
        };

        let geometry_slab = futures_lite::future::block_on(stage.read(..)).unwrap();
        let light_slab = futures_lite::future::block_on(lighting.light_slab.read(..)).unwrap();
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

        let depth_texture = DepthTexture::new(&ctx, shadows.texture.texture.clone());
        let shadow_depth_img = depth_texture.read_image().unwrap();
        let mut depth_img = shadow_depth_img.clone();
        img_diff::normalize_gray_img(&mut depth_img);
        img_diff::save("shadows/shadow_mapping_sanity_depth.png", depth_img);

        assert_eq!(renderlet_vertex_info.len(), shadow_vertex_info.len());

        for (i, (pbr_info, shadow_info)) in renderlet_vertex_info
            .into_iter()
            .zip(shadow_vertex_info.into_iter())
            .enumerate()
        {
            // log::info!("{i}");
            let RenderletPbrVertexInfo {
                renderlet_id,
                vertex_index,
                vertex,
                transform,
                model_matrix,
                out_pos,
                view_projection,
                out_clip_pos,
                ..
            } = pbr_info;
            let pbr_info = ShadowMappingVertexInfo {
                renderlet_id,
                vertex_index,
                vertex,
                transform,
                model_matrix,
                world_pos: out_pos,
                view_projection,
                clip_pos: out_clip_pos,
            };
            pretty_assertions::assert_eq!(pbr_info, shadow_info, "vertex {i} is not equal");
        }

        let top_of_green_sphere_pos = {
            use crate::math::{CpuSampler, CpuTexture2d, Sample2d};
            // Here we'll check the `shadow_calculation` function to make sure it's calculating
            // the shadow correctly
            let sphere_001 = doc
                .nodes
                .iter()
                .find(|n| n.name.as_deref() == Some("Sphere.001"))
                .unwrap();
            let shadow_map = CpuTexture2d::from_image(shadow_depth_img);
            let light_space_transform = light_transform;
            // Fragment position in world space
            let frag_pos = sphere_001.global_transform().translation;
            log::info!("frag_pos: {frag_pos}");
            let frag_pos_in_light_space = light_space_transform.project_point3(frag_pos);
            log::info!("frag_pos_in_light_space: {frag_pos_in_light_space}");
            let proj_coords =
                frag_pos_in_light_space * Vec3::new(0.5, -0.5, 1.0) + Vec3::new(0.5, 0.5, 0.0);
            log::info!("proj_coords: {proj_coords}");
            log::info!(
                "proj_coords in pixels: {}",
                proj_coords.xy() * Vec2::new(w, h)
            );
            let image::Luma([closest_depth_luma_8]) =
                shadow_map.sample_by_lod(CpuSampler, proj_coords.xy(), 0.0);
            log::info!("closest_depth_luma: {closest_depth_luma_8:?}");
            let closest_depth = closest_depth_luma_8 as f32 / u8::MAX as f32;
            log::info!("closest_depth: {closest_depth:?}");
            let current_depth = proj_coords.z;
            assert!(
                current_depth > closest_depth,
                "top of green sphere is not in shadow"
            );

            frag_pos
        };

        {
            // Ensure the light slab has the correct light transform
            let light_slab = futures_lite::future::block_on(lighting.light_slab.read(..)).unwrap();
            let desc = light_slab.read_unchecked(Id::<LightingDescriptor>::new(0));
            let light_space_transform = light_slab.read_unchecked(desc.shadow_map_light_transform);
            assert_eq!(
                light_transform, light_space_transform,
                "light space transforms are not equal"
            );

            let frag_pos_in_light_space =
                light_space_transform.project_point3(top_of_green_sphere_pos);
            log::info!("frag_pos_in_light_space: {frag_pos_in_light_space}");
        }

        // Now do the rendering *with the shadow map* to see if it works.
        let frame = ctx.get_next_frame().unwrap();
        stage.render_with(&frame.view(), Some(&shadows.texture));
        let img = frame.read_image().unwrap();
        frame.present();
        img_diff::save("shadows/shadow_mapping_sanity_stage_render.png", img);
    }

    #[test]
    fn light_transform_depth_sanity() {
        let camera = Camera::default_perspective(800.0, 800.0);
        let directional_light = DirectionalLight {
            direction: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        };
        let (light_projection, light_view) =
            directional_light.shadow_mapping_projection_and_view(&Mat4::IDENTITY, &camera);
        let light_camera = Camera::new(light_projection, light_view);
        log::info!("z_near: {}", light_camera.z_near());
        log::info!("z_far: {}", light_camera.z_far());
        log::info!("depth: {}", light_camera.depth());
        log::info!("near_plane: {}", light_camera.near_plane());
        log::info!("far_plane: {}", light_camera.far_plane());
        log::info!("position: {}", light_camera.position());
    }
}
