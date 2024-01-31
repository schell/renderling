//! `wgpu` helper functions for tests.
use crate::{
    frame::FrameTextureView, Atlas, DepthTexture, Device, GraphError, Queue, Renderling, Skybox,
    View,
};
use crabslab::{CpuSlab, WgpuBuffer};

use super::{raymarch::Raymarch, *};

mod sdf_renderer;
pub use sdf_renderer::SdfRenderer;

pub fn new_render_pipeline(
    fragment_linkage: crate::linkage::ShaderLinkage,
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let label = Some("raymarch pipeline");
    let vertex_linkage = crate::linkage::sdf__raymarch__raymarch_vertex(device);
    let slab_layout = crate::linkage::slab_bindgroup_layout(device);
    let atlas_and_skybox_layout = crate::linkage::atlas_and_skybox_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&slab_layout, &atlas_and_skybox_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_linkage.module,
            entry_point: vertex_linkage.entry_point,
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
            module: &fragment_linkage.module,
            entry_point: fragment_linkage.entry_point,
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

pub struct RaymarchingRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub rays_pipeline: wgpu::RenderPipeline,
    pub slab_bindgroup: wgpu::BindGroup,
    pub atlas_and_skybox_bindgroup: wgpu::BindGroup,
    pub renderling: Renderling,
    pub slab: CpuSlab<WgpuBuffer>,
    pub atlas: Atlas,
    pub skybox: Skybox,
}

impl RaymarchingRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_with_capacity(width, height, 256)
    }

    fn make_atlas_and_skybox_bindgroup(
        &self,
        device: &wgpu::Device,
        atlas: &Atlas,
        skybox: &Skybox,
    ) -> wgpu::BindGroup {
        let layout = self.pipeline.get_bind_group_layout(1);
        crate::linkage::atlas_and_skybox_bindgroup(device, &layout, atlas, skybox)
    }

    pub fn with_skybox(mut self, skybox: Skybox) -> Self {
        self.skybox = skybox;
        self.atlas_and_skybox_bindgroup = self.make_atlas_and_skybox_bindgroup(
            self.renderling.get_device(),
            &self.atlas,
            &self.skybox,
        );
        self
    }

    pub fn new_with_capacity(width: u32, height: u32, cap: usize) -> Self {
        let mut renderling = Renderling::headless(width, height);
        configure_graph(&mut renderling);
        let (d, q) = renderling.get_device_and_queue_owned();
        let pipeline = new_render_pipeline(
            crate::linkage::sdf__raymarch__raymarch_fragment(&d),
            &d,
            renderling.get_render_target().format(),
        );
        let rays_pipeline = new_render_pipeline(
            crate::linkage::sdf__raymarch__raymarch_rays(&d),
            &d,
            renderling.get_render_target().format(),
        );
        let slab = CpuSlab::new(WgpuBuffer::new(&d, &q, cap));
        let bindgroup_layout = crate::linkage::slab_bindgroup_layout(&d);
        let slab_bindgroup =
            crate::linkage::slab_bindgroup(&d, slab.as_ref().get_buffer(), &bindgroup_layout);
        let atlas = Atlas::empty(&d, &q);
        let skybox = Skybox::empty(d.clone(), q.clone());
        let atlas_and_skybox_layout = crate::linkage::atlas_and_skybox_bindgroup_layout(&d);
        let atlas_and_skybox_bindgroup = crate::linkage::atlas_and_skybox_bindgroup(
            &d,
            &atlas_and_skybox_layout,
            &atlas,
            &skybox,
        );
        Self {
            pipeline,
            rays_pipeline,
            slab_bindgroup,
            renderling,
            slab,
            atlas,
            skybox,
            atlas_and_skybox_bindgroup,
        }
    }

    fn render_raymarching(
        id: Id<Raymarch>,
        pipeline: &wgpu::RenderPipeline,
        slab_bindgroup: &wgpu::BindGroup,
        atlas_and_skybox_bindgroup: &wgpu::BindGroup,
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
            render_pass.set_bind_group(0, slab_bindgroup, &[]);
            render_pass.set_bind_group(1, atlas_and_skybox_bindgroup, &[]);
            render_pass.draw(0..6, id.inner()..id.inner() + 1);
        }
        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render_rays(&mut self, raymarch: Id<Raymarch>) -> image::RgbaImage {
        self.renderling
            .render_local(
                |(device, queue, frame, depth_texture): (
                    View<Device>,
                    View<Queue>,
                    View<FrameTextureView>,
                    View<DepthTexture>,
                )|
                 -> Result<(), GraphError> {
                    Self::render_raymarching(
                        raymarch,
                        &self.rays_pipeline,
                        &self.slab_bindgroup,
                        &self.atlas_and_skybox_bindgroup,
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

    pub fn render(&mut self, raymarch: Id<Raymarch>) {
        self.renderling
            .render_local(
                |(device, queue, frame, depth_texture): (
                    View<Device>,
                    View<Queue>,
                    View<FrameTextureView>,
                    View<DepthTexture>,
                )|
                 -> Result<(), GraphError> {
                    Self::render_raymarching(
                        raymarch,
                        &self.pipeline,
                        &self.slab_bindgroup,
                        &self.atlas_and_skybox_bindgroup,
                        &device,
                        &queue,
                        &frame,
                        &depth_texture,
                    );
                    Ok(())
                },
            )
            .unwrap();
    }

    pub fn render_image(&mut self, raymarch: Id<Raymarch>) -> image::RgbaImage {
        let start = std::time::Instant::now();
        self.render(raymarch);
        let after_render = std::time::Instant::now();
        let render_duration = (after_render - start).as_secs_f32();
        let img = self.renderling.read_image().unwrap();
        let after_read = std::time::Instant::now();
        let read_duration = (after_read - after_render).as_secs_f32();
        let total = (after_read - start).as_secs_f32();
        println!(
            "render: {render_duration:?}s {}fps {}%",
            1.0 / render_duration,
            render_duration / total * 100.0
        );
        println!(
            "read: {read_duration:?}s {}%",
            read_duration / total * 100.0
        );
        img
    }
}

pub fn configure_graph(r: &mut Renderling) {
    // set up the render graph
    use crate::{
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
    ), ()>("raymarch_render");
    r.graph.add_barrier();

    // post
    r.graph.add_subgraph(graph!(copy_frame_to_post, present));
}
