//! Shader code for `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

use core::ops::Mul;

use glam::{Quat, Vec3, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::num_traits::Zero;

pub mod array;
pub mod bits;
pub mod convolution;
pub mod debug;
pub mod gltf;
pub mod id;
pub mod math;
pub mod pbr;
pub mod skybox;
pub mod slab;
pub mod stage;
pub mod texture;
pub mod tonemapping;
pub mod tutorial;
pub mod ui;

/// Additional methods for vector types.
pub trait IsVector {
    /// Normalize or return zero.
    ///
    /// This is required instead of using [`glam::Vec3::normalize_or_zero`],
    /// because that compares against `f32::INFINITY`, which causes an error
    /// in naga's WGSL output.
    ///
    /// See [this issue](https://github.com/gfx-rs/naga/issues/2461) and `crate::linkage::test`
    /// for more info.
    fn alt_norm_or_zero(&self) -> Self;
}

impl IsVector for glam::Vec3 {
    fn alt_norm_or_zero(&self) -> Self {
        if self.length().is_zero() {
            glam::Vec3::ZERO
        } else {
            self.normalize()
        }
    }
}

pub trait IsMatrix {
    /// Extracts `scale`, `rotation` and `translation` from `self`. The input
    /// matrix is expected to be a 3D affine transformation matrix otherwise
    /// the output will be invalid.
    ///
    /// Will return `(Vec3::ONE, Quat::IDENTITY, Vec3::ZERO)` if the determinant of
    /// `self` is zero or if the resulting scale vector contains any zero elements
    /// when `glam_assert` is enabled.
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
