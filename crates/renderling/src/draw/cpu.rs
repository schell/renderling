//! CPU-only side of renderling/draw.rs

use std::sync::Arc;

use crabslab::Id;
use glam::UVec2;
use rustc_hash::FxHashMap;

use crate::{
    cull::{ComputeCulling, CullingError},
    slab::{Gpu, Hybrid, SlabAllocator, WeakHybrid},
    stage::Renderlet,
    texture::Texture,
    Context,
};

use super::DrawIndirectArgs;

/// Used to track renderlets internally.
#[repr(transparent)]
struct InternalRenderlet {
    inner: WeakHybrid<Renderlet>,
}

impl InternalRenderlet {
    fn has_external_references(&self) -> bool {
        self.inner.strong_count() > 0
    }

    fn from_hybrid_renderlet(hr: &Hybrid<Renderlet>) -> Self {
        Self {
            inner: WeakHybrid::from_hybrid(hr),
        }
    }

    fn copy_inner(&self) -> Option<Renderlet> {
        self.inner.upgrade().map(|hy| hy.get())
    }
}

/// Issues indirect draw calls.
///
/// Issues draw calls and performs culling.
pub struct IndirectDraws {
    pub(crate) slab: SlabAllocator<wgpu::Buffer>,
    pub(crate) draws: Vec<Gpu<DrawIndirectArgs>>,
    pub(crate) compute_culling: ComputeCulling,
}

impl IndirectDraws {
    fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        Self {
            compute_culling: ComputeCulling::new(device, size, sample_count),
            slab: SlabAllocator::default(),
            draws: vec![],
        }
    }

    fn invalidate(&mut self) {
        if !self.draws.is_empty() {
            let _ = self.draws.drain(..);
        }
    }

    fn get_indirect_buffer(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Arc<wgpu::Buffer> {
        self.slab.get_updated_buffer((
            device,
            queue,
            Some("indirect draw upkeep"),
            wgpu::BufferUsages::INDIRECT,
        ))
    }

    fn sync_with_internal_renderlets(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        internal_renderlets: &[InternalRenderlet],
        redraw_args: bool,
    ) -> Arc<wgpu::Buffer> {
        if redraw_args || self.draws.len() != internal_renderlets.len() {
            self.invalidate();
            // Pre-upkeep to reclaim resources - this is necessary because
            // the draw buffer has to be contiguous (it can't start with a bunch of trash)
            if let Some(_indirect_buffer) = self.slab.upkeep((
                device,
                queue,
                Some("indirect draw pre upkeep"),
                wgpu::BufferUsages::INDIRECT,
            )) {
                // Invalidate the compute culling buffer, it will be regenerated later...
                self.compute_culling.invalidate_bindgroup();
            }
            self.draws = internal_renderlets
                .iter()
                .map(|ir: &InternalRenderlet| {
                    self.slab
                        .new_value(DrawIndirectArgs::from(ir.inner.id()))
                        .into_gpu_only()
                })
                .collect();
        }
        self.get_indirect_buffer(device, queue)
    }

    /// Read the images from the hierarchical z-buffer used for occlusion
    /// culling.
    ///
    /// This is primarily for testing.
    pub async fn read_hzb_images(
        &self,
        ctx: &crate::Context,
    ) -> Result<Vec<image::ImageBuffer<image::Luma<f32>, Vec<f32>>>, CullingError> {
        self.compute_culling
            .compute_depth_pyramid
            .depth_pyramid
            .read_images(ctx)
            .await
    }
}

impl From<Id<Renderlet>> for DrawIndirectArgs {
    fn from(id: Id<Renderlet>) -> Self {
        // This is obviously incomplete, but that's ok because
        // the rest of this struct is filled out on the GPU during
        // culling.
        DrawIndirectArgs {
            vertex_count: 0,
            instance_count: 0,
            first_vertex: 0,
            first_instance: id.inner(),
        }
    }
}

pub(crate) enum DrawingStrategy {
    /// The standard drawing method that includes compute culling.
    Indirect(IndirectDraws),
    /// Fallback drawing method for web targets.
    ///
    /// Does not include compute culling, as the MULTI_DRAW_INDIRECT
    /// `wgpu` feature is required and not available on web.
    Direct,
}

impl DrawingStrategy {
    #[cfg(test)]
    pub fn as_indirect(&self) -> Option<&IndirectDraws> {
        if let DrawingStrategy::Indirect(i) = self {
            Some(i)
        } else {
            None
        }
    }
}

/// Used to determine which objects are drawn and maintains the
/// list of all [`Renderlet`]s.
pub struct DrawCalls {
    /// Internal representation of all staged renderlets.
    internal_renderlets: Vec<InternalRenderlet>,
    pub(crate) drawing_strategy: DrawingStrategy,
}

impl DrawCalls {
    /// Create a new [`DrawCalls`].
    ///
    /// `use_compute_culling` can be used to set whether frustum culling is used
    /// as a GPU compute step before drawing. This is a native-only option.
    pub fn new(ctx: &Context, use_compute_culling: bool, size: UVec2, sample_count: u32) -> Self {
        let can_use_multi_draw_indirect = ctx.get_adapter().features().contains(
            wgpu::Features::INDIRECT_FIRST_INSTANCE | wgpu::Features::MULTI_DRAW_INDIRECT,
        );
        if use_compute_culling && !can_use_multi_draw_indirect {
            log::warn!(
                "`use_compute_culling` is `true`, but the MULTI_DRAW_INDIRECT feature is not \
                 available. No compute culling will occur."
            )
        }
        let can_use_compute_culling = use_compute_culling && can_use_multi_draw_indirect;
        Self {
            internal_renderlets: vec![],
            drawing_strategy: {
                if can_use_compute_culling {
                    log::debug!("Using indirect drawing method and compute culling");
                    DrawingStrategy::Indirect(IndirectDraws::new(
                        ctx.get_device(),
                        size,
                        sample_count,
                    ))
                } else {
                    log::debug!("Using direct drawing method");
                    DrawingStrategy::Direct
                }
            },
        }
    }

    /// Returns whether compute culling is available.
    pub fn get_compute_culling_available(&self) -> bool {
        matches!(&self.drawing_strategy, DrawingStrategy::Indirect(_))
    }

    /// Add a renderlet to the drawing queue.
    ///
    /// Returns the number of draw calls in the queue.
    pub fn add_renderlet(&mut self, renderlet: &Hybrid<Renderlet>) -> usize {
        log::trace!("adding renderlet {:?}", renderlet.id());
        if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
            indirect.invalidate();
        }
        self.internal_renderlets
            .push(InternalRenderlet::from_hybrid_renderlet(renderlet));
        self.internal_renderlets.len()
    }

    /// Erase the given renderlet from the internal list of renderlets to be
    /// drawn each frame.
    ///
    /// Returns the number of draw calls remaining in the queue.
    pub fn remove_renderlet(&mut self, renderlet: &Hybrid<Renderlet>) -> usize {
        let id = renderlet.id();
        self.internal_renderlets.retain(|ir| ir.inner.id() != id);

        if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
            indirect.invalidate();
        }

        self.internal_renderlets.len()
    }

    /// Re-order the renderlets to that of the given list of identifiers.
    ///
    /// This determines the order in which they are drawn each frame.
    ///
    /// If the `order` iterator is missing any renderlet ids, those missing
    /// renderlets will be drawn _before_ the ordered ones, in no particular
    /// order.
    pub fn reorder_renderlets(&mut self, order: impl IntoIterator<Item = Id<Renderlet>>) {
        let mut ordered = vec![];
        let mut m = FxHashMap::from_iter(
            std::mem::take(&mut self.internal_renderlets)
                .into_iter()
                .map(|r| (r.inner.id(), r)),
        );
        for id in order.into_iter() {
            if let Some(renderlet) = m.remove(&id) {
                ordered.push(renderlet);
            }
        }
        self.internal_renderlets.extend(m.into_values());
        self.internal_renderlets.extend(ordered);
        if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
            indirect.invalidate();
        }
    }

    /// Iterator over all staged [`Renderlet`]s.
    pub fn renderlets_iter(&self) -> impl Iterator<Item = Renderlet> {
        self.internal_renderlets
            .iter()
            .filter_map(|ir| ir.copy_inner())
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Invalidates any resources that depend on the external stage slab buffer
    /// which holds the [`Renderlet`]s that have been queued to draw.
    pub fn invalidate_external_slab_buffer(&mut self) {
        if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
            indirect.compute_culling.invalidate_bindgroup();
        }
    }

    /// Perform upkeep on queued draw calls and synchronize internal buffers.
    pub fn upkeep(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut redraw_args = false;

        // Drop any renderlets that have no external references.
        self.internal_renderlets.retain_mut(|ir| {
            if ir.has_external_references() {
                true
            } else {
                redraw_args = true;
                log::trace!("dropping '{:?}' from drawing", ir.inner.id());
                false
            }
        });

        if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
            indirect.sync_with_internal_renderlets(
                device,
                queue,
                &self.internal_renderlets,
                redraw_args,
            );
        }
    }

    /// Returns the number of draw calls (direct or indirect) that will be
    /// made during pre-rendering (compute culling) and rendering.
    pub fn draw_count(&self) -> usize {
        self.internal_renderlets.len()
    }

    /// Perform pre-draw steps like compute culling, if available.
    ///
    /// This does not do upkeep, please call [`DrawCalls::upkeep`] before
    /// calling this function.
    ///
    /// Returns the indirect draw buffer.
    pub fn pre_draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
        depth_texture: &Texture,
    ) -> Result<Option<Arc<wgpu::Buffer>>, CullingError> {
        let num_draw_calls = self.draw_count();
        // Only do compute culling if there are things we need to draw, otherwise
        // `wgpu` will err with something like:
        // "Buffer with 'indirect draw upkeep' label binding size is zero"
        if num_draw_calls > 0 {
            log::trace!("num_draw_calls: {num_draw_calls}");
            // TODO: Cull on GPU even when `multidraw_indirect` is unavailable.
            //
            // We can do this without multidraw by running GPU culling and then
            // copying `indirect_buffer` back to the CPU.
            if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
                let maybe_buffer = indirect.slab.get_buffer();
                if let Some(indirect_buffer) = maybe_buffer.as_ref() {
                    log::trace!("performing culling on {num_draw_calls} renderlets");
                    indirect.compute_culling.run(
                        device,
                        queue,
                        slab_buffer,
                        indirect_buffer,
                        num_draw_calls as u32,
                        depth_texture,
                    )?;
                } else {
                    log::warn!(
                        "DrawCalls::pre_render called without first calling `upkeep` - no culling \
                         was performed"
                    );
                }
                Ok(maybe_buffer)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Draw into the given `RenderPass`.
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        let num_draw_calls = self.draw_count();
        if num_draw_calls > 0 {
            match &self.drawing_strategy {
                DrawingStrategy::Indirect(indirect) => {
                    log::trace!("drawing {num_draw_calls} renderlets using indirect");
                    if let Some(indirect_buffer) = indirect.slab.get_buffer() {
                        render_pass.multi_draw_indirect(&indirect_buffer, 0, num_draw_calls as u32);
                    } else {
                        log::warn!(
                            "could not get the indirect buffer - was `DrawCall::upkeep` called?"
                        );
                    }
                }
                DrawingStrategy::Direct => {
                    log::trace!("drawing {num_draw_calls} renderlets using direct");
                    for ir in self.internal_renderlets.iter() {
                        // UNWRAP: panic on purpose
                        if let Some(hr) = ir.inner.upgrade() {
                            let ir = hr.get();
                            let vertex_range = 0..ir.get_vertex_count();
                            let id = hr.id();
                            let instance_range = id.inner()..id.inner() + 1;
                            render_pass.draw(vertex_range, instance_range);
                        }
                    }
                }
            }
        }
    }
}
