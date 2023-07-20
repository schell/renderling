//! Mathematical helper types and functions.
pub use glam::*;

pub fn triangle_face_normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
    let a = p1 - p2;
    let b = p1 - p3;
    let n: Vec3 = a.cross(b).normalize();
    n
}

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
            _ => unreachable!()
        })
        .collect::<Vec<_>>()
}
