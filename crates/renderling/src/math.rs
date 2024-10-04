//! Mathematical helper types and functions.
//!
//! Primarily this module re-exports types from `glam`. It also adds
//! some traits to help using `glam` types on the GPU without panicking,
//! as well as a few traits to aid in writing generic shader code that can be
//! run on the CPU.
//!
//! Lastly, it provides some constant geometry used in many shaders.
use core::ops::Mul;
use spirv_std::{
    image::{Cubemap, Image2d, Image2dArray},
    Sampler,
};

pub use glam::*;
pub use spirv_std::num_traits::{clamp, Float, Zero};
pub trait IsSampler: Copy + Clone {}

impl IsSampler for Sampler {}

pub trait Sample2d {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4;
}

impl Sample2d for Image2d {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

pub trait Sample2dArray {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec3, lod: f32) -> glam::Vec4;
}

impl Sample2dArray for Image2dArray {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec3, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

pub trait SampleCube {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4;
}

impl SampleCube for Cubemap {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

#[cfg(not(target_arch = "spirv"))]
mod cpu {
    use image::GenericImageView;

    use super::*;

    /// A CPU-side texture sampler.
    ///
    /// Provided primarily for testing purposes.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct CpuSampler;

    impl IsSampler for CpuSampler {}

    #[derive(Debug, Default)]
    pub struct CpuTexture2d {
        pub image: image::DynamicImage,
    }

    impl Sample2d for CpuTexture2d {
        type Sampler = CpuSampler;

        fn sample_by_lod(&self, _sampler: Self::Sampler, uv: glam::Vec2, _lod: f32) -> glam::Vec4 {
            // TODO: lerp the CPU texture sampling
            let x = uv.x as u32;
            if x >= self.image.width() {
                return glam::Vec4::ZERO;
            }

            let y = uv.y as u32;
            if y >= self.image.height() {
                return glam::Vec4::ZERO;
            }

            let image::Rgba([r, g, b, a]) = self.image.get_pixel(uv.x as u32, uv.y as u32);
            glam::Vec4::new(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            )
        }
    }

    /// A CPU-side cubemap texture.
    ///
    /// Provided primarily for testing purposes.
    #[derive(Default)]
    pub struct CpuCubemap {
        pub images: [image::DynamicImage; 6],
    }

    impl SampleCube for CpuCubemap {
        type Sampler = CpuSampler;

        fn sample_by_lod(
            &self,
            _sampler: Self::Sampler,
            direction: glam::Vec3,
            _lod: f32,
        ) -> glam::Vec4 {
            // Take the absolute value of the direction vector components
            let abs_direction = direction.abs();
            let (max_dim, u, v): (usize, f32, f32);

            // Determine which face of the cubemap the direction vector is pointing towards
            // by finding the largest component of the vector.
            // The u and v texture coordinates within that face are calculated by dividing
            // the other two components of the direction vector by the largest component.
            if abs_direction.x >= abs_direction.y && abs_direction.x >= abs_direction.z {
                max_dim = if direction.x >= 0.0 { 0 } else { 1 };
                u = -direction.z / abs_direction.x;
                v = -direction.y / abs_direction.x;
            } else if abs_direction.y >= abs_direction.x && abs_direction.y >= abs_direction.z {
                max_dim = if direction.y >= 0.0 { 2 } else { 3 };
                u = direction.x / abs_direction.y;
                v = -direction.z / abs_direction.y;
            } else {
                max_dim = if direction.z >= 0.0 { 4 } else { 5 };
                u = direction.x / abs_direction.z;
                v = direction.y / abs_direction.z;
            }

            // Get the dimensions of the cubemap image
            let (width, height) = self.images[max_dim].dimensions();
            // Convert the u and v coordinates from [-1, 1] to [0, width/height]
            let tex_u = ((u + 1.0) * 0.5 * (width as f32 - 1.0)).round() as u32;
            if tex_u >= self.images[max_dim].width() {
                return glam::Vec4::ZERO;
            }
            let tex_v = ((1.0 - v) * 0.5 * (height as f32 - 1.0)).round() as u32;
            if tex_v >= self.images[max_dim].height() {
                return glam::Vec4::ZERO;
            }

            // Sample and return the color from the appropriate image in the cubemap
            let pixel = self.images[max_dim].get_pixel(tex_u, tex_v);
            glam::Vec4::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
                pixel[3] as f32 / 255.0,
            )
        }
    }
}
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Additional/replacement methods for glam vector types.
///
/// These are required because `naga` (`wgpu`'s translation layer) doesn't like
/// certain contstants like `f32::INFINITY` or `f32::NaN`, which cause errors in
/// naga's WGSL output.
///
/// See [this issue](https://github.com/gfx-rs/naga/issues/2461) and `crate::linkage::test`
/// for more info.
pub trait IsVector {
    /// Normalize or return zero.
    fn alt_norm_or_zero(&self) -> Self;

    /// Return a vector with `signum_or_zero` applied to each component.
    fn signum_or_zero(&self) -> Self;

    /// Returns the dot product of a vector with itself (the square of its
    /// length).
    fn dot2(&self) -> f32;
}

impl IsVector for glam::Vec2 {
    fn alt_norm_or_zero(&self) -> Self {
        if self.length().is_zero() {
            glam::Vec2::ZERO
        } else {
            self.normalize()
        }
    }

    fn signum_or_zero(&self) -> Self {
        Vec2::new(signum_or_zero(self.x), signum_or_zero(self.y))
    }

    fn dot2(&self) -> f32 {
        self.dot(*self)
    }
}

impl IsVector for glam::Vec3 {
    fn alt_norm_or_zero(&self) -> Self {
        if self.length().is_zero() {
            glam::Vec3::ZERO
        } else {
            self.normalize()
        }
    }

    fn signum_or_zero(&self) -> Self {
        Vec3::new(
            signum_or_zero(self.x),
            signum_or_zero(self.y),
            signum_or_zero(self.z),
        )
    }

    fn dot2(&self) -> f32 {
        self.dot(*self)
    }
}

/// Additional/replacement methods for glam matrix types.
///
/// These are required because `naga` (`wgpu`'s translation layer) doesn't like
/// certain contstants like `f32::INFINITY` or `f32::NaN`, which cause errors in
/// naga's WGSL output.
///
/// See [this issue](https://github.com/gfx-rs/naga/issues/2461) and `crate::linkage::test`
/// for more info.
pub trait IsMatrix {
    /// Extracts `scale`, `rotation` and `translation` from `self`. The input
    /// matrix is expected to be a 3D affine transformation matrix otherwise
    /// the output will be invalid.
    ///
    /// Will return `(Vec3::ONE, Quat::IDENTITY, Vec3::ZERO)` if the determinant
    /// of `self` is zero or if the resulting scale vector contains any zero
    /// elements when `glam_assert` is enabled.
    ///
    /// This is required instead of using
    /// [`glam::Mat4::to_scale_rotation_translation`], because that uses
    /// f32::signum, which compares against `f32::NAN`, which causes an error
    /// in naga's WGSL output.
    ///
    /// See [this issue](https://github.com/gfx-rs/naga/issues/2461) and `crate::linkage::test`
    /// for more info.
    fn to_scale_rotation_translation_or_id(&self) -> (glam::Vec3, glam::Quat, glam::Vec3);
}

/// From the columns of a 3x3 rotation matrix.
///
/// All of this because we can't use NaNs.
#[inline]
fn from_rotation_axes(x_axis: glam::Vec3, y_axis: glam::Vec3, z_axis: glam::Vec3) -> glam::Quat {
    // Based on https://github.com/microsoft/DirectXMath `XM$quaternionRotationMatrix`
    let (m00, m01, m02) = x_axis.into();
    let (m10, m11, m12) = y_axis.into();
    let (m20, m21, m22) = z_axis.into();
    if m22 <= 0.0 {
        // x^2 + y^2 >= z^2 + w^2
        let dif10 = m11 - m00;
        let omm22 = 1.0 - m22;
        if dif10 <= 0.0 {
            // x^2 >= y^2
            let four_xsq = omm22 - dif10;
            let inv4x = 0.5 / four_xsq.sqrt();
            glam::Quat::from_xyzw(
                four_xsq * inv4x,
                (m01 + m10) * inv4x,
                (m02 + m20) * inv4x,
                (m12 - m21) * inv4x,
            )
        } else {
            // y^2 >= x^2
            let four_ysq = omm22 + dif10;
            let inv4y = 0.5 / four_ysq.sqrt();
            glam::Quat::from_xyzw(
                (m01 + m10) * inv4y,
                four_ysq * inv4y,
                (m12 + m21) * inv4y,
                (m20 - m02) * inv4y,
            )
        }
    } else {
        // z^2 + w^2 >= x^2 + y^2
        let sum10 = m11 + m00;
        let opm22 = 1.0 + m22;
        if sum10 <= 0.0 {
            // z^2 >= w^2
            let four_zsq = opm22 - sum10;
            let inv4z = 0.5 / four_zsq.sqrt();
            glam::Quat::from_xyzw(
                (m02 + m20) * inv4z,
                (m12 + m21) * inv4z,
                four_zsq * inv4z,
                (m01 - m10) * inv4z,
            )
        } else {
            // w^2 >= z^2
            let four_wsq = opm22 + sum10;
            let inv4w = 0.5 / four_wsq.sqrt();
            glam::Quat::from_xyzw(
                (m12 - m21) * inv4w,
                (m20 - m02) * inv4w,
                (m01 - m10) * inv4w,
                four_wsq * inv4w,
            )
        }
    }
}

const fn srt_id() -> (Vec3, Quat, Vec3) {
    (Vec3::ONE, Quat::IDENTITY, Vec3::ZERO)
}

impl IsMatrix for glam::Mat4 {
    #[inline]
    fn to_scale_rotation_translation_or_id(&self) -> (glam::Vec3, glam::Quat, glam::Vec3) {
        let det = self.determinant();
        if det == 0.0 {
            crate::println!("det == 0.0, returning identity");
            return srt_id();
        }

        let det_sign = if det >= 0.0 { 1.0 } else { -1.0 };

        let scale = glam::Vec3::new(
            self.x_axis.length() * det_sign,
            self.y_axis.length(),
            self.z_axis.length(),
        );

        if !scale.cmpne(glam::Vec3::ZERO).all() {
            return srt_id();
        }

        let inv_scale = scale.recip();

        let rotation = from_rotation_axes(
            self.x_axis.mul(inv_scale.x).xyz(),
            self.y_axis.mul(inv_scale.y).xyz(),
            self.z_axis.mul(inv_scale.z).xyz(),
        );

        let translation = self.w_axis.xyz();

        (scale, rotation, translation)
    }
}

/// Returns `1.0` if `n` is greater than or equal to `0.0`.
/// Returns `1.0` if `n` is greater than or equal to `-0.0`.
/// Returns `-1.0` if `n` is less than `0.0`.
/// Returns `0.0` if `n` is `NaN`.
pub fn signum_or_zero(n: f32) -> f32 {
    ((n >= 0.0) as u32) as f32 - ((n < 0.0) as u32) as f32
}

/// Return `1.0` when `value` is greater than or equal to `edge` and `0.0` where
/// `value` is less than `edge`.
#[inline(always)]
pub fn step_ge(value: f32, edge: f32) -> f32 {
    ((value >= edge) as u32) as f32
}

/// Return `1.0` when `value` is less than or equal to `edge`
/// and `0.0` when `value` is greater than `edge`.
#[inline(always)]
pub fn step_le(value: f32, edge: f32) -> f32 {
    ((value <= edge) as u32) as f32
}

pub fn smoothstep(edge_in: f32, edge_out: f32, mut x: f32) -> f32 {
    // Scale, and clamp x to 0..1 range
    x = clamp((x - edge_in) / (edge_out - edge_in), 0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

pub fn triangle_face_normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
    let a = p1 - p2;
    let b = p1 - p3;
    let n: Vec3 = a.cross(b).normalize();
    n
}

/// Convert a color from a hexadecimal number (eg. `0x52b14eff`) into a Vec4.
pub fn hex_to_vec4(color: u32) -> Vec4 {
    let r = ((color >> 24) & 0xFF) as f32 / 255.0;
    let g = ((color >> 16) & 0xFF) as f32 / 255.0;
    let b = ((color >> 8) & 0xFF) as f32 / 255.0;
    let a = (color & 0xFF) as f32 / 255.0;

    Vec4::new(r, g, b, a)
}

pub const UNIT_QUAD_CCW: [Vec3; 6] = {
    let tl = Vec3::new(-0.5, 0.5, 0.0);
    let tr = Vec3::new(0.5, 0.5, 0.0);
    let bl = Vec3::new(-0.5, -0.5, 0.0);
    let br = Vec3::new(0.5, -0.5, 0.0);
    [bl, br, tr, tr, tl, bl]
};

pub const CLIP_QUAD_CCW: [Vec3; 6] = {
    let tl = Vec3::new(-1.0, 1.0, 0.0);
    let tr = Vec3::new(1.0, 1.0, 0.0);
    let bl = Vec3::new(-1.0, -1.0, 0.0);
    let br = Vec3::new(1.0, -1.0, 0.0);
    [bl, br, tr, tr, tl, bl]
};

pub const CLIP_SPACE_COORD_QUAD_CCW: [Vec4; 6] = {
    let tl = Vec4::new(-1.0, 1.0, 0.5, 1.0);
    let tr = Vec4::new(1.0, 1.0, 0.5, 1.0);
    let bl = Vec4::new(-1.0, -1.0, 0.5, 1.0);
    let br = Vec4::new(1.0, -1.0, 0.5, 1.0);
    [bl, br, tr, tr, tl, bl]
};

pub const UV_COORD_QUAD_CCW: [Vec2; 6] = {
    let tl = Vec2::new(0.0, 0.0);
    let tr = Vec2::new(1.0, 0.0);
    let bl = Vec2::new(0.0, 1.0);
    let br = Vec2::new(1.0, 1.0);
    [bl, br, tr, tr, tl, bl]
};

pub const POINTS_2D_TEX_QUAD: [Vec2; 6] = {
    let tl = Vec2::new(0.0, 0.0);
    let tr = Vec2::new(1.0, 0.0);
    let bl = Vec2::new(0.0, 1.0);
    let br = Vec2::new(1.0, 1.0);
    [tl, bl, tr, tr, bl, br]
};

/// Points around the unit cube.
///
///    y           1_____2     _____
///    |           /    /|    /|    |  (same box, left and front sides removed)
///    |___x     0/___3/ |   /7|____|6
///   /           |    | /   | /    /
/// z/            |____|/   4|/____/5
pub const UNIT_POINTS: [Vec3; 8] = {
    let p0 = Vec3::new(-0.5, 0.5, 0.5);
    let p1 = Vec3::new(-0.5, 0.5, -0.5);
    let p2 = Vec3::new(0.5, 0.5, -0.5);
    let p3 = Vec3::new(0.5, 0.5, 0.5);

    let p4 = Vec3::new(-0.5, -0.5, 0.5);
    let p7 = Vec3::new(-0.5, -0.5, -0.5);
    let p6 = Vec3::new(0.5, -0.5, -0.5);
    let p5 = Vec3::new(0.5, -0.5, 0.5);

    [p0, p1, p2, p3, p4, p5, p6, p7]
};

/// Triangle faces of the unit cube, winding CCW.
pub const UNIT_INDICES: [usize; 36] = [
    0, 2, 1, 0, 3, 2, // top
    0, 4, 3, 4, 5, 3, // front
    3, 6, 2, 3, 5, 6, // right
    1, 7, 0, 7, 4, 0, // left
    4, 6, 5, 4, 7, 6, // bottom
    2, 7, 1, 2, 6, 7, // back
];

#[cfg(not(target_arch = "spirv"))]
pub fn unit_cube() -> Vec<(Vec3, Vec3)> {
    UNIT_INDICES
        .chunks_exact(3)
        .flat_map(|chunk| match chunk {
            [a, b, c] => {
                let a = UNIT_POINTS[*a];
                let b = UNIT_POINTS[*b];
                let c = UNIT_POINTS[*c];
                let n = triangle_face_normal(a, b, c);
                [(a, n), (b, n), (c, n)]
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

/// Points on the unit cube that create a triangle-list mesh.
///
/// Use [`unit_cube`] for a mesh that includes normals.
///
/// `rust-gpu` doesn't like nested/double indexing so we do this here.
/// See [this comment on discord](https://discord.com/channels/750717012564770887/750717499737243679/1131395331368693770)
pub const CUBE: [Vec3; 36] = {
    let p0 = Vec3::new(-0.5, 0.5, 0.5);
    let p1 = Vec3::new(-0.5, 0.5, -0.5);
    let p2 = Vec3::new(0.5, 0.5, -0.5);
    let p3 = Vec3::new(0.5, 0.5, 0.5);
    let p4 = Vec3::new(-0.5, -0.5, 0.5);
    let p7 = Vec3::new(-0.5, -0.5, -0.5);
    let p6 = Vec3::new(0.5, -0.5, -0.5);
    let p5 = Vec3::new(0.5, -0.5, 0.5);
    convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7])
};

pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    let n = n.alt_norm_or_zero();
    i - 2.0 * n.dot(i) * n
}

pub struct Plane {
    pub point: Vec3,
    pub norm: Vec3,
}

pub const fn convex_mesh([p0, p1, p2, p3, p4, p5, p6, p7]: [Vec3; 8]) -> [Vec3; 36] {
    [
        p0, p2, p1, p0, p3, p2, // top
        p0, p4, p3, p4, p5, p3, // front
        p3, p6, p2, p3, p5, p6, // right
        p1, p7, p0, p7, p4, p0, // left
        p4, p6, p5, p4, p7, p6, // bottom
        p2, p7, p1, p2, p6, p7, // back
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn step_sanity() {
        assert_eq!(0.0, step_le(0.0, -0.33333));
        assert_eq!(1.0, step_le(0.0, 0.33333));
        assert_eq!(1.0, step_le(0.0, 0.0));
    }

    #[test]
    #[allow(clippy::bool_comparison)]
    fn nan_sanity() {
        let n = f32::NAN;
        assert!(n.is_nan());
        assert!((n <= 0.0) == false);
        assert!((n > 0.0) == false);
    }

    #[test]
    fn signum_sanity() {
        assert_eq!(1.0, signum_or_zero(0.33));
        assert_eq!(1.0, signum_or_zero(0.0));
        assert_eq!(1.0, signum_or_zero(-0.0));
        assert_eq!(-1.0, signum_or_zero(-0.33));

        let nan = f32::NAN;
        assert_eq!(0.0, signum_or_zero(nan));
    }
}
