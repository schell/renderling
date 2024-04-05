//! High definition rendering types and techniques.
//!
//! Also includes bloom effect.
use crabslab::{CpuSlab, Slab, SlabItem, WgpuBuffer};
use moongraph::*;

use crate::{
    frame::FrameTextureView, math::Vec4, tonemapping::TonemapConstants, BackgroundColor,
    DepthTexture, Device, Queue, RenderTarget, ScreenSize, WgpuStateError,
};

/// A texture, tonemapping pipeline and uniform used for high dynamic range
/// rendering.
///
/// See https://learnopengl.com/Advanced-Lighting/HDR.
pub struct HdrSurface {
    pub hdr_texture: crate::Texture,
    pub bindgroup: wgpu::BindGroup,
    pub tonemapping_pipeline: wgpu::RenderPipeline,
    pub slab: CpuSlab<WgpuBuffer>,
}

impl HdrSurface {
    pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

    pub fn create_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) -> crate::Texture {
        crate::Texture::new_with(
            &device,
            &queue,
            Some("HdrRenderBuffer"),
            Some(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING),
            Some({
                device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                })
            }),
            Self::TEXTURE_FORMAT,
            4,
            // TODO: pretty sure this should be `2`
            1,
            width,
            height,
            1,
            &[],
        )
    }

    pub fn create_bindgroup(
        device: &wgpu::Device,
        hdr_texture: &crate::Texture,
        slab_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HdrSurface bindgroup"),
            layout: &bindgroup_layout(&device, Some("HdrSurface bindgroup")),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: slab_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&hdr_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&hdr_texture.sampler),
                },
            ],
        })
    }

    pub fn color_attachments(&self) -> [Option<wgpu::RenderPassColorAttachment>; 1] {
        [Some(wgpu::RenderPassColorAttachment {
            view: &self.hdr_texture.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })]
    }
}

pub fn bindgroup_layout(device: &wgpu::Device, label: Option<&str>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            // slab
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
            // hdr texture
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // hdr sampler
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    })
}

pub fn create_hdr_render_surface(
    (device, queue, size, target): (
        View<Device, NoDefault>,
        View<Queue, NoDefault>,
        View<ScreenSize, NoDefault>,
        View<RenderTarget, NoDefault>,
    ),
) -> Result<(HdrSurface,), WgpuStateError> {
    let buffer = WgpuBuffer::new(&*device, &*queue, TonemapConstants::SLAB_SIZE);
    let mut slab = CpuSlab::new(buffer);
    // TODO: make the tonemapping configurable
    slab.write(0u32.into(), &TonemapConstants::default());
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let hdr_texture = HdrSurface::create_texture(&device, &queue, size.width, size.height);
    let label = Some("hdr tonemapping");
    let vertex_linkage = crate::linkage::tonemapping_vertex::linkage(&device);
    let fragment_linkage = crate::linkage::tonemapping_fragment::linkage(&device);
    let hdr_layout = bindgroup_layout(&device, Some("hdr tonemapping"));
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&hdr_layout],
        push_constant_ranges: &[],
    });
    let tonemapping_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        fragment: Some(wgpu::FragmentState {
            module: &fragment_linkage.module,
            entry_point: fragment_linkage.entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: target.format().add_srgb_suffix(),
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    Ok((HdrSurface {
        bindgroup: HdrSurface::create_bindgroup(&device, &hdr_texture, slab.as_ref().get_buffer()),
        hdr_texture,
        tonemapping_pipeline,
        slab,
    },))
}

// TODO: create a "node"  or "render_graph" module and put all the graph nodes in it

/// Conduct a clear pass on the window surface, the hdr surface and the depth
/// texture.
pub fn clear_surface_hdr_and_depth(
    (device, queue, frame, hdr, depth, color): (
        View<Device, NoDefault>,
        View<Queue, NoDefault>,
        View<FrameTextureView, NoDefault>,
        View<HdrSurface, NoDefault>,
        View<DepthTexture, NoDefault>,
        View<BackgroundColor, NoDefault>,
    ),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    let c: Vec4 = if frame.format.is_srgb() {
        color.0.powf(2.2)
    } else {
        color.0
    };
    let color = wgpu::Color {
        r: c.x.into(),
        g: c.y.into(),
        b: c.z.into(),
        a: c.w.into(),
    };
    crate::frame::conduct_clear_pass(
        &device,
        &queue,
        Some("clear_surface_hdr_and_depth"),
        vec![&frame.view, &hdr.hdr_texture.view],
        Some(&depth_view),
        color,
    );
    Ok(())
}

/// Resize the HDR surface to match [`ScreenSize`].
pub fn resize_hdr_surface(
    (device, queue, size, mut hdr): (
        View<Device, NoDefault>,
        View<Queue, NoDefault>,
        View<ScreenSize, NoDefault>,
        ViewMut<HdrSurface, NoDefault>,
    ),
) -> Result<(), WgpuStateError> {
    let ScreenSize { width, height } = *size;
    hdr.hdr_texture = HdrSurface::create_texture(&device, &queue, width, height);
    hdr.bindgroup =
        HdrSurface::create_bindgroup(&device, &hdr.hdr_texture, hdr.slab.as_ref().get_buffer());
    Ok(())
}
