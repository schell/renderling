//! SDF functions for use in shaders.
//!
//! For more info, see these great articles:
//! - <https://iquilezles.org/articles/distfunctions2d/>

use crabslab::SlabItem;
use glam::Vec2;

#[derive(Clone, Copy, SlabItem)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn distance(&self, point: Vec2) -> f32 {
        let p = point - self.center;
        p.length() - self.radius
    }
}

#[derive(Clone, Copy, SlabItem)]
pub struct Box {
    pub center: Vec2,
    pub half_extent: Vec2,
}

impl Box {
    pub fn distance(&self, point: Vec2) -> f32 {
        let p = point - self.center;
        let component_edge_distance = p.abs() - self.half_extent;
        let outside = component_edge_distance.max(Vec2::ZERO).length();
        let inside = component_edge_distance
            .x
            .max(component_edge_distance.y)
            .min(0.0);
        inside + outside
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sdf_circle_sanity() {
        let mut img = image::ImageBuffer::<image::Luma<f32>, Vec<f32>>::new(32, 32);

        let circle = Circle {
            center: Vec2::new(12.0, 12.0),
            radius: 4.0,
        };

        img.enumerate_pixels_mut().for_each(|(x, y, p)| {
            let distance = circle.distance(Vec2::new(x as f32 + 0.5, y as f32 + 0.5));
            p.0[0] = distance / circle.radius;
        });

        img_diff::assert_img_eq(
            "sdf/circle_sanity.png",
            image::DynamicImage::from(img).into_rgb8(),
        );
    }

    #[test]
    fn sdf_box_sanity() {
        let mut img = image::ImageBuffer::<image::Luma<f32>, Vec<f32>>::new(32, 32);

        let bx = Box {
            center: Vec2::new(12.0, 12.0),
            half_extent: Vec2::new(4.0, 6.0),
        };

        img.enumerate_pixels_mut().for_each(|(x, y, p)| {
            let distance = bx.distance(Vec2::new(x as f32 + 0.5, y as f32 + 0.5));
            p.0[0] = distance / bx.half_extent.max_element();
        });

        img_diff::assert_img_eq(
            "sdf/box_sanity.png",
            image::DynamicImage::from(img).into_rgb8(),
        );
    }
}
