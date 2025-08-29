//! 2d signed distance fields.
use glam::Vec2;

/// Returns the distance to the edge of a circle of radius `r` with center at `p`.
fn distance_circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub struct Circle {
    origin: Vec2,
    radius: f32,
}

impl Circle {
    pub fn distance(&self) -> f32 {
        distance_circle(self.origin, self.radius)
    }
}

// #[spirv_std::spirv(vertex)]
// pub fn vertex_circle(

// )
