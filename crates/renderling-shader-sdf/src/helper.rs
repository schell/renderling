//! `wgpu` helper functions for tests.
use crabslab::{CpuSlab, GrowableSlab, WgpuBuffer};
use renderling::{
    frame::FrameTextureView, DepthTexture, Device, GraphError, Queue, Renderling, View,
};

use super::*;

pub fn new_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let visibility =
        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE;
    let slab = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    let entries = vec![slab];
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("slab"),
        entries: &entries,
    })
}

pub fn new_bindgroup(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    bindgroup_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    let label = Some("slab");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
        }],
    })
}

pub fn new_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let label = Some("sdf pipeline");
    let vertex_shader = device.create_shader_module(wgpu::include_spirv!(
        "../../renderling/src/linkage/sdf_shape_vertex.spv"
    ));
    let fragment_shader = device.create_shader_module(wgpu::include_spirv!(
        "../../renderling/src/linkage/sdf_shape_fragment.spv"
    ));
    let slab_layout = new_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&slab_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "sdf_shape_vertex",
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
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: 1,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "sdf_shape_fragment",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    pipeline
}

pub struct SdfRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub bindgroup: wgpu::BindGroup,
    pub renderling: Renderling,
    pub slab: CpuSlab<WgpuBuffer>,
    legend_id: Id<ShapeLegend>,
    vertex_count: u32,
}

impl SdfRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_with_capacity(width, height, 256)
    }

    pub fn new_with_capacity(width: u32, height: u32, cap: usize) -> Self {
        let mut renderling = Renderling::headless(width, height);
        configure_graph(&mut renderling);
        let (d, q) = renderling.get_device_and_queue_owned();
        let pipeline = new_render_pipeline(&d, renderling.get_render_target().format());
        let mut slab = CpuSlab::new(WgpuBuffer::new(&d, &q, cap));
        let legend_id = slab.append::<ShapeLegend>(&ShapeLegend::default());
        let bindgroup_layout = new_bindgroup_layout(&d);
        let bindgroup = new_bindgroup(&d, slab.as_ref().get_buffer(), &bindgroup_layout);
        Self {
            pipeline,
            bindgroup,
            renderling,
            slab,
            legend_id,
            vertex_count: 0,
        }
    }

    pub fn set_shape(&mut self, shape: SdfShape) -> Id<SdfShape> {
        self.vertex_count = shape.vertex_count();
        let id = self.slab.append(&shape);
        self.slab
            .write(self.legend_id + ShapeLegend::offset_of_shape(), &id);
        id
    }

    pub fn set_debug_point(&mut self, point: Vec2) {
        self.slab.write(
            self.legend_id + ShapeLegend::offset_of_debug_point(),
            &point,
        );
    }

    fn render(
        pipeline: &wgpu::RenderPipeline,
        bindgroup: &wgpu::BindGroup,
        legend_id: Id<ShapeLegend>,
        vertex_count: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &FrameTextureView,
        depth_texture: &DepthTexture,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sdf render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bindgroup, &[]);
            render_pass.draw(0..vertex_count, legend_id.inner()..legend_id.inner() + 1);
        }
        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render_image(&mut self) -> image::RgbaImage {
        self.renderling
            .render_local(
                |(device, queue, frame, depth_texture): (
                    View<Device>,
                    View<Queue>,
                    View<FrameTextureView>,
                    View<DepthTexture>,
                )|
                 -> Result<(), GraphError> {
                    Self::render(
                        &self.pipeline,
                        &self.bindgroup,
                        self.legend_id,
                        self.vertex_count,
                        &device,
                        &queue,
                        &frame,
                        &depth_texture,
                    );
                    Ok(())
                },
            )
            .unwrap();
        self.renderling.read_image().unwrap()
    }
}

pub fn configure_graph(r: &mut Renderling) {
    // set up the render graph
    use renderling::{
        frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present},
        graph::{graph, Graph},
    };

    // pre-render
    r.graph
        .add_subgraph(graph!(create_frame, clear_frame_and_depth))
        .add_barrier();

    // render
    r.graph.add_local::<(
        View<Device>,
        View<Queue>,
        View<FrameTextureView>,
        View<DepthTexture>,
    ), ()>("render");
    r.graph.add_barrier();

    // post
    r.graph.add_subgraph(graph!(copy_frame_to_post, present));
}
