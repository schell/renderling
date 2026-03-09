//! Tests for the 2D/UI renderer.

#[cfg(test)]
mod tests {
    use glam::{Vec2, Vec4};

    use crate::{GradientDescriptor, UiRenderer};
    use renderling::context::Context;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    /// Save the rendered image for visual inspection and as a baseline
    /// reference. Uses `img_diff::assert_img_eq` which will create the
    /// expected image on first run.
    fn save_and_assert(name: &str, img: image::RgbaImage) {
        // Save a copy to test_output for inspection.
        img_diff::save(name, img.clone());
        // If the expected image doesn't exist yet, save it as the baseline.
        let test_img_path = renderling_build::test_img_dir().join(name);
        if !test_img_path.exists() {
            std::fs::create_dir_all(test_img_path.parent().unwrap()).unwrap();
            image::DynamicImage::from(img.clone())
                .save(&test_img_path)
                .unwrap();
            log::info!("saved baseline image: {}", test_img_path.display());
        }
        img_diff::assert_img_eq(name, img);
    }

    #[test]
    fn can_render_rect() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        let _rect = ui
            .add_rect()
            .with_position(Vec2::new(10.0, 10.0))
            .with_size(Vec2::new(80.0, 60.0))
            .with_fill_color(Vec4::new(0.2, 0.4, 0.8, 1.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/rect.png", img);
    }

    #[test]
    fn can_render_rounded_rect() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        let _rect = ui
            .add_rect()
            .with_position(Vec2::new(10.0, 10.0))
            .with_size(Vec2::new(120.0, 80.0))
            .with_corner_radii(Vec4::splat(16.0))
            .with_fill_color(Vec4::new(0.8, 0.2, 0.3, 1.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/rounded_rect.png", img);
    }

    #[test]
    fn can_render_circle() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        let _circle = ui
            .add_circle()
            .with_center(Vec2::new(100.0, 100.0))
            .with_radius(40.0)
            .with_fill_color(Vec4::new(0.1, 0.7, 0.3, 1.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/circle.png", img);
    }

    #[test]
    fn can_render_bordered_rect() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        let _rect = ui
            .add_rect()
            .with_position(Vec2::new(20.0, 20.0))
            .with_size(Vec2::new(100.0, 80.0))
            .with_corner_radii(Vec4::splat(12.0))
            .with_fill_color(Vec4::new(0.95, 0.95, 0.8, 1.0))
            .with_border(3.0, Vec4::new(0.2, 0.2, 0.2, 1.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/bordered_rect.png", img);
    }

    #[test]
    fn can_render_multiple_shapes() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(300, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        // Background rect
        let _rect = ui
            .add_rect()
            .with_position(Vec2::new(10.0, 10.0))
            .with_size(Vec2::new(120.0, 80.0))
            .with_corner_radii(Vec4::splat(8.0))
            .with_fill_color(Vec4::new(0.2, 0.4, 0.8, 1.0))
            .with_z(0.0);

        // Circle on top
        let _circle = ui
            .add_circle()
            .with_center(Vec2::new(200.0, 100.0))
            .with_radius(35.0)
            .with_fill_color(Vec4::new(0.9, 0.3, 0.1, 1.0))
            .with_border(2.0, Vec4::new(0.0, 0.0, 0.0, 1.0))
            .with_z(0.1);

        // Ellipse
        let _ellipse = ui
            .add_ellipse()
            .with_center(Vec2::new(150.0, 150.0))
            .with_radii(Vec2::new(60.0, 30.0))
            .with_fill_color(Vec4::new(0.1, 0.8, 0.4, 0.8))
            .with_z(0.2);

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/multiple_shapes.png", img);
    }

    #[test]
    fn can_render_image() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        // Create a programmatic 64x64 checkerboard image.
        let size = 64u32;
        let mut img = image::RgbaImage::new(size, size);
        for y in 0..size {
            for x in 0..size {
                let checker = ((x / 8) + (y / 8)) % 2 == 0;
                let c = if checker { 255 } else { 80 };
                img.put_pixel(x, y, image::Rgba([c, c, c, 255]));
            }
        }

        let atlas_img: renderling::atlas::AtlasImage = image::DynamicImage::ImageRgba8(img).into();
        let _image_el = ui.add_image(atlas_img).with_position(Vec2::new(20.0, 20.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let output = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/image.png", output);
    }

    #[test]
    fn can_render_image_with_tint() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        // Create a 64x64 solid white image.
        let size = 64u32;
        let img = image::RgbaImage::from_pixel(size, size, image::Rgba([255, 255, 255, 255]));
        let atlas_img: renderling::atlas::AtlasImage = image::DynamicImage::ImageRgba8(img).into();

        // Apply a red tint.
        let _image_el = ui
            .add_image(atlas_img)
            .with_position(Vec2::new(50.0, 50.0))
            .with_tint(Vec4::new(1.0, 0.0, 0.0, 1.0));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let output = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/image_tint.png", output);
    }

    #[test]
    fn can_render_gradient_rect() {
        init_logging();
        let ctx = futures_lite::future::block_on(Context::headless(200, 200));
        let mut ui = UiRenderer::new(&ctx).with_background_color(Vec4::ONE);

        let _rect = ui
            .add_rect()
            .with_position(Vec2::new(20.0, 20.0))
            .with_size(Vec2::new(160.0, 100.0))
            .with_corner_radii(Vec4::splat(12.0))
            .with_gradient(Some(GradientDescriptor {
                gradient_type: 1, // Linear
                start: Vec2::new(0.0, 0.0),
                end: Vec2::new(1.0, 0.0),
                radius: 0.0,
                color_start: Vec4::new(1.0, 0.0, 0.0, 1.0),
                color_end: Vec4::new(0.0, 0.0, 1.0, 1.0),
            }));

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = futures_lite::future::block_on(frame.read_image()).unwrap();
        save_and_assert("ui2d/gradient_rect.png", img);
    }
}
