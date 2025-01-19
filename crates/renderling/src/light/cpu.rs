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
    texture: crate::texture::Texture,
    stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    _light_transform: Hybrid<Mat4>,
    pipeline: Arc<wgpu::RenderPipeline>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup: ManagedBindGroup,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    fn create_shadow_map_texture(
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
        runtime: impl AsRef<WgpuRuntime>,
        light_slab: &SlabAllocator<WgpuRuntime>,
        light_transform: Mat4,
        size: wgpu::Extent3d,
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
    ) -> Self {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
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

        let _light_transform = light_slab.new_value(light_transform);
        let light_slab_buffer = light_slab.upkeep();
        let bindgroup = ManagedBindGroup::new(Self::create_bindgroup(
            device,
            &bindgroup_layout,
            stage_slab_buffer,
            &light_slab_buffer,
        ));

        ShadowMap {
            stage_slab_buffer: Arc::new(RwLock::new(stage_slab_buffer.clone())),
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            texture: Self::create_shadow_map_texture(device, size),
            _light_transform,
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
    pub fn update<'a>(
        &self,
        lighting_manager: &Lighting,
        renderlets: impl IntoIterator<Item = &'a Hybrid<Renderlet>>,
    ) {
        let runtime = lighting_manager.light_slab.runtime();
        let device = &runtime.device;
        let queue = &runtime.queue;
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
        // Note:
        // We should think about copying the depth buffer to a storage buffer
        // (possibly just for lights) which we could "sample" from, since WebGPU
        // doesn't provide arrays of textures to bind to.
        //
        // Currently we either have to choose:
        // * set the number of shadow maps statically at compile time (code change)
        // * run shading once per shadow map (don't know if would work)
        // * use a texture array and each shadow map is the same size
        //
        // With a storage buffer for lights we could store the shadow map's depth
        // buffer at any size, and reference it from the slab.
        // But we would lose sampling conveniences.
        queue.submit(Some(encoder.finish()));
    }
}

/// Manages lighting for an entire scene.
#[derive(Clone)]
pub struct Lighting {
    light_slab: SlabAllocator<WgpuRuntime>,
    light_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,
    _lighting_descriptor: Hybrid<LightingDescriptor>,
    shadow_map: ShadowMap,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup: ManagedBindGroup,
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
    pub fn get_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        let mut light_slab_buffer = self.light_slab_buffer.write().unwrap();
        let should_invalidate = light_slab_buffer.synchronize();
        self.bindgroup.get(should_invalidate, || {
            Self::create_bindgroup(
                self.light_slab.device(),
                &self.bindgroup_layout,
                &light_slab_buffer,
                &self.shadow_map.texture,
            )
        })
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
        let shadow_map = ShadowMap::new(
            runtime,
            &light_slab,
            Mat4::IDENTITY,
            wgpu::Extent3d::default(),
            stage_slab_buffer,
        );
        let light_slab_buffer = light_slab.upkeep();
        let bindgroup_layout = Self::create_bindgroup_layout(&runtime.device);
        let bindgroup = ManagedBindGroup::new(Self::create_bindgroup(
            &runtime.device,
            &bindgroup_layout,
            &light_slab_buffer,
            &shadow_map.texture,
        ));

        Self {
            light_slab,
            light_slab_buffer: Arc::new(RwLock::new(light_slab_buffer)),
            _lighting_descriptor,
            shadow_map,
            bindgroup_layout: bindgroup_layout.into(),
            bindgroup,
        }
    }
}

#[cfg(test)]
mod test {
    use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

    use crate::{
        camera::Camera,
        light::{LightStyle, ShadowMappingVertexInfo},
        prelude::Transform,
        stage::RenderletPbrVertexInfo,
        texture::DepthTexture,
    };

    use super::*;

    #[test]
    fn shadow_mapping_sanity() {
        let w = 800.0;
        let h = 800.0;
        let ctx = crate::Context::headless(w as u32, h as u32);
        let mut stage = ctx.new_stage().with_lighting(false);
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
        let gltf_light = doc.lights.first().unwrap();
        log::info!("light_id: {:?}", gltf_light.light.id());
        log::info!("light.index: {:?}", gltf_light.light.get().index);
        log::info!(
            "light.transform_id: {:?}",
            gltf_light.light.get().transform_id
        );
        stage.set_lights([gltf_light.light.id()]);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        // Rendering the scene without shadows as a sanity check
        img_diff::assert_img_eq("shadows/shadow_mapping_sanity_scene_before.png", img);

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

            img_diff::assert_img_eq("shadows/shadow_mapping_sanity_light_pov.png", img);

            let mut depth_img = stage.get_depth_texture().read_image().unwrap();
            // Normalize the value
            img_diff::normalize_gray_img(&mut depth_img);
            img_diff::assert_img_eq(
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

        let stage_slab_buffer = stage.stage_slab_buffer.read().unwrap();
        let lighting = Lighting::new(&ctx, &stage_slab_buffer);

        let shadows = ShadowMap::new(
            &ctx,
            &lighting.light_slab,
            light_transform,
            wgpu::Extent3d {
                width: w as u32,
                height: h as u32,
                depth_or_array_layers: 1,
            },
            &stage.get_buffer().unwrap(),
        );
        shadows.update(&lighting, doc.renderlets_iter());

        let slab = futures_lite::future::block_on(stage.read(..)).unwrap();
        let mut shadow_vertex_info = vec![];
        for hybrid in doc.renderlets_iter() {
            let renderlet = hybrid.get();
            for vertex_index in 0..renderlet.get_vertex_count() {
                let mut info = ShadowMappingVertexInfo::default();
                crate::light::shadow_mapping_vertex(
                    hybrid.id(),
                    vertex_index,
                    &slab,
                    &light_transform,
                    &mut Default::default(),
                    &mut info,
                );
                shadow_vertex_info.push(info);
            }
        }

        let depth_texture = DepthTexture::new(&ctx, shadows.texture.texture.clone());
        let mut depth_img = depth_texture.read_image().unwrap();
        img_diff::normalize_gray_img(&mut depth_img);
        img_diff::assert_img_eq("shadows/shadow_mapping_sanity_depth.png", depth_img);

        assert_eq!(renderlet_vertex_info.len(), shadow_vertex_info.len());

        for (i, (pbr_info, shadow_info)) in renderlet_vertex_info
            .into_iter()
            .zip(shadow_vertex_info.into_iter())
            .enumerate()
        {
            log::info!("{i}");
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

    fn assert_sanity_strings(
        seen_directional_light: DirectionalLight,
        seen_parent_light_transform: Mat4,
        seen_camera: Camera,
        seen_projection: Mat4,
        seen_view: Mat4,
        seen_view_projection: Mat4,
    ) {
        let directional_light = r#"DirectionalLight {
    direction: Vec3(
        0.0,
        0.0,
        -1.0,
    ),
    color: Vec4(
        1.0,
        1.0,
        1.0,
        1.0,
    ),
    intensity: 10.0,
}"#;
        pretty_assertions::assert_str_eq!(
            directional_light,
            format!("{seen_directional_light:#?}")
        );
        let parent_light_transform = r#"Mat4 {
    x_axis: Vec4(
        -0.5525446,
        0.7096175,
        0.4371941,
        0.0,
    ),
    y_axis: Vec4(
        -0.12115364,
        -0.587348,
        0.80021566,
        0.0,
    ),
    z_axis: Vec4(
        0.82463145,
        0.3891868,
        0.41050822,
        0.0,
    ),
    w_axis: Vec4(
        4.0762453,
        1.005454,
        5.903862,
        1.0,
    ),
}"#;
        pretty_assertions::assert_str_eq!(
            parent_light_transform,
            format!("{seen_parent_light_transform:#?}")
        );
        let camera = r#"Camera {
    projection: Mat4 {
        x_axis: Vec4(
            2.4142134,
            0.0,
            0.0,
            0.0,
        ),
        y_axis: Vec4(
            0.0,
            2.4142134,
            0.0,
            0.0,
        ),
        z_axis: Vec4(
            0.0,
            0.0,
            -1.001001,
            -1.0,
        ),
        w_axis: Vec4(
            0.0,
            0.0,
            -0.1001001,
            0.0,
        ),
    },
    view: Mat4 {
        x_axis: Vec4(
            0.56048316,
            -0.04112576,
            0.8271439,
            -0.0,
        ),
        y_axis: Vec4(
            0.82782656,
            0.05640688,
            -0.55814093,
            0.0,
        ),
        z_axis: Vec4(
            -0.023702562,
            0.9975604,
            0.06566016,
            -0.0,
        ),
        w_axis: Vec4(
            2.3725195,
            -3.6266158,
            -19.559895,
            1.0,
        ),
    },
    position: Vec3(
        14.69995,
        -12.676652,
        4.9583097,
    ),
    frustum: Frustum {
        planes: [
            Vec4(
                -0.8271441,
                0.55814105,
                -0.06566017,
                19.509872,
            ),
            Vec4(
                0.20128462,
                0.97840333,
                -0.047025368,
                9.67717,
            ),
            Vec4(
                -0.8343532,
                -0.5512207,
                -0.0032287433,
                5.2933264,
            ),
            Vec4(
                -0.3545296,
                0.2657045,
                0.89649874,
                4.1346927,
            ),
            Vec4(
                -0.27853903,
                0.16147813,
                -0.9467527,
                10.835804,
            ),
            Vec4(
                0.8271441,
                -0.55814105,
                0.06566017,
                80.44148,
            ),
        ],
        points: [
            Vec3(
                14.646105,
                -12.664715,
                4.976187,
            ),
            Vec3(
                14.6693325,
                -12.630406,
                4.975204,
            ),
            Vec3(
                14.647809,
                -12.667053,
                4.9348445,
            ),
            Vec3(
                14.671038,
                -12.632747,
                4.9338613,
            ),
            Vec3(
                -92.93541,
                11.184539,
                40.694893,
            ),
            Vec3(
                -46.502815,
                79.76488,
                38.731274,
            ),
            Vec3(
                -89.52838,
                6.511578,
                -41.94686,
            ),
            Vec3(
                -43.095795,
                75.09193,
                -43.91048,
            ),
        ],
        center: Vec3(
            14.658571,
            -12.64873,
            4.9550242,
        ),
    },
}"#;
        pretty_assertions::assert_str_eq!(camera, format!("{seen_camera:#?}"));
        let projection = r#"Mat4 {
    x_axis: Vec4(
        0.032823686,
        0.0,
        0.0,
        0.0,
    ),
    y_axis: Vec4(
        0.0,
        0.032823686,
        0.0,
        0.0,
    ),
    z_axis: Vec4(
        0.0,
        0.0,
        -0.016411843,
        0.0,
    ),
    w_axis: Vec4(
        -0.0,
        -0.0,
        -0.0,
        1.0,
    ),
}"#;

        pretty_assertions::assert_str_eq!(projection, format!("{seen_projection:#?}"));
        let view = r#"Mat4 {
    x_axis: Vec4(
        -0.42680675,
        -0.37124008,
        0.8246313,
        0.0,
    ),
    y_axis: Vec4(
        0.90434283,
        -0.17520764,
        0.3891867,
        0.0,
    ),
    z_axis: Vec4(
        0.0,
        0.9118569,
        0.41050813,
        0.0,
    ),
    w_axis: Vec4(
        -9.536743e-7,
        9.536743e-7,
        -30.465805,
        1.0,
    ),
}"#;
        pretty_assertions::assert_str_eq!(view, format!("{seen_view:#?}"));
        let view_projection = r#"Mat4 {
    x_axis: Vec4(
        -0.0140093705,
        -0.012185467,
        -0.013533719,
        0.0,
    ),
    y_axis: Vec4(
        0.029683866,
        -0.0057509607,
        -0.006387271,
        0.0,
    ),
    z_axis: Vec4(
        0.0,
        0.029930504,
        -0.006737195,
        0.0,
    ),
    w_axis: Vec4(
        -3.1303106e-8,
        3.1303106e-8,
        0.5,
        1.0,
    ),
}"#;
        pretty_assertions::assert_str_eq!(view_projection, format!("{seen_view_projection:#?}"));
    }
}
