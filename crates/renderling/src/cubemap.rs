//! Render pipelines and layouts for creating cubemaps.

use glam::{Mat4, UVec2, Vec3};

use crate::{camera::Camera, stage::Stage, texture::Texture};

/// Represents one side of a cubemap.
///
/// Assumes the camera is at the origin.
pub struct CubemapFaceDirection {
    /// Where is the camera
    pub dir: Vec3,
    /// Which direction is up
    pub up: Vec3,
    /// Which direct is right
    pub right: Vec3,
}

impl CubemapFaceDirection {
    pub const X: Self = Self {
        dir: Vec3::X,
        up: Vec3::Y,
        right: Vec3::Z,
    };
    pub const NEG_X: Self = Self {
        dir: Vec3::NEG_X,
        up: Vec3::Y,
        right: Vec3::NEG_Z,
    };
    pub const Y: Self = Self {
        dir: Vec3::Y,
        up: Vec3::Z,
        right: Vec3::X,
    };
    pub const NEG_Y: Self = Self {
        dir: Vec3::NEG_Y,
        up: Vec3::NEG_Z,
        right: Vec3::X,
    };
    pub const Z: Self = Self {
        dir: Vec3::Z,
        up: Vec3::Y,
        right: Vec3::NEG_X,
    };
    pub const NEG_Z: Self = Self {
        dir: Vec3::NEG_Z,
        up: Vec3::Y,
        right: Vec3::X,
    };
    pub const FACES: [Self; 6] = [
        CubemapFaceDirection::X,
        CubemapFaceDirection::NEG_X,
        CubemapFaceDirection::Y,
        CubemapFaceDirection::NEG_Y,
        CubemapFaceDirection::Z,
        CubemapFaceDirection::NEG_Z,
    ];

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(Vec3::ZERO, self.dir, self.up)
    }

    pub fn to_corners_tr_tl_br_bl(self) -> [Vec3; 4] {
        let tr = self.dir + self.up + self.right;
        let tl = self.dir + self.up - self.right;
        let br = self.dir - self.up + self.right;
        let bl = self.dir - self.up - self.right;
        [tr, tl, br, bl]
    }

    pub fn to_tri_list(self) -> [Vec3; 6] {
        let [tr, tl, br, bl] = self.to_corners_tr_tl_br_bl();
        [tr, tl, bl, tr, bl, br]
    }
}

/// A cubemap that acts as a render target for an entire scene.
///
/// Use this to create and update a skybox with scene geometry.
pub struct SceneCubemap {
    cubemap_texture: wgpu::Texture,
}

impl SceneCubemap {
    pub fn new(device: &wgpu::Device, size: UVec2, format: wgpu::TextureFormat) -> Self {
        let cubemap_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene-to-cubemap"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        Self { cubemap_texture }
    }

    pub fn run(&self, stage: &Stage) {
        // create a camera for our cube
        let camera = stage.new_value(Camera::default());

        let mut prev_camera_ids = vec![];
        for rlet in stage.renderlets_iter() {
            if let Some(rlet) = rlet.upgrade() {
                let mut guard = rlet.lock();
                prev_camera_ids.push(guard.camera_id);
                // Overwrite the renderlet's camera
                guard.camera_id = camera.id();
            }
        }

        // By setting this to 90 degrees (PI/2 radians) we make sure the viewing field
        // is exactly large enough to fill a single face of the cubemap such that all
        // faces align correctly to each other at the edges.
        let fovy = std::f32::consts::FRAC_PI_2;
        let aspect = self.cubemap_texture.width() as f32 / self.cubemap_texture.height() as f32;
        let projection = Mat4::perspective_rh(fovy, aspect, 1.0, 25.0);
        // Render each face by rendering the scene from each camera angle into the cubemap
        for (i, face) in CubemapFaceDirection::FACES.iter().enumerate() {
            // Update the camera angle, no need to sync as calling `Stage::render` does this
            // implicitly
            camera.modify(|c| c.set_projection_and_view(projection, face.view()));
            let label_s = format!("scene-to-cubemap-{i}");
            let view = self
                .cubemap_texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&label_s),
                    base_array_layer: i as u32,
                    array_layer_count: Some(1),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    ..Default::default()
                });
            stage.render(&view);
        }
    }
}

/// A render pipeline for blitting an equirectangular image as a cubemap.
pub struct EquirectangularImageToCubemapBlitter(pub wgpu::RenderPipeline);

impl EquirectangularImageToCubemapBlitter {
    pub fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cubemap-making bindgroup"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }

    pub fn create_bindgroup(
        device: &wgpu::Device,
        label: Option<&str>,
        buffer: &wgpu::Buffer,
        // The texture to sample the environment from
        texture: &Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &Self::create_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        })
    }

    /// Create the rendering pipeline that creates cubemaps from equirectangular
    /// images.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        log::trace!("creating cubemap-making render pipeline with format '{format:?}'");
        let vertex_linkage = crate::linkage::skybox_cubemap_vertex::linkage(device);
        let fragment_linkage = crate::linkage::skybox_equirectangular_fragment::linkage(device);
        let bg_layout = Self::create_bindgroup_layout(device);
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cubemap-making pipeline layout"),
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        EquirectangularImageToCubemapBlitter(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("cubemap-making pipeline"),
                layout: Some(&pp_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_linkage.module,
                    entry_point: Some(vertex_linkage.entry_point),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_linkage.module,
                    entry_point: Some(fragment_linkage.entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                multiview: None,
                cache: None,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use glam::Vec4;

    use crate::{
        math::{UNIT_INDICES, UNIT_POINTS},
        stage::{Renderlet, Vertex},
    };

    use super::*;

    #[test]
    fn hand_rolled_cubemap_sampling() {
        let width = 256;
        let height = 256;
        let ctx = crate::Context::headless(width, height);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::ZERO)
            .with_lighting(false)
            .with_msaa_sample_count(4);
        let camera =
            stage.new_value(
                Camera::default_perspective(width as f32, height as f32)
                    .with_view(Mat4::look_at_rh(Vec3::splat(3.0), Vec3::ZERO, Vec3::Y)),
            );
        // geometry is the "clip cube" where colors are normalized 3d space coords
        let vertices = stage.new_array(UNIT_POINTS.map(|unit_cube_point| {
            Vertex::default()
                // multiply by 2.0 because the unit cube's AABB bounds are at 0.5, and we want 1.0
                .with_position(unit_cube_point * 2.0)
                // "normalize" (really "shift") the space coord from [-0.5, 0.5] to [0.0, 1.0]
                .with_color((unit_cube_point + 0.5).extend(1.0))
        }));
        let indices = stage.new_array(UNIT_INDICES.map(|u| u as u32));
        let renderlet = stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            camera_id: camera.id(),
            ..Default::default()
        });
        stage.add_renderlet(&renderlet);

        let scene_cubemap = SceneCubemap::new(
            ctx.get_device(),
            UVec2::new(width, height),
            wgpu::TextureFormat::Rgba8Unorm,
        );

        scene_cubemap.run(&stage);

        let frame = ctx.get_next_frame().unwrap();
        crate::test::capture_gpu_frame(
            &ctx,
            "cubemap/hand_rolled_cubemap_sampling/frame.gputrace",
            || stage.render(&frame.view()),
        );
        let img = frame.read_image().unwrap();
        img_diff::save("cubemap/hand_rolled_cubemap_sampling/cube.png", img);
        frame.present();
    }
}
