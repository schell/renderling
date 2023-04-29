//! GLTF support for renderling GPU scenes.
use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};
use gltf::khr_lights_punctual::Kind;
use rustc_hash::FxHashMap;
use snafu::prelude::*;
// use splines::Interpolate;

use super::*;

#[derive(Debug, Snafu)]
pub enum GltfLoaderError {
    #[snafu(display("Unsupported gltf image format: {:?}", format))]
    UnsupportedImageFormat { format: gltf::image::Format },

    #[snafu(display("Missing image {}", index))]
    MissingImage { index: usize },

    #[snafu(display("Invalid image"))]
    InvalidImage,

    #[snafu(display("Error during scene building phase: {source}"))]
    Scene { source: crate::SceneError },

    #[snafu(display("Scene builder is missing a texture"))]
    MissingTexture { index: usize },

    #[snafu(display("Unsupported primitive mode: {:?}", mode))]
    PrimitiveMode { mode: gltf::mesh::Mode },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("Missing material {:?} {:?}", index, name))]
    MissingMaterial {
        index: Option<usize>,
        name: Option<String>,
    },

    #[snafu(display("Missing mesh {:?} {:?}", index, name))]
    MissingMesh {
        index: Option<usize>,
        name: Option<String>,
    },
}

fn _image_data_format_to_wgpu(
    gltf_format: gltf::image::Format,
) -> Result<wgpu::TextureFormat, GltfLoaderError> {
    let format = match gltf_format {
        gltf::image::Format::R8 => wgpu::TextureFormat::R8Unorm,
        gltf::image::Format::R8G8 => wgpu::TextureFormat::Rg8Unorm,
        // wgpu doesn't have an rgb8unorm texture format 🤷
        gltf::image::Format::R8G8B8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        gltf::image::Format::R8G8B8A8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        format => return Err(GltfLoaderError::UnsupportedImageFormat { format }),
    };
    Ok(format)
}

fn _image_data_format_num_channels(gltf_format: gltf::image::Format) -> u32 {
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

pub struct GltfStore<T> {
    dense: Vec<Option<T>>,
    names: FxHashMap<String, Vec<usize>>,
}

impl<T> Default for GltfStore<T> {
    fn default() -> Self {
        Self {
            dense: Default::default(),
            names: Default::default(),
        }
    }
}

impl<T> GltfStore<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.dense.iter().flatten()
    }

    pub fn remove(&mut self, index: usize, name: Option<String>) -> Option<T> {
        if let Some(name) = name {
            if let Some(indices) = self.names.get_mut(&name) {
                indices.retain(|i| *i != index);
            }
        }
        self.dense.get_mut(index)?.take()
    }

    pub fn insert(&mut self, index: usize, name: Option<String>, item: T) -> Option<T> {
        if self.dense.len() <= index {
            self.dense.resize_with(index + 1, || None);
        }
        while self.dense.len() <= index {
            self.dense.push(None);
        }
        let existing = self.remove(index, name.clone());
        self.dense[index] = Some(item);
        if let Some(name) = name {
            let indices = self.names.entry(name).or_default();
            indices.push(index);
        }
        existing
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.dense.get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.dense.get_mut(index)?.as_mut()
    }

    pub fn get_by_name(&self, name: &str) -> Option<Result<&T, impl Iterator<Item = &T> + '_>> {
        let indices = self.names.get(name)?;
        match indices.as_slice() {
            [index] => self.get(*index).map(|t| Ok(t)),
            indices => Some(Err(indices.iter().flat_map(|index| self.get(*index)))),
        }
    }
}

#[derive(Clone)]
pub enum GltfNode {
    Camera(usize),
    Light(usize),
    Mesh(GpuEntity, Vec<GpuEntity>),
    Container(GpuEntity),
}

#[derive(Clone, Copy)]
pub struct GltfMeshPrim {
    vertex_start: u32,
    vertex_count: u32,
    material_id: u32,
}

#[derive(Default)]
pub struct GltfLoader {
    pub cameras: GltfStore<(Mat4, Mat4)>,
    pub lights: GltfStore<GpuLight>,
    pub materials: GltfStore<(u32, GpuMaterial)>,
    pub meshes: GltfStore<Vec<GltfMeshPrim>>,
    pub nodes: GltfStore<GltfNode>,
}

impl GltfLoader {
    /// Load everything into a scene builder
    pub fn load(
        device: crate::Device,
        queue: crate::Queue,
        document: &gltf::Document,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<(GltfLoader, SceneBuilder), GltfLoaderError> {
        let mut loader = GltfLoader::default();
        let mut builder = SceneBuilder::new(device.0, queue.0);

        for image in images {
            // let format = image_data_format_to_wgpu(image.format)?;
            // let num_channels = image_data_format_num_channels(image.format);
            let img = match image.format {
                gltf::image::Format::R8 => todo!(),
                gltf::image::Format::R8G8 => todo!(),
                gltf::image::Format::R8G8B8 => {
                    let img: image::DynamicImage =
                        image::RgbImage::from_vec(image.width, image.height, image.pixels.clone())
                            .context(InvalidImageSnafu)?
                            .into();
                    img.to_rgba8()
                }
                gltf::image::Format::R8G8B8A8 => {
                    image::RgbaImage::from_vec(image.width, image.height, image.pixels.clone())
                        .context(InvalidImageSnafu)?
                        .into()
                }
                gltf::image::Format::R16 => todo!(),
                gltf::image::Format::R16G16 => todo!(),
                gltf::image::Format::R16G16B16 => todo!(),
                gltf::image::Format::R16G16B16A16 => todo!(),
                gltf::image::Format::R32G32B32FLOAT => todo!(),
                gltf::image::Format::R32G32B32A32FLOAT => todo!(),
            };
            builder.add_image(img);
        }

        loader.load_materials(document, &mut builder)?;

        log::debug!("adding meshlets");
        for mesh in document.meshes() {
            loader.load_mesh(mesh, buffers, &mut builder)?;
        }

        log::debug!("adding nodes");
        for node in document.nodes() {
            loader.load_node(node, &mut builder)?;
        }

        Ok((loader, builder))
    }

    fn load_materials(
        &mut self,
        document: &gltf::Document,
        builder: &mut SceneBuilder,
    ) -> Result<(), GltfLoaderError> {
        log::debug!("loading materials");
        for material in document.materials() {
            self.load_material(material, builder);
        }
        Ok(())
    }

    fn load_material(&mut self, material: gltf::Material<'_>, builder: &mut SceneBuilder) {
        let mut gpu_material = GpuMaterial::default();
        gpu_material.lighting_model = if material.unlit() {
            LightingModel::NO_LIGHTING
        } else {
            LightingModel::PHONG_LIGHTING
        };

        let pbr = material.pbr_metallic_roughness();

        gpu_material.factor0 = pbr.base_color_factor().into();
        if let Some(info) = pbr.base_color_texture() {
            let index = info.texture().index();
            gpu_material.texture0 = index as u32;
        }

        gpu_material.factor1.x = pbr.metallic_factor();
        gpu_material.factor1.y = pbr.roughness_factor();
        if let Some(info) = pbr.metallic_roughness_texture() {
            let index = info.texture().index();
            gpu_material.texture1 = index as u32;
        }
        let material_id = builder.add_material(gpu_material);
        if let Some(index) = material.index() {
            let _ = self.materials.insert(
                index,
                material.name().map(String::from),
                (material_id, gpu_material),
            );
        }
    }

    fn load_mesh(
        &mut self,
        mesh: gltf::Mesh<'_>,
        buffers: &[gltf::buffer::Data],
        builder: &mut SceneBuilder,
    ) -> Result<(), GltfLoaderError> {
        let mesh_index = mesh.index();
        let mesh_name = mesh.name().map(String::from);
        log::trace!("loading mesh {mesh_index} {mesh_name:?}");

        let mut mesh_primitives = vec![];
        for primitive in mesh.primitives() {
            log::trace!("  reading primitive {}", primitive.index());
            snafu::ensure!(
                primitive.mode() == gltf::mesh::Mode::Triangles,
                PrimitiveModeSnafu {
                    mode: primitive.mode()
                }
            );
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader
                .read_positions()
                .context(MissingAttributeSnafu {
                    attribute: gltf::Semantic::Positions,
                })?
                .map(Vec3::from)
                .collect::<Vec<_>>();
            let mut gen_normals = false;
            let normals: Box<dyn Iterator<Item = Vec3>> =
                if let Some(normals) = reader.read_normals() {
                    Box::new(normals.map(Vec3::from))
                } else {
                    gen_normals = true;
                    Box::new(std::iter::repeat(Vec3::ZERO))
                };
            let colors: Box<dyn Iterator<Item = Vec4>> = if let Some(colors) = reader.read_colors(0)
            {
                let colors = colors.into_rgba_f32();
                Box::new(colors.map(Vec4::from))
            } else {
                Box::new(std::iter::repeat(Vec4::splat(1.0)))
            };
            let uv0: Box<dyn Iterator<Item = Vec2>> = if let Some(uvs) = reader.read_tex_coords(0) {
                let uvs = uvs.into_f32().map(Vec2::from);
                log::trace!("--uvs: {} vertices", uvs.len());
                Box::new(uvs)
            } else {
                log::trace!("--uvs: none");
                Box::new(std::iter::repeat(Vec2::ZERO))
            };
            let uv1: Box<dyn Iterator<Item = Vec2>> = if let Some(uvs) = reader.read_tex_coords(1) {
                let uvs = uvs.into_f32().map(Vec2::from);
                log::trace!("--uvs: {} vertices", uvs.len());
                Box::new(uvs)
            } else {
                log::trace!("--uvs: none");
                Box::new(std::iter::repeat(Vec2::ZERO))
            };
            let uvs = uv0
                .zip(uv1)
                .map(|(uv0, uv1)| Vec4::new(uv0.x, uv0.y, uv1.x, uv1.y));
            let vertices = positions
                .iter()
                .zip(colors.zip(uvs.zip(normals)))
                .map(|(position, (color, (uv, normal)))| GpuVertex {
                    position: position.extend(0.0),
                    color,
                    uv,
                    normal: normal.extend(0.0),
                })
                .collect::<Vec<_>>();
            // we don't yet support indices, so we'll just repeat vertices
            let mut vertices =
                if let Some(indices) = reader.read_indices() {
                    let indices = indices.into_u32();
                    indices.map(|i| vertices[i as usize]).collect::<Vec<_>>()
                } else {
                    vertices
                };
            if gen_normals {
                vertices.chunks_mut(3).for_each(|t| match t {
                    [a, b, c] => {
                        let ab = b.position.xyz() - a.position.xyz();
                        let ac = c.position.xyz() - a.position.xyz();
                        let n = ab.cross(ac).extend(0.0);
                        a.normal = n;
                        b.normal = n;
                        c.normal = n;
                    }
                    _ => unreachable!("safe because we know these are triangles"),
                });
            }

            let (vertex_start, vertex_count) = builder.add_meshlet(vertices);
            let material_id = primitive
                .material()
                .index()
                .map(|i| i as u32)
                .unwrap_or(ID_NONE);
            mesh_primitives.push(GltfMeshPrim {
                vertex_start,
                vertex_count,
                material_id,
            });
        }
        let _ = self
            .meshes
            .insert(mesh.index(), mesh.name().map(String::from), mesh_primitives);
        Ok(())
    }

    // fn get_entity_of_node<'a>(
    //    &'a mut self,
    //    gltf_node: &'a mut GltfNode,
    //) -> Result<Option<&'a mut GpuEntity>, GltfLoaderError> {
    //    match gltf_node {
    //        GltfNode::Camera(_) => Ok(None),
    //        GltfNode::Light(_) => Ok(None),
    //        GltfNode::Mesh(index) => Ok(Some(
    //            &mut self
    //                .meshes
    //                .get_mut(*index)
    //                .context(MissingMeshSnafu {
    //                    index: *index,
    //                    name: "some node".to_string(),
    //                })?
    //                .0,
    //        )),
    //        GltfNode::Container(e) => Ok(Some(e)),
    //    }
    //}

    fn load_node(
        &mut self,
        node: gltf::Node<'_>,
        builder: &mut SceneBuilder,
    ) -> Result<GltfNode, GltfLoaderError> {
        log::trace!(
            "loading node {} {:?}",
            node.index(),
            node.name().map(String::from)
        );
        let (position, rotation, scale) = node.transform().decomposed();
        let position = Vec3::from(position);
        let rotation = Quat::from_array(rotation);
        let scale = Vec3::from(scale);

        let gltf_node = if let Some(camera) = node.camera() {
            log::trace!("  node is a camera");
            let projection = match camera.projection() {
                gltf::camera::Projection::Orthographic(o) => Mat4::orthographic_rh(
                    -o.xmag(),
                    o.xmag(),
                    -o.ymag(),
                    o.ymag(),
                    o.znear(),
                    o.zfar(),
                ),
                gltf::camera::Projection::Perspective(p) => {
                    let fovy = p.yfov();
                    let aspect = p.aspect_ratio().unwrap_or(1.0);
                    if let Some(zfar) = p.zfar() {
                        Mat4::perspective_rh(fovy, aspect, p.znear(), zfar)
                    } else {
                        Mat4::perspective_infinite_rh(
                            p.yfov(),
                            p.aspect_ratio().unwrap_or(1.0),
                            p.znear(),
                        )
                    }
                }
            };
            let view: Mat4 = Mat4::from_cols_array_2d(&node.transform().matrix()).inverse();

            let _ = self.cameras.insert(
                camera.index(),
                camera.name().map(String::from),
                (projection, view),
            );

            GltfNode::Camera(camera.index())
        } else if let Some(mesh) = node.mesh() {
            let index = mesh.index();
            log::trace!("  node is mesh {index}");
            let prims = self.meshes.get(index).context(MissingMeshSnafu {
                index,
                name: mesh
                    .name()
                    .map(String::from)
                    .unwrap_or("unknown".to_string()),
            })?;

            let parent = builder
                .new_entity()
                .with_position(position)
                .with_rotation(rotation)
                .with_scale(scale);
            let (parent, children) = if prims.len() == 1 {
                (
                    parent
                        .with_starting_vertex_and_count(
                            prims[0].vertex_start,
                            prims[0].vertex_count,
                        )
                        .with_material(prims[0].material_id)
                        .build(),
                    vec![],
                )
            } else {
                let parent = parent.build();
                let children = prims
                    .iter()
                    .map(|child_prim| {
                        builder
                            .new_entity()
                            .with_starting_vertex_and_count(
                                child_prim.vertex_start,
                                child_prim.vertex_count,
                            )
                            .with_material(child_prim.material_id)
                            .with_parent(&parent)
                            .build()
                    })
                    .collect::<Vec<_>>();
                (parent, children)
            };
            GltfNode::Mesh(parent, children)
        } else if let Some(light) = node.light() {
            let color = Vec3::from(light.color()).extend(1.0);
            let transparent = Vec4::splat(0.0);
            let direction = Mat4::from_quat(rotation).transform_vector3(Vec3::NEG_Z);
            let gpu_light = match light.kind() {
                Kind::Directional => builder
                    .new_directional_light()
                    .with_direction(direction)
                    .with_diffuse_color(color)
                    .with_specular_color(color)
                    .with_ambient_color(transparent)
                    .build(),
                Kind::Point => builder
                    .new_point_light()
                    .with_position(position)
                    .with_diffuse_color(color)
                    .build(),
                Kind::Spot {
                    inner_cone_angle,
                    outer_cone_angle,
                } => builder
                    .new_spot_light()
                    .with_position(position)
                    .with_direction(direction)
                    .with_diffuse_color(color)
                    .with_cutoff(inner_cone_angle, outer_cone_angle)
                    .build(),
            };
            let _ = self.lights.insert(light.index(), None, gpu_light);
            GltfNode::Light(light.index())
        } else {
            // this node is just a parent/container of other nodes
            let entity = builder
                .new_entity()
                .with_position(position)
                .with_rotation(rotation)
                .with_scale(scale)
                .build();
            GltfNode::Container(entity)
        };

        let may_parent_id = match &gltf_node {
            GltfNode::Camera(_) => None,
            GltfNode::Light(_) => None,
            GltfNode::Mesh(parent, _) => Some(parent.id),
            GltfNode::Container(parent) => Some(parent.id),
        };

        if let Some(parent) = may_parent_id {
            for child in node.children() {
                let mut child_node = self.load_node(child, builder)?;
                let updated = match &mut child_node {
                    GltfNode::Mesh(child_entity, _) => {
                        child_entity.parent = parent;
                        Some(*child_entity)
                    }
                    GltfNode::Container(child_entity) => {
                        child_entity.parent = parent;
                        Some(*child_entity)
                    }
                    _ => None,
                };
                if let Some(e) = updated {
                    builder.update_entity(e).context(SceneSnafu)?;
                }
            }
        }

        let _ = self.nodes.insert(
            node.index(),
            node.name().map(String::from),
            gltf_node.clone(),
        );

        Ok(gltf_node)
    }
}
