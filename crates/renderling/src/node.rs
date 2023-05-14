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

use crate::{
    BackgroundColor, BufferDimensions, CopiedTextureBuffer, DepthTexture, Device, Frame, Queue,
    RenderTarget, ScreenSize, WgpuStateError,
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

pub struct FrameTextureView(Arc<wgpu::TextureView>);

impl Deref for FrameTextureView {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Create the next screen frame texture, frame texture view and depth texture.
pub fn create_frame(
    render_target: Read<RenderTarget>,
) -> Result<(Frame, FrameTextureView), WgpuStateError> {
    let frame = render_target.get_current_frame()?;
    let frame_view = default_frame_texture_view(frame.texture());
    Ok((frame, FrameTextureView(frame_view.into())))
}

/// Perform a clearing render pass on a frame and/or a depth texture.
pub fn conduct_clear_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_view: Option<&wgpu::TextureView>,
    depth_view: Option<&wgpu::TextureView>,
    clear_color: wgpu::Color,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("renderling clear pass"),
    });

    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[frame_view.map(|view| wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: true,
            },
        })],
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
    let [r, g, b, a] = color.0.to_array();
    let color = wgpu::Color {
        r: r.into(),
        g: g.into(),
        b: b.into(),
        a: a.into(),
    };
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        Some(&frame_view),
        Some(&depth_view),
        color,
    );
    Ok(())
}

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
                    bytes_per_row: Some(
                        dimensions.padded_bytes_per_row as u32,
                    ),
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
