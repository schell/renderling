//! Mathematical helper types and functions.
use nalgebra::{Point3, UnitVector3, Vector3};

/// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
pub fn perspective_rh(
    fov_y_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> [[f32; 4]; 4] {
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

pub fn triangle_face_normal(
    p1: &Point3<f32>,
    p2: &Point3<f32>,
    p3: &Point3<f32>,
) -> UnitVector3<f32> {
    let a = p1 - p2;
    let b = p1 - p3;
    let n: Vector3<f32> = a.cross(&b);
    UnitVector3::new_normalize(n)
}

/// Points around the unit cube.
///
///    yb          1_____2     _____
///    |           /    /|    /|    |  (same box, left and front sides removed)
///    |___x     0/___3/ |   /7|____|6
///   /    g      |    | /   | /    /
/// z/r           |____|/   4|/____/5
pub fn unit_points() -> [Point3<f32>; 8] {
    let p0 = Point3::from([-0.5, 0.5, 0.5]);
    let p1 = Point3::from([-0.5, 0.5, -0.5]);
    let p2 = Point3::from([0.5, 0.5, -0.5]);
    let p3 = Point3::from([0.5, 0.5, 0.5]);

    let p4 = Point3::from([-0.5, -0.5, 0.5]);
    let p7 = Point3::from([-0.5, -0.5, -0.5]);
    let p6 = Point3::from([0.5, -0.5, -0.5]);
    let p5 = Point3::from([0.5, -0.5, 0.5]);

    [p0, p1, p2, p3, p4, p5, p6, p7]
}

pub fn unit_cube() -> Vec<(Point3<f32>, UnitVector3<f32>)> {
    let points = unit_points();
    let triangles: [(usize, usize, usize); 12] = [
        (0, 1, 2),
        (0, 2, 3), // top
        (0, 3, 4),
        (4, 3, 5), // front
        (3, 2, 6),
        (3, 6, 5), // right
        (1, 0, 7),
        (7, 0, 4), // left
        (4, 5, 6),
        (4, 6, 7), // bottom
        (2, 1, 7),
        (2, 7, 6), // back
    ];
    triangles
        .iter()
        .flat_map(|(a, b, c)| {
            let a = points[*a];
            let b = points[*b];
            let c = points[*c];
            let n = triangle_face_normal(&c, &b, &a);
            vec![(a, n), (b, n), (c, n)]
        })
        .collect::<Vec<_>>()
}
