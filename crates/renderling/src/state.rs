//! Rendering state management.
use snafu::prelude::*;
use std::sync::{Arc, RwLock};

#[cfg(feature = "text")]
use ab_glyph::FontArc;

use crate::{
    forward::ForwardPipeline, linkage::conduct_clear_pass, renderer::Renderling, ui::UiPipeline,
};

#[derive(Debug, Snafu)]
pub enum WgpuStateError {
    #[snafu(display("cannot create adaptor"))]
    CannotCreateAdaptor,

    #[snafu(display("cannot request device: {}", source))]
    CannotRequestDevice { source: wgpu::RequestDeviceError },

    #[snafu(display("surface is incompatible with adapter"))]
    IncompatibleSurface,

    #[snafu(display("could not create surface: {}", source))]
    CreateSurface { source: wgpu::CreateSurfaceError },

    #[snafu(display("missing surface texture: {}", source))]
    MissingSurfaceTexture { source: wgpu::SurfaceError },

    #[snafu(display("{}", source))]
    Texture { source: crate::TextureError },

    #[snafu(display("missing the target frame - call WgpuState::prepare_target_frame first"))]
    MissingTargetFrame,

    #[snafu(display("could not map buffer"))]
    CouldNotMapBuffer { source: wgpu::BufferAsyncError },

    #[snafu(display("could not convert image buffer"))]
    CouldNotConvertImageBuffer,
}

pub enum RenderTarget {
    Surface {
        surface: wgpu::Surface,
        surface_config: wgpu::SurfaceConfiguration,
    },
    Texture {
        texture: Arc<wgpu::Texture>,
    },
}

impl From<wgpu::Texture> for RenderTarget {
    fn from(value: wgpu::Texture) -> Self {
        RenderTarget::Texture {
            texture: Arc::new(value),
        }
    }
}

impl RenderTarget {
    pub fn resize(&mut self, width: u32, height: u32, device: &wgpu::Device) {
        match self {
            RenderTarget::Surface {
                surface,
                surface_config,
            } => {
                surface_config.width = width;
                surface_config.height = height;
                surface.configure(&device, &surface_config);
            }
            RenderTarget::Texture { texture } => {
                let texture_desc = wgpu::TextureDescriptor {
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    label: None,
                    view_formats: &[]
                };
                *texture = Arc::new(device.create_texture(&texture_desc));
            }
        }
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        match self {
            RenderTarget::Surface { surface_config, .. } => surface_config.format,
            RenderTarget::Texture { .. } => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }

    /// Get the current render target frame.
    ///
    /// Errs if the render target is a surface and there was an error getting the next swapchain texture.
    pub fn get_current_frame(&self) -> Result<Frame, WgpuStateError> {
        match self {
            RenderTarget::Surface { surface, .. } => {
                let surface_texture = surface
                    .get_current_texture()
                    .context(MissingSurfaceTextureSnafu)?;
                Ok(Frame::Surface(surface_texture))
            }
            RenderTarget::Texture { texture, .. } => Ok(Frame::Texture(texture.clone())),
        }
    }
}

/// Abstracts over window and texture render targets.
///
/// Either a [`SurfaceTexture`] or a [`Texture`].
pub enum Frame {
    Surface(wgpu::SurfaceTexture),
    Texture(Arc<wgpu::Texture>),
}

impl Frame {
    /// Returns the underlying texture of this target.
    pub fn texture(&self) -> &wgpu::Texture {
        match self {
            Frame::Surface(s) => &s.texture,
            Frame::Texture(t) => &t,
        }
    }

    /// If self is `TargetFrame::Surface` this presents the surface frame.
    ///
    /// If self is a `TargetFrame::Texture` this is a noop.
    pub fn present(self) {
        match self {
            Frame::Surface(s) => s.present(),
            Frame::Texture(_) => {}
        }
    }
}

pub struct WgpuState {
    pub target: RenderTarget,
    pub current_frame: Option<Frame>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub size: Arc<RwLock<(u32, u32)>>,
    pub depth_texture: crate::Texture,
    pub default_background_color: wgpu::Color,
}

impl WgpuState {
    pub async fn try_new_headless(width: u32, height: u32) -> Result<Self, WgpuStateError> {
        let size = (width, height);

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = if cfg!(target_arch = "wasm32") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .context(CannotCreateAdaptorSnafu)?;
        let limits = if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits,
                    label: None,
                },
                None, // Trace path
            )
            .await
            .context(CannotRequestDeviceSnafu)?;

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };
        let texture = Arc::new(device.create_texture(&texture_desc));
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);
        let target = RenderTarget::Texture { texture };

        Ok(WgpuState {
            target,
            current_frame: None,
            device: Arc::new(device),
            queue: Arc::new(queue),
            size: Arc::new(RwLock::new(size)),
            depth_texture,
            default_background_color: wgpu::Color::TRANSPARENT,
        })
    }

    #[cfg(feature = "raw-window-handle")]
    pub async fn try_new<W>(width: u32, height: u32, window: &W) -> Result<Self, WgpuStateError>
    where
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    {
        let size = (width, height);

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = if cfg!(target_arch = "wasm32") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let surface = unsafe { instance.create_surface(window) }.context(CreateSurfaceSnafu)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context(CannotCreateAdaptorSnafu)?;
        let limits = if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits,
                    label: None,
                },
                None, // Trace path
            )
            .await
            .context(CannotRequestDeviceSnafu)?;

        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .context(IncompatibleSurfaceSnafu)?;
        surface.configure(&device, &surface_config);
        let target = RenderTarget::Surface {
            surface,
            surface_config,
        };
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);

        Ok(WgpuState {
            target,
            current_frame: None,
            device: Arc::new(device),
            queue: Arc::new(queue),
            size: Arc::new(RwLock::new(size)),
            depth_texture,
            default_background_color: wgpu::Color::TRANSPARENT,
        })
    }

    #[cfg(feature = "raw-window-handle")]
    pub fn from_handle(
        window_handle: &(impl raw_window_handle::HasRawWindowHandle
              + raw_window_handle::HasRawDisplayHandle),
        width: u32,
        height: u32,
    ) -> Result<Self, WgpuStateError> {
        futures_lite::future::block_on(WgpuState::try_new(width, height, window_handle))
    }

    #[cfg(feature = "winit")]
    pub fn from_window(window: &winit::window::Window) -> Result<Self, WgpuStateError> {
        let inner_size = window.inner_size();
        Self::from_handle(window, inner_size.width, inner_size.height)
    }

    pub fn headless(width: u32, height: u32) -> Result<Self, WgpuStateError> {
        futures_lite::future::block_on(Self::try_new_headless(width, height))
    }

    pub fn get_size(&self) -> (u32, u32) {
        *self.size.read().unwrap()
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        *self.size.write().unwrap() = size;
        self.target.resize(size.0, size.1, &self.device);
        self.depth_texture = crate::Texture::create_depth_texture(&self.device, size.0, size.1);
    }

    #[cfg(feature = "image")]
    pub fn create_texture<P>(
        &self,
        label: Option<&str>,
        img: &image::ImageBuffer<P, Vec<u8>>,
    ) -> Result<crate::Texture, WgpuStateError>
    where
        P: image::PixelWithColorType,
        image::ImageBuffer<P, Vec<u8>>: image::GenericImage + std::ops::Deref<Target = [u8]>,
    {
        let name = label.unwrap_or("unknown");
        crate::Texture::from_image_buffer(
            &self.device,
            &self.queue,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
        )
        .context(TextureSnafu)
    }

    /// Loads [`current_target_frame`] with a [`TargetFrame`], if possible.
    ///
    /// Errs if the render target is a surface and the next swapchain texture cannot be produced.
    pub fn prepare_target_frame(&mut self) -> Result<(), WgpuStateError> {
        let current_surface_frame = self.target.get_current_frame()?;
        // Save the frame for subsequent systems' render passes
        self.current_frame = Some(current_surface_frame);
        Ok(())
    }

    /// Get the next frame texture and depth texture, if possible.
    pub fn next_frame(
        &mut self,
    ) -> Result<(Arc<wgpu::TextureView>, Arc<wgpu::TextureView>), WgpuStateError> {
        if self.current_frame.is_none() {
            self.prepare_target_frame()?;
        }
        let frame = self.current_frame.as_ref().unwrap();
        let frame_view = Arc::new(Self::default_frame_texture_view(frame.texture()));
        Ok((frame_view, self.depth_texture.view.clone()))
    }

    /// Get the next frame texture and depth texture, if possible, and clear both
    /// with a clearing render pass.
    ///
    /// This is equivalent to using [`WgpuState::next_frame`] and then calling [`WgpuState::clear`]
    /// on the result:
    /// ```rust
    /// use renderling::{WgpuState};
    ///
    /// let mut gpu = WgpuState::headless(100, 100).unwrap();
    /// let (frame, depth) = gpu.next_frame().unwrap();
    /// gpu.clear(Some(&frame), Some(&depth));
    /// ```
    pub fn next_frame_cleared(
        &mut self,
    ) -> Result<(Arc<wgpu::TextureView>, Arc<wgpu::TextureView>), WgpuStateError> {
        let (frame, depth) = self.next_frame()?;
        self.clear(Some(&frame), Some(&depth));
        Ok((frame, depth))
    }

    pub fn default_frame_texture_view(frame_texture: &wgpu::Texture) -> wgpu::TextureView {
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

    /// Clear the given texture and/or depth texture.
    pub fn clear(
        &self,
        frame_view: Option<&wgpu::TextureView>,
        depth_view: Option<&wgpu::TextureView>,
    ) {
        conduct_clear_pass(
            &self.device,
            &self.queue,
            Some("WgpuState::clear"),
            frame_view,
            depth_view,
            self.default_background_color,
        )
    }

    /// Present the current frame, if possible.
    pub fn present(&mut self) -> Result<(), WgpuStateError> {
        let frame = self.current_frame.take().context(MissingTargetFrameSnafu)?;
        frame.present();
        Ok(())
    }

}

/// Helper for retreiving a rendered frame in a texture.
pub struct BufferDimensions {
    pub width: usize,
    pub height: usize,
    pub unpadded_bytes_per_row: usize,
    pub padded_bytes_per_row: usize,
}

impl BufferDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = std::mem::size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

/// Helper for retreiving a rendered frame.
pub struct PostRenderBuffer {
    pub dimensions: BufferDimensions,
    pub buffer: wgpu::Buffer,
}

impl PostRenderBuffer {
    #[cfg(feature = "image")]
    /// Convert the post render buffer into an RgbaImage.
    pub async fn convert_to_rgba(self) -> Result<image::RgbaImage, WgpuStateError> {
        let buffer_slice = self.buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, {
            move |result| {
                tx.send(result).unwrap();
            }
        });
        loop {
            if let Ok(result) = rx.try_recv() {
                result.context(CouldNotMapBufferSnafu)?;
                break;
            } else {
                futures_lite::future::yield_now().await;
            }
        }

        let padded_buffer = buffer_slice.get_mapped_range();
        let mut unpadded_buffer = vec![];
        // from the padded_buffer we write just the unpadded bytes into the
        // unpadded_buffer
        for chunk in padded_buffer.chunks(self.dimensions.padded_bytes_per_row) {
            unpadded_buffer.extend_from_slice(&chunk[..self.dimensions.unpadded_bytes_per_row]);
        }
        let img_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(
                self.dimensions.width as u32,
                self.dimensions.height as u32,
                unpadded_buffer,
            )
            .context(CouldNotConvertImageBufferSnafu)?;
        Ok(image::DynamicImage::ImageRgba8(img_buffer).to_rgba8())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn headless() {
        let _wgpu_state = WgpuState::headless(100, 100).unwrap();
    }
}
