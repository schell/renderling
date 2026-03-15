//! GPU shader entry points for the 2D/UI rendering pipeline.
//!
//! These shaders are compiled via rust-gpu and used by the `renderling-ui`
//! crate's `UiRenderer`.

use crabslab::{Id, Slab, SlabItem};
use glam::{Vec2, Vec4};
use spirv_std::{image::Image2dArray, spirv, Sampler};

use super::{GradientType, UiDrawCallDescriptor, UiElementType, UiVertex, UiViewport};
use crate::atlas::shader::AtlasTextureDescriptor;

/// SDF for a rounded rectangle.
///
/// `p` is the point relative to the rectangle center.
/// `half_ext` is the half-extents of the rectangle.
/// `radii` are the corner radii: (top-left, top-right, bottom-right,
/// bottom-left).
fn sdf_rounded_rect(p: Vec2, half_ext: Vec2, radii: Vec4) -> f32 {
    // Select the appropriate corner radius based on quadrant.
    let r = if p.x > 0.0 {
        if p.y > 0.0 {
            // bottom-right (in screen coords, +Y is down)
            radii.z
        } else {
            // top-right
            radii.y
        }
    } else if p.y > 0.0 {
        // bottom-left
        radii.w
    } else {
        // top-left
        radii.x
    };
    let q = p.abs() - half_ext + Vec2::splat(r);
    let outside = q.max(Vec2::ZERO).length();
    let inside = q.x.max(q.y).min(0.0);
    outside + inside - r
}

/// SDF for a circle.
fn sdf_circle(p: Vec2, radius: f32) -> f32 {
    p.length() - radius
}

/// SDF for an ellipse (approximation using the Iq formula).
fn sdf_ellipse(p: Vec2, radii: Vec2) -> f32 {
    // Simplified ellipse SDF (not exact but good for UI).
    let p_norm = p / radii;
    let d = p_norm.length() - 1.0;
    d * radii.x.min(radii.y)
}

/// Evaluate a gradient at the given local UV coordinate.
fn eval_gradient(
    gradient_type: u32,
    start: Vec2,
    end: Vec2,
    radius: f32,
    color_start: Vec4,
    color_end: Vec4,
    local_uv: Vec2,
) -> Vec4 {
    let gt = GradientType::from_u32(gradient_type);
    match gt {
        GradientType::None => color_start,
        GradientType::Linear => {
            let dir = end - start;
            let len_sq = dir.dot(dir);
            let t = if len_sq > 0.0 {
                let t = (local_uv - start).dot(dir) / len_sq;
                t.clamp(0.0, 1.0)
            } else {
                0.0
            };
            color_start + (color_end - color_start) * t
        }
        GradientType::Radial => {
            let d = (local_uv - start).length();
            let t = if radius > 0.0 {
                (d / radius).clamp(0.0, 1.0)
            } else {
                0.0
            };
            color_start + (color_end - color_start) * t
        }
    }
}

/// 2D UI vertex shader.
///
/// For SDF-based elements (Rectangle, Circle, Ellipse), this generates
/// 6 vertices (2 triangles) per instance from the element's position and
/// size, reading from the slab. The vertex index (0..5) selects which
/// corner of the quad.
///
/// For Path and TextGlyph elements, the vertex data is read directly
/// from the slab (pre-tessellated vertices).
#[spirv(vertex)]
pub fn ui_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(instance_index)] draw_call_id: Id<UiDrawCallDescriptor>,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    out_uv: &mut Vec2,
    out_color: &mut Vec4,
    #[spirv(flat)] out_draw_call_id: &mut u32,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let viewport: UiViewport = slab.read_unchecked(Id::new(0));
    let draw_call: UiDrawCallDescriptor = slab.read_unchecked(draw_call_id);

    *out_draw_call_id = draw_call_id.inner();

    match draw_call.element_type {
        UiElementType::Path => {
            // For path elements, the draw_call stores an offset into the
            // slab where UiVertex data lives. We read the vertex directly.
            // The atlas_descriptor_id field stores the vertex slab offset.
            let vertex_offset = draw_call.atlas_descriptor_id.inner();
            let vertex_id =
                Id::<UiVertex>::new(vertex_offset + vertex_index * UiVertex::SLAB_SIZE as u32);
            let vertex: UiVertex = slab.read_unchecked(vertex_id);
            *out_uv = vertex.uv;
            *out_color = vertex.color;

            let pos4 = viewport.projection
                * Vec4::new(vertex.position.x, vertex.position.y, draw_call.z, 1.0);
            *out_clip_pos = pos4;
        }
        _ => {
            // SDF-based element: generate quad vertices.
            // Quad corners in CCW order for two triangles:
            //   0: top-left, 1: bottom-left, 2: bottom-right,
            //   3: bottom-right, 4: top-right, 5: top-left
            let vi = vertex_index % 6;
            let (corner_x, corner_y) = match vi {
                0 => (0.0f32, 0.0f32), // top-left
                1 => (0.0, 1.0),       // bottom-left
                2 => (1.0, 1.0),       // bottom-right
                3 => (1.0, 1.0),       // bottom-right
                4 => (1.0, 0.0),       // top-right
                _ => (0.0, 0.0),       // top-left
            };

            let local_uv = Vec2::new(corner_x, corner_y);
            *out_uv = local_uv;
            *out_color = draw_call.fill_color;

            let screen_pos = draw_call.position
                + Vec2::new(corner_x * draw_call.size.x, corner_y * draw_call.size.y);

            let pos4 =
                viewport.projection * Vec4::new(screen_pos.x, screen_pos.y, draw_call.z, 1.0);
            *out_clip_pos = pos4;
        }
    }
}

/// 2D UI fragment shader.
///
/// Evaluates SDF shapes, gradients, textures, and text glyphs depending
/// on the element type.
#[spirv(fragment)]
pub fn ui_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 1)] atlas: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 2)] atlas_sampler: &Sampler,
    in_uv: Vec2,
    in_color: Vec4,
    #[spirv(flat)] in_draw_call_id: u32,
    frag_color: &mut Vec4,
) {
    let draw_call_id = Id::<UiDrawCallDescriptor>::new(in_draw_call_id);
    let draw_call: UiDrawCallDescriptor = slab.read_unchecked(draw_call_id);
    #[allow(unused_assignments)]
    let mut color = Vec4::ZERO;

    match draw_call.element_type {
        UiElementType::Path => {
            // Pre-tessellated path: start with vertex color.
            color = in_color;
            // If an atlas texture is set, sample it and multiply.
            if !draw_call.atlas_texture_id.is_none() {
                let atlas_tex: AtlasTextureDescriptor =
                    slab.read_unchecked(draw_call.atlas_texture_id);
                let viewport: UiViewport = slab.read_unchecked(Id::new(0));
                let atlas_uv = atlas_tex.uv(in_uv, viewport.atlas_size);
                let sample: Vec4 = atlas.sample_by_lod(*atlas_sampler, atlas_uv, 0.0);
                color *= sample;
            }
        }
        UiElementType::TextGlyph => {
            // Text glyph: sample the glyph texture and multiply by color.
            let atlas_tex: AtlasTextureDescriptor = slab.read_unchecked(draw_call.atlas_texture_id);
            let viewport: UiViewport = slab.read_unchecked(Id::new(0));
            let atlas_uv = atlas_tex.uv(in_uv, viewport.atlas_size);
            let sample: Vec4 = atlas.sample_by_lod(*atlas_sampler, atlas_uv, 0.0);
            color = draw_call.fill_color;
            color.w *= sample.w;
        }
        UiElementType::Image => {
            // Textured quad: sample the atlas texture.
            let atlas_tex: AtlasTextureDescriptor = slab.read_unchecked(draw_call.atlas_texture_id);
            let viewport: UiViewport = slab.read_unchecked(Id::new(0));
            let atlas_uv = atlas_tex.uv(in_uv, viewport.atlas_size);
            color = atlas.sample_by_lod(*atlas_sampler, atlas_uv, 0.0);
            // Modulate with fill color (tint).
            color *= draw_call.fill_color;
        }
        _ => {
            // SDF-based element (Rectangle, Circle, Ellipse).
            let half_size = draw_call.size * 0.5;
            // Convert UV (0..1) to element-local coords centered on element
            // center.
            let local_pos = (in_uv - Vec2::splat(0.5)) * draw_call.size;

            let distance = match draw_call.element_type {
                UiElementType::Rectangle => {
                    sdf_rounded_rect(local_pos, half_size, draw_call.corner_radii)
                }
                UiElementType::Circle => {
                    let radius = half_size.x.min(half_size.y);
                    sdf_circle(local_pos, radius)
                }
                UiElementType::Ellipse => sdf_ellipse(local_pos, half_size),
                _ => 0.0,
            };

            // Evaluate fill color (possibly gradient).
            let fill = eval_gradient(
                draw_call.gradient.gradient_type,
                draw_call.gradient.start,
                draw_call.gradient.end,
                draw_call.gradient.radius,
                draw_call.gradient.color_start,
                draw_call.gradient.color_end,
                in_uv,
            );
            // If gradient is None, use the solid fill color.
            let fill = if draw_call.gradient.gradient_type == 0 {
                draw_call.fill_color
            } else {
                fill
            };

            // Anti-aliased edge using smoothstep.
            let aa_width = 1.0; // 1 pixel of anti-aliasing
            let fill_alpha = 1.0 - crate::math::smoothstep(-aa_width, aa_width, distance);

            if draw_call.border_width > 0.0 {
                // Border: the border region is between the outer edge and
                // (outer edge - border_width).
                let inner_distance = distance + draw_call.border_width;
                let border_alpha =
                    1.0 - crate::math::smoothstep(-aa_width, aa_width, inner_distance);
                // Inside the border but outside the fill = border color.
                let in_border = fill_alpha;
                let in_fill = border_alpha;
                color = draw_call.border_color * (in_border - in_fill) + fill * in_fill;
                color.w = draw_call.border_color.w * (in_border - in_fill) + fill.w * in_fill;
            } else {
                color = fill;
                color.w *= fill_alpha;
            }
        }
    }

    // Apply element opacity.
    color.w *= draw_call.opacity;

    *frag_color = color;
}
