//! Rendering context initialization
//!
//! This module contains [`Context`] initialization and frame management.
//! This module provides the setup and management of rendering targets,
//! frames, and surface configurations.
use core::fmt::Debug;
use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

use glam::{UVec2, UVec3};
use snafu::prelude::*;

use crate::{
    stage::Stage,
    texture::{BufferDimensions, CopiedTextureBuffer, Texture, TextureError},
    ui::Ui,
};

pub use craballoc::runtime::WgpuRuntime;

/// Represents the internal structure of a render target, which can either be a surface or a texture.
pub(crate) enum RenderTargetInner {
    Surface {
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
    },
    Texture {
        texture: Arc<wgpu::Texture>,
    },
}

#[repr(transparent)]
/// Represents a render target that can either be a surface or a texture.
/// It will be a surface if the context was created with a window or canvas,
/// and a texture if the context is headless.
pub struct RenderTarget(pub(crate) RenderTargetInner);

/// Converts a `wgpu::Texture` into a `RenderTarget`.
impl From<wgpu::Texture> for RenderTarget {
    fn from(value: wgpu::Texture) -> Self {
        RenderTarget(RenderTargetInner::Texture {
            texture: Arc::new(value),
        })
    }
}

impl RenderTarget {
    /// Resizes the render target to the specified width and height using the provided device.
    pub fn resize(&mut self, width: u32, height: u32, device: &wgpu::Device) {
        match &mut self.0 {
            RenderTargetInner::Surface {
                surface,
                surface_config,
            } => {
                surface_config.width = width;
                surface_config.height = height;
                surface.configure(device, surface_config)
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

    /// Returns the format of the render target.
    pub fn format(&self) -> wgpu::TextureFormat {
        match &self.0 {
            RenderTargetInner::Surface { surface_config, .. } => surface_config.format,
            RenderTargetInner::Texture { texture } => texture.format(),
        }
    }

    /// Checks if the render target is headless (i.e., a texture).
    pub fn is_headless(&self) -> bool {
        match &self.0 {
            RenderTargetInner::Surface { .. } => false,
            RenderTargetInner::Texture { .. } => true,
        }
    }

    /// Returns the underlying target as a texture, if possible.
    pub fn as_texture(&self) -> Option<&wgpu::Texture> {
        match &self.0 {
            RenderTargetInner::Surface { .. } => None,
            RenderTargetInner::Texture { texture } => Some(texture),
        }
    }

    /// Returns the size of the render target as a `UVec2`.
    pub fn get_size(&self) -> UVec2 {
        match &self.0 {
            RenderTargetInner::Surface {
                surface: _,
                surface_config,
            } => UVec2::new(surface_config.width, surface_config.height),
            RenderTargetInner::Texture { texture } => {
                let s = texture.size();
                UVec2::new(s.width, s.height)
            }
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
/// Represents errors that can occur within the rendering context.
pub enum ContextError {
    #[snafu(display("missing surface texture: {}", source))]
    Surface { source: wgpu::SurfaceError },

    #[snafu(display("cannot create adaptor: {source}"))]
    CannotCreateAdaptor { source: wgpu::RequestAdapterError },

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

/// Represents the surface of a frame, which can either be a surface texture or a texture.
pub(crate) enum FrameSurface {
    Surface(wgpu::SurfaceTexture),
    Texture(Arc<wgpu::Texture>),
}

/// Represents the current frame of a render target.
///
/// Returned by [`Context::get_next_frame`].
pub struct Frame {
    pub(crate) runtime: WgpuRuntime,
    pub(crate) surface: FrameSurface,
}

impl Frame {
    /// Returns the underlying texture of this target.
    pub fn texture(&self) -> &wgpu::Texture {
        match &self.surface {
            FrameSurface::Surface(s) => &s.texture,
            FrameSurface::Texture(t) => t,
        }
    }

    /// Returns a view of the current frame's texture.
    pub fn view(&self) -> wgpu::TextureView {
        let texture = self.texture();
        let format = texture.format().add_srgb_suffix();
        texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Frame::view"),
            format: Some(format),
            ..Default::default()
        })
    }

    /// Copies the current frame to a buffer for further processing.
    pub fn copy_to_buffer(&self, width: u32, height: u32) -> CopiedTextureBuffer {
        let dimensions = BufferDimensions::new(4, 1, width as usize, height as usize);
        // The output buffer lets us retrieve the self as an array
        let buffer = self.runtime.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("RenderTarget::copy_to_buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder =
            self.runtime
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("post render screen capture encoder"),
                });
        let texture = self.texture();
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
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

        self.runtime.queue.submit(std::iter::once(encoder.finish()));

        CopiedTextureBuffer {
            dimensions,
            buffer,
            format: texture.format(),
        }
    }

    pub fn get_size(&self) -> UVec2 {
        let s = self.texture().size();
        UVec2::new(s.width, s.height)
    }

    /// Reads the current frame buffer into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// The resulting image will be in the color space of the frame.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub async fn read_image(&self) -> Result<image::RgbaImage, TextureError> {
        let size = self.get_size();
        let buffer = self.copy_to_buffer(size.x, size.y);
        let is_srgb = self.texture().format().is_srgb();
        let img = if is_srgb {
            buffer.into_srgba(&self.runtime.device).await?
        } else {
            buffer.into_linear_rgba(&self.runtime.device).await?
        };
        Ok(img)
    }

    /// Reads the frame into an image in a sRGB color space.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub async fn read_srgb_image(&self) -> Result<image::RgbaImage, TextureError> {
        let size = self.get_size();
        let buffer = self.copy_to_buffer(size.x, size.y);
        log::trace!("read image has the format: {:?}", buffer.format);
        buffer.into_srgba(&self.runtime.device).await
    }
    /// Reads the frame into an image in a linear color space.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub async fn read_linear_image(&self) -> Result<image::RgbaImage, TextureError> {
        let size = self.get_size();
        let buffer = self.copy_to_buffer(size.x, size.y);
        buffer.into_linear_rgba(&self.runtime.device).await
    }

    /// Presents the surface frame if the frame is a `TargetFrame::Surface`.
    /// If the frame is a `TargetFrame::Texture`, this is a no-op.
    pub fn present(self) {
        match self.surface {
            FrameSurface::Surface(s) => s.present(),
            FrameSurface::Texture(_) => {}
        }
    }
}

/// Configurable default values to use when creating new [`Stage`]s.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GlobalStageConfig {
    pub(crate) atlas_size: wgpu::Extent3d,
    pub(crate) shadow_map_atlas_size: wgpu::Extent3d,
    pub(crate) use_compute_culling: bool,
}

/// Contains the adapter, device, queue, [`RenderTarget`] and configuration.
///
/// A `Context` is created to initialize rendering to a window, canvas or
/// texture.
///
/// ```
/// use renderling::context::Context;
///
/// let ctx = futures_lite::future::block_on(Context::headless(100, 100));
/// ```
pub struct Context {
    runtime: WgpuRuntime,
    adapter: Arc<wgpu::Adapter>,
    render_target: RenderTarget,
    pub(crate) stage_config: Arc<RwLock<GlobalStageConfig>>,
}

impl AsRef<WgpuRuntime> for Context {
    fn as_ref(&self) -> &WgpuRuntime {
        &self.runtime
    }
}

impl Context {
    /// Creates a new `Context` with the specified target, adapter, device, and queue.
    pub fn new(
        target: RenderTarget,
        adapter: impl Into<Arc<wgpu::Adapter>>,
        device: impl Into<Arc<wgpu::Device>>,
        queue: impl Into<Arc<wgpu::Queue>>,
    ) -> Self {
        let adapter: Arc<wgpu::Adapter> = adapter.into();
        let limits = adapter.limits();
        let w = limits
            .max_texture_dimension_2d
            .min(crate::atlas::ATLAS_SUGGESTED_SIZE);
        let stage_config = Arc::new(RwLock::new(GlobalStageConfig {
            atlas_size: wgpu::Extent3d {
                width: w,
                height: w,
                depth_or_array_layers: adapter
                    .limits()
                    .max_texture_array_layers
                    .min(crate::atlas::ATLAS_SUGGESTED_LAYERS),
            },
            shadow_map_atlas_size: wgpu::Extent3d {
                width: w,
                height: w,
                depth_or_array_layers: 4,
            },
            use_compute_culling: false,
        }));
        Self {
            adapter,
            runtime: WgpuRuntime {
                device: device.into(),
                queue: queue.into(),
            },
            render_target: target,
            stage_config,
        }
    }

    /// Attempts to create a new headless `Context` with the specified width, height, and backends.
    pub async fn try_new_headless(
        width: u32,
        height: u32,
        backends: Option<wgpu::Backends>,
    ) -> Result<Self, ContextError> {
        log::trace!("creating headless context of size ({width}, {height})");
        let instance = crate::internal::new_instance(backends);
        let (adapter, device, queue, target) =
            crate::internal::new_headless_device_queue_and_target(width, height, &instance).await?;
        Ok(Self::new(target, adapter, device, queue))
    }

    /// Attempts to create a new `Context` with a surface, using the specified width, height, backends, and window.
    pub async fn try_new_with_surface(
        width: u32,
        height: u32,
        backends: Option<wgpu::Backends>,
        window: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> Result<Self, ContextError> {
        let instance = crate::internal::new_instance(backends);
        let (adapter, device, queue, target) =
            crate::internal::new_windowed_adapter_device_queue(width, height, &instance, window)
                .await?;
        Ok(Self::new(target, adapter, device, queue))
    }

    #[cfg(feature = "winit")]
    /// Create a [`Context`] from an existing [`winit::window::Window`].
    ///
    /// ## Panics
    /// Panics if the context could not be created.
    pub async fn from_winit_window(
        backends: Option<wgpu::Backends>,
        window: Arc<winit::window::Window>,
    ) -> Self {
        let inner_size = window.inner_size();
        Self::try_new_with_surface(inner_size.width, inner_size.height, backends, window)
            .await
            .unwrap()
    }

    /// Creates a new headless renderer.
    ///
    /// Immediately proxies to `Context::try_new_headless` and unwraps.
    ///
    /// ## Panics
    /// This function will panic if an adapter cannot be found. For example, this
    /// would happen on machines without a GPU.
    pub async fn headless(width: u32, height: u32) -> Self {
        let result = Self::try_new_headless(width, height, None).await;
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::UnwrapThrowExt;
            result.expect_throw("Could not create context")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            result.expect("Could not create context")
        }
    }

    pub fn get_size(&self) -> UVec2 {
        self.render_target.get_size()
    }

    /// Sets the size of the render target.
    pub fn set_size(&mut self, size: UVec2) {
        self.render_target
            .resize(size.x, size.y, &self.runtime.device);
    }

    /// Creates a texture from an image buffer.
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
        Texture::from_image_buffer(
            self,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
            None,
        )
    }

    /// Creates a `Texture` from a `wgpu::Texture` and an optional sampler.
    pub fn texture_from_wgpu_tex(
        &self,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> Texture {
        Texture::from_wgpu_tex(self.get_device(), texture, sampler, None)
    }

    /// Returns a reference to the `WgpuRuntime`.
    pub fn runtime(&self) -> &WgpuRuntime {
        &self.runtime
    }

    /// Returns a reference to the `wgpu::Device`.
    pub fn get_device(&self) -> &wgpu::Device {
        &self.runtime.device
    }

    /// Returns a reference to the `wgpu::Queue`.
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.runtime.queue
    }

    /// Returns a reference to the `wgpu::Adapter`.
    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    /// Returns the adapter in an owned wrapper.
    pub fn get_adapter_owned(&self) -> Arc<wgpu::Adapter> {
        self.adapter.clone()
    }

    /// Returns a reference to the `RenderTarget`.
    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }

    /// Gets the next frame from the render target.
    ///
    /// A surface context (window or canvas) will return the next swapchain
    /// texture.
    ///
    /// A headless context will return the underlying headless texture.
    ///
    /// ## Errors
    /// Errs if the render target is a surface and there was an error getting
    /// the next swapchain texture. This can happen if the frame has already
    /// been acquired.
    pub fn get_next_frame(&self) -> Result<Frame, ContextError> {
        Ok(Frame {
            runtime: self.runtime.clone(),
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

    /// Sets the default texture size for the material atlas.
    ///
    /// * Width is `size.x` and must be a power of two.
    /// * Height is `size.y`, must match `size.x` and must be a power of two.
    /// * Layers is `size.z` and must be two or greater.
    pub fn set_default_atlas_texture_size(&self, size: impl Into<UVec3>) -> &Self {
        let size = size.into();
        let size = wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: size.z,
        };
        crate::atlas::check_size(size);
        self.stage_config.write().unwrap().atlas_size = size;
        self
    }

    /// Sets the default texture size for the material atlas.
    ///
    /// * Width is `size.x` and must be a power of two.
    /// * Height is `size.y`, must match `size.x` and must be a power of two.
    /// * Layers is `size.z` and must be greater than zero.
    ///
    /// ## Panics
    /// Will panic if the above conditions are not met.
    pub fn with_default_atlas_texture_size(self, size: impl Into<UVec3>) -> Self {
        self.set_default_atlas_texture_size(size);
        self
    }

    /// Sets the default texture size for the shadow mapping atlas.
    ///
    /// * Width is `size.x` and must be a power of two.
    /// * Height is `size.y`, must match `size.x` and must be a power of two.
    /// * Layers is `size.z` and must be two or greater.
    pub fn set_shadow_mapping_atlas_texture_size(&self, size: impl Into<UVec3>) -> &Self {
        let size = size.into();
        let size = wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: size.z,
        };
        crate::atlas::check_size(size);
        self.stage_config.write().unwrap().shadow_map_atlas_size = size;
        self
    }

    /// Sets the default texture size for the shadow mapping atlas.
    ///
    /// * Width is `size.x` and must be a power of two.
    /// * Height is `size.y`, must match `size.x` and must be a power of two.
    /// * Layers is `size.z` and must be greater than zero.
    ///
    /// ## Panics
    /// Will panic if the above conditions are not met.
    pub fn with_shadow_mapping_atlas_texture_size(self, size: impl Into<UVec3>) -> Self {
        self.set_shadow_mapping_atlas_texture_size(size);
        self
    }

    /// Sets the use of direct drawing.
    ///
    /// Default is **false**.
    ///
    /// If set to **true**, all compute culling, including frustum and occlusion culling,
    /// will **not** run.
    pub fn set_use_direct_draw(&self, use_direct_drawing: bool) {
        self.stage_config.write().unwrap().use_compute_culling = !use_direct_drawing;
    }

    /// Sets the use of direct drawing.
    ///
    /// Default is **false**.
    ///
    /// If set to **true**, all compute culling is turned **off**.
    /// This includes frustum and occlusion culling.
    pub fn with_use_direct_draw(self, use_direct_drawing: bool) -> Self {
        self.set_use_direct_draw(use_direct_drawing);
        self
    }

    /// Returns whether direct drawing is used.
    pub fn get_use_direct_draw(&self) -> bool {
        !self.stage_config.read().unwrap().use_compute_culling
    }

    /// Creates and returns a new [`Stage`] renderer.
    pub fn new_stage(&self) -> Stage {
        Stage::new(self)
    }

    /// Creates and returns a new [`Ui`] renderer.
    pub fn new_ui(&self) -> Ui {
        Ui::new(self)
    }
}
