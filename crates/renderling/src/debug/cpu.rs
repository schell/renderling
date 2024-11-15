//! CPU side of drawing debugging overlays.

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DebugOverlay {
    pipeline: Arc<wgpu::RenderPipeline>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup: Arc<Mutex<Option<wgpu::BindGroup>>>,
}

impl DebugOverlay {
    const LABEL: Option<&'static str> = Some("debug-overlay");

    fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // stage slab
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
                // draw calls
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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

    fn create_pipeline_layout(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Self::LABEL,
            bind_group_layouts: &[bindgroup_layout],
            push_constant_ranges: &[],
        })
    }

    fn create_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let vertex = crate::linkage::debug_overlay_vertex::linkage(device);
        let fragment = crate::linkage::debug_overlay_fragment::linkage(device);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Self::LABEL,
            layout: Some(&Self::create_pipeline_layout(device, bindgroup_layout)),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: None,
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

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragment.module,
                entry_point: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        })
    }

    pub fn create_bindgroup(
        &self,
        device: &wgpu::Device,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout: &self.bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        indirect_draw_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let bindgroup_layout = Arc::new(Self::create_bindgroup_layout(device));
        Self {
            pipeline: Self::create_pipeline(device, &bindgroup_layout, format).into(),
            bindgroup_layout,
            bindgroup: Arc::new(Mutex::new(None)),
        }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Self::LABEL,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            // UNWRAP: panic on purpose
            let mut guard = self.bindgroup.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.create_bindgroup(device, slab_buffer, indirect_draw_buffer));
            }
            render_pass.set_bind_group(0, guard.as_ref(), &[]);
            render_pass.draw(0..6, 0..1);
        }
        queue.submit(Some(encoder.finish()));
    }
}
