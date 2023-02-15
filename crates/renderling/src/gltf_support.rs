//! Collections of textures, materials, meshes, lights cameras and objects
//! arranged in 3d space.
//!
//! A scene is the structure that is built by importing a gltf file.
use std::sync::Arc;

use glam::{Mat4, Quat, Vec3};
use gltf::khr_lights_punctual::Kind;
use rustc_hash::FxHashMap;
use snafu::prelude::*;

use crate::{
    linkage::pbr, BlinnPhongMaterial, Camera, ForwardPipeline, LocalTransform, Mesh, MeshBuilder,
    Object, ObjectBuilderError, Renderling, Texture,
};

#[derive(Debug, Snafu)]
pub enum GltfError {
    #[snafu(display("Unsupported gltf image format: {:?}", format))]
    UnsupportedImageFormat { format: gltf::image::Format },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("Missing buffer"))]
    MissingBuffer,

    #[snafu(display("Missing view"))]
    MissingView,

    #[snafu(display("{}", source))]
    Object { source: ObjectBuilderError },
}

fn image_data_format_to_wgpu(
    gltf_format: gltf::image::Format,
) -> Result<wgpu::TextureFormat, GltfError> {
    let format = match gltf_format {
        gltf::image::Format::R8 => wgpu::TextureFormat::R8Unorm,
        gltf::image::Format::R8G8 => wgpu::TextureFormat::Rg8Unorm,
        // wgpu doesn't have an rgb8unorm texture format 🤷
        gltf::image::Format::R8G8B8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        gltf::image::Format::R8G8B8A8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        format => return Err(GltfError::UnsupportedImageFormat { format }),
    };
    Ok(format)
}

fn image_data_format_num_channels(gltf_format: gltf::image::Format) -> u32 {
    match gltf_format {
        gltf::image::Format::R8 => 1,
        gltf::image::Format::R8G8 => 2,
        // wgpu doesn't have an rgb8unorm texture format 🤷, so we map to rgba8unormsrgb
        gltf::image::Format::R8G8B8 => 4,
        gltf::image::Format::R8G8B8A8 => 4,
        gltf::image::Format::R16 => 1,
        gltf::image::Format::R16G16 => 2,
        gltf::image::Format::R16G16B16 => 3,
        gltf::image::Format::R16G16B16A16 => 4,
        gltf::image::Format::R32G32B32FLOAT => 3,
        gltf::image::Format::R32G32B32A32FLOAT => 4,
    }
}

#[derive(Default)]
pub struct GltfScene {
    cameras: Vec<Camera>,
    name_to_camera_index: FxHashMap<String, usize>,
    prims: Vec<Vec<Object>>,
    objects: Vec<Object>,
    name_to_object_index: FxHashMap<String, usize>,
}

fn camera_projection(projection: gltf::camera::Projection) -> Mat4 {
    match projection {
        gltf::camera::Projection::Orthographic(o) => Mat4::orthographic_rh(
            -o.xmag(),
            o.xmag(),
            -o.ymag(),
            o.ymag(),
            o.znear(),
            o.zfar(),
        ),
        gltf::camera::Projection::Perspective(p) => {
            if let Some(zfar) = p.zfar() {
                Mat4::perspective_rh(p.yfov(), p.aspect_ratio().unwrap_or(1.0), p.znear(), zfar)
            } else {
                Mat4::perspective_infinite_rh(p.yfov(), p.aspect_ratio().unwrap_or(1.0), p.znear())
            }
        }
    }
}

fn decomposed_transform(transform: gltf::scene::Transform) -> LocalTransform {
    let (t, r, s) = transform.decomposed();
    LocalTransform::default()
        .with_position(Vec3::from(t))
        .with_rotation(Quat::from_array(r))
        .with_scale(Vec3::from(s))
}

pub struct GltfLoader<'a, 'b> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    default_texture: Option<Texture>,
    name_to_mesh_index: FxHashMap<&'b str, usize>,
    meshes: Vec<Vec<Arc<Mesh>>>,
    textures: Vec<Texture>,
    name_to_material_index: FxHashMap<&'b str, usize>,
    materials: Vec<BlinnPhongMaterial>,
}

impl<'a, 'b> GltfLoader<'a, 'b> {
    /// Create a new loader.
    pub fn new(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> Self {
        GltfLoader {
            device,
            queue,
            default_texture: None,
            name_to_mesh_index: Default::default(),
            meshes: vec![],
            textures: vec![],
            name_to_material_index: Default::default(),
            materials: vec![],
        }
    }

    /// Get a reference to all the textures loaded thus far.
    pub fn textures(&self) -> &[Texture] {
        &self.textures
    }

    pub fn get_default_texture(&mut self) -> Texture {
        if self.default_texture.is_none() {
            self.default_texture = Some(Texture::new(
                self.device,
                self.queue,
                Some("GltfLoader::new-default"),
                None,
                4,
                1,
                1,
                &[0xff, 0xff, 0xff, 0xff],
            ));
        }

        // UNWRAP: we just ensured it's Some
        self.default_texture.clone().unwrap()
    }

    /// Load all materials from the gltf data into the loader.
    ///
    /// This also loads all textures.
    pub fn load_materials(
        &mut self,
        document: &'b gltf::Document,
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        log::trace!("loading materials and textures");
        let mut textures = vec![];
        for dat in images.into_iter() {
            let format = image_data_format_to_wgpu(dat.format)?;
            let num_channels = image_data_format_num_channels(dat.format);
            let pixels = if dat.format == gltf::image::Format::R8G8B8 {
                dat.pixels
                    .as_slice()
                    .chunks(3)
                    .flat_map(|rgb| match rgb {
                        [r, g, b] => vec![*r, *g, *b, 255],
                        // UNREACHABLE: we check the format above
                        _ => unreachable!("not rgb"),
                    })
                    .collect::<Vec<_>>()
            } else {
                dat.pixels.to_vec()
            };
            textures.push(Texture::new_with_format(
                self.device,
                self.queue,
                None,
                None,
                format,
                num_channels,
                dat.width,
                dat.height,
                &pixels,
            ));
        }
        self.textures = textures;

        for material in document.materials() {
            let metallic = material.pbr_metallic_roughness();
            log::trace!("material: {:?} {:?}", material.index(), material.name());
            log::trace!(
                "-metallic: base_color_factor: {:?}",
                metallic.base_color_factor()
            );
            let diffuse_texture = if let Some(texture_info) = metallic.base_color_texture() {
                let tex = texture_info.texture();
                log::trace!("--base_color_texture: {} {:?}", tex.index(), tex.name());
                self.textures[tex.index()].clone()
            } else {
                self.get_default_texture()
            };

            let specular_texture = self.get_default_texture();

            let blinn_phong = BlinnPhongMaterial {
                diffuse_texture,
                specular_texture,
                shininess: 16.0,
            };

            if let Some(name) = material.name() {
                self.name_to_material_index
                    .insert(name, self.materials.len());
            }
            self.materials.push(blinn_phong);
        }
        Ok(())
    }

    /// Load all mesh primitives from the gltf document into the loader.
    pub fn load_meshes(
        &mut self,
        document: &'b gltf::Document,
        buffers: &[gltf::buffer::Data],
    ) -> Result<(), GltfError> {
        log::trace!("loading meshes");
        for mesh in document.meshes() {
            let mut prims = vec![];
            log::trace!("mesh: {} {:?}", mesh.index(), mesh.name());
            log::trace!("-weights: {:?}", mesh.weights());
            for primitive in mesh.primitives() {
                log::trace!("-primitive: {}", primitive.index());
                log::trace!("--mode: {:?}", primitive.mode());
                log::trace!("--bounding_box: {:?}", primitive.bounding_box());
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let positions = reader.read_positions().context(MissingAttributeSnafu {
                    attribute: gltf::Semantic::Positions,
                })?;
                log::trace!("--positions: {} vertices", positions.len());
                let normals = reader.read_positions().context(MissingAttributeSnafu {
                    attribute: gltf::Semantic::Normals,
                })?;
                log::trace!("--normals: {} vertices", normals.len());
                let uvs: Box<dyn Iterator<Item = [f32; 2]>> =
                    if let Some(uvs) = reader.read_tex_coords(0) {
                        let uvs = uvs.into_f32();
                        log::trace!("--uvs: {} vertices", uvs.len());
                        Box::new(uvs)
                    } else {
                        Box::new(std::iter::repeat([0.0; 2]))
                    };

                let builder = positions.zip(normals.zip(uvs)).fold(
                    MeshBuilder::<pbr::Vertex>::default(),
                    |builder, (position, (normal, uv))| -> MeshBuilder<_> {
                        builder.with_vertex(pbr::Vertex {
                            position,
                            normal,
                            uv,
                        })
                    },
                );
                let builder = if let Some(indices) = reader.read_indices() {
                    let indices = match indices {
                        gltf::mesh::util::ReadIndices::U8(_) => todo!(),
                        gltf::mesh::util::ReadIndices::U16(indices) => indices,
                        gltf::mesh::util::ReadIndices::U32(_) => todo!(),
                    };
                    log::trace!("--indices: length {}", indices.len());
                    builder.with_indices(indices)
                } else {
                    builder
                };
                let prim = builder.build(Some("gltf_support"), self.device);
                prims.push(Arc::new(prim));
            }
            if let Some(name) = mesh.name() {
                self.name_to_mesh_index.insert(name, self.meshes.len());
            }
            self.meshes.push(prims);
        }

        Ok(())
    }

    pub fn build_node(
        &mut self,
        r: &mut Renderling<ForwardPipeline>,
        node: gltf::Node,
        scene: &mut GltfScene,
        depth: usize,
    ) -> Result<Option<Object>, GltfError> {
        let pad = std::iter::repeat("-")
            .take(depth)
            .collect::<Vec<_>>()
            .join("");
        log::trace!("{pad}-node: {} {:?}", node.index(), node.name());
        if let Some(camera) = node.camera() {
            log::trace!("{pad}--camera: {} {:?}", camera.index(), camera.name());
            let view = match node.transform() {
                gltf::scene::Transform::Matrix { matrix } => Mat4::from_cols_array_2d(&matrix),
                gltf::scene::Transform::Decomposed {
                    translation,
                    rotation,
                    scale,
                } => LocalTransform::default()
                    .with_position(Vec3::from(translation))
                    .with_rotation(Quat::from_array(rotation))
                    .with_scale(Vec3::from(scale))
                    .into(),
            };
            let r_camera = r
                .new_camera()
                .with_projection(camera_projection(camera.projection()))
                .with_view(view)
                .build();
            if let Some(name) = camera.name() {
                scene
                    .name_to_camera_index
                    .insert(name.to_string(), scene.cameras.len());
            }
            scene.cameras.push(r_camera);
        }

        let transform = decomposed_transform(node.transform());

        if let Some(mesh) = node.mesh() {
            log::trace!("{pad}--mesh: {} {:?}", mesh.index(), mesh.name());
            let prims = &self.meshes[mesh.index()];
            let object = if prims.len() > 1 {
                // the mesh is made up of multiple mesh "primitives", so add those
                // as children
                let mut children = vec![];
                for prim in prims.iter() {
                    let child = r
                        .new_object()
                        .with_mesh(prim.clone())
                        .build()
                        .context(ObjectSnafu)?;
                    children.push(child);
                }
                let object = r
                    .new_object()
                    .with_transform(transform.clone())
                    .with_children(children.iter())
                    .build()
                    .context(ObjectSnafu)?;
                log::trace!("{pad}--mesh-primitive-children: {}", children.len());
                // hold on to the child objects so they don't get dropped
                scene.prims.push(children);
                object
            } else {
                r.new_object()
                    .with_transform(transform.clone())
                    .with_mesh(prims[0].clone())
                    .build()
                    .context(ObjectSnafu)?
            };
            if let Some(name) = node.name() {
                scene
                    .name_to_object_index
                    .insert(name.to_string(), scene.objects.len());
            }
            scene.objects.push(object.clone());
        }

        if let Some(light) = node.light() {
            log::trace!("{pad}--light: {} {:?}", light.index(), light.name());
            let diffuse_color = Vec3::from(light.color()).extend(1.0);
            let direction = Mat4::from_quat(transform.rotate).transform_vector3(Vec3::NEG_Z);
            match light.kind() {
                Kind::Directional => {
                    log::trace!("{pad}---kind: directional");
                    log::trace!("{pad}----color: {:?}", diffuse_color);
                    log::trace!("{pad}----direction: {:?}", direction);
                    let _ = r
                        .new_directional_light()
                        .with_direction(direction)
                        .with_diffuse_color(diffuse_color)
                        .build();
                }
                Kind::Point => {
                    log::trace!("{pad}---kind: point");
                    log::trace!("{pad}----color: {:?}", diffuse_color);
                    log::trace!("{pad}----position: {:?}", transform.translate);
                    let _ = r
                        .new_point_light()
                        .with_position(transform.translate)
                        .with_diffuse_color(diffuse_color)
                        .build();
                }
                Kind::Spot {
                    inner_cone_angle,
                    outer_cone_angle,
                } => {
                    log::trace!("{pad}---kind: spot");
                    log::trace!("{pad}----color: {:?}", diffuse_color);
                    log::trace!("{pad}----direction: {:?}", direction);
                    let _ = r
                        .new_spot_light()
                        .with_position(transform.translate)
                        .with_direction(direction)
                        .with_diffuse_color(diffuse_color)
                        .with_cutoff(inner_cone_angle, outer_cone_angle)
                        .build();
                }
            }
        }

        for node in node.children() {
            log::trace!("{pad}--child");
            let _ = self.build_node(r, node, scene, depth + 1);
        }

        Ok(None)
    }

    /// Load and return a scene.
    ///
    /// Loads all display objects (meshes, materials and transforms) into the
    /// loader and into the given renderling.
    pub fn load_scene(
        &mut self,
        index: Option<usize>,
        r: &mut Renderling<ForwardPipeline>,
        document: &'b gltf::Document,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        let index = index.unwrap_or_default();
        log::trace!("loading scene {}", index);

        self.load_materials(document, images)?;
        self.load_meshes(document, buffers)?;

        let mut r_scene = GltfScene::default();
        for scene in document.scenes() {
            log::trace!("scene: {} {:?}", scene.index(), scene.name());
            if scene.index() != index {
                log::trace!("  skipping");
                continue;
            }

            for node in scene.nodes() {
                let _ = self.build_node(r, node, &mut r_scene, 0);
            }
        }

        Ok(())
    }
}
