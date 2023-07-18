//! An HDR skybox.

use glam::{Mat4, Vec3};

use crate::SceneImage;

/// An HDR skybox.
pub struct Skybox {
    pub environment_texture: crate::Texture,
    pub irradiance_map: crate::Texture,
    pub prefiltered_environment_map: crate::Texture,
    pub brdf_lut: crate::Texture,
}

impl Skybox {
    ///// Create a new `Skybox`.
    // pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, hdr_data: &[u8]) ->
    // Self {    let hdr_texture = Skybox::create_hdr_texture(device, queue,
    // hdr_data);    let proj = Mat4::perspective_rh(std::f32::consts::PI /
    // 180.0 * 90.0, 1.0, 0.1, 10.0);    let views = [
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(1.0, 0.0, 0.0),
    //            Vec3::new(0.0, -1.0, 0.0),
    //        ),
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(-1.0, 0.0, 0.0),
    //            Vec3::new(0.0, -1.0, 0.0),
    //        ),
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(0.0, -1.0, 0.0),
    //            Vec3::new(0.0, 0.0, -1.0),
    //        ),
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(0.0, 1.0, 0.0),
    //            Vec3::new(0.0, 0.0, 1.0),
    //        ),
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(0.0, 0.0, 1.0),
    //            Vec3::new(0.0, -1.0, 0.0),
    //        ),
    //        Mat4::look_at_rh(
    //            Vec3::new(0.0, 0.0, 0.0),
    //            Vec3::new(0.0, 0.0, -1.0),
    //            Vec3::new(0.0, -1.0, 0.0),
    //        ),
    //    ];

    //    // Create environment map.
    //    let environment_texture =
    //        Skybox::create_environment_map_from_hdr(device, queue, &hdr_texture,
    // &proj, &views);

    //    // Convolve the environment map.
    //    let irradiance_map =
    //        Skybox::create_irradiance_map(device, queue, &environment_texture,
    // &proj, &views);

    //    // Generate specular IBL pre-filtered environment map.
    //    let prefiltered_environment_map =
    // Skybox::create_prefiltered_environment_map(        device,
    //        queue,
    //        &environment_texture,
    //        &proj,
    //        &views,
    //    );

    //    Skybox {
    //        environment_texture,
    //        irradiance_map,
    //        prefiltered_environment_map,
    //        brdf_lut: Skybox::create_precomputed_brdf_texture(device, queue),
    //    }
    //}

    /// Create an HDR equirectangular texture.
    pub fn create_hdr_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_data: &[u8],
    ) -> crate::Texture {
        let img = SceneImage::from_hdr_bytes(hdr_data).unwrap();
        crate::Texture::new_with(
            device,
            queue,
            Some("create hdr texture"),
            None,
            None,
            wgpu::TextureFormat::Rgba32Float,
            4,
            4,
            img.width,
            img.height,
            &img.pixels,
        )
    }

    // fn create_environment_map_from_hdr(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //    hdr_texture: &crate::Texture,
    //    proj: &Mat4,
    //    views: &[Mat4; 6],
    //) -> crate::Texture {
    //    // Create HDR material.
    //    let hdr_material_params = hdr_texture;

    //    let hdr_material = Box::new(HdrCvtMaterial::new(device,
    // &hdr_material_params));

    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("HDR convert"),
    //    });

    //    let mut cubemap_faces = Vec::new();

    //    // Render every cube face.
    //    for i in 0..6 {
    //        let cubemap_face = Texture::new_framebuffer_texture(
    //            device,
    //            512,
    //            512,
    //            wgpu::TextureFormat::Rgba16Float,
    //        );

    //        let transforms = HdrTransformBindGroup {
    //            proj_matrix: proj.to_homogeneous(),
    //            view_matrix: views[i].to_homogeneous(),
    //        };

    //        material_base::update_uniform_buffer(
    //            device,
    //            &hdr_material.transform_bind_group_buffer,
    //            &mut encoder,
    //            &transforms,
    //        );

    //        {
    //            let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                    attachment: &cubemap_face.view,
    //                    resolve_target: None,
    //                    load_op: wgpu::LoadOp::Clear,
    //                    store_op: wgpu::StoreOp::Store,
    //                    clear_color: wgpu::Color::BLACK,
    //                }],
    //                depth_stencil_attachment: None,
    //            });

    //            render_pass.set_pipeline(&hdr_material.render_pipeline);
    //            render_pass.set_bind_group(0, &hdr_material.transform_bind_group,
    // &[]);            render_pass.set_bind_group(1,
    // &hdr_material.cvt_bind_group, &[]);

    //            unit_cube_mesh.draw(&mut render_pass);
    //        }

    //        cubemap_faces.push(cubemap_face);
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    // Create environment cubemap.
    //    Texture::new_cubemap_texture(
    //        device,
    //        queue,
    //        512,
    //        512,
    //        cubemap_faces.as_slice(),
    //        wgpu::TextureFormat::Rgba16Float,
    //        1,
    //    )
    //}

    // fn create_irradiance_map(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //    environment_texture: &crate::Texture,
    //    proj: &Mat4,
    //    views: &[Mat4; 6],
    //) -> crate::Texture {
    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("HDR convolve"),
    //    });

    //    let convolve_params = HdrConvolveDiffuseBindGroup {
    //        environment_texture: environment_texture,
    //    };

    //    let convolve_material = HdrConvolveDiffuseMaterial::new(device,
    // &convolve_params);

    //    let mut cubemap_faces = Vec::new();

    //    for i in 0..6 {
    //        let cubemap_face =
    //            Texture::new_framebuffer_texture(device, 32, 32,
    // wgpu::TextureFormat::Rgba16Float);

    //        let transforms = HdrTransformBindGroup {
    //            proj_matrix: proj.to_homogeneous(),
    //            view_matrix: views[i].to_homogeneous(),
    //        };

    //        material_base::update_uniform_buffer(
    //            device,
    //            &convolve_material.transform_bind_group_buffer,
    //            &mut encoder,
    //            &transforms,
    //        );

    //        {
    //            let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                    attachment: &cubemap_face.view,
    //                    resolve_target: None,
    //                    load_op: wgpu::LoadOp::Clear,
    //                    store_op: wgpu::StoreOp::Store,
    //                    clear_color: wgpu::Color::BLACK,
    //                }],
    //                depth_stencil_attachment: None,
    //            });

    //            render_pass.set_pipeline(&convolve_material.render_pipeline);
    //            render_pass.set_bind_group(0,
    // &convolve_material.transform_bind_group, &[]);
    // render_pass.set_bind_group(1, &convolve_material.convolve_bind_group, &[]);

    //            unit_cube_mesh.draw(&mut render_pass);
    //        }

    //        cubemap_faces.push(cubemap_face);
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    Texture::new_cubemap_texture(
    //        device,
    //        queue,
    //        32,
    //        32,
    //        cubemap_faces.as_slice(),
    //        wgpu::TextureFormat::Rgba16Float,
    //        1,
    //    )
    //}

    // fn create_prefiltered_environment_map(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //    environment_texture: &crate::Texture,
    //    proj: &Mat4,
    //    views: &[Mat4; 6],
    //) -> crate::Texture {
    //    let hdr_specular_convolution_material = HdrConvolveSpecularMaterial::new(
    //        device,
    //        &HdrConvolveSpecularBindGroup {
    //            environment_texture: environment_texture,
    //            roughness: 0.0,
    //        },
    //    );

    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("HDR convolve specular"),
    //    });

    //    let mut cubemap_faces = Vec::new();

    //    for mip_level in 0..5 {
    //        let mip_width: u32 = 128 >> mip_level;
    //        let mip_height: u32 = 128 >> mip_level;

    //        let roughness = mip_level as f32 / 4.0;

    //        material_base::update_uniform_buffer(
    //            device,
    //            &hdr_specular_convolution_material.roughness_bind_group_buffer,
    //            &mut encoder,
    //            &[roughness],
    //        );

    //        for i in 0..6 {
    //            let cubemap_face = Texture::new_framebuffer_texture(
    //                device,
    //                mip_width,
    //                mip_height,
    //                wgpu::TextureFormat::Rgba16Float,
    //            );

    //            let transforms = HdrTransformBindGroup {
    //                proj_matrix: proj.to_homogeneous(),
    //                view_matrix: views[i].to_homogeneous(),
    //            };

    //            material_base::update_uniform_buffer(
    //                device,
    //
    // &hdr_specular_convolution_material.transform_bind_group_buffer,
    //                &mut encoder,
    //                &transforms,
    //            );

    //            {
    //                let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                        attachment: &cubemap_face.view,
    //                        resolve_target: None,
    //                        load_op: wgpu::LoadOp::Clear,
    //                        store_op: wgpu::StoreOp::Store,
    //                        clear_color: wgpu::Color::BLACK,
    //                    }],
    //                    depth_stencil_attachment: None,
    //                });

    // render_pass.set_pipeline(&hdr_specular_convolution_material.render_pipeline);
    //                render_pass.set_bind_group(
    //                    0,
    //                    &hdr_specular_convolution_material.transform_bind_group,
    //                    &[],
    //                );
    //                render_pass.set_bind_group(
    //                    1,
    //                    &hdr_specular_convolution_material.convolve_bind_group,
    //                    &[],
    //                );
    //                render_pass.set_bind_group(
    //                    2,
    //                    &hdr_specular_convolution_material.roughness_bind_group,
    //                    &[],
    //                );

    //                unit_cube_mesh.draw(&mut render_pass);
    //            }

    //            cubemap_faces.push(cubemap_face);
    //        }
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    Texture::new_cubemap_texture(
    //        device,
    //        queue,
    //        128,
    //        128,
    //        cubemap_faces.as_slice(),
    //        wgpu::TextureFormat::Rgba16Float,
    //        5,
    //    )
    //}

    // fn create_precomputed_brdf_texture(
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //) -> crate::Texture {
    //    let screen_space_quad_vertices: [[f32; 3]; 4] = [
    //        [-1.0, -1.0, 0.0],
    //        [-1.0, 1.0, 0.0],
    //        [1.0, 1.0, 0.0],
    //        [1.0, -1.0, 0.0],
    //    ];

    //    let screen_space_quad_tex_coords: [[f32; 2]; 4] =
    //        [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

    //    let screen_space_quad_elements: [u32; 6] = [0, 1, 2, 2, 3, 0];

    //    let screen_space_quad_vertex_array = screen_space_quad_vertices
    //        .iter()
    //        .zip(screen_space_quad_tex_coords.iter())
    //        .map(|(&pos, &tex_coord)| Vertex {
    //            position: pos,
    //            normal: [0.0, 0.0, 0.0],
    //            tangent: [0.0, 0.0, 0.0, 0.0],
    //            tex_coord: tex_coord,
    //        })
    //        .collect::<Vec<Vertex>>();

    //    let screen_space_quad_mesh = Mesh::new(
    //        device,
    //        &screen_space_quad_vertex_array,
    //        Some(&screen_space_quad_elements),
    //    );

    //    let brdf_mat = HdrConvolveBrdfMaterial::new(device);

    //    let mut encoder =
    // device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //        label: Some("brdf_precompute"),
    //    });

    //    let transforms = HdrTransformBindGroup {
    //        proj_matrix: Matrix3::identity().to_homogeneous(),
    //        view_matrix: Similarity3::identity().to_homogeneous(),
    //    };

    //    material_base::update_uniform_buffer(
    //        device,
    //        &brdf_mat.transform_bind_group_buffer,
    //        &mut encoder,
    //        &transforms,
    //    );

    //    let framebuffer =
    //        Texture::new_framebuffer_texture(device, 512, 512,
    // wgpu::TextureFormat::Rg16Float);

    //    {
    //        let mut render_pass =
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //                attachment: &framebuffer.view,
    //                resolve_target: None,
    //                load_op: wgpu::LoadOp::Clear,
    //                store_op: wgpu::StoreOp::Store,
    //                clear_color: wgpu::Color::BLACK,
    //            }],
    //            depth_stencil_attachment: None,
    //        });

    //        render_pass.set_pipeline(&brdf_mat.render_pipeline);
    //        render_pass.set_bind_group(0, &brdf_mat.transform_bind_group, &[]);

    //        screen_space_quad_mesh.draw(&mut render_pass);
    //    }

    //    let cmd_buffer = encoder.finish();

    //    queue.submit(&[cmd_buffer]);

    //    Texture::new_texture_from_framebuffer(
    //        device,
    //        queue,
    //        512,
    //        512,
    //        &framebuffer,
    //        wgpu::TextureFormat::Rg16Float,
    //        wgpu::AddressMode::ClampToEdge,
    //    )
    //}
}

#[cfg(test)]
mod test {
    use glam::{Vec2, Vec4};
    use renderling_shader::{
        scene::LightingModel,
        ui::{UiMode, UiVertex},
    };

    use super::*;
    use crate::{setup_scene_render_graph, setup_ui_render_graph, Renderling};

    #[test]
    fn hdr_texture() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let hdr_data = std::fs::read("../../img/hdr/helipad.hdr").unwrap();
        let tex = Skybox::create_hdr_texture(r.get_device(), r.get_queue(), &hdr_data);
        let builder = r.new_ui_scene().with_canvas_size(600, 400);
        let obj = builder
            .new_object()
            .with_draw_mode(UiMode::DEFAULT)
            .with_texture(&tex)
            .with_vertices({
                let tl = UiVertex {
                    position: Vec2::new(0.0, 0.0),
                    uv: Vec2::new(0.0, 0.0),
                    color: Vec4::ONE,
                };
                let tr = UiVertex {
                    position: Vec2::new(1.0, 0.0),
                    uv: Vec2::new(1.0, 0.0),
                    color: Vec4::ONE,
                };
                let bl = UiVertex {
                    position: Vec2::new(0.0, 1.0),
                    uv: Vec2::new(0.0, 1.0),
                    color: Vec4::ONE,
                };
                let br = UiVertex {
                    position: Vec2::new(1.0, 1.0),
                    uv: Vec2::new(1.0, 1.0),
                    color: Vec4::ONE,
                };
                [tl, bl, tr, tr, bl, br]
            })
            .with_scale([400.0, 400.0])
            .build();
        let scene = builder.build();
        setup_ui_render_graph(scene, [obj], &mut r, true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("hdr_texture.png", img);
    }

    #[test]
    fn hdr_skybox_scene() {
        let mut r = Renderling::headless(600, 400).unwrap();
        let hdr_data = std::fs::read("../../img/hdr/helipad.hdr").unwrap();
        let tex = Skybox::create_hdr_texture(r.get_device(), r.get_queue(), &hdr_data);

        let (proj, view) = crate::camera::default_perspective(600.0, 400.0);
        let mut builder = r.new_scene().with_camera(proj, view);
        for m in builder.materials.iter_mut() {
            m.lighting_model = LightingModel::NO_LIGHTING;
        }
        for v in builder.vertices.iter_mut() {
            v.color = Vec4::ONE;
        }
        let _key_light = builder
            .new_directional_light()
            .with_color(Vec4::ONE)
            .with_direction(Vec3::new(0.5, -0.7071, -0.5))
            .with_intensity(1.0)
            .build();
        let _fill_light = builder
            .new_directional_light()
            .with_color(Vec4::ONE)
            .with_direction(Vec3::new(-0.5, 0.7071, 0.5))
            .with_intensity(0.5)
            .build();
        let _loader = builder.gltf_load("../../gltf/cube.glb").unwrap();
        let scene = builder.build().unwrap();
        setup_scene_render_graph(scene, &mut r, true);
        let img = r.render_image().unwrap();
        img_diff::save("hdr_skybox_scene.png", img);
    }
}
