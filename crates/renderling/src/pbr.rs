//! "Physically based" types and functions.
//!
//! ## References
//! * <https://learnopengl.com/PBR/Theory>
//! * <https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/pbr.frag>
//! * <https://github.khronos.org/glTF-Sample-Viewer-Release/>

pub mod brdf;
pub mod debug;
pub mod ibl;

pub mod shader;

#[cfg(test)]
mod test {
    use crate::{
        atlas::AtlasImage,
        geometry::Vertex,
        glam::{Vec3, Vec4},
        pbr::brdf::BrdfLut,
        test::BlockOnFuture,
    };

    #[test]
    // TODO: Move this over to a manual example
    // Tests the initial implementation of pbr metallic roughness on an array of
    // spheres with different metallic roughnesses lit by an environment map.
    //
    // see https://learnopengl.com/PBR/Lighting
    fn pbr_metallic_roughness_spheres() {
        let ss = 600;
        let ctx = crate::context::Context::headless(ss, ss).block();
        let stage = ctx.new_stage();

        let radius = 0.5;
        let ss = ss as f32;
        let projection = crate::camera::perspective(ss, ss);
        let k = 7;
        let diameter = 2.0 * radius;
        let spacing = radius * 0.25;
        let len = (k - 1) as f32 * (diameter + spacing);
        let half = len / 2.0;
        let view = crate::camera::look_at(
            Vec3::new(half, half, 1.6 * len),
            Vec3::new(half, half, 0.0),
            Vec3::Y,
        );
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);

        let geometry = stage.new_vertices({
            let mut icosphere = icosahedron::Polyhedron::new_isocahedron(radius, 5);
            icosphere.compute_triangle_normals();
            let icosahedron::Polyhedron {
                positions,
                normals,
                cells,
                ..
            } = icosphere;
            log::info!("icosphere created on CPU");

            let to_vertex = |ndx: &usize| -> Vertex {
                let p: [f32; 3] = positions[*ndx].0.into();
                let n: [f32; 3] = normals[*ndx].0.into();
                Vertex::default().with_position(p).with_normal(n)
            };
            cells
                .iter()
                .flat_map(|icosahedron::Triangle { a, b, c }| {
                    let p0 = to_vertex(a);
                    let p1 = to_vertex(b);
                    let p2 = to_vertex(c);
                    vec![p0, p1, p2]
                })
                .collect::<Vec<_>>()
        });
        let mut spheres = vec![];
        for i in 0..k {
            let roughness = i as f32 / (k - 1) as f32;
            let x = (diameter + spacing) * i as f32;
            for j in 0..k {
                let metallic = j as f32 / (k - 1) as f32;
                let y = (diameter + spacing) * j as f32;

                let rez = stage
                    .new_primitive()
                    .with_material(
                        stage
                            .new_material()
                            .with_albedo_factor(Vec4::new(1.0, 1.0, 1.0, 1.0))
                            .with_metallic_factor(metallic)
                            .with_roughness_factor(roughness),
                    )
                    .with_transform(stage.new_transform().with_translation(Vec3::new(x, y, 0.0)))
                    .with_vertices(&geometry);
                spheres.push(rez);
            }
        }

        let hdr_image = AtlasImage::from_hdr_path("../../img/hdr/resting_place.hdr").unwrap();
        let skybox = crate::skybox::Skybox::new(&ctx, hdr_image);
        stage.use_skybox(&skybox);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("pbr/metallic_roughness_spheres.png", img);
    }
}
