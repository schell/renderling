//! Core `UiRenderer` implementation.
//!
//! This module contains the GPU pipeline setup, element management,
//! and rendering logic for the 2D/UI renderer.
//!
//! ## Architecture
//!
//! The renderer uses a [`SlabAllocator`] from `craballoc` to manage GPU
//! memory. Each UI element is backed by a [`Hybrid<UiDrawCallDescriptor>`]
//! which keeps a CPU copy in sync with a GPU slab allocation. Calling
//! [`SlabAllocator::commit`] flushes all pending changes to the GPU buffer.
//!
//! Element wrapper types ([`UiRect`], [`UiCircle`],
//! [`UiEllipse`]) follow the same pattern as
//! [`renderling::camera::Camera`] — each wraps a `Hybrid` and provides
//! typed setter methods that queue GPU updates automatically.

use craballoc::{
    prelude::*,
    slab::{SlabAllocator, SlabBuffer},
    value::Hybrid,
};
use crabslab::Id;
use glam::{Mat4, UVec2, Vec2, Vec4};
use renderling::{
    context::Context,
    ui_slab::{GradientDescriptor, UiDrawCallDescriptor, UiElementType, UiViewport},
};

// ---------------------------------------------------------------------------
// Element wrapper types (follow the Camera pattern from camera/cpu.rs)
// ---------------------------------------------------------------------------

/// A live handle to a rectangle element in the renderer.
///
/// Modifications via the `set_*` methods are reflected on the GPU after
/// the next call to [`UiRenderer::render`].
///
/// Clones of this type all point to the same underlying GPU data.
///
/// **Dropping this handle does NOT remove the element** — call
/// [`UiRenderer::remove_rect`] explicitly.
#[derive(Clone, Debug)]
pub struct UiRect {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiRect {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the top-left position in screen pixels.
    pub fn set_position(&self, position: Vec2) -> &Self {
        self.inner.modify(|d| d.position = position);
        self
    }

    /// Set the top-left position in screen pixels (builder).
    pub fn with_position(self, position: Vec2) -> Self {
        self.set_position(position);
        self
    }

    /// Set the size in screen pixels.
    pub fn set_size(&self, size: Vec2) -> &Self {
        self.inner.modify(|d| d.size = size);
        self
    }

    /// Set the size in screen pixels (builder).
    pub fn with_size(self, size: Vec2) -> Self {
        self.set_size(size);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set per-corner radii (top-left, top-right, bottom-right,
    /// bottom-left).
    pub fn set_corner_radii(&self, radii: Vec4) -> &Self {
        self.inner.modify(|d| d.corner_radii = radii);
        self
    }

    /// Set per-corner radii (builder).
    pub fn with_corner_radii(self, radii: Vec4) -> Self {
        self.set_corner_radii(radii);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity (0.0 = transparent, 1.0 = opaque).
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting. Lower values are drawn first.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

/// A live handle to a circle element in the renderer.
///
/// See [`UiRect`] for general usage notes.
#[derive(Clone, Debug)]
pub struct UiCircle {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiCircle {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the center position in screen pixels.
    pub fn set_center(&self, center: Vec2) -> &Self {
        self.inner.modify(|d| {
            let radius = d.size.x / 2.0;
            d.position = center - Vec2::splat(radius);
        });
        self
    }

    /// Set the center position in screen pixels (builder).
    pub fn with_center(self, center: Vec2) -> Self {
        self.set_center(center);
        self
    }

    /// Set the radius in screen pixels.
    pub fn set_radius(&self, radius: f32) -> &Self {
        self.inner.modify(|d| {
            let center = d.position + d.size / 2.0;
            d.size = Vec2::splat(radius * 2.0);
            d.position = center - Vec2::splat(radius);
        });
        self
    }

    /// Set the radius in screen pixels (builder).
    pub fn with_radius(self, radius: f32) -> Self {
        self.set_radius(radius);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity.
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

/// A live handle to an ellipse element in the renderer.
///
/// See [`UiRect`] for general usage notes.
#[derive(Clone, Debug)]
pub struct UiEllipse {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiEllipse {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the center position in screen pixels.
    pub fn set_center(&self, center: Vec2) -> &Self {
        self.inner.modify(|d| {
            let radii = d.size / 2.0;
            d.position = center - radii;
        });
        self
    }

    /// Set the center position in screen pixels (builder).
    pub fn with_center(self, center: Vec2) -> Self {
        self.set_center(center);
        self
    }

    /// Set the radii (horizontal, vertical) in screen pixels.
    pub fn set_radii(&self, radii: Vec2) -> &Self {
        self.inner.modify(|d| {
            let center = d.position + d.size / 2.0;
            d.size = radii * 2.0;
            d.position = center - radii;
        });
        self
    }

    /// Set the radii (builder).
    pub fn with_radii(self, radii: Vec2) -> Self {
        self.set_radii(radii);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity.
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

// ---------------------------------------------------------------------------
// Internal draw call entry
// ---------------------------------------------------------------------------

/// Internal representation of a draw call for the renderer.
struct DrawCall {
    /// The hybrid descriptor (shared with the element wrapper).
    descriptor: Hybrid<UiDrawCallDescriptor>,
    /// Number of vertices (6 for quads, variable for paths).
    vertex_count: u32,
}

// ---------------------------------------------------------------------------
// UiRenderer
// ---------------------------------------------------------------------------

/// The 2D/UI renderer.
///
/// This renderer maintains its own lightweight GPU pipeline separate from
/// renderling's 3D PBR pipeline. It renders directly to a provided
/// `TextureView` with no intermediate HDR buffer, bloom, or tonemapping.
///
/// GPU memory is managed via a [`SlabAllocator`]. Each element is a
/// [`Hybrid<UiDrawCallDescriptor>`] — modifications via the element
/// wrapper types are automatically synced to the GPU on the next
/// [`render`](Self::render) call.
pub struct UiRenderer {
    slab: SlabAllocator<WgpuRuntime>,
    #[allow(dead_code)]
    viewport: Hybrid<UiViewport>,
    pipeline: wgpu::RenderPipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    /// Cached slab buffer from the last commit.
    slab_buffer: Option<SlabBuffer<wgpu::Buffer>>,
    /// Cached bind group (recreated when slab buffer changes).
    bindgroup: Option<wgpu::BindGroup>,
    /// All active draw calls, sorted by z before rendering.
    draw_calls: Vec<DrawCall>,
    /// Viewport size.
    viewport_size: UVec2,
    /// Background clear color.
    background_color: Option<Vec4>,
    /// MSAA sample count.
    msaa_sample_count: u32,
    /// The texture format of the render target.
    format: wgpu::TextureFormat,
    /// MSAA resolve texture (if msaa_sample_count > 1).
    msaa_texture: Option<wgpu::TextureView>,
    /// Dummy atlas texture view (1x1x1, created once).
    dummy_atlas_view: wgpu::TextureView,
    /// Dummy atlas sampler (created once).
    dummy_atlas_sampler: wgpu::Sampler,
}

impl UiRenderer {
    const LABEL: Option<&'static str> = Some("renderling-ui");

    /// Create a new `UiRenderer` from a renderling `Context`.
    pub fn new(ctx: &Context) -> Self {
        let device = ctx.get_device();
        let size = ctx.get_size();
        let format = ctx.get_render_target().format();

        let slab = SlabAllocator::new(ctx.runtime(), "ui-slab", wgpu::BufferUsages::empty());
        let viewport = slab.new_value(UiViewport {
            projection: Self::ortho2d(size.x as f32, size.y as f32),
            size,
        });

        let bindgroup_layout = Self::create_bindgroup_layout(device);
        let pipeline = Self::create_pipeline(device, &bindgroup_layout, format, 1);

        // Create the dummy atlas texture and sampler once.
        let (dummy_atlas_view, dummy_atlas_sampler) = Self::create_dummy_atlas(device);

        Self {
            slab,
            viewport,
            pipeline,
            bindgroup_layout,
            slab_buffer: None,
            bindgroup: None,
            draw_calls: Vec::new(),
            viewport_size: size,
            background_color: None,
            msaa_sample_count: 1,
            format,
            msaa_texture: None,
            dummy_atlas_view,
            dummy_atlas_sampler,
        }
    }

    /// Set the background clear color. `None` means don't clear
    /// (load existing content).
    pub fn set_background_color(&mut self, color: Option<Vec4>) -> &mut Self {
        self.background_color = color;
        self
    }

    /// Builder-style background color setter.
    pub fn with_background_color(mut self, color: Vec4) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the viewport size (typically matches the render target size).
    pub fn set_size(&mut self, size: UVec2) {
        if self.viewport_size != size {
            self.viewport_size = size;
            self.viewport.modify(|v| {
                v.projection = Self::ortho2d(size.x as f32, size.y as f32);
                v.size = size;
            });

            // Recreate MSAA texture if needed.
            if self.msaa_sample_count > 1 {
                self.msaa_texture = Some(Self::create_msaa_texture(
                    self.slab.device(),
                    self.format,
                    size,
                    self.msaa_sample_count,
                ));
            }
        }
    }

    /// Add a rectangle element and return a live handle.
    ///
    /// The element starts with sensible defaults (100x100 white rect
    /// at the origin). Use the `with_*` builder methods or `set_*`
    /// methods to configure it.
    ///
    /// ```ignore
    /// let rect = ui.add_rect()
    ///     .with_position(Vec2::new(10.0, 10.0))
    ///     .with_size(Vec2::new(200.0, 100.0))
    ///     .with_fill_color(Vec4::new(0.2, 0.4, 0.8, 1.0));
    /// ```
    pub fn add_rect(&mut self) -> UiRect {
        let desc = self.default_descriptor(UiElementType::Rectangle);
        let hybrid = self.slab.new_value(desc);
        let element = UiRect {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Add a circle element and return a live handle.
    ///
    /// The element starts centered at (0, 0) with radius 50 and
    /// white fill. Use `with_center`, `with_radius`, etc. to
    /// configure.
    pub fn add_circle(&mut self) -> UiCircle {
        let desc = self.default_descriptor(UiElementType::Circle);
        let hybrid = self.slab.new_value(desc);
        let element = UiCircle {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Add an ellipse element and return a live handle.
    ///
    /// The element starts centered at (0, 0) with size 100x100 and
    /// white fill. Use `with_center`, `with_radii`, etc. to
    /// configure.
    pub fn add_ellipse(&mut self) -> UiEllipse {
        let desc = self.default_descriptor(UiElementType::Ellipse);
        let hybrid = self.slab.new_value(desc);
        let element = UiEllipse {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Remove a rectangle element by its handle.
    pub fn remove_rect(&mut self, element: &UiRect) {
        self.remove_by_id(element.id());
    }

    /// Remove a circle element by its handle.
    pub fn remove_circle(&mut self, element: &UiCircle) {
        self.remove_by_id(element.id());
    }

    /// Remove an ellipse element by its handle.
    pub fn remove_ellipse(&mut self, element: &UiEllipse) {
        self.remove_by_id(element.id());
    }

    /// Remove all elements.
    pub fn clear(&mut self) {
        self.draw_calls.clear();
        // Dropping the Hybrid values reclaims slab memory on next
        // commit.
    }

    /// Render all UI elements to the given texture view.
    pub fn render(&mut self, view: &wgpu::TextureView) {
        if self.draw_calls.is_empty() {
            return;
        }

        // Sort draw calls by z (painter's algorithm).
        // We read z from the CPU-side Hybrid each frame.
        let mut sorted_indices: Vec<usize> = (0..self.draw_calls.len()).collect();
        sorted_indices.sort_by(|a, b| {
            let z_a = self.draw_calls[*a].descriptor.get().z;
            let z_b = self.draw_calls[*b].descriptor.get().z;
            z_a.partial_cmp(&z_b).unwrap_or(core::cmp::Ordering::Equal)
        });

        // Commit slab changes to the GPU.
        let buffer = self.slab.commit();
        let should_recreate_bindgroup = buffer.is_new_this_commit() || self.bindgroup.is_none();

        if should_recreate_bindgroup {
            self.bindgroup = Some(self.create_bindgroup(&buffer));
        }
        self.slab_buffer = Some(buffer);

        let device = self.slab.device();
        let queue = self.slab.queue();

        // Create command encoder.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });

        let load_op = if let Some(bg) = self.background_color {
            wgpu::LoadOp::Clear(wgpu::Color {
                r: bg.x as f64,
                g: bg.y as f64,
                b: bg.z as f64,
                a: bg.w as f64,
            })
        } else {
            wgpu::LoadOp::Load
        };

        let (color_view, resolve_target) = if self.msaa_sample_count > 1 {
            if let Some(msaa_view) = &self.msaa_texture {
                (msaa_view, Some(view))
            } else {
                (view, None)
            }
        } else {
            (view, None)
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Self::LABEL,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: load_op,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, self.bindgroup.as_ref().unwrap(), &[]);

            // Issue one draw call per element, sorted by z.
            // The instance_index encodes the slab offset of the
            // UiDrawCallDescriptor.
            for &idx in &sorted_indices {
                let dc = &self.draw_calls[idx];
                let inst = dc.descriptor.id().inner();
                render_pass.draw(0..dc.vertex_count, inst..inst + 1);
            }
        }

        queue.submit(Some(encoder.finish()));
    }

    // --- Private helpers ---

    fn ortho2d(width: f32, height: f32) -> Mat4 {
        Mat4::orthographic_rh(
            0.0,    // left
            width,  // right
            height, // bottom
            0.0,    // top
            -1.0,   // near
            1.0,    // far
        )
    }

    /// Build a default [`UiDrawCallDescriptor`] for the given element
    /// type, using the current viewport as the clip rect.
    fn default_descriptor(&self, element_type: UiElementType) -> UiDrawCallDescriptor {
        UiDrawCallDescriptor {
            element_type,
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 100.0),
            corner_radii: Vec4::ZERO,
            border_width: 0.0,
            border_color: Vec4::ZERO,
            fill_color: Vec4::ONE,
            gradient: GradientDescriptor::default(),
            atlas_texture_id: u32::MAX,
            atlas_descriptor_id: u32::MAX,
            clip_rect: Vec4::new(
                0.0,
                0.0,
                self.viewport_size.x as f32,
                self.viewport_size.y as f32,
            ),
            opacity: 1.0,
            z: 0.0,
        }
    }

    fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // Binding 0: Slab storage buffer.
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Binding 1: Atlas texture (2D array).
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                // Binding 2: Atlas sampler.
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    fn create_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        format: wgpu::TextureFormat,
        msaa_sample_count: u32,
    ) -> wgpu::RenderPipeline {
        let vertex_linkage = renderling::linkage::ui_vertex::linkage(device);
        let fragment_linkage = renderling::linkage::ui_fragment::linkage(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Self::LABEL,
            bind_group_layouts: &[bindgroup_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Self::LABEL,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: msaa_sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        })
    }

    fn create_msaa_texture(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        size: UVec2,
        sample_count: u32,
    ) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("renderling-ui-msaa"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Create a dummy 1x1x1 atlas texture and sampler (used until
    /// texture/image support is fully wired up).
    fn create_dummy_atlas(device: &wgpu::Device) -> (wgpu::TextureView, wgpu::Sampler) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("renderling-ui-dummy-atlas"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("renderling-ui-dummy-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        (view, sampler)
    }

    /// Create a bind group using the given slab buffer.
    fn create_bindgroup(&self, buffer: &SlabBuffer<wgpu::Buffer>) -> wgpu::BindGroup {
        self.slab
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Self::LABEL,
                layout: &self.bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&self.dummy_atlas_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&self.dummy_atlas_sampler),
                    },
                ],
            })
    }

    /// Remove a draw call by its slab ID.
    fn remove_by_id(&mut self, id: Id<UiDrawCallDescriptor>) {
        self.draw_calls.retain(|dc| dc.descriptor.id() != id);
        // The Hybrid is dropped here (removed from draw_calls Vec),
        // which will cause the slab to reclaim its memory on the
        // next commit.
    }
}
