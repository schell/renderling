//! Camera projections.

use nalgebra::Matrix4;

#[derive(Clone, Debug)]
pub enum Projection {
    Perspective {
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
    Any(Matrix4<f32>),
}

/// Creates a right-handed perspective projection matrix with a depth range 0.0 to 1.0
pub fn perspective_rh(
    fov_y_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> Matrix4<f32> {
    assert!(z_near > 0.0 && z_far > 0.0);
    let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
    let h = cos_fov / sin_fov;
    let w = h / aspect_ratio;
    let r = z_far / (z_near - z_far);
    Matrix4::new(
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
}

impl Projection {
    pub fn to_homogeneous(&self) -> Matrix4<f32> {
        match self {
            Projection::Perspective {
                aspect,
                fovy,
                znear,
                zfar,
            } => perspective_rh(*fovy, *aspect, *znear, *zfar),
            Projection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => {
                let rcp_width = 1.0 / (right - left);
                let rcp_height = 1.0 / (top - bottom);
                let r = 1.0 / (far - near);
                Matrix4::new(
                    rcp_width + rcp_width,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    rcp_height + rcp_height,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    r,
                    0.0,
                    -(left + right) * rcp_width,
                    -(top + bottom) * rcp_height,
                    -r * near,
                    1.0,
                )
                .transpose()
            }
            Projection::Any(m) => m.clone(),
        }
    }
}

impl From<&Projection> for Matrix4<f32> {
    fn from(value: &Projection) -> Self {
        value.to_homogeneous()
    }
}
