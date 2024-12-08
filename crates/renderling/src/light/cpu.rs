//! CPU-only lighting and shadows.

use std::sync::{Arc, Mutex};

use crabslab::Id;

use crate::{
    slab::{Hybrid, SlabAllocator},
    stage::Renderlet,
};

use super::{DirectionalLight, Light, PointLight, SpotLight};

/// A wrapper around all types of analytical lighting.
#[derive(Clone, Debug)]
pub enum LightDetails {
    Directional(Hybrid<DirectionalLight>),
    Point(Hybrid<PointLight>),
    Spot(Hybrid<SpotLight>),
}

/// A depth map rendering of a scene from a light's point of view.
// TODO: Separate pipeline and bindgroup layout from ShadowMap
pub struct ShadowMap {
    /// A depth texture used to store the scene from the light's POV.
    texture: Arc<wgpu::Texture>,
    slab: SlabAllocator<wgpu::Buffer>,
    light_id: Hybrid<Id<Light>>,
    pipeline: Arc<wgpu::RenderPipeline>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    pub fn new(device: &wgpu::Device, light_id: Id<Light>, size: wgpu::Extent3d) -> Self {
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
        let slab = SlabAllocator::default();
        let light_id = slab.new_value(light_id);
        ShadowMap {
            texture: device
                .create_texture(&wgpu::TextureDescriptor {
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
                })
                .into(),
            slab,
            light_id,
            bindgroup_layout: bindgroup_layout.into(),
            bindgroup: Default::default(),
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
                        cull_mode: Some(wgpu::Face::Back),
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
        self.texture.size()
    }

    fn get_bindgroup(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
    ) -> Arc<wgpu::BindGroup> {
        let mut guard = self.bindgroup.lock().unwrap();
        let (desc_buffer, desc_buffer_was_updated) = self.slab.get_updated_buffer_and_check((
            device,
            queue,
            Self::LABEL,
            wgpu::BufferUsages::empty(),
        ));
        if desc_buffer_was_updated {
            // Invalidate the buffer
            let _ = guard.take();
        }
        if let Some(bg) = guard.as_ref() {
            bg.clone()
        } else {
            let bg: Arc<_> = device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Self::LABEL,
                    layout: &self.bindgroup_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(
                                slab_buffer.as_entire_buffer_binding(),
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Buffer(
                                desc_buffer.as_entire_buffer_binding(),
                            ),
                        },
                    ],
                })
                .into();
            *guard = Some(bg.clone());
            bg
        }
    }

    pub fn invalidate_bindgroup(&self) {
        let _ = self.bindgroup.lock().unwrap().take();
    }

    pub fn update<'a>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
        renderlets: impl IntoIterator<Item = &'a Hybrid<Renderlet>>,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Self::LABEL,
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture.create_view(&wgpu::TextureViewDescriptor {
                        label: Self::LABEL,
                        format: None,
                        dimension: None,
                        aspect: wgpu::TextureAspect::DepthOnly,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer: 0,
                        array_layer_count: None,
                    }),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            let bindgroup = self.get_bindgroup(device, queue, slab_buffer);
            render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
            for rlet in renderlets {
                let id = rlet.id();
                let rlet = rlet.get();
                let vertex_range = 0..rlet.get_vertex_count();
                let instance_range = id.inner()..id.inner() + 1;
                render_pass.draw(vertex_range, instance_range);
            }
        }
        queue.submit(Some(encoder.finish()));
    }
}

#[cfg(test)]
mod test {
    use crate::camera::Camera;

    use super::*;

    #[test]
    fn shadow_mapping_sanity() {
        let ctx = crate::Context::headless(1024, 800);
        let mut stage = ctx.new_stage();
        let camera = stage.new_value(Camera::default());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_sanity.gltf"),
                camera.id(),
            )
            .unwrap();
        log::info!("cameras: {:#?}", doc.cameras);
        let gltf_camera = doc.cameras.first().unwrap();
        camera.set(gltf_camera.get_camera());
        let gltf_light = doc.lights.first().unwrap();
        stage.set_lights([gltf_light.light.id()]);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        img_diff::assert_img_eq("shadows/shadow_mapping_sanity.png", img);

        let shadows = ShadowMap::new(
            ctx.get_device(),
            gltf_light.light.id(),
            wgpu::Extent3d {
                width: 1024,
                height: 800,
                depth_or_array_layers: 1,
            },
        );
        shadows.update(
            ctx.get_device(),
            ctx.get_queue(),
            &stage.get_buffer().unwrap(),
            doc.renderlets.values().flatten(),
        );

        let img = crate::texture::read_depth_texture_to_image(
            ctx.get_device(),
            ctx.get_queue(),
            1024,
            800,
            &shadows.texture,
        )
        .unwrap();
        img_diff::save("shadows/shadow_mapping_sanity_depth.png", img);
    }
}
