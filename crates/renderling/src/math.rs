//! Mathematical helper types and functions.
use glam::Vec3;

pub fn triangle_face_normal(
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
) -> Vec3 {
    let a = p1 - p2;
    let b = p1 - p3;
    let n: Vec3 = a.cross(b).normalize();
    n
}

/// Points around the unit cube.
///
///    yb          1_____2     _____
///    |           /    /|    /|    |  (same box, left and front sides removed)
///    |___x     0/___3/ |   /7|____|6
///   /    g      |    | /   | /    /
/// z/r           |____|/   4|/____/5
pub fn unit_points() -> [Vec3; 8] {
    let p0 = Vec3::from([-0.5, 0.5, 0.5]);
    let p1 = Vec3::from([-0.5, 0.5, -0.5]);
    let p2 = Vec3::from([0.5, 0.5, -0.5]);
    let p3 = Vec3::from([0.5, 0.5, 0.5]);

    let p4 = Vec3::from([-0.5, -0.5, 0.5]);
    let p7 = Vec3::from([-0.5, -0.5, -0.5]);
    let p6 = Vec3::from([0.5, -0.5, -0.5]);
    let p5 = Vec3::from([0.5, -0.5, 0.5]);

    [p0, p1, p2, p3, p4, p5, p6, p7]
}

pub fn unit_cube() -> Vec<(Vec3, Vec3)> {
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
            let n = triangle_face_normal(c, b, a);
            vec![(a, n), (b, n), (c, n)]
        })
        .collect::<Vec<_>>()
}
