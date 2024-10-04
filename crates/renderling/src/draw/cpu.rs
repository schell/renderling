//! CPU-only side of renderling/draw.rs

use std::sync::{Arc, RwLock, Weak};

use crabslab::Id;
use rustc_hash::FxHashMap;

use crate::{
    cull::ComputeCulling,
    slab::{Gpu, Hybrid, SlabAllocator},
    stage::Renderlet,
    Context,
};

use super::DrawIndirectArgs;

/// Used to track renderlets internally.
struct InternalRenderlet {
    id: Id<Renderlet>,
    vertex_count: u32,
    is_visible: bool,
    weak_cpu: Weak<RwLock<Renderlet>>,
    weak_gpu: Weak<dyn std::any::Any>,
}

impl InternalRenderlet {
    fn has_external_references(&self) -> bool {
        self.weak_gpu.strong_count() > 0
    }

    fn from_hybrid_renderlet(hr: &Hybrid<Renderlet>) -> Self {
        let r = hr.get();
        let weak_cpu = Arc::downgrade(&hr.cpu_value);
        let weak_gpu = Arc::downgrade(&hr.gpu_value.update);
        InternalRenderlet {
            id: hr.id(),
            vertex_count: r.get_vertex_count(),
            is_visible: r.visible,
            weak_cpu,
            weak_gpu,
        }
    }
}

struct IndirectDraws {
    slab: SlabAllocator<wgpu::Buffer>,
    draws: Vec<Gpu<DrawIndirectArgs>>,
    compute_culling: ComputeCulling,
}

impl IndirectDraws {
    fn new(device: &wgpu::Device) -> Self {
        Self {
            compute_culling: ComputeCulling::new(device),
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
                        .new_value(DrawIndirectArgs::from(ir.id))
                        .into_gpu_only()
                })
                .collect();
        }
        self.get_indirect_buffer(device, queue)
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
            first_instance: id,
        }
    }
}

enum DrawingStrategy {
    /// The standard drawing method that includes compute culling.
    Indirect(IndirectDraws),
    /// Fallback drawing method for web targets.
    ///
    /// Does not include compute culling, as the MULTI_DRAW_INDIRECT
    /// `wgpu` feature is required and not available on web.
    Direct,
}

/// Used to determine which objects are drawn and maintains the
/// list of all [`Renderlet`]s.
pub struct DrawCalls {
    /// Internal representation of all staged renderlets.
    internal_renderlets: Vec<InternalRenderlet>,
    drawing_strategy: DrawingStrategy,
}

impl DrawCalls {
    /// Create a new [`DrawCalls`].
    ///
    /// `use_compute_culling` can be used to set whether frustum culling is used as a GPU compute
    /// step before drawing. This is a native-only option.
    pub fn new(ctx: &Context, use_compute_culling: bool) -> Self {
        let can_use_multi_draw_indirect = ctx.get_adapter().features().contains(
            wgpu::Features::INDIRECT_FIRST_INSTANCE | wgpu::Features::MULTI_DRAW_INDIRECT,
        );
        if use_compute_culling && !can_use_multi_draw_indirect {
            log::warn!(
                "`use_compute_culling` is `true`, but the MULTI_DRAW_INDIRECT feature \
                 is not available. No compute culling will occur."
            )
        }
        let can_use_compute_culling = use_compute_culling && can_use_multi_draw_indirect;
        Self {
            internal_renderlets: vec![],
            drawing_strategy: {
                if can_use_compute_culling {
                    log::debug!("Using indirect drawing method and compute culling");
                    DrawingStrategy::Indirect(IndirectDraws::new(ctx.get_device()))
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
        self.internal_renderlets.retain(|ir| ir.id != id);

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
                .map(|r| (r.id, r)),
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
                // Update the cached data (for direct drawing), if possible
                //
                // Note:
                // If we can't upgrade here, it means the CPU value has been
                // dropped. In that case we don't need to update the cache here,
                // since the user can no longer update the cache either.
                if let Some(arc_renderlet) = Weak::upgrade(&ir.weak_cpu) {
                    let r = arc_renderlet.read().unwrap();
                    ir.vertex_count = r.get_vertex_count();
                    ir.is_visible = r.visible;
                }
                true
            } else {
                redraw_args = true;
                log::trace!("dropping '{:?}' from drawing", ir.id);
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

    /// Perform pre-draw steps like compute culling, if available.
    ///
    /// This does not do upkeep, please call [`DrawCalls::upkeep`] before calling this
    /// function.
    pub fn pre_draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
    ) {
        let num_draw_calls = self.internal_renderlets.len();
        // Only do compute culling if there are things we need to draw, otherwise
        // `wgpu` will err with something like:
        // "Buffer with 'indirect draw upkeep' label binding size is zero"
        if num_draw_calls > 0 {
            if let DrawingStrategy::Indirect(indirect) = &mut self.drawing_strategy {
                if let Some(indirect_buffer) = indirect.slab.get_buffer() {
                    log::trace!("performing culling on {num_draw_calls} renderlets");
                    indirect.compute_culling.run(
                        device,
                        queue,
                        slab_buffer,
                        &indirect_buffer,
                        num_draw_calls as u32,
                    );
                } else {
                    log::warn!(
                        "DrawCalls::pre_render called without first calling `upkeep` \
                            - no culling was performed"
                    );
                }
            }
        }
    }

    /// Draw into the given `RenderPass`.
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        let num_draw_calls = self.internal_renderlets.len();
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
                        if ir.is_visible {
                            let vertex_range = 0..ir.vertex_count;
                            let id = ir.id;
                            let instance_range = id.inner()..id.inner() + 1;
                            render_pass.draw(vertex_range, instance_range);
                        }
                    }
                }
            }
        }
    }
}
