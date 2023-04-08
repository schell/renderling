//! `wgpu` state management.
//!
//! Contains stuff for initializing [`Renderling`]s and doing GPU things like
//! building pipelines, etc.
use snafu::prelude::*;
use std::sync::Arc;

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
                    view_formats: &[],
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
    /// Errs if the render target is a surface and there was an error getting
    /// the next swapchain texture.
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

pub type CreateSurfaceFn<'a> =
    Box<dyn FnOnce(&wgpu::Instance) -> Result<wgpu::Surface, WgpuStateError> + 'a>;

pub async fn new_device_queue_and_target<'a>(
    width: u32,
    height: u32,
    create_surface: Option<
        impl FnOnce(&wgpu::Instance) -> Result<wgpu::Surface, WgpuStateError> + 'a,
    >,
) -> (wgpu::Device, wgpu::Queue, RenderTarget) {
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let backends = wgpu::Backends::all();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler: wgpu::Dx12Compiler::default(),
    });

    fn limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
        if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        }
    }

    async fn adapter(
        instance: &wgpu::Instance,
        compatible_surface: Option<&wgpu::Surface>,
    ) -> wgpu::Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface,
                force_fallback_adapter: false,
            })
            .await
            .unwrap()
    }

    async fn device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                        | wgpu::Features::MULTI_DRAW_INDIRECT
                        // this one is a funny requirement, it seems it is needed if using storage buffers in
                        // vertex shaders, even if those shaders are read-only
                        | wgpu::Features::VERTEX_WRITABLE_STORAGE,
                    limits: limits(&adapter),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap()
    }

    if let Some(create_surface) = create_surface {
        let surface = (create_surface)(&instance).unwrap();
        let adapter = adapter(&instance, Some(&surface)).await;
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        let (device, queue) = device(&adapter).await;
        surface.configure(&device, &surface_config);
        let target = RenderTarget::Surface {
            surface,
            surface_config,
        };
        (device, queue, target)
    } else {
        let adapter = adapter(&instance, None).await;
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
        let (device, queue) = device(&adapter).await;
        let texture = Arc::new(device.create_texture(&texture_desc));
        let target = RenderTarget::Texture { texture };
        (device, queue, target)
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
