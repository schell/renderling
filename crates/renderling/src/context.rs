//! Rendering context initialization and frame management.
use std::{ops::Deref, sync::Arc};

use glam::UVec2;
use snafu::prelude::*;

use crate::{
    stage::Stage,
    texture::{BufferDimensions, CopiedTextureBuffer, Texture, TextureError},
};

enum RenderTargetInner {
    Surface {
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
    },
    Texture {
        texture: Arc<wgpu::Texture>,
    },
}

#[repr(transparent)]
/// Either a surface or a texture.
///
/// Will be a surface if the context was created with a window or canvas.
///
/// Will be a texture if the context is headless.
pub struct RenderTarget(RenderTargetInner);

impl From<wgpu::Texture> for RenderTarget {
    fn from(value: wgpu::Texture) -> Self {
        RenderTarget(RenderTargetInner::Texture {
            texture: Arc::new(value),
        })
    }
}

impl RenderTarget {
    pub fn resize(&mut self, width: u32, height: u32, device: &wgpu::Device) {
        match &mut self.0 {
            RenderTargetInner::Surface {
                surface,
                surface_config,
            } => {
                surface_config.width = width;
                surface_config.height = height;
                surface.configure(&device, &surface_config);
            }
            RenderTargetInner::Texture { texture } => {
                let usage = texture.usage();
                let format = texture.format();
                let texture_desc = wgpu::TextureDescriptor {
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    label: Some("RenderTarget texture"),
                    view_formats: &[],
                };
                *texture = Arc::new(device.create_texture(&texture_desc));
            }
        }
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        match &self.0 {
            RenderTargetInner::Surface { surface_config, .. } => surface_config.format,
            RenderTargetInner::Texture { texture } => texture.format(),
        }
    }

    pub fn is_headless(&self) -> bool {
        match &self.0 {
            RenderTargetInner::Surface { .. } => false,
            RenderTargetInner::Texture { .. } => true,
        }
    }
}

fn limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
    if cfg!(target_arch = "wasm32") {
        wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
    } else {
        wgpu::Limits::default()
    }
}

async fn adapter<'window>(
    instance: &wgpu::Instance,
    compatible_surface: Option<&wgpu::Surface<'window>>,
) -> Result<wgpu::Adapter, ContextError> {
    log::trace!(
        "creating adapter for a {} context",
        if compatible_surface.is_none() {
            "headless"
        } else {
            "surface-based"
        }
    );
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface,
            force_fallback_adapter: false,
        })
        .await
        .context(CannotCreateAdaptorSnafu)?;
    let info = adapter.get_info();
    log::trace!(
        "using adapter: '{}' backend:{:?} driver:'{}'",
        info.name,
        info.backend,
        info.driver
    );
    Ok(adapter)
}

async fn device(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                        | wgpu::Features::MULTI_DRAW_INDIRECT
                        // this one is a funny requirement, it seems it is needed if using storage buffers in
                        // vertex shaders, even if those shaders are read-only
                        | wgpu::Features::VERTEX_WRITABLE_STORAGE, //| wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                //// when debugging rust-gpu shader miscompilation it's nice to have this
                //| wgpu::Features::SPIRV_SHADER_PASSTHROUGH
                required_limits: limits(&adapter),
                label: None,
            },
            None, // Trace path
        )
        .await
}

fn new_instance() -> wgpu::Instance {
    log::trace!(
        "creating instance - available backends: {:#?}",
        wgpu::Instance::enabled_backend_features()
    );
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let backends = wgpu::Backends::PRIMARY;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapters = instance.enumerate_adapters(backends);
        log::trace!("available adapters: {adapters:#?}");
    }

    instance
}

async fn new_windowed_adapter_device_queue(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
    window: impl Into<wgpu::SurfaceTarget<'static>>,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), ContextError> {
    let surface = instance
        .create_surface(window)
        .map_err(|e| ContextError::CreateSurface { source: e })?;
    let adapter = adapter(&instance, Some(&surface)).await?;
    let surface_caps = surface.get_capabilities(&adapter);
    let fmt = if surface_caps
        .formats
        .contains(&wgpu::TextureFormat::Rgba8UnormSrgb)
    {
        wgpu::TextureFormat::Rgba8UnormSrgb
    } else {
        surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0])
    };
    let view_fmts = if fmt.is_srgb() {
        vec![]
    } else {
        vec![fmt.add_srgb_suffix()]
    };
    log::debug!("surface capabilities: {surface_caps:#?}");
    let mut surface_config = surface
        .get_default_config(&adapter, width, height)
        .context(IncompatibleSurfaceSnafu)?;
    surface_config.view_formats = view_fmts;
    let (device, queue) = device(&adapter).await.context(CannotRequestDeviceSnafu)?;
    surface.configure(&device, &surface_config);
    let target = RenderTarget(RenderTargetInner::Surface {
        surface,
        surface_config,
    });
    Ok((adapter, device, queue, target))
}

async fn new_headless_device_queue_and_target(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), ContextError> {
    let adapter = adapter(&instance, None).await?;
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
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        label: None,
        view_formats: &[],
    };
    let (device, queue) = device(&adapter).await.context(CannotRequestDeviceSnafu)?;
    let texture = Arc::new(device.create_texture(&texture_desc));
    let target = RenderTarget(RenderTargetInner::Texture { texture });
    Ok((adapter, device, queue, target))
}

#[derive(Debug, Snafu)]
pub enum ContextError {
    #[snafu(display("missing surface texture: {}", source))]
    Surface { source: wgpu::SurfaceError },

    #[snafu(display("cannot create adaptor"))]
    CannotCreateAdaptor,

    #[snafu(display("cannot request device: {}", source))]
    CannotRequestDevice { source: wgpu::RequestDeviceError },

    #[snafu(display("surface is incompatible with adapter"))]
    IncompatibleSurface,

    #[snafu(display("could not create surface: {}", source))]
    CreateSurface { source: wgpu::CreateSurfaceError },
}

/// A thin wrapper over [`wgpu::TextureView`] returned by [`Frame::view`].
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

/// Represents the current frame of a render target.
///
/// Returned by [`Context::get_next_frame`].
pub struct Frame {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
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
    pub fn read_image(&self) -> Result<image::RgbaImage, TextureError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        let is_srgb = self.texture().format().is_srgb();
        let img = if is_srgb {
            buffer.into_srgba(&self.device)?
        } else {
            buffer.into_linear_rgba(&self.device)?
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
    pub fn read_srgb_image(&self) -> Result<image::RgbaImage, TextureError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        buffer.into_srgba(&self.device)
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
    pub fn read_linear_image(&self) -> Result<image::RgbaImage, TextureError> {
        let buffer = self.copy_to_buffer(&self.device, &self.queue, self.size.x, self.size.y);
        buffer.into_linear_rgba(&self.device)
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
pub(crate) fn conduct_clear_pass<'a>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_views: impl IntoIterator<Item = &'a wgpu::TextureView>,
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

/// Contains the adapter, device, queue and [`RenderTarget`].
///
/// A `Context` is created to initialize rendering to a window, canvas or
/// texture.
///
/// ```
/// use renderling::Context;
///
/// let ctx = Context::headless(100, 100);
/// ```
pub struct Context {
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    render_target: RenderTarget,
    size: UVec2,
}

impl Context {
    pub fn new(
        target: RenderTarget,
        adapter: impl Into<Arc<wgpu::Adapter>>,
        device: impl Into<Arc<wgpu::Device>>,
        queue: impl Into<Arc<wgpu::Queue>>,
        size: UVec2,
    ) -> Self {
        Self {
            adapter: adapter.into(),
            device: device.into(),
            queue: queue.into(),
            render_target: target,
            size,
        }
    }

    pub async fn try_new_headless(width: u32, height: u32) -> Result<Self, ContextError> {
        log::trace!("creating headless context of size ({width}, {height})");
        let size = UVec2::new(width, height);
        let instance = new_instance();
        let (adapter, device, queue, target) =
            new_headless_device_queue_and_target(width, height, &instance).await?;
        Ok(Self::new(target, adapter, device, queue, size))
    }

    pub async fn try_from_raw_window(
        width: u32,
        height: u32,
        window: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> Result<Self, ContextError> {
        let size = UVec2::new(width, height);
        let instance = new_instance();
        let (adapter, device, queue, target) =
            new_windowed_adapter_device_queue(width, height, &instance, window).await?;
        Ok(Self::new(target, adapter, device, queue, size))
    }

    #[cfg(feature = "winit")]
    pub async fn from_window_async(window: Arc<winit::window::Window>) -> Self {
        let inner_size = window.inner_size();
        Self::try_from_raw_window(inner_size.width, inner_size.height, window)
            .await
            .unwrap()
    }

    #[cfg(all(feature = "winit", not(target_arch = "wasm32")))]
    /// Create a new context from a `winit::window::Window`, blocking until it
    /// is created.
    ///
    /// ## Panics
    /// Panics if the context cannot be created.
    pub fn from_window(window: Arc<winit::window::Window>) -> Self {
        futures_lite::future::block_on(Self::from_window_async(window))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_from_raw_window_handle(
        window_handle: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, ContextError> {
        futures_lite::future::block_on(Self::try_from_raw_window(width, height, window_handle))
    }

    #[cfg(all(feature = "winit", not(target_arch = "wasm32")))]
    pub fn try_from_window(window: Arc<winit::window::Window>) -> Result<Self, ContextError> {
        let inner_size = window.inner_size();
        Self::try_from_raw_window_handle(window, inner_size.width, inner_size.height)
    }

    /// Create a new headless renderer.
    ///
    /// ## Panics
    /// This function will panic if an adapter cannot be found. For example this
    /// would happen on machines without a GPU.
    pub fn headless(width: u32, height: u32) -> Self {
        futures_lite::future::block_on(Self::try_new_headless(width, height)).unwrap()
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }

    pub fn set_size(&mut self, size: UVec2) {
        self.size = size;
        let (device, _) = self.get_device_and_queue_owned();
        self.render_target.resize(size.x, size.y, &device);
    }

    /// Convenience method for creating textures from an image buffer.
    pub fn create_texture<P>(
        &self,
        label: Option<&str>,
        img: &image::ImageBuffer<P, Vec<u8>>,
    ) -> Result<Texture, TextureError>
    where
        P: image::PixelWithColorType,
        image::ImageBuffer<P, Vec<u8>>: image::GenericImage + std::ops::Deref<Target = [u8]>,
    {
        let name = label.unwrap_or("unknown");
        let device = &self.device;
        let queue = &self.queue;
        Texture::from_image_buffer(
            device,
            queue,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
            None,
        )
    }

    pub fn texture_from_wgpu_tex(
        &self,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> Texture {
        Texture::from_wgpu_tex(self.get_device(), texture, sampler, None)
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    /// Returns a the adapter in an owned wrapper.
    pub fn get_adapter_owned(&self) -> Arc<wgpu::Adapter> {
        self.adapter.clone()
    }

    /// Returns a pair of the device and queue in an owned wrapper.
    pub fn get_device_and_queue_owned(&self) -> (Arc<wgpu::Device>, Arc<wgpu::Queue>) {
        (self.device.clone(), self.queue.clone())
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }

    /// Get the next frame from the render target.
    ///
    /// A surface context (window or canvas) will return the next swapchain
    /// texture.
    ///
    /// A headless context will return the underlying headless texture.
    ///
    /// Errs if the render target is a surface and there was an error getting
    /// the next swapchain texture. This can happen if the frame has already
    /// been acquired.
    pub fn get_next_frame(&self) -> Result<Frame, ContextError> {
        Ok(Frame {
            device: self.device.clone(),
            queue: self.queue.clone(),
            size: self.size,
            surface: match &self.render_target.0 {
                RenderTargetInner::Surface { surface, .. } => {
                    let surface_texture = surface.get_current_texture().context(SurfaceSnafu)?;
                    FrameSurface::Surface(surface_texture)
                }
                RenderTargetInner::Texture { texture, .. } => {
                    FrameSurface::Texture(texture.clone())
                }
            },
        })
    }

    /// Create and return a new [`Stage`] renderer.
    pub fn new_stage(&self) -> Stage {
        Stage::new(&self)
    }
}
