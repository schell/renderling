//! Various render nodes.
//!
//! A render node is simply a function whose input parameters are all
//! [`Edges`](moongraph::Edges), and whose result is
//! [`NodeResults`](moongraph::NodeResults).
//!
//! Render nodes are registered using [`Renderer::add_node`] or
//! [`Renderer::with_node`].
//!
//! See `moongraph`'s [`Node`] documentation for more info.
use std::{ops::Deref, sync::Arc};

use moongraph::*;
use renderling_shader::tonemapping::TonemapConstants;

use crate::{
    math::Vec4, BackgroundColor, BufferDimensions, CopiedTextureBuffer, DepthTexture, Device,
    Frame, Queue, RenderTarget, ScreenSize, Uniform, WgpuStateError,
};

fn default_frame_texture_view(frame_texture: &wgpu::Texture) -> wgpu::TextureView {
    frame_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("WgpuState::default_frame_texture_view"),
        format: None,
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    })
}

pub struct FrameTextureView {
    view: Arc<wgpu::TextureView>,
    format: wgpu::TextureFormat,
}

impl Deref for FrameTextureView {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

/// Create the next screen frame texture, frame texture view and depth texture.
pub fn create_frame(
    render_target: Read<RenderTarget>,
) -> Result<(Frame, FrameTextureView), WgpuStateError> {
    let frame = render_target.get_current_frame()?;
    let format = render_target.format();
    let frame_view = default_frame_texture_view(frame.texture());
    Ok((
        frame,
        FrameTextureView {
            view: frame_view.into(),
            format,
        },
    ))
}

/// Perform a clearing render pass on a frame and/or a depth texture.
pub fn conduct_clear_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_views: Vec<&wgpu::TextureView>,
    depth_view: Option<&wgpu::TextureView>,
    clear_color: wgpu::Color,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("renderling clear pass"),
    });

    let frame_views = frame_views.into_iter().map(|view| Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear_color),
            store: true,
        },
    })).collect::<Vec<_>>();
    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &frame_views,
        depth_stencil_attachment: depth_view.map(|view| wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),
    });

    queue.submit(std::iter::once(encoder.finish()));
}

/// Conduct a clear pass on the global frame and depth textures.
pub fn clear_frame_and_depth(
    (device, queue, frame_view, depth, color): (
        Read<Device>,
        Read<Queue>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
        Read<BackgroundColor>,
    ),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    let color: Vec4 = if frame_view.format.is_srgb() {
        color.0.powf(2.2)
    } else {
        color.0
    };
    let [r, g, b, a] = color.to_array();
    let color = wgpu::Color {
        r: r.into(),
        g: g.into(),
        b: b.into(),
        a: a.into(),
    };
    let frames = vec![frame_view.view.as_ref()];
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        frames,
        Some(&depth_view),
        color,
    );
    Ok(())
}

/// Conduct a clear pass on **only the depth texture**.
pub fn clear_depth(
    (device, queue, depth): (Read<Device>, Read<Queue>, Read<DepthTexture>),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_depth"),
        vec![],
        Some(&depth_view),
        Default::default(),
    );
    Ok(())
}

/// A texture, tonemapping pipeline and uniform used for high dynamic range
/// rendering.
///
/// See https://learnopengl.com/Advanced-Lighting/HDR.
pub struct HdrSurface {
    pub texture: crate::Texture,
    pub texture_bindgroup: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub constants: Uniform<TonemapConstants>,
}

fn scene_hdr_surface_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("hdr buffer bindgroup"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    })
}

pub fn create_hdr_render_surface(
    (device, queue, size, target): (Read<Device>, Read<Queue>, Read<ScreenSize>, Read<RenderTarget>),
) -> Result<(HdrSurface,), WgpuStateError> {
    let (constants, constants_layout) = Uniform::new_and_layout(
        &device,
        TonemapConstants::default(),
        wgpu::BufferUsages::UNIFORM,
        wgpu::ShaderStages::FRAGMENT,
    );
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let hdr_format = wgpu::TextureFormat::Rgba16Float;
    let texture = crate::Texture::new_with(
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
        hdr_format,
        4,
        size.width,
        size.height,
        &[],
    );
    let label = Some("scene render pipeline");
    let vertex_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/vertex_tonemapping.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/fragment_tonemapping.spv"));
    let hdr_texture_layout = scene_hdr_surface_bindgroup_layout(&device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&hdr_texture_layout, &constants_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "vertex_tonemapping",
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
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: 1,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "fragment_tonemapping",
            targets: &[Some(wgpu::ColorTargetState {
                format: target.format(),
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    let texture_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("hdr surface texture bindgroup"),
        layout: &hdr_texture_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    });

    Ok((HdrSurface {
        texture,
        texture_bindgroup,
        pipeline,
        constants,
    },))
}

/// Update the `HdrSurface` uniforms.
pub fn hdr_surface_update(
    (queue, mut hdr_surface): (Read<Queue>, Write<HdrSurface>),
) -> Result<(), WgpuStateError> {
    hdr_surface.constants.update(&queue);
    Ok(())
}

/// Conduct a clear pass on the window surface, the hdr surface and the depth texture.
pub fn clear_surface_hdr_and_depth(
    (device, queue, frame, hdr, depth, color): (
        Read<Device>,
        Read<Queue>,
        Read<FrameTextureView>,
        Read<HdrSurface>,
        Read<DepthTexture>,
        Read<BackgroundColor>,
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
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        vec![&frame.view, &hdr.texture.view],
        Some(&depth_view),
        color,
    );
    Ok(())
}

// pub fn create_hdr_render_buffer

/// A buffer holding a copy of the last frame's buffer/texture.
pub struct PostRenderBuffer(pub CopiedTextureBuffer);

/// Render node that copies the current frame into a buffer.
#[derive(Edges)]
pub struct PostRenderBufferCreate {
    device: Read<Device>,
    queue: Read<Queue>,
    size: Read<ScreenSize>,
    frame: Read<Frame>,
}

impl PostRenderBufferCreate {
    /// Copies the current frame into a `PostRenderBuffer` resource.
    ///
    /// If rendering to a window surface, this should be called after rendering,
    /// before presentation.
    pub fn create(self) -> Result<(PostRenderBuffer,), WgpuStateError> {
        let ScreenSize { width, height } = *self.size;
        let dimensions = BufferDimensions::new(4, 1, width as usize, height as usize);
        // The output buffer lets us retrieve the self as an array
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("screen capture buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("post render screen capture encoder"),
            });
        let texture = self.frame.texture();
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(dimensions.padded_bytes_per_row as u32),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: dimensions.width as u32,
                height: dimensions.height as u32,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok((PostRenderBuffer(CopiedTextureBuffer { dimensions, buffer }),))
    }
}

/// Consume and present the screen frame to the screen.
pub fn present(frame: Move<Frame>) -> Result<(), WgpuStateError> {
    let frame = frame.into();
    frame.present();
    Ok(())
}
