//! Mathematical helper types and functions.

/// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
pub fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
    assert!(z_near > 0.0 && z_far > 0.0);
    let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
    let h = cos_fov / sin_fov;
    let w = h / aspect_ratio;
    let r = z_far / (z_near - z_far);
    nalgebra::Matrix4::new(
        w,
        0.0,
        0.0,
        0.0,
        0.0,
        h,
        0.0,
        0.0,
        0.0,
        0.0,
        r,
        -1.0,
        0.0,
        0.0,
        r * z_near,
        0.0,
    )
        .transpose()
        .into()
}
