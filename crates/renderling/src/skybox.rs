//! An HDR skybox.

/// An HDR skybox.
pub struct Skybox {
    pub environment_texture: crate::Texture,
    pub irradiance_map: crate::Texture,
    pub prefiltered_environment_map: crate::Texture,
    pub brdf_lut: crate::Texture,
}

impl Skybox {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_data: &[u8],
    ) -> (Skybox, Renderable) {
        let hdr_texture = Skybox::create_hdr_texture(device, queue, hdr_data);

        // Create unit cube for projections.
        let cube_vertices: [[f32; 3]; 8] = [
            // front
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
            // back
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ];

        let cube_elements: [u32; 36] = [
            // front
            0, 1, 2, 2, 3, 0, // right
            1, 5, 6, 6, 2, 1, // back
            7, 6, 5, 5, 4, 7, // left
            4, 0, 3, 3, 7, 4, // bottom
            4, 5, 1, 1, 0, 4, // top
            3, 2, 6, 6, 7, 3,
        ];

        let cube_vertex_array = cube_vertices
            .iter()
            .map(|&x| Vertex {
                position: x,
                normal: [0.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 0.0, 0.0],
                tex_coord: [0.0, 0.0],
            })
            .collect::<Vec<Vertex>>();

        let unit_cube_mesh = Mesh::new(device, cube_vertex_array.as_slice(), Some(&cube_elements));

        let proj = Perspective3::new(1.0, std::f32::consts::PI / 180.0 * 90.0, 0.1, 10.0);

        let views = [
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(1.0, 0.0, 0.0),
                &Vector3::new(0.0, -1.0, 0.0),
                1.0,
            ),
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(-1.0, 0.0, 0.0),
                &Vector3::new(0.0, -1.0, 0.0),
                1.0,
            ),
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(0.0, -1.0, 0.0),
                &Vector3::new(0.0, 0.0, -1.0),
                1.0,
            ),
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(0.0, 1.0, 0.0),
                &Vector3::new(0.0, 0.0, 1.0),
                1.0,
            ),
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(0.0, 0.0, 1.0),
                &Vector3::new(0.0, -1.0, 0.0),
                1.0,
            ),
            nalgebra::Similarity3::look_at_rh(
                &Point3::new(0.0, 0.0, 0.0),
                &Point3::new(0.0, 0.0, -1.0),
                &Vector3::new(0.0, -1.0, 0.0),
                1.0,
            ),
        ];

        // Create environment map.
        let environment_texture = Skybox::create_environment_map_from_hdr(
            device,
            queue,
            &hdr_texture,
            &unit_cube_mesh,
            &proj,
            &views,
        );

        // Convolve the environment map.
        let irradiance_map = Skybox::create_irradiance_map(
            device,
            queue,
            &environment_texture,
            &unit_cube_mesh,
            &proj,
            &views,
        );

        // Generate specular IBL pre-filtered environment map.
        let prefiltered_environment_map = Skybox::create_prefiltered_environment_map(
            device,
            queue,
            &environment_texture,
            &unit_cube_mesh,
            &proj,
            &views,
        );

        let precomputed_brdf = Skybox::create_precomputed_brdf_texture(device, queue);

        let skybox_params = environment_texture;

        let material = Box::new(SkyboxMaterial::new(device, sc_desc, &skybox_params));
        let skybox = Skybox {
            environment_texture: skybox_params.environment_texture,
            irradiance_map,
            prefiltered_environment_map,
            brdf_lut: precomputed_brdf,
        };

        (
            skybox,
            Renderable::new_from_single_mesh(unit_cube_mesh, material),
        )
    }

    fn create_hdr_texture(device: &wgpu::Device, queue: &wgpu::Queue, hdr_data: &[u8]) -> Texture {
        // Decode HDR data.
        let reader = image::hdr::HdrDecoder::new(hdr_data);
        let decoder = reader.unwrap();

        let width = decoder.metadata().width;
        let height = decoder.metadata().height;
        let pixels = decoder.read_image_hdr().unwrap();

        // Add alpha data.
        let mut pixel_data = Vec::new();

        for pixel in pixels {
            pixel_data.push(pixel[0]);
            pixel_data.push(pixel[1]);
            pixel_data.push(pixel[2]);
            pixel_data.push(1.0);
        }

        // Create HDR equirectangular texture.
        let pixel_data_bytes = unsafe {
            let len = pixel_data.len() * std::mem::size_of::<f32>();
            let ptr = pixel_data.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, len)
        };

        Texture::new_texture_from_data(
            device,
            queue,
            width,
            height,
            pixel_data_bytes,
            wgpu::TextureFormat::Rgba32Float,
            wgpu::AddressMode::ClampToEdge,
        )
    }

    fn create_environment_map_from_hdr(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_texture: &Texture,
        unit_cube_mesh: &Mesh,
        proj: &nalgebra::Perspective3<f32>,
        views: &[nalgebra::Similarity3<f32>; 6],
    ) -> Texture {
        // Create HDR material.
        let hdr_material_params = HdrCvtBindGroup {
            equirectangular_texture: hdr_texture,
        };

        let hdr_material = Box::new(HdrCvtMaterial::new(device, &hdr_material_params));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("HDR convert"),
        });

        let mut cubemap_faces = Vec::new();

        // Render every cube face.
        for i in 0..6 {
            let cubemap_face = Texture::new_framebuffer_texture(
                device,
                512,
                512,
                wgpu::TextureFormat::Rgba16Float,
            );

            let transforms = HdrTransformBindGroup {
                proj_matrix: proj.to_homogeneous(),
                view_matrix: views[i].to_homogeneous(),
            };

            material_base::update_uniform_buffer(
                device,
                &hdr_material.transform_bind_group_buffer,
                &mut encoder,
                &transforms,
            );

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &cubemap_face.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLACK,
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&hdr_material.render_pipeline);
                render_pass.set_bind_group(0, &hdr_material.transform_bind_group, &[]);
                render_pass.set_bind_group(1, &hdr_material.cvt_bind_group, &[]);

                unit_cube_mesh.draw(&mut render_pass);
            }

            cubemap_faces.push(cubemap_face);
        }

        let cmd_buffer = encoder.finish();

        queue.submit(&[cmd_buffer]);

        // Create environment cubemap.
        Texture::new_cubemap_texture(
            device,
            queue,
            512,
            512,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            1,
        )
    }

    fn create_irradiance_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        environment_texture: &Texture,
        unit_cube_mesh: &Mesh,
        proj: &nalgebra::Perspective3<f32>,
        views: &[nalgebra::Similarity3<f32>; 6],
    ) -> Texture {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("HDR convolve"),
        });

        let convolve_params = HdrConvolveDiffuseBindGroup {
            environment_texture: environment_texture,
        };

        let convolve_material = HdrConvolveDiffuseMaterial::new(device, &convolve_params);

        let mut cubemap_faces = Vec::new();

        for i in 0..6 {
            let cubemap_face =
                Texture::new_framebuffer_texture(device, 32, 32, wgpu::TextureFormat::Rgba16Float);

            let transforms = HdrTransformBindGroup {
                proj_matrix: proj.to_homogeneous(),
                view_matrix: views[i].to_homogeneous(),
            };

            material_base::update_uniform_buffer(
                device,
                &convolve_material.transform_bind_group_buffer,
                &mut encoder,
                &transforms,
            );

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &cubemap_face.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLACK,
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&convolve_material.render_pipeline);
                render_pass.set_bind_group(0, &convolve_material.transform_bind_group, &[]);
                render_pass.set_bind_group(1, &convolve_material.convolve_bind_group, &[]);

                unit_cube_mesh.draw(&mut render_pass);
            }

            cubemap_faces.push(cubemap_face);
        }

        let cmd_buffer = encoder.finish();

        queue.submit(&[cmd_buffer]);

        Texture::new_cubemap_texture(
            device,
            queue,
            32,
            32,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            1,
        )
    }

    fn create_prefiltered_environment_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        environment_texture: &Texture,
        unit_cube_mesh: &Mesh,
        proj: &nalgebra::Perspective3<f32>,
        views: &[nalgebra::Similarity3<f32>; 6],
    ) -> Texture {
        let hdr_specular_convolution_material = HdrConvolveSpecularMaterial::new(
            device,
            &HdrConvolveSpecularBindGroup {
                environment_texture: environment_texture,
                roughness: 0.0,
            },
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("HDR convolve specular"),
        });

        let mut cubemap_faces = Vec::new();

        for mip_level in 0..5 {
            let mip_width: u32 = 128 >> mip_level;
            let mip_height: u32 = 128 >> mip_level;

            let roughness = mip_level as f32 / 4.0;

            material_base::update_uniform_buffer(
                device,
                &hdr_specular_convolution_material.roughness_bind_group_buffer,
                &mut encoder,
                &[roughness],
            );

            for i in 0..6 {
                let cubemap_face = Texture::new_framebuffer_texture(
                    device,
                    mip_width,
                    mip_height,
                    wgpu::TextureFormat::Rgba16Float,
                );

                let transforms = HdrTransformBindGroup {
                    proj_matrix: proj.to_homogeneous(),
                    view_matrix: views[i].to_homogeneous(),
                };

                material_base::update_uniform_buffer(
                    device,
                    &hdr_specular_convolution_material.transform_bind_group_buffer,
                    &mut encoder,
                    &transforms,
                );

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &cubemap_face.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::BLACK,
                        }],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_pipeline(&hdr_specular_convolution_material.render_pipeline);
                    render_pass.set_bind_group(
                        0,
                        &hdr_specular_convolution_material.transform_bind_group,
                        &[],
                    );
                    render_pass.set_bind_group(
                        1,
                        &hdr_specular_convolution_material.convolve_bind_group,
                        &[],
                    );
                    render_pass.set_bind_group(
                        2,
                        &hdr_specular_convolution_material.roughness_bind_group,
                        &[],
                    );

                    unit_cube_mesh.draw(&mut render_pass);
                }

                cubemap_faces.push(cubemap_face);
            }
        }

        let cmd_buffer = encoder.finish();

        queue.submit(&[cmd_buffer]);

        Texture::new_cubemap_texture(
            device,
            queue,
            128,
            128,
            cubemap_faces.as_slice(),
            wgpu::TextureFormat::Rgba16Float,
            5,
        )
    }

    fn create_precomputed_brdf_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let screen_space_quad_vertices: [[f32; 3]; 4] = [
            [-1.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, -1.0, 0.0],
        ];

        let screen_space_quad_tex_coords: [[f32; 2]; 4] =
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let screen_space_quad_elements: [u32; 6] = [0, 1, 2, 2, 3, 0];

        let screen_space_quad_vertex_array = screen_space_quad_vertices
            .iter()
            .zip(screen_space_quad_tex_coords.iter())
            .map(|(&pos, &tex_coord)| Vertex {
                position: pos,
                normal: [0.0, 0.0, 0.0],
                tangent: [0.0, 0.0, 0.0, 0.0],
                tex_coord: tex_coord,
            })
            .collect::<Vec<Vertex>>();

        let screen_space_quad_mesh = Mesh::new(
            device,
            &screen_space_quad_vertex_array,
            Some(&screen_space_quad_elements),
        );

        let brdf_mat = HdrConvolveBrdfMaterial::new(device);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("brdf_precompute"),
        });

        let transforms = HdrTransformBindGroup {
            proj_matrix: Matrix3::identity().to_homogeneous(),
            view_matrix: Similarity3::identity().to_homogeneous(),
        };

        material_base::update_uniform_buffer(
            device,
            &brdf_mat.transform_bind_group_buffer,
            &mut encoder,
            &transforms,
        );

        let framebuffer =
            Texture::new_framebuffer_texture(device, 512, 512, wgpu::TextureFormat::Rg16Float);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &framebuffer.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&brdf_mat.render_pipeline);
            render_pass.set_bind_group(0, &brdf_mat.transform_bind_group, &[]);

            screen_space_quad_mesh.draw(&mut render_pass);
        }

        let cmd_buffer = encoder.finish();

        queue.submit(&[cmd_buffer]);

        Texture::new_texture_from_framebuffer(
            device,
            queue,
            512,
            512,
            &framebuffer,
            wgpu::TextureFormat::Rg16Float,
            wgpu::AddressMode::ClampToEdge,
        )
    }
}
