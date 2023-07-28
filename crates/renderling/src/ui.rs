//! User interface rendering.
//!
//! This is traditional 2d rendering.

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use glam::{UVec2, Vec2, Vec4};
use snafu::prelude::*;

use crate::{
    mesh::Mesh, node::FrameTextureView, Device, Queue, RenderTarget, Renderling, Texture, Uniform,
    View, ViewMut,
};

pub use renderling_shader::ui::{UiConstants, UiDrawParams, UiMode, UiVertex};

#[derive(Debug, Snafu)]
pub enum UiSceneError {
    #[snafu(display("UiDrawObject update containts indices but we have no index buffer"))]
    NoIndexBuffer,
}

pub fn constants_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("ui constants"),
    })
}

pub fn draw_params_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("ui draw params"),
    })
}

pub fn ui_texture_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("ui shader texture"),
    })
}

pub fn ui_texture_bindgroup(device: &wgpu::Device, texture: &Texture) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("UiDrawObject texture_bindgroup"),
        layout: &ui_texture_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

pub fn create_ui_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let label = Some("ui render pipeline");
    let vertex_shader = device.create_shader_module(wgpu::include_spirv!("linkage/ui_vertex.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/ui_fragment.spv"));
    let constants_bindgroup_layout = constants_bindgroup_layout(device);
    let draw_params_bindgroup_layout = draw_params_bindgroup_layout(device);
    let texture_bindgroup_layout = ui_texture_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[
            &constants_bindgroup_layout,
            &texture_bindgroup_layout,
            &draw_params_bindgroup_layout,
        ],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "ui_vertex",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: {
                    let position_size = std::mem::size_of::<Vec2>();
                    let uv_size = std::mem::size_of::<Vec2>();
                    let color_size = std::mem::size_of::<Vec4>();
                    (position_size + color_size + uv_size) as wgpu::BufferAddress
                },
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x2,
                    1 => Float32x2,
                    2 => Float32x4
                ],
            }],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
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
            module: &fragment_shader,
            entry_point: "ui_fragment",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    pipeline
}

pub struct UiRenderPipeline(pub wgpu::RenderPipeline);

pub struct UiDrawObject {
    draw_params: Uniform<UiDrawParams>,
    texture_bindgroup: Option<wgpu::BindGroup>,
    updated_texture: Option<Texture>,
    updated_vertices: Option<(Vec<UiVertex>, Option<Vec<u16>>)>,
    mesh: Mesh,
}

impl UiDrawObject {
    pub fn new(
        device: &wgpu::Device,
        draw_params: UiDrawParams,
        vertices: impl IntoIterator<Item = UiVertex>,
        may_indices: Option<impl IntoIterator<Item = u16>>,
        texture_bindgroup: Option<wgpu::BindGroup>,
    ) -> Self {
        Self {
            draw_params: Uniform::new(
                device,
                draw_params,
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                wgpu::ShaderStages::VERTEX,
            ),
            updated_texture: None,
            updated_vertices: None,
            texture_bindgroup,
            mesh: Mesh::new(device, Some("UiDrawObject index"), vertices, may_indices),
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.draw_params.translation
    }

    pub fn get_scale(&self) -> Vec2 {
        self.draw_params.scale
    }

    pub fn get_rotation(&self) -> f32 {
        self.draw_params.rotation
    }

    pub fn set_position(&mut self, p: impl Into<Vec2>) {
        self.draw_params.deref_mut().translation = p.into();
    }

    pub fn set_scale(&mut self, s: impl Into<Vec2>) {
        self.draw_params.deref_mut().scale = s.into();
    }

    pub fn set_rotation(&mut self, r: impl Into<f32>) {
        self.draw_params.deref_mut().rotation = r.into();
    }

    pub fn set_vertices_and_indices(
        &mut self,
        vertices: impl IntoIterator<Item = UiVertex>,
        indices: Option<impl IntoIterator<Item = u16>>,
    ) {
        let vertices = vertices.into_iter().collect();
        let indices = indices.map(|is| is.into_iter().collect());
        self.updated_vertices = Some((vertices, indices));
    }

    pub fn set_vertices(&mut self, vertices: impl IntoIterator<Item = UiVertex>) {
        let vertices = vertices.into_iter().collect();
        self.updated_vertices = Some((vertices, None));
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.updated_texture = Some(texture.clone());
    }

    pub fn remove_texture(&mut self) {
        self.texture_bindgroup = None;
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), UiSceneError> {
        self.draw_params.update(queue);
        if let Some(texture) = self.updated_texture.take() {
            log::trace!("updating UiDrawObject texture");
            self.texture_bindgroup = Some(ui_texture_bindgroup(device, &texture))
        }
        if let Some((vertices, may_indices)) = self.updated_vertices.take() {
            log::trace!("updating UiDrawObject vertices");
            self.mesh.update(
                device,
                Some("UiDrawObject::update mesh"),
                vertices,
                may_indices,
            );
        }
        Ok(())
    }
}

pub struct UiDrawObjectBuilder<'a> {
    draw_params: Option<UiDrawParams>,
    vertices: Option<Vec<UiVertex>>,
    indices: Option<Vec<u16>>,
    device: &'a wgpu::Device,
    texture_bindgroup: Option<wgpu::BindGroup>,
}

impl<'a> UiDrawObjectBuilder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        UiDrawObjectBuilder {
            draw_params: None,
            vertices: None,
            indices: None,
            texture_bindgroup: None,
            device,
        }
    }

    pub fn with_draw_mode(mut self, mode: UiMode) -> Self {
        let params = self.draw_params.get_or_insert(UiDrawParams::default());
        params.mode = mode;
        self
    }

    pub fn with_position(mut self, p: impl Into<Vec2>) -> Self {
        let params = self.draw_params.get_or_insert(UiDrawParams::default());
        params.translation = p.into();
        self
    }

    pub fn with_scale(mut self, s: impl Into<Vec2>) -> Self {
        let params = self.draw_params.get_or_insert(UiDrawParams::default());
        params.scale = s.into();
        self
    }

    pub fn with_rotation(mut self, r: f32) -> Self {
        let params = self.draw_params.get_or_insert(UiDrawParams::default());
        params.rotation = r;
        self
    }

    pub fn with_texture(mut self, texture: &Texture) -> Self {
        let bindgroup = ui_texture_bindgroup(self.device, texture);
        self.texture_bindgroup = Some(bindgroup);
        self
    }

    pub fn with_vertices(mut self, vertices: impl IntoIterator<Item = UiVertex>) -> Self {
        self.vertices = Some(vertices.into_iter().collect());
        self
    }

    pub fn with_indices(mut self, indices: impl IntoIterator<Item = u16>) -> Self {
        self.indices = Some(indices.into_iter().collect());
        self
    }

    pub fn build(self) -> UiDrawObject {
        let UiDrawObjectBuilder {
            draw_params,
            vertices,
            indices: may_indices,
            device,
            texture_bindgroup,
        } = self;
        UiDrawObject::new(
            device,
            draw_params.unwrap_or_default(),
            vertices.unwrap_or_default(),
            may_indices,
            texture_bindgroup,
        )
    }
}

pub struct UiScene {
    device: Arc<wgpu::Device>,
    constants: Uniform<UiConstants>,
    _default_texture: Texture,
    default_texture_bindgroup: wgpu::BindGroup,
}

impl UiScene {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: &wgpu::Queue,
        canvas_size: UVec2,
        camera_translation: Vec2,
    ) -> Self {
        let constants = Uniform::new(
            &device,
            UiConstants {
                canvas_size,
                camera_translation,
            },
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::ShaderStages::VERTEX,
        );
        let texture = Texture::new(
            &device,
            queue,
            Some("UiScene.default_texture"),
            None,
            4,
            1,
            1,
            &[255, 255, 255, 255],
        );
        let default_texture_bindgroup = ui_texture_bindgroup(&device, &texture);
        UiScene {
            device,
            constants,
            _default_texture: texture,
            default_texture_bindgroup,
        }
    }

    pub fn set_canvas_size(&mut self, width: u32, height: u32) {
        self.constants.deref_mut().canvas_size = UVec2::new(width, height);
    }

    pub fn update<'a>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        draw_objects: impl IntoIterator<Item = &'a mut UiDrawObject>,
    ) -> Result<(), UiSceneError> {
        self.constants.update(queue);
        for obj in draw_objects.into_iter() {
            obj.update(device, queue)?;
        }
        Ok(())
    }

    pub fn constants_bindgroup(&self) -> &wgpu::BindGroup {
        self.constants.bindgroup()
    }

    pub fn default_texture_bindgroup(&self) -> &wgpu::BindGroup {
        &self.default_texture_bindgroup
    }

    pub fn new_object(&self) -> UiDrawObjectBuilder<'_> {
        UiDrawObjectBuilder::new(&self.device)
    }
}

pub struct UiSceneBuilder<'a> {
    device: Arc<wgpu::Device>,
    queue: &'a wgpu::Queue,
    canvas_size: UVec2,
    camera_translation: Vec2,
}

impl<'a> UiSceneBuilder<'a> {
    pub fn new(device: Arc<wgpu::Device>, queue: &'a wgpu::Queue) -> Self {
        Self {
            device,
            queue,
            canvas_size: UVec2::new(800, 600),
            camera_translation: Vec2::ZERO,
        }
    }

    pub fn with_canvas_size(mut self, width: u32, height: u32) -> Self {
        self.canvas_size = UVec2::new(width, height);
        self
    }

    pub fn with_camera_position(mut self, x: f32, y: f32) -> Self {
        self.camera_translation = Vec2::new(x, y);
        self
    }

    pub fn new_object(&self) -> UiDrawObjectBuilder<'_> {
        UiDrawObjectBuilder::new(&self.device)
    }

    pub fn new_texture_from_dynamic_image(&self, img: image::DynamicImage) -> Texture {
        Texture::from_dynamic_image(
            &self.device,
            self.queue,
            img,
            Some("UiSceneBuilder::new_texture_from_dynamic_image"),
            None,
        )
    }

    pub fn build(self) -> UiScene {
        let UiSceneBuilder {
            device,
            queue,
            canvas_size,
            camera_translation,
        } = self;
        UiScene::new(device.clone(), queue, canvas_size, camera_translation)
    }
}

#[repr(transparent)]
pub struct UiDrawObjects(pub Vec<UiDrawObject>);

impl Deref for UiDrawObjects {
    type Target = Vec<UiDrawObject>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UiDrawObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn ui_scene_update(
    (device, queue, mut scene, mut objects): (
        View<crate::Device>,
        View<crate::Queue>,
        ViewMut<UiScene>,
        ViewMut<UiDrawObjects>,
    ),
) -> Result<(), UiSceneError> {
    scene.update(&device, &queue, &mut objects.0)
}

impl UiDrawObject {
    pub fn draw<'a, 'b, 'c>(
        &'a self,
        render_pass: &'c mut wgpu::RenderPass<'b>,
        default_texture_bindgroup: &'a wgpu::BindGroup,
    ) where
        'a: 'b,
        'a: 'c,
    {
        let bindgroup = self
            .texture_bindgroup
            .as_ref()
            .unwrap_or(default_texture_bindgroup);
        render_pass.set_bind_group(1, bindgroup, &[]);
        render_pass.set_bind_group(2, self.draw_params.bindgroup(), &[]);
        self.mesh.draw(render_pass);
    }
}

pub fn ui_scene_render(
    (device, queue, scene, objects, pipeline, frame): (
        View<Device>,
        View<Queue>,
        View<UiScene>,
        View<UiDrawObjects>,
        View<UiRenderPipeline>,
        View<FrameTextureView>,
    ),
) -> Result<(), UiSceneError> {
    let label = Some("ui scene render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(&pipeline.0);
    render_pass.set_bind_group(0, scene.constants.bindgroup(), &[]);
    for object in objects.0.iter() {
        object.draw(&mut render_pass, &scene.default_texture_bindgroup);
    }
    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

pub fn setup_ui_render_graph(
    scene: UiScene,
    objects: impl IntoIterator<Item = UiDrawObject>,
    r: &mut Renderling,
    with_screen_capture: bool,
) {
    let objects = UiDrawObjects(objects.into_iter().collect::<Vec<_>>());
    let pipeline = UiRenderPipeline(
        r.graph
            .visit(|(device, target): (View<Device>, View<RenderTarget>)| {
                create_ui_pipeline(&device, target.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(scene);
    r.graph.add_resource(objects);
    r.graph.add_resource(pipeline);

    use crate::{
        graph,
        node::{clear_frame_and_depth, create_frame, present},
        Graph,
    };
    let pre_render =
        crate::graph!(create_frame, clear_frame_and_depth, ui_scene_update).with_barrier();
    let render = crate::graph!(ui_scene_render).with_barrier();
    let post_render = if with_screen_capture {
        let copy_frame_to_post = crate::node::PostRenderBufferCreate::create;
        crate::graph!(copy_frame_to_post < present)
    } else {
        crate::graph!(present)
    };
    r.graph
        .add_subgraph(pre_render)
        .add_subgraph(render)
        .add_subgraph(post_render);
}

#[cfg(test)]
mod test {
    use glam::Vec4;

    use super::*;

    #[test]
    fn ui_tri() {
        let mut r = Renderling::headless(50, 50).unwrap();
        let builder = r.new_ui_scene().with_canvas_size(50, 50);
        let tri = builder
            .new_object()
            .with_vertices(vec![
                UiVertex::default()
                    .with_position(Vec2::ZERO)
                    .with_color(Vec4::new(1.0, 0.0, 0.0, 1.0)),
                UiVertex::default()
                    .with_position(Vec2::new(0.0, 1.0))
                    .with_color(Vec4::new(0.0, 0.0, 1.0, 1.0)),
                UiVertex::default()
                    .with_position(Vec2::new(1.0, 0.0))
                    .with_color(Vec4::new(0.0, 1.0, 0.0, 1.0)),
            ])
            .with_scale(Vec2::splat(50.0))
            .build();
        let scene = builder.build();
        setup_ui_render_graph(scene, vec![tri], &mut r, true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("ui_tri.png", img);
    }

    #[test]
    fn ui_image() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(0.0));
        let builder = r.new_ui_scene().with_canvas_size(100, 100);
        let img = image::open("../../img/dirt.jpg").unwrap();
        let texture = builder.new_texture_from_dynamic_image(img);
        let tl = UiVertex::default()
            .with_position(Vec2::ZERO)
            .with_uv(Vec2::ZERO)
            .with_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
        let tr = UiVertex::default()
            .with_position(Vec2::X)
            .with_uv(Vec2::X)
            .with_color(Vec4::new(0.0, 1.0, 0.0, 1.0));
        let bl = UiVertex::default()
            .with_position(Vec2::Y)
            .with_uv(Vec2::Y)
            .with_color(Vec4::new(0.0, 0.0, 1.0, 1.0));
        let br = UiVertex::default()
            .with_position(Vec2::ONE)
            .with_uv(Vec2::ONE)
            .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0));
        let obj = builder
            .new_object()
            .with_texture(&texture)
            .with_vertices(vec![tl, bl, br, tl, br, tr])
            .with_scale(Vec2::splat(100.0))
            .build();
        let scene = builder.build();
        setup_ui_render_graph(scene, vec![obj], &mut r, true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("ui_image.png", img);
    }

    #[cfg(feature = "text")]
    #[test]
    fn ui_text() {
        use crate::{FontArc, GlyphCache, Section, Text};
        let mut r = Renderling::headless(100, 50)
            .unwrap()
            .with_background_color(Vec4::splat(0.0));

        let bytes: Vec<u8> =
            std::fs::read("../../fonts/Font Awesome 6 Free-Regular-400.otf").unwrap();

        let font = FontArc::try_from_vec(bytes).unwrap();
        let mut glyph_cache = GlyphCache::new(vec![font]);
        glyph_cache.brush.queue(
            Section::default()
                .add_text(
                    Text::new("")
                        .with_color([1.0, 1.0, 0.0, 1.0])
                        .with_scale(32.0),
                )
                .add_text(
                    Text::new("")
                        .with_color([1.0, 0.0, 1.0, 1.0])
                        .with_scale(32.0),
                )
                .add_text(
                    Text::new("")
                        .with_color([0.0, 1.0, 1.0, 1.0])
                        .with_scale(32.0),
                ),
        );
        let (may_mesh, may_texture) = glyph_cache.get_updated(r.get_device(), r.get_queue());
        let mesh = may_mesh.unwrap();
        let texture = may_texture.unwrap();

        let builder = r.new_ui_scene().with_canvas_size(100, 50);
        let obj_a = builder
            .new_object()
            .with_draw_mode(UiMode::TEXT)
            .with_texture(&texture)
            .with_vertices(mesh.iter().copied())
            .build();
        let obj_b = builder
            .new_object()
            .with_draw_mode(UiMode::TEXT)
            .with_texture(&texture)
            .with_vertices(mesh)
            .with_position([15.0, 15.0])
            .build();
        let scene = builder.build();
        setup_ui_render_graph(scene, vec![obj_a, obj_b], &mut r, true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("ui_text.png", img);
    }
}
