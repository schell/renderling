//! Frame creation and clearing.
//!
//! Contains graph nodes for creating and clearing frames, as well as a
//! `PostRenderBuffer` resource that holds a copy of the last frame's buffer.
use std::{ops::Deref, sync::Arc};

use glam::UVec2;
use moongraph::*;
use snafu::prelude::*;

use crate::{
    math::Vec4, BackgroundColor, BufferDimensions, CopiedTextureBuffer, DepthTexture, Device,
    Queue, WgpuStateError,
};

#[derive(Debug, Snafu)]
pub enum FrameError {
    #[snafu(display("{}", source))]
    Texture { source: crate::TextureError },
}

pub fn default_frame_texture_view(
    frame_texture: &wgpu::Texture,
) -> (wgpu::TextureView, wgpu::TextureFormat) {
    let format = frame_texture.format().add_srgb_suffix();
    let view = frame_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("WgpuState::default_frame_texture_view"),
        format: Some(format),
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    });
    (view, format)
}

pub struct FrameTextureView {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}

impl Deref for FrameTextureView {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

pub(crate) enum FrameSurface {
    Surface(wgpu::SurfaceTexture),
    Texture(Arc<wgpu::Texture>),
}

/// Abstracts over window and texture render targets.
///
/// Either a [`SurfaceTexture`] or a [`Texture`].
pub struct Frame {
    pub(crate) device: crate::Device,
    pub(crate) queue: crate::Queue,
    pub(crate) surface: FrameSurface,
    pub(crate) size: UVec2,
}

impl Frame {
    /// Returns the underlying texture of this target.
    pub fn texture(&self) -> &wgpu::Texture {
        match &self.surface {
            FrameSurface::Surface(s) => &s.texture,
            FrameSurface::Texture(t) => &t,
        }
    }

    pub fn view(&self) -> wgpu::TextureView {
        let texture = self.texture();
        let format = texture.format().add_srgb_suffix();
        texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Frame::view"),
            format: Some(format),
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        })
    }

    pub fn copy_to_buffer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) -> CopiedTextureBuffer {
        let dimensions = BufferDimensions::new(4, 1, width as usize, height as usize);
        // The output buffer lets us retrieve the self as an array
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("RenderTarget::copy_to_buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("post render screen capture encoder"),
        });
        let texture = self.texture();
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

        queue.submit(std::iter::once(encoder.finish()));

        CopiedTextureBuffer {
            dimensions,
            buffer,
            format: texture.format(),
        }
    }

    /// Read the current frame buffer into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// The resulting image will be in the color space of the frame.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub fn read_image(&self) -> Result<image::RgbaImage, FrameError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        let is_srgb = self.texture().format().is_srgb();
        let img = if is_srgb {
            buffer.into_srgba(&self.device).context(TextureSnafu)?
        } else {
            buffer
                .into_linear_rgba(&self.device)
                .context(TextureSnafu)?
        };
        Ok(img)
    }

    /// Read the frame into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// The resulting image will be in a linear color space.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub fn read_srgb_image(&self) -> Result<image::RgbaImage, FrameError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        buffer.into_srgba(&self.device).context(TextureSnafu)
    }
    /// Read the frame into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// The resulting image will be in a linear color space.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub fn read_linear_image(&self) -> Result<image::RgbaImage, FrameError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        buffer.into_linear_rgba(&self.device).context(TextureSnafu)
    }

    /// If self is `TargetFrame::Surface` this presents the surface frame.
    ///
    /// If self is a `TargetFrame::Texture` this is a noop.
    pub fn present(self) {
        match self.surface {
            FrameSurface::Surface(s) => s.present(),
            FrameSurface::Texture(_) => {}
        }
    }
}

/// Perform a clearing render pass on a frame and/or a depth texture.
///
/// ## Note
/// This clears the depth to 1.0.
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

    let frame_views = frame_views
        .into_iter()
        .map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })
        })
        .collect::<Vec<_>>();
    let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &frame_views,
        depth_stencil_attachment: depth_view.map(|view| wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    });
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
}

/// Render graph node to conduct a clear pass on the global frame and depth
/// textures.
pub fn clear_frame_and_depth(
    (device, queue, frame_view, depth, color): (
        View<Device, NoDefault>,
        View<Queue, NoDefault>,
        View<FrameTextureView, NoDefault>,
        View<DepthTexture, NoDefault>,
        View<BackgroundColor, NoDefault>,
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
    (device, queue, depth): (
        View<Device, NoDefault>,
        View<Queue, NoDefault>,
        View<DepthTexture, NoDefault>,
    ),
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

/// Consume and present the screen frame to the screen.
pub fn present(frame: Move<Frame>) -> Result<(), WgpuStateError> {
    let frame = frame.into();
    frame.present();
    Ok(())
}
