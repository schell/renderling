//! Lighting effects.
//!
//! This module includes support for various types of lights such as
//! directional, point, and spot lights.
//!
//! Additionally, the module provides shadow mapping to create realistic shadows.
//!
//! Also provided is an implementation of light tiling, a technique that optimizes
//! the rendering of thousands of analytical lights. If you find your scene performing
//! poorly under the load of very many lights, [`LightTiling`] can speed things up.
//!
//! ## Analytical lights
//!
//! Analytical lights are a fundamental lighting effect in a scene.
//! These lights can be created directly from the [`Stage`] using the methods
//! * [`Stage::new_directional_light`]
//! * [`Stage::new_point_light`]
//! * [`Stage::new_spot_light`]
//!
//! Each of these methods returns an [`AnalyticalLight`] instance that can be
//! manipulated to simulate different lighting conditions.
//!
//! Once created, these lights can be positioned and oriented using
//! [`Transform`] or [`NestedTransform`] objects. The [`Transform`] allows you
//! to set the position, rotation, and scale of the light, while
//! [`NestedTransform`] enables hierarchical transformations, which is useful
//! for complex scenes where lights need to follow specific objects or structures.
//!
//! By adjusting the properties of these lights, such as intensity, color, and
//! direction, you can achieve a wide range of lighting effects, from simulating
//! sunlight with directional lights to creating focused spotlights or ambient
//! point lights. These lights can also be combined with shadow mapping
//! techniques to enhance the realism of shadows in the scene.
//!
//! ## Shadow mapping
//!
//! Shadow mapping is a technique used to add realistic shadows to a scene by
//! simulating the way light interacts with objects.
//!
//! To create a [`ShadowMap`], use the [`Stage::new_shadow_map`] method, passing in
//! the light source and desired parameters such as the size of the shadow map
//! and the near and far planes of the light's frustum.  Once created, the
//! shadow map needs to be updated each frame (or as needed) using the
//! [`ShadowMap::update`] method, which renders the scene from the light's
//! perspective to determine which areas are in shadow.
//!
//! This technique allows for dynamic shadows that change with the movement of
//! lights and objects, enhancing the realism of the scene. Proper
//! configuration of shadow map parameters, such as bias and resolution, is
//! crucial to achieving high-quality shadows without artifacts, and varies
//! with each scene.
//!
//! ## Light tiling
//!
//! Light tiling is a technique used to optimize the rendering of scenes with a
//! large number of lights.
//!
//! It divides the rendering surface into a grid of tiles, allowing for
//! efficient computation of lighting effects by processing each tile
//! independently and cutting down on the overall lighting calculations.
//!
//! To create a [`LightTiling`], use the [`Stage::new_light_tiling`] method,
//! providing a [`LightTilingConfig`] to specify parameters such as tile size
//! and maximum lights per tile.
//!
//! Once created, the [`LightTiling`] instance should be kept in sync with the
//! scene by calling the [`LightTiling::run`] method each frame, or however you
//! see fit. This method updates the lighting calculations for each tile based
//! on the current scene configuration, ensuring optimal performance even with
//! many lights.
//!
//! By using light tiling, you can significantly improve the performance of your
//! rendering pipeline, especially in complex scenes with numerous light sources.

#[cfg(doc)]
use crate::{
    stage::Stage,
    transform::{NestedTransform, Transform},
};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

#[cfg(cpu)]
mod shadow_map;
#[cfg(cpu)]
pub use shadow_map::*;

#[cfg(cpu)]
mod tiling;
#[cfg(cpu)]
pub use tiling::*;

pub mod shader;

#[cfg(test)]
mod test {
    use crabslab::Array;
    use glam::{UVec2, UVec3, Vec2, Vec3};

    use crate::math::{GpuRng, IsVector};

    use super::shader::*;

    #[cfg(feature = "gltf")]
    #[test]
    fn position_direction_sanity() {
        // With GLTF, the direction of a light is given by the light's node's transform.
        // Specifically we get the node's transform and use the rotation quaternion to
        // rotate the vector Vec3::NEG_Z - the result is our direction.

        use glam::{Mat4, Quat};
        println!("{:#?}", std::env::current_dir());
        let (document, _buffers, _images) = gltf::import("../../gltf/four_spotlights.glb").unwrap();
        for node in document.nodes() {
            use glam::Vec3;

            println!("node: {} {:?}", node.index(), node.name());

            let gltf_transform = node.transform();
            let (translation, rotation, _scale) = gltf_transform.decomposed();
            let position = Vec3::from_array(translation);
            let direction =
                Mat4::from_quat(Quat::from_array(rotation)).transform_vector3(Vec3::NEG_Z);
            println!("position: {position}");
            println!("direction: {direction}");

            // In Blender, our lights are sitting at (0, 0, 1) pointing at -Z, +Z, +X and
            // +Y. But alas, it is a bit more complicated than that because this
            // file is exported with UP being +Y, so Z and Y have been
            // flipped...
            assert_eq!(Vec3::Y, position);
            let expected_direction = match node.name() {
                Some("light_negative_z") => Vec3::NEG_Y,
                Some("light_positive_z") => Vec3::Y,
                Some("light_positive_x") => Vec3::X,
                Some("light_positive_y") => Vec3::NEG_Z,
                n => panic!("unexpected node '{n:?}'"),
            };
            // And also there are rounding ... imprecisions...
            assert_approx_eq::assert_approx_eq!(expected_direction.x, direction.x);
            assert_approx_eq::assert_approx_eq!(expected_direction.y, direction.y);
            assert_approx_eq::assert_approx_eq!(expected_direction.z, direction.z);
        }
    }

    #[test]
    /// Test that we can determine if a point is inside clip space or not.
    fn clip_space_bounds_sanity() {
        let inside = Vec3::ONE;
        assert!(
            crate::math::is_inside_clip_space(inside),
            "should be inside"
        );
        let inside = Vec3::new(0.5, -0.5, 0.8);
        assert!(
            crate::math::is_inside_clip_space(inside),
            "should be inside"
        );
        let outside_neg_z = Vec3::new(0.5, -0.5, -0.8);
        assert!(
            !crate::math::is_inside_clip_space(outside_neg_z),
            "negative z should be outside (wgpu z range is [0, 1])"
        );
        let outside = Vec3::new(0.5, 0.0, 1.3);
        assert!(
            !crate::math::is_inside_clip_space(outside),
            "should be outside"
        );
    }

    #[test]
    fn finding_orthogonal_vectors_sanity() {
        const THRESHOLD: f32 = f32::EPSILON * 3.0;

        let mut prng = GpuRng::new(0);
        for _ in 0..100 {
            let v2 = prng.gen_vec2(Vec2::splat(-100.0), Vec2::splat(100.0));
            let v2_ortho = v2.orthonormal_vectors();
            let v2_dot = v2.dot(v2_ortho);
            if v2_dot.abs() >= THRESHOLD {
                panic!("{v2} • {v2_ortho} < {THRESHOLD}, saw {v2_dot}")
            }

            let v3 = prng
                .gen_vec3(Vec3::splat(-100.0), Vec3::splat(100.0))
                .alt_norm_or_zero();
            for v3_ortho in v3.orthonormal_vectors() {
                let v3_dot = v3.dot(v3_ortho);
                if v3_dot.abs() >= THRESHOLD {
                    panic!("{v3} • {v3_ortho} < {THRESHOLD}, saw {v3_dot}");
                }
            }
        }
    }

    #[test]
    fn next_light_sanity() {
        {
            let lights_array = Array::new(0, 1);
            // When there's only one light we only need one invocation to check that one light
            // (per tile)
            let mut next_light = NextLightIndex::new(UVec3::new(0, 0, 0), 16, lights_array);
            assert_eq!(Some(0u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next invocation won't check anything
            let mut next_light = NextLightIndex::new(UVec3::new(1, 0, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
            // Neither will the next row
            let mut next_light = NextLightIndex::new(UVec3::new(0, 1, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
        }
        {
            let lights_array = Array::new(0, 2);
            // When there's two lights we need two invocations
            let mut next_light = NextLightIndex::new(UVec3::new(0, 0, 0), 16, lights_array);
            assert_eq!(Some(0u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next invocation checks the second light
            let mut next_light = NextLightIndex::new(UVec3::new(1, 0, 0), 16, lights_array);
            assert_eq!(Some(1u32.into()), next_light.next());
            assert_eq!(None, next_light.next());
            // The next one doesn't check anything
            let mut next_light = NextLightIndex::new(UVec3::new(2, 0, 0), 16, lights_array);
            assert_eq!(None, next_light.next());
        }
        {
            // With 256 lights (16*16), each fragment in the tile checks exactly one light
            let lights_array = Array::new(0, 16 * 16);
            let mut checked_lights = vec![];
            for y in 0..16 {
                for x in 0..16 {
                    let mut next_light = NextLightIndex::new(UVec3::new(x, y, 0), 16, lights_array);
                    let next_index = next_light.next_index();
                    let checked_light = next_light.next().unwrap();
                    assert_eq!(next_index, checked_light.index());
                    checked_lights.push(checked_light);
                    assert_eq!(None, next_light.next());
                }
            }
            println!("checked_lights: {checked_lights:#?}");
            assert_eq!(256, checked_lights.len());
        }
    }

    #[test]
    fn frag_coord_to_tile_index() {
        let tiling_desc = LightTilingDescriptor {
            depth_texture_size: UVec2::new(1024, 800),
            ..Default::default()
        };
        for x in 0..16 {
            let index = tiling_desc.tile_index_for_fragment(Vec2::new(x as f32, 0.0));
            assert_eq!(0, index);
        }
        let index = tiling_desc.tile_index_for_fragment(Vec2::new(16.0, 0.0));
        assert_eq!(1, index);
        let index = tiling_desc.tile_index_for_fragment(Vec2::new(0.0, 16.0));
        assert_eq!(1024 / 16, index);

        let tiling_desc = LightTilingDescriptor {
            depth_texture_size: UVec2::new(
                (10.0 * 2.0f32.powi(8)) as u32,
                (9.0 * 2.0f32.powi(8)) as u32,
            ),
            ..Default::default()
        };
        let frag_coord = Vec2::new(1917.0, 979.0);
        let tile_coord = tiling_desc.tile_coord_for_fragment(frag_coord);
        assert_eq!(UVec2::new(119, 61), tile_coord);
    }
}
