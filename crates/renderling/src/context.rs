//! Builds the UI pipeline and manages resources.
use glam::{UVec2, Vec4};
use snafu::prelude::*;
use std::{ops::Deref, sync::Arc};

use crate::{new_default_instance, RenderTarget, Stage, TextureError, WgpuStateError};

#[derive(Debug, Snafu)]
pub enum RenderlingError {
    #[snafu(display("{}", source))]
    Texture { source: crate::TextureError },

    #[snafu(display("missing surface texture: {}", source))]
    MissingSurfaceTexture { source: wgpu::SurfaceError },

    #[snafu(display("could not map buffer"))]
    CouldNotMapBuffer { source: wgpu::BufferAsyncError },

    #[snafu(display("could not convert image buffer"))]
    CouldNotConvertImageBuffer,

    #[snafu(display("missing the current frame"))]
    RenderTargetMissingFrame,

    #[snafu(display(
        "missing a material uniform, have you called Renderling::update at least once?"
    ))]
    MissingDefaultMaterial,

    #[snafu(display("missing a camera, this should not have happened"))]
    MissingCamera,

    #[cfg(feature = "gltf")]
    #[snafu(display("gltf import failed: {}", source))]
    GltfImport { source: gltf::Error },

    #[snafu(display(
        "Missing PostRenderBuffer resource. Ensure a node that creates PostRenderBuffer (like \
         PostRenderbufferCreate) is present in the graph: {source}"
    ))]
    MissingPostRenderBuffer { source: moongraph::GraphError },

    #[snafu(display("Timeout while waiting for a screengrab"))]
    ScreenGrabTimeout { source: TextureError },

    #[snafu(display("{source}"))]
    State { source: WgpuStateError },
}

impl From<WgpuStateError> for RenderlingError {
    fn from(source: WgpuStateError) -> Self {
        Self::State { source }
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Device`.
#[derive(Clone)]
pub struct Device(pub Arc<wgpu::Device>);

impl Deref for Device {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Arc<wgpu::Device>> for &Device {
    fn into(self) -> Arc<wgpu::Device> {
        self.0.clone()
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Queue`.
#[derive(Clone)]
pub struct Queue(pub Arc<wgpu::Queue>);

impl Deref for Queue {
    type Target = wgpu::Queue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Arc<wgpu::Queue>> for &Queue {
    fn into(self) -> Arc<wgpu::Queue> {
        self.0.clone()
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Adapter`.
#[derive(Clone)]
pub struct Adapter(pub Arc<wgpu::Adapter>);

impl Deref for Adapter {
    type Target = wgpu::Adapter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A type for the screen/frame size.
#[derive(Clone, Copy, Debug)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

/// A depth texture.
pub struct DepthTexture(pub(crate) crate::Texture);

impl Deref for DepthTexture {
    type Target = crate::Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DepthTexture {
    pub fn width(&self) -> u32 {
        self.0.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.0.texture.height()
    }

    /// Converts the depth texture into an image.
    ///
    /// Assumes the format is single channel 32bit.
    pub fn into_image(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<image::DynamicImage> {
        let depth_copied_buffer = crate::Texture::read(
            &self.texture,
            &device,
            &queue,
            self.width() as usize,
            self.height() as usize,
            1,
            4,
        );
        let pixels = depth_copied_buffer.pixels(device);
        let pixels: Vec<f32> = bytemuck::cast_slice(&pixels).to_vec();
        let img_buffer: image::ImageBuffer<image::Luma<f32>, Vec<f32>> =
            image::ImageBuffer::from_raw(self.width(), self.height(), pixels)?;
        Some(image::DynamicImage::from(img_buffer))
    }
}

/// The global background color.
pub struct BackgroundColor(pub Vec4);

/// Contains the adapter, device, queue and [`RenderTarget`].
pub struct Context {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub render_target: RenderTarget,
    pub size: UVec2,
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
            adapter: Adapter(adapter.into()),
            device: Device(device.into()),
            queue: Queue(queue.into()),
            render_target: target,
            size,
        }
    }

    pub async fn try_new_headless(width: u32, height: u32) -> Result<Self, RenderlingError> {
        let size = UVec2::new(width, height);
        let instance = new_default_instance();
        let (adapter, device, queue, target) =
            crate::state::new_headless_device_queue_and_target(width, height, &instance).await?;
        Ok(Self::new(target, adapter, device, queue, size))
    }

    pub async fn try_from_raw_window(
        width: u32,
        height: u32,
        window: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> Result<Self, RenderlingError> {
        let size = UVec2::new(width, height);
        let instance = new_default_instance();
        let (adapter, device, queue, target) =
            crate::state::new_windowed_adapter_device_queue(width, height, &instance, window)
                .await?;
        Ok(Self::new(target, adapter, device, queue, size))
    }

    pub async fn from_window_async(window: Arc<winit::window::Window>) -> Self {
        let inner_size = window.inner_size();
        Self::try_from_raw_window(inner_size.width, inner_size.height, window)
            .await
            .unwrap()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_from_raw_window_handle(
        window_handle: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, RenderlingError> {
        futures_lite::future::block_on(Self::try_from_raw_window(width, height, window_handle))
    }

    #[cfg(all(feature = "winit", not(target_arch = "wasm32")))]
    pub fn try_from_window(window: Arc<winit::window::Window>) -> Result<Self, RenderlingError> {
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

    pub fn create_texture<P>(
        &self,
        label: Option<&str>,
        img: &image::ImageBuffer<P, Vec<u8>>,
    ) -> Result<crate::Texture, RenderlingError>
    where
        P: image::PixelWithColorType,
        image::ImageBuffer<P, Vec<u8>>: image::GenericImage + std::ops::Deref<Target = [u8]>,
    {
        let name = label.unwrap_or("unknown");
        let device = &self.device;
        let queue = &self.queue;
        crate::Texture::from_image_buffer(
            device,
            queue,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
            None,
        )
        .context(TextureSnafu)
    }

    pub fn texture_from_wgpu_tex(
        &self,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> crate::Texture {
        crate::Texture::from_wgpu_tex(self.get_device(), texture, sampler, None)
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
    pub fn get_adapter_owned(&self) -> crate::Adapter {
        self.adapter.clone()
    }

    /// Returns a pair of the device and queue in an owned wrapper.
    pub fn get_device_and_queue_owned(&self) -> (crate::Device, crate::Queue) {
        (self.device.clone(), self.queue.clone())
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }
    /// Get the current render target frame.
    ///
    /// Errs if the render target is a surface and there was an error getting
    /// the next swapchain texture.
    pub fn get_current_frame(&self) -> Result<crate::frame::Frame, RenderlingError> {
        Ok(crate::frame::Frame {
            device: self.device.clone(),
            queue: self.queue.clone(),
            size: self.size,
            surface: match &self.render_target {
                RenderTarget::Surface { surface, .. } => {
                    let surface_texture = surface
                        .get_current_texture()
                        .context(MissingSurfaceTextureSnafu)?;
                    crate::frame::FrameSurface::Surface(surface_texture)
                }
                RenderTarget::Texture { texture, .. } => {
                    crate::frame::FrameSurface::Texture(texture.clone())
                }
            },
        })
    }

    pub fn new_stage(&self) -> Stage {
        Stage::new(&self)
    }

    // /// Run the render graph.
    // pub fn render(&mut self) -> Result<(), RenderlingError> {
    //     self.graph.run().context(GraphSnafu)
    // }

    // /// Run the render graph with a local render function.
    // pub fn render_local<Input, Output, E>(
    //     &mut self,
    //     f: impl FnOnce(Input) -> Result<Output, E>,
    // ) -> Result<(), RenderlingError>
    // where
    //     Input: moongraph::Edges + std::any::Any + Send + Sync,
    //     Output: moongraph::NodeResults + std::any::Any + Send + Sync,
    //     E: ToString,
    // {
    //     self.graph.run_with_local(f).context(GraphSnafu)?;
    //     Ok(())
    // }
}
