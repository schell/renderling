//! CPU-only side of renderling/draw.rs

use craballoc::{
    prelude::{Gpu, SlabAllocator, WgpuRuntime},
    slab::SlabBuffer,
};
use crabslab::Id;

use crate::{
    cull::{ComputeCulling, CullingError},
    primitive::{shader::PrimitiveDescriptor, Primitive},
    texture::Texture,
    Context,
};

use super::DrawIndirectArgs;

/// Issues indirect draw calls.
///
/// Issues draw calls and performs culling.
pub struct IndirectDraws {
    pub(crate) slab: SlabAllocator<WgpuRuntime>,
    pub(crate) draws: Vec<Gpu<DrawIndirectArgs>>,
    pub(crate) compute_culling: ComputeCulling,
}

impl IndirectDraws {
    fn new(
        runtime: impl AsRef<WgpuRuntime>,
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &Texture,
    ) -> Self {
        let runtime = runtime.as_ref();
        let indirect_slab =
            SlabAllocator::new(runtime, "indirect-slab", wgpu::BufferUsages::INDIRECT);
        Self {
            compute_culling: ComputeCulling::new(
                runtime,
                stage_slab_buffer,
                &indirect_slab.commit(),
                depth_texture,
            ),
            slab: indirect_slab,
            draws: vec![],
        }
    }

    fn invalidate(&mut self) {
        if !self.draws.is_empty() {
            log::trace!("draining indirect draws after invalidation");
            let _ = self.draws.drain(..);
        }
    }

    /// Read the images from the hierarchical z-buffer used for occlusion
    /// culling.
    ///
    /// This is primarily for testing.
    pub async fn read_hzb_images(&self) -> Result<Vec<image::GrayImage>, CullingError> {
        self.compute_culling
            .compute_depth_pyramid
            .depth_pyramid
            .read_images()
            .await
    }
}

impl From<Id<PrimitiveDescriptor>> for DrawIndirectArgs {
    fn from(id: Id<PrimitiveDescriptor>) -> Self {
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

/// The drawing method used to send geometry to the GPU.
///
/// This is one of either:
/// * Indirect drawing - standard drawing method that includes compute culling.
/// * Direct drawing - fallback drawing method for web targets.
///   Does not include compute culling, as the MULTI_DRAW_INDIRECT
///   `wgpu` feature is required and not available on web.
pub(crate) struct DrawingStrategy {
    indirect: Option<IndirectDraws>,
}

impl DrawingStrategy {
    #[cfg(test)]
    pub fn as_indirect(&self) -> Option<&IndirectDraws> {
        self.indirect.as_ref()
    }
}

/// Used to determine which objects are drawn and maintains the
/// list of all [`Primitive`]s.
pub struct DrawCalls {
    /// Internal representation of all staged renderlets.
    renderlets: Vec<Primitive>,
    pub(crate) drawing_strategy: DrawingStrategy,
}

impl DrawCalls {
    /// Create a new [`DrawCalls`].
    ///
    /// `use_compute_culling` can be used to set whether frustum culling is used
    /// as a GPU compute step before drawing. This is a native-only option.
    ///
    /// ## Note
    /// A [`Context`] is required because `DrawCalls` needs to query for the set of available driver
    /// features.
    pub fn new(
        ctx: &Context,
        use_compute_culling: bool,
        stage_slab_buffer: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &Texture,
    ) -> Self {
        let supported_features = ctx.get_adapter().features();
        log::trace!("supported features: {supported_features:#?}");
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
            renderlets: vec![],
            drawing_strategy: DrawingStrategy {
                indirect: if can_use_compute_culling {
                    log::debug!("Using indirect drawing method and compute culling");
                    Some(IndirectDraws::new(ctx, stage_slab_buffer, depth_texture))
                } else {
                    log::debug!("Using direct drawing method");
                    None
                },
            },
        }
    }

    /// Returns whether compute culling is available.
    pub fn get_compute_culling_available(&self) -> bool {
        matches!(
            &self.drawing_strategy,
            DrawingStrategy { indirect: Some(_) }
        )
    }

    /// Add a renderlet to the drawing queue.
    ///
    /// Returns the number of draw calls in the queue.
    pub fn add_primitive(&mut self, renderlet: &Primitive) -> usize {
        log::trace!("adding renderlet {:?}", renderlet.id());
        if let Some(indirect) = &mut self.drawing_strategy.indirect {
            indirect.invalidate();
        }
        self.renderlets.push(renderlet.clone());
        self.renderlets.len()
    }

    /// Erase the given renderlet from the internal list of renderlets to be
    /// drawn each frame.
    ///
    /// Returns the number of draw calls remaining in the queue.
    pub fn remove_primitive(&mut self, renderlet: &Primitive) -> usize {
        let id = renderlet.id();
        self.renderlets.retain(|ir| ir.descriptor.id() != id);

        if let Some(indirect) = &mut self.drawing_strategy.indirect {
            indirect.invalidate();
        }

        self.renderlets.len()
    }

    /// Sort draw calls using a function compairing [`Primitive`]s.
    pub fn sort_primitives(&mut self, f: impl Fn(&Primitive, &Primitive) -> std::cmp::Ordering) {
        self.renderlets.sort_by(f);
        if let Some(indirect) = &mut self.drawing_strategy.indirect {
            indirect.invalidate();
        }
    }

    /// Returns the number of draw calls (direct or indirect) that will be
    /// made during pre-rendering (compute culling) and rendering.
    pub fn draw_count(&self) -> usize {
        self.renderlets.len()
    }

    /// Perform pre-draw steps like frustum and occlusion culling, if available.
    ///
    /// Returns the indirect draw buffer.
    pub fn pre_draw(
        &mut self,
        depth_texture: &Texture,
    ) -> Result<Option<SlabBuffer<wgpu::Buffer>>, CullingError> {
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
            if let Some(indirect) = &mut self.drawing_strategy.indirect {
                if indirect.draws.len() != self.renderlets.len() {
                    indirect.invalidate();
                    // Pre-upkeep to reclaim resources - this is necessary because
                    // the draw buffer has to be contiguous (it can't start with a bunch of trash)
                    let indirect_buffer = indirect.slab.commit();
                    if indirect_buffer.is_new_this_commit() {
                        log::warn!("new indirect buffer");
                    }
                    indirect.draws = self
                        .renderlets
                        .iter()
                        .map(|r| {
                            indirect
                                .slab
                                .new_value(DrawIndirectArgs::from(r.descriptor.id()))
                                .into_gpu_only()
                        })
                        .collect();
                }
                let indirect_buffer = indirect.slab.commit();
                log::trace!("performing culling on {num_draw_calls} renderlets");
                indirect
                    .compute_culling
                    .run(num_draw_calls as u32, depth_texture);
                Ok(Some(indirect_buffer))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Draw into the given `RenderPass` by directly calling each draw.
    pub fn draw_direct(&self, render_pass: &mut wgpu::RenderPass) {
        if self.renderlets.is_empty() {
            log::warn!("no internal renderlets, nothing to draw");
        }
        for ir in self.renderlets.iter() {
            // UNWRAP: panic on purpose
            let desc = ir.descriptor.get();
            let vertex_range = 0..desc.get_vertex_count();
            let id = ir.descriptor.id();
            let instance_range = id.inner()..id.inner() + 1;
            render_pass.draw(vertex_range, instance_range);
        }
    }

    /// Draw into the given `RenderPass`.
    ///
    /// This method draws using the indirect draw buffer, if possible, otherwise
    /// it falls back to `draw_direct`.
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        let num_draw_calls = self.draw_count();
        if num_draw_calls > 0 {
            if let Some(indirect) = &self.drawing_strategy.indirect {
                log::trace!("drawing {num_draw_calls} renderlets using indirect");
                if let Some(indirect_buffer) = indirect.slab.get_buffer() {
                    render_pass.multi_draw_indirect(&indirect_buffer, 0, num_draw_calls as u32);
                } else {
                    log::warn!(
                        "could not get the indirect buffer - was `DrawCall::upkeep` called?"
                    );
                }
            } else {
                log::trace!("drawing {num_draw_calls} renderlets using direct");
                self.draw_direct(render_pass);
            }
        } else {
            log::warn!("zero draw calls");
        }
    }
}
