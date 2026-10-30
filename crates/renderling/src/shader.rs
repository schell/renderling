//! WGSL shader helpers.
//!
//! Constants, types and procedures.
//!
//! Entry points are kept in their own domain modules.

#[wgsl_rs::wgsl]
pub mod constants {
    //! Common constants.

    use wgsl_rs::std::*;

    pub const INV_ATAN: Vec2f = Vec2f {
        x: 0.1591,
        y: 0.31830987,
    };

    pub const TOP_LEFT_2D: Vec2f = vec2f(0.0, 0.0);
    pub const TOP_RIGHT_2D: Vec2f = vec2f(1.0, 0.0);
    pub const BOTTOM_LEFT_2D: Vec2f = vec2f(0.0, 1.0);
    pub const BOTTOM_RIGHT_2D: Vec2f = vec2f(1.0, 1.0);

    pub const UV_COORD_QUAD_CCW: [Vec2f; 6] = [
        BOTTOM_LEFT_2D,
        BOTTOM_RIGHT_2D,
        TOP_RIGHT_2D,
        TOP_RIGHT_2D,
        TOP_LEFT_2D,
        BOTTOM_LEFT_2D,
    ];

    pub const CLIP_SPACE_COORD_QUAD_CCW_TL: Vec4f = vec4f(-1.0, 1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_BL: Vec4f = vec4f(-1.0, -1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_TR: Vec4f = vec4f(1.0, 1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_BR: Vec4f = vec4f(1.0, -1.0, 0.5, 1.0);

    pub const CLIP_SPACE_COORD_QUAD_CCW: [Vec4f; 6] = [
        CLIP_SPACE_COORD_QUAD_CCW_BL,
        CLIP_SPACE_COORD_QUAD_CCW_BR,
        CLIP_SPACE_COORD_QUAD_CCW_TR,
        CLIP_SPACE_COORD_QUAD_CCW_TR,
        CLIP_SPACE_COORD_QUAD_CCW_TL,
        CLIP_SPACE_COORD_QUAD_CCW_BL,
    ];

    pub const F32_EPSILON: f32 = 1.1920929e-7;

    #[wgsl_ignore]
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn epsilon() {
            const { assert!(F32_EPSILON == f32::EPSILON) };
        }
    }
}

#[wgsl_rs::wgsl]
pub mod math {
    //! Common math procedures.
    use super::constants::*;
    use wgsl_rs::std::*;

    /// Return `0.0` if the length of the vector is very close to zero.
    pub fn vec3_norm_or_zero(v: Vec3f) -> Vec3f {
        if length(v) <= F32_EPSILON {
            Vec3f::ZERO
        } else {
            normalize(v)
        }
    }

    pub fn triangle_face_normal(p1: Vec3f, p2: Vec3f, p3: Vec3f) -> Vec3f {
        let a = p1 - p2;
        let b = p1 - p3;
        let n = normalize(cross(a, b));

        // TODO: re-enable this after https://github.com/schell/wgsl-rs/issues/234
        // #[cfg(cpu)]
        // debug_assert_ne!(
        //     Vec3::ZERO,
        //     n,
        //     "normal is zero - p1: {p1}, p2: {p2}, p3: {p3}"
        // );
        n
    }

    /// Takes the four corners of an axis-aligned box and turns it into a mesh.
    pub const fn convex_mesh(corners: [Vec3f; 8]) -> [Vec3f; 36] {
        let p0 = corners[0];
        let p1 = corners[1];
        let p2 = corners[2];
        let p3 = corners[3];
        let p4 = corners[4];
        let p5 = corners[5];
        let p6 = corners[6];
        let p7 = corners[7];
        [
            p0, p2, p1, p0, p3, p2, // top
            p0, p4, p3, p4, p5, p3, // front
            p3, p6, p2, p3, p5, p6, // right
            p1, p7, p0, p7, p4, p0, // left
            p4, p6, p5, p4, p7, p6, // bottom
            p2, p7, p1, p2, p6, p7, // back
        ]
    }

    /// Points on the unit cube that create a triangle-list mesh.
    ///
    /// Use [`unit_cube`] for a mesh that includes normals.
    ///
    /// `rust-gpu` doesn't like nested/double indexing so we do this here.
    /// See [this comment on discord](https://discord.com/channels/750717012564770887/750717499737243679/1131395331368693770)
    pub const fn cube() -> [Vec3f; 36] {
        let p0 = vec3f(-0.5, 0.5, 0.5);
        let p1 = vec3f(-0.5, 0.5, -0.5);
        let p2 = vec3f(0.5, 0.5, -0.5);
        let p3 = vec3f(0.5, 0.5, 0.5);
        let p4 = vec3f(-0.5, -0.5, 0.5);
        let p7 = vec3f(-0.5, -0.5, -0.5);
        let p6 = vec3f(0.5, -0.5, -0.5);
        let p5 = vec3f(0.5, -0.5, 0.5);
        convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7])
    }

    /// Normalize a plane.
    pub fn normalize_plane(plane: Vec4f) -> Vec4f {
        let mut plane = plane;
        let normal_magnitude = max(
            sqrt(plane.x * plane.x + plane.y * plane.y + plane.z * plane.z),
            F32_EPSILON,
        );
        plane.x /= normal_magnitude;
        plane.y /= normal_magnitude;
        plane.z /= normal_magnitude;
        plane.w /= normal_magnitude;
        plane
    }

    /// Find the intersection point of three planes.
    ///
    /// # Notes
    /// This assumes that the planes will not intersect in a line.
    pub fn intersect_planes(p0: Vec4f, p1: Vec4f, p2: Vec4f) -> Vec3f {
        let bxc = cross(p1.xyz(), p2.xyz());
        let cxa = cross(p2.xyz(), p0.xyz());
        let axb = cross(p0.xyz(), p1.xyz());
        let r = -bxc * p0.w - cxa * p1.w - axb * p2.w;
        r * (1.0 / dot(bxc, p0.xyz()))
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod bvol {
    //! Bounding volume types and helpers.
    //
    // Re-export math's items so sibling wgsl modules can glob this module
    // alone: rustfmt's `imports_granularity = "crate"` re-groups any two
    // same-root imports into grouped-use form, which the wgsl parser does
    // not support (wgsl-rs#171).
    pub use crate::shader::math::*;
    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    /// Axis aligned bounding box.
    #[derive(Clone, Copy, Debug, Default, PartialEq, SlabItem)]
    pub struct Aabb {
        pub min: Vec3f,
        pub max: Vec3f,
    }

    /// Six planes of a view frustum.
    #[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
    pub struct Frustum {
        /// Planes constructing the sides of the frustum,
        /// each expressed as a normal vector (xyz) and the distance (w)
        /// from the origin along that vector.
        pub planes: [Vec4f; 6],
        /// Points representing the corners of the frustum
        pub points: [Vec3f; 8],
        /// Centroid of the corners of the frustum
        pub center: Vec3f,
    }

    impl Frustum {
        pub fn test_against_aabb(frustum: Frustum, aabb: Aabb) -> bool {
            for i in 0..3usize {
                let mut out = 0u32;
                for j in 0..8usize {
                    if frustum.points[j][i] < aabb.min[i] {
                        out += 1;
                    }
                }
                if out == 8 {
                    return false;
                }
                out = 0;
                for j in 0..8usize {
                    if frustum.points[j][i] > aabb.max[i] {
                        out += 1;
                    }
                }
                if out == 8 {
                    return false;
                }
            }
            true
        }

        /// Returns the depth of the frustum.
        pub fn depth(frustum: Self) -> f32 {
            abs(frustum.planes[0].w - frustum.planes[5].w)
        }
    }
}

#[wgsl_rs::wgsl(extensions = [crabslab_wgsl::SlabItemExt])]
pub mod camera {
    use super::bvol::*;
    use crabslab_wgsl::*;
    use wgsl_rs::std::*;

    /// GPU descriptor of a camera.
    ///
    /// Used for transforming the stage during rendering.
    ///
    /// Use [`CameraDescriptor::new`] to create a new camera.
    #[derive(Default, Clone, Copy, PartialEq, SlabItem, core::fmt::Debug)]
    pub struct CameraDescriptor {
        projection: Mat4f,
        view: Mat4f,
        position: Vec3f,
        frustum: Frustum,
        /// Nearest center point on the frustum
        z_near_point: Vec3f,
        /// Furthest center point on the frustum
        z_far_point: Vec3f,
    }

    impl CameraDescriptor {
        pub fn projection(cd: CameraDescriptor) -> Mat4f {
            cd.projection
        }

        pub fn view(cd: CameraDescriptor) -> Mat4f {
            cd.view
        }

        pub fn position(cd: CameraDescriptor) -> Vec3f {
            cd.position
        }

        pub fn view_projection(cd: CameraDescriptor) -> Mat4f {
            cd.projection * cd.view
        }

        /// Compute a frustum in world space from the given
        /// [`CameraDescriptor`].
        pub fn frustum(camera: CameraDescriptor) -> Frustum {
            let mvp = CameraDescriptor::view_projection(camera);

            let left = normalize_plane(vec4f(
                mvp[0u32][0u32] + mvp[0u32][3u32],
                mvp[1u32][0u32] + mvp[1u32][3u32],
                mvp[2u32][0u32] + mvp[2u32][3u32],
                mvp[3u32][0u32] + mvp[3u32][3u32],
            ));
            let right = normalize_plane(vec4f(
                -mvp[0u32][0u32] + mvp[0u32][3u32],
                -mvp[1u32][0u32] + mvp[1u32][3u32],
                -mvp[2u32][0u32] + mvp[2u32][3u32],
                -mvp[3u32][0u32] + mvp[3u32][3u32],
            ));
            let bottom = normalize_plane(vec4f(
                mvp[0u32][1u32] + mvp[0u32][3u32],
                mvp[1u32][1u32] + mvp[1u32][3u32],
                mvp[2u32][1u32] + mvp[2u32][3u32],
                mvp[3u32][1u32] + mvp[3u32][3u32],
            ));
            let top = normalize_plane(vec4f(
                -mvp[0u32][1u32] + mvp[0u32][3u32],
                -mvp[1u32][1u32] + mvp[1u32][3u32],
                -mvp[2u32][1u32] + mvp[2u32][3u32],
                -mvp[3u32][1u32] + mvp[3u32][3u32],
            ));
            // For wgpu/Vulkan/D3u32D (z_ndc ∈ [0, 1u32]) the near plane is just
            // row2, not row2 + row3u32 (which is the OpenGL z_ndc ∈
            // [-1u32, 1u32] convention).
            let near = normalize_plane(vec4f(
                mvp[0u32][2u32],
                mvp[1u32][2u32],
                mvp[2u32][2u32],
                mvp[3u32][2u32],
            ));
            let far = normalize_plane(vec4f(
                -mvp[0u32][2u32] + mvp[0u32][3u32],
                -mvp[1u32][2u32] + mvp[1u32][3u32],
                -mvp[2u32][2u32] + mvp[2u32][3u32],
                -mvp[3u32][2u32] + mvp[3u32][3u32],
            ));

            // Account for the possibility that the projection is infinite.
            //
            // See <https://renderling.xyz/devlog/index.html#actual_frustum_culling>
            // for more details.
            let far_v3 = near.xyz() * -1.0;
            let far = vec4f(far_v3.x, far_v3.y, far_v3.z, far.w);

            let flt = intersect_planes(far, left, top);
            let frt = intersect_planes(far, right, top);
            let flb = intersect_planes(far, left, bottom);
            let frb = intersect_planes(far, right, bottom);
            let nlt = intersect_planes(near, left, top);
            let nrt = intersect_planes(near, right, top);
            let nlb = intersect_planes(near, left, bottom);
            let nrb = intersect_planes(near, right, bottom);

            Frustum {
                planes: [near, left, right, bottom, top, far],
                points: [nlt, nrt, nlb, nrb, flt, frt, flb, frb],
                center: (nlt + nrt + nlb + nrb) / 4.0,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crabslab::SlabItem as LegacySlabItem;
    use crabslab_wgsl::CpuSlabItem;
    use glam::Mat4;

    use crate::{
        bvol::Frustum as FrustumGlam,
        camera::shader::CameraDescriptor as CameraDescriptorGlam,
        shader::{bvol::Frustum as FrustumWgsl, camera::CameraDescriptor as CameraDescriptorWgsl},
    };

    /// A consumer module living in a different file composes the camera
    /// types out of the `crate::shader` tree and calls their impl methods -
    /// the exact shape the skybox shaders will use, proving the camera
    /// types are consumable WGSL before that port begins.
    #[allow(dead_code)]
    #[wgsl_rs::wgsl]
    pub mod smoke {
        use crate::shader::camera::*;
        use wgsl_rs::std::*;

        pub fn view_projection(camera: CameraDescriptor) -> Vec4f {
            CameraDescriptor::view_projection(camera) * vec4f(0.0, 0.0, 0.0, 1.0)
        }

        pub fn frustum_center(camera: CameraDescriptor) -> Vec3f {
            CameraDescriptor::frustum(camera).center
        }
    }

    /// The two stacks agree on slab sizes: a `CameraDescriptor` written by
    /// the legacy glam stack and read back by the wgsl-rs stack's WGSL
    /// module (and vice versa) must occupy the same number of `u32` slots,
    /// or all neighboring slab reads corrupt.
    #[test]
    fn slab_sizes_match() {
        // Frustum: planes [Vec4; 6] (24) + points [Vec3; 8] (24) + center (3).
        assert_eq!(51, FrustumGlam::SLAB_SIZE);
        assert_eq!(51, <FrustumWgsl as crabslab_wgsl::SlabItem>::SLAB_SIZE);

        // CameraDescriptor: projection (16) + view (16) + position (3) +
        // frustum (51) + z_near_point (3) + z_far_point (3).
        assert_eq!(92, CameraDescriptorGlam::SLAB_SIZE);
        assert_eq!(
            92,
            <CameraDescriptorWgsl as crabslab_wgsl::SlabItem>::SLAB_SIZE
        );
    }

    /// Slab records are bit-identical in both directions: the legacy glam
    /// stack's writer against the wgsl-rs stack's reader, and the wgsl-rs
    /// writer against the legacy reader. This is the transition contract -
    /// during the port the stages write with glam hybrids and the WGSL
    /// shaders read the same slab.
    ///
    /// Pure CPU on both sides (both slab impls are plain Rust) - the WGSL
    /// world, GPU proper, is exercised separately by
    /// `camera_module_validates`, which runs naga over the transpiled
    /// module.
    #[test]
    fn camera_descriptor_slab_compatibility() {
        let cameras = [
            (
                "perspective",
                CameraDescriptorGlam::default_perspective(640.0, 480.0),
            ),
            (
                "translated",
                CameraDescriptorGlam::new(
                    Mat4::perspective_rh(0.5, 1.4, 0.5, 100.0),
                    Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0)),
                ),
            ),
        ];

        for (name, glam_cam) in cameras {
            // LEGACY -> WGSL
            let size = CameraDescriptorGlam::SLAB_SIZE;
            let mut legacy_slab = vec![0u32; size];
            let end = glam_cam.write_slab(0, &mut legacy_slab);
            assert_eq!(end, size, "{name}: legacy write end index");
            assert!(
                legacy_slab.iter().any(|slot| *slot != 0),
                "{name}: slab should not be all zeros"
            );

            let wgsl_cam: CameraDescriptorWgsl =
                <CameraDescriptorWgsl as CpuSlabItem>::read_slab(0, &legacy_slab);

            // WGSL -> LEGACY
            let mut wgsl_slab = vec![0u32; size];
            let end = wgsl_cam.write_slab(0, &mut wgsl_slab);
            assert_eq!(end, size, "{name}: wgsl write end index");
            assert_eq!(legacy_slab, wgsl_slab, "{name}: round trip bit exact");

            let glam_back: CameraDescriptorGlam =
                LegacySlabItem::read_slab(0, wgsl_slab.as_slice());
            assert_eq!(glam_cam, glam_back, "{name}: glam read of wgsl write");
        }
    }

    /// The WGSL camera module's frustum construction runs on the CPU (every
    /// `#[wgsl]` function is plain Rust there) and is compared against the
    /// glam reference: the same camera must produce the same planes, points
    /// and center. The GPU world spelling of the same fn is checked by naga
    /// in `camera_module_validates`.
    #[test]
    fn frustum_construction_matches_glam() {
        let glam_cam = CameraDescriptorGlam::default_perspective(640.0, 480.0);
        let mut slab = vec![0u32; CameraDescriptorGlam::SLAB_SIZE];
        glam_cam.write_slab(0, &mut slab);
        let wgsl_cam: CameraDescriptorWgsl =
            <CameraDescriptorWgsl as CpuSlabItem>::read_slab(0, &slab);

        let glam_frustum = FrustumGlam::from_camera(&glam_cam);
        let wgsl_frustum = CameraDescriptorWgsl::frustum(wgsl_cam);

        let assert_close = |label: String, glam_vals: &[f32], wgsl_vals: &[f32]| {
            assert_eq!(glam_vals.len(), wgsl_vals.len(), "{label} length");
            for (j, (a, b)) in glam_vals.iter().zip(wgsl_vals.iter()).enumerate() {
                let d = (a - b).abs();
                assert!(
                    d <= 1e-6 * (a.abs().max(b.abs()).max(1.0)),
                    "{label}[{j}]: glam {a} vs wgsl {b}"
                );
            }
        };

        for i in 0..6usize {
            let wgsl_plane = wgsl_frustum.planes[i];
            assert_close(
                format!("plane{i}"),
                glam_frustum.planes[i].to_array().as_slice(),
                [wgsl_plane.x, wgsl_plane.y, wgsl_plane.z, wgsl_plane.w].as_slice(),
            );
        }
        for i in 0..8usize {
            let wgsl_point = wgsl_frustum.points[i];
            assert_close(
                format!("point{i}"),
                glam_frustum.points[i].to_array().as_slice(),
                [wgsl_point.x, wgsl_point.y, wgsl_point.z].as_slice(),
            );
        }
        assert_close(
            "center".to_string(),
            glam_frustum.center.to_array().as_slice(),
            [
                wgsl_frustum.center.x,
                wgsl_frustum.center.y,
                wgsl_frustum.center.z,
            ]
            .as_slice(),
        );

        // The accessors the skybox shaders call, compared against the glam
        // fields - matrices are column-major in both worlds.
        let assert_mat_close = |label: &str, glam_mat: &Mat4, wgsl_mat: wgsl_rs::std::Mat4x4f| {
            let glam_cols = glam_mat.to_cols_array();
            for i in 0..4u32 {
                let wgsl_col = wgsl_mat[i];
                for j in 0..4u32 {
                    let glam = glam_cols[(i * 4 + j) as usize];
                    let wgsl = wgsl_col[j];
                    let d = (glam - wgsl).abs();
                    assert!(
                        d <= 1e-6 * (glam.abs().max(wgsl.abs()).max(1.0)),
                        "{label} col{i} row{j}: glam {glam} vs wgsl {wgsl}"
                    );
                }
            }
        };
        assert_mat_close(
            "projection",
            &glam_cam.projection(),
            CameraDescriptorWgsl::projection(wgsl_cam),
        );
        assert_mat_close(
            "view",
            &glam_cam.view(),
            CameraDescriptorWgsl::view(wgsl_cam),
        );
        let glam_pos = glam_cam.position().to_array();
        let wgsl_pos = CameraDescriptorWgsl::position(wgsl_cam);
        assert_close(
            "position".to_string(),
            glam_pos.as_slice(),
            [wgsl_pos.x, wgsl_pos.y, wgsl_pos.z].as_slice(),
        );
        let wgsl_vp = CameraDescriptorWgsl::view_projection(wgsl_cam);
        let wgsl_vp_arr_2d = [wgsl_vp[0u32], wgsl_vp[1u32], wgsl_vp[2u32], wgsl_vp[3u32]]
            .map(|p| [p.x, p.y, p.z, p.w]);
        assert_close(
            "view_projection".to_string(),
            glam_cam.view_projection().to_cols_array().as_slice(),
            wgsl_vp_arr_2d.as_flattened(),
        );
    }
}
