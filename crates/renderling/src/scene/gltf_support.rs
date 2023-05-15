//! GLTF support for renderling GPU scenes.
use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};
use gltf::khr_lights_punctual::Kind;
use rustc_hash::FxHashMap;
use snafu::prelude::*;
// use splines::Interpolate;

use super::*;

mod anime;
mod img;
pub use anime::*;

#[derive(Debug, Snafu)]
pub enum GltfLoaderError {
    #[snafu(display("{source}"))]
    Gltf { source: gltf::Error },

    #[snafu(display("Unsupported gltf image format: {:?}", format))]
    UnsupportedImageFormat { format: gltf::image::Format },

    #[snafu(display("Missing image {}", index))]
    MissingImage { index: usize },

    #[snafu(display("Invalid image"))]
    InvalidImage,

    #[snafu(display("Error during scene building phase: {source}"))]
    Scene { source: crate::SceneError },

    #[snafu(display("{what} is missing texture={index}"))]
    MissingTexture { what: &'static str, index: usize },

    #[snafu(display("Unsupported primitive mode: {:?}", mode))]
    PrimitiveMode { mode: gltf::mesh::Mode },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("{what} is missing material {:?} {:?}", index, name))]
    MissingMaterial {
        what: &'static str,
        index: Option<usize>,
        name: Option<String>,
    },

    #[snafu(display("Missing mesh {:?} {:?}", index, name))]
    MissingMesh {
        index: Option<usize>,
        name: Option<String>,
    },

    #[snafu(display("Missing entity {id:?}"))]
    MissingEntity { id: Id<GpuEntity> },

    #[snafu(display("Missing animation channel inputs"))]
    MissingInputs,

    #[snafu(display("Missing animation channel outputs"))]
    MissingOutputs,
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

    pub fn get_by_name(&self, name: &str) -> impl Iterator<Item = &T> + '_ {
        if let Some(indices) = self.names.get(name) {
            Box::new(indices.iter().flat_map(|index| self.get(*index)))
                as Box<dyn Iterator<Item = &T>>
        } else {
            Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &T>>
        }
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }
}

#[derive(Clone, Copy)]
pub struct GltfBoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl From<gltf::mesh::Bounds<[f32; 3]>> for GltfBoundingBox {
    fn from(gltf::mesh::Bounds { min, max }: gltf::mesh::Bounds<[f32; 3]>) -> Self {
        GltfBoundingBox {
            min: min.into(),
            max: max.into(),
        }
    }
}

#[derive(Clone)]
pub enum GltfNode {
    // Contains an index into the GltfLoader.cameras field.
    Camera(usize),
    // Contains an index into the GltfLoader.lights field.
    Light(usize),
    // Contains the id of the entity and the ids of its children.
    Mesh(Id<GpuEntity>, Vec<Id<GpuEntity>>),
    // Contains the id of the entity.
    Container(Id<GpuEntity>),
}

impl GltfNode {
    /// Attempt to return this node's entity id.
    ///
    /// Cameras and lights are not represented by entities in a renderling
    /// scene and so nodes of those types will return `None`.
    pub fn as_entity(&self) -> Option<Id<GpuEntity>> {
        match self {
            GltfNode::Mesh(id, _) => Some(*id),
            GltfNode::Container(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GltfMeshPrim {
    pub vertex_start: u32,
    pub vertex_count: u32,
    pub material_id: Id<GpuMaterial>,
    pub bounding_box: GltfBoundingBox,
}

/// The result of loading a gltf file into a [`SceneBuilder`].
///
/// Contains indexed and named lookups for resources contained within the loaded
/// gltf file.
///
/// To load a gltf file into a scene thereby creating a `GltfLoader` you can use
/// [`SceneBuilder::gltf_load`].
#[derive(Default)]
pub struct GltfLoader {
    // Contains the indices of SceneBuilder images loaded
    pub images: Vec<usize>,
    pub cameras: GltfStore<(Mat4, Mat4)>,
    pub lights: GltfStore<Id<GpuLight>>,
    pub textures: GltfStore<Id<GpuTexture>>,
    pub default_material: Id<GpuMaterial>,
    pub materials: GltfStore<Id<GpuMaterial>>,
    pub meshes: GltfStore<Vec<GltfMeshPrim>>,
    pub nodes: GltfStore<GltfNode>,
    pub animations: GltfStore<GltfAnimation>,
}

impl GltfLoader {
    /// Load everything into a scene builder and return the loader.
    pub fn load(
        builder: &mut SceneBuilder,
        document: gltf::Document,
        buffers: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfLoader, GltfLoaderError> {
        let mut loader = GltfLoader::default();
        if !builder.materials.is_empty() {
            loader.default_material = Id::new(0);
        }

        for (i, image) in images.into_iter().enumerate() {
            // let format = image_data_format_to_wgpu(image.format)?;
            // let num_channels = image_data_format_num_channels(image.format);
            log::trace!("adding image {} with format {:?}", i, image.format);
            let dyn_img = img::gltf_image_data_to_dyn(image)?;
            let image_index = builder.add_image(dyn_img.to_rgba8());
            loader.images.push(image_index);
            log::trace!("  with index={image_index} in the scene builder");
        }

        loader.load_textures(builder, &document)?;
        loader.load_materials(builder, &document)?;

        log::debug!("adding meshlets");
        for mesh in document.meshes() {
            loader.load_mesh(mesh, builder, &document, &buffers)?;
        }

        log::debug!("adding nodes");
        for node in document.nodes() {
            loader.load_node(node, builder)?;
        }

        log::debug!("adding animations");
        loader.load_animations(&document, &buffers)?;

        Ok(loader)
    }

    fn load_textures(
        &mut self,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
    ) -> Result<(), GltfLoaderError> {
        log::debug!("loading textures");
        for texture in document.textures() {
            self.load_texture(texture, builder)?;
        }
        Ok(())
    }

    fn load_texture(
        &mut self,
        texture: gltf::Texture<'_>,
        builder: &mut SceneBuilder,
    ) -> Result<Id<GpuTexture>, GltfLoaderError> {
        let index = texture.index();
        let name = texture.name().map(String::from);
        let image_loader_index = texture.source().index();
        let image_index =
            self.images
                .get(image_loader_index)
                .copied()
                .context(MissingImageSnafu {
                    index: image_loader_index,
                })?;
        fn mode(mode: gltf::texture::WrappingMode) -> TextureAddressMode {
            match mode {
                gltf::texture::WrappingMode::ClampToEdge => TextureAddressMode::CLAMP_TO_EDGE,
                gltf::texture::WrappingMode::MirroredRepeat => TextureAddressMode::MIRRORED_REPEAT,
                gltf::texture::WrappingMode::Repeat => TextureAddressMode::REPEAT,
            }
        }
        let mode_s = mode(texture.sampler().wrap_s());
        let mode_t = mode(texture.sampler().wrap_t());
        let params = TextureParams {
            image_index,
            mode_s,
            mode_t,
            ..Default::default()
        };

        let texture_id = builder.add_texture(params);
        log::trace!(
            "adding texture index:{index} name:{name:?} id:{texture_id:?} with wrapping \
             s:{mode_s} t:{mode_t}"
        );
        let _ = self.textures.insert(index, name, texture_id);
        Ok(texture_id)
    }

    /// Return the scene `Id<GpuTexture>` for the gltf texture at the given
    /// index, if possible.
    ///
    /// If the texture at the given index has not been loaded into the
    /// [`SceneBuilder`], it will be.
    pub fn texture_at(
        &mut self,
        index: usize,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
    ) -> Result<Id<GpuTexture>, GltfLoaderError> {
        if let Some(id) = self.textures.get(index) {
            Ok(*id)
        } else {
            let texture =
                document
                    .textures()
                    .find(|t| t.index() == index)
                    .context(MissingTextureSnafu {
                        what: "document",
                        index,
                    })?;
            self.load_texture(texture, builder)
        }
    }

    fn load_materials(
        &mut self,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
    ) -> Result<(), GltfLoaderError> {
        log::debug!("loading materials");
        for material in document.materials() {
            self.load_material(material, builder, document)?;
        }
        Ok(())
    }

    fn load_material(
        &mut self,
        material: gltf::Material<'_>,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
    ) -> Result<Id<GpuMaterial>, GltfLoaderError> {
        let index = material.index();
        let name = material.name().map(String::from);
        log::trace!("loading material {index:?} {name:?}");
        let pbr = material.pbr_metallic_roughness();
        let gpu_material_id = if material.unlit() {
            log::trace!("  is unlit");
            // TODO: add tex_coord params to the unlit materials
            let tex_id = if let Some(info) = pbr.base_color_texture() {
                let index = info.texture().index();
                self.texture_at(index, builder, document)?
            } else {
                Id::NONE
            };
            builder
                .new_unlit_material()
                .with_base_color(pbr.base_color_factor())
                .with_texture0(tex_id)
                .build()
        } else {
            log::trace!("  is pbr");
            let base_color = pbr.base_color_factor();
            let (base_color_tex_id, base_color_tex_coord) =
                if let Some(info) = pbr.base_color_texture() {
                    let index = info.texture().index();
                    let tex_id = self.texture_at(index, builder, document)?;
                    (tex_id, info.tex_coord())
                } else {
                    (Id::NONE, 0)
                };

            let (metallic, roughness, metallic_roughness_tex_id, metallic_roughness_tex_coord) =
                if let Some(info) = pbr.metallic_roughness_texture() {
                    let index = info.texture().index();
                    let tex_id = self.texture_at(index, builder, document)?;
                    (1.0, 1.0, tex_id, info.tex_coord())
                } else {
                    (pbr.metallic_factor(), pbr.roughness_factor(), Id::NONE, 0)
                };

            log::trace!("  base_color: {base_color:?}");
            log::trace!("  base_color_tex_id: {base_color_tex_id:?}");
            log::trace!("  base_color_tex_coord: {base_color_tex_coord}");
            log::trace!("  metallic: {metallic}");
            log::trace!("  roughness: {roughness}");
            log::trace!("  metallic_roughness_tex_id: {metallic_roughness_tex_id:?}");
            log::trace!("  metallic_roughness_tex_coord: {metallic_roughness_tex_coord}");
            builder
                .new_pbr_material()
                .with_base_color_factor(base_color)
                .with_base_color_texture(base_color_tex_id)
                .with_base_color_texture_coord(base_color_tex_coord)
                .with_metallic_factor(metallic)
                .with_roughness_factor(roughness)
                .with_metallic_roughness_texture(metallic_roughness_tex_id)
                .with_metallic_roughness_texture_coord(metallic_roughness_tex_coord)
                .build()
        };

        if let Some(norm_tex) = material.normal_texture() {
            let tex_id = self.texture_at(norm_tex.texture().index(), builder, document)?;
            // UNWRAP: ok because we just stored this material above
            let gpu_material = builder.materials.get_mut(gpu_material_id.index()).unwrap();
            gpu_material.texture2 = tex_id;
            gpu_material.texture2_tex_coord = norm_tex.tex_coord();
            log::trace!("  using normal map");
            log::trace!("    texture_id:        {:?}", gpu_material.texture2);
            log::trace!("    texture_tex_coord: {}", gpu_material.texture2_tex_coord);
        }
        if let Some(occlusion_tex) = material.occlusion_texture() {
            let tex_id = self.texture_at(occlusion_tex.texture().index(), builder, document)?;
            // UNWRAP: ok because we just stored this material above
            let gpu_material = builder.materials.get_mut(gpu_material_id.index()).unwrap();
            gpu_material.texture3 = tex_id;
            gpu_material.texture3_tex_coord = occlusion_tex.tex_coord();
            log::trace!("  using occlusion map");
            log::trace!("    texture_id:        {:?}", gpu_material.texture3);
            log::trace!("    texture_tex_coord: {}", gpu_material.texture3_tex_coord);
        }

        // If this material doesn't have an index it's because it's the default material
        // for this gltf file.
        if let Some(index) = index {
            let _ = self.materials.insert(index, name, gpu_material_id);
        } else {
            self.default_material = gpu_material_id;
        }
        Ok(gpu_material_id)
    }

    /// Return the scene `Id<GpuMaterial>` for the gltf material at the given
    /// index, if possible.
    ///
    /// If the material at the given index has not been loaded into the
    /// [`SceneBuilder`], it will be.
    ///
    /// Providing `None` returns the id of the default material, or `Id::NONE`
    /// if there is none.
    pub fn material_at(
        &mut self,
        may_index: Option<usize>,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
    ) -> Result<Id<GpuMaterial>, GltfLoaderError> {
        if let Some(index) = may_index {
            if let Some(material_id) = self.materials.get(index) {
                Ok(*material_id)
            } else {
                let material = document
                    .materials()
                    .find(|material| material.index() == Some(index))
                    .context(MissingMaterialSnafu {
                        what: "document",
                        name: None,
                        index: Some(index),
                    })?;
                self.load_material(material, builder, document)
            }
        } else {
            Ok(self.default_material)
        }
    }

    fn load_mesh(
        &mut self,
        mesh: gltf::Mesh<'_>,
        builder: &mut SceneBuilder,
        document: &gltf::Document,
        buffers: &[gltf::buffer::Data],
    ) -> Result<(), GltfLoaderError> {
        let mesh_index = mesh.index();
        let mesh_name = mesh.name().map(String::from);
        log::trace!("loading mesh {mesh_index} {mesh_name:?}");

        let mut mesh_primitives = vec![];
        for primitive in mesh.primitives() {
            log::trace!("  reading primitive {}", primitive.index());
            log::trace!("    bounds: {:?}", primitive.bounding_box());
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
            let mut gen_tangents = false;
            let tangents: Box<dyn Iterator<Item = Vec4>> =
                if let Some(tangents) = reader.read_tangents() {
                    Box::new(tangents.map(Vec4::from))
                } else {
                    gen_tangents = true;
                    Box::new(std::iter::repeat(Vec4::ZERO))
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
                log::trace!("    uvs: {} vertices", uvs.len());
                Box::new(uvs)
            } else {
                log::trace!("    uvs: none");
                Box::new(std::iter::repeat(Vec2::ZERO))
            };
            let uv1: Box<dyn Iterator<Item = Vec2>> = if let Some(uvs) = reader.read_tex_coords(1) {
                let uvs = uvs.into_f32().map(Vec2::from);
                log::trace!("    uvs: {} vertices", uvs.len());
                Box::new(uvs)
            } else {
                log::trace!("    uvs: none");
                Box::new(std::iter::repeat(Vec2::ZERO))
            };
            let uvs = uv0
                .zip(uv1)
                .map(|(uv0, uv1)| Vec4::new(uv0.x, uv0.y, uv1.x, uv1.y));
            let vertices = positions
                .iter()
                .zip(colors.zip(uvs.zip(normals.zip(tangents))))
                .map(|(position, (color, (uv, (normal, tangent))))| GpuVertex {
                    position: position.extend(0.0),
                    color,
                    uv,
                    normal: normal.extend(0.0),
                    tangent,
                })
                .collect::<Vec<_>>();
            // we don't yet support indices, so we'll just repeat vertices
            let mut vertices = if let Some(indices) = reader.read_indices() {
                let indices = indices.into_u32();
                indices.map(|i| vertices[i as usize]).collect::<Vec<_>>()
            } else {
                vertices
            };
            if gen_normals || gen_tangents {
                vertices.chunks_mut(3).for_each(|t| match t {
                    [a, b, c] => {
                        let ab = b.position.xyz() - a.position.xyz();
                        let ac = c.position.xyz() - a.position.xyz();
                        let n = if gen_normals {
                            let n = ab.cross(ac).extend(0.0);
                            a.normal = n;
                            b.normal = n;
                            c.normal = n;
                            n.xyz()
                        } else {
                            a.normal.xyz()
                        };
                        if gen_tangents {
                            let d_uv1 = b.uv.xy() - a.uv.xy();
                            let d_uv2 = c.uv.xy() - a.uv.xy();
                            let f = 1.0 / (d_uv1.x * d_uv2.y - d_uv2.x * d_uv1.y);
                            let s = f * Vec3::new(
                                d_uv2.y * ab.x - d_uv1.y * ac.x,
                                d_uv2.y * ab.y - d_uv1.y * ac.y,
                                d_uv2.y * ab.z - d_uv1.y * ac.z,
                            );
                            let t = f * Vec3::new(
                                d_uv1.x * ac.x - d_uv2.x * ab.x,
                                d_uv1.x * ac.y - d_uv2.x * ab.y,
                                d_uv1.x * ac.z - d_uv2.x * ab.z,
                            );
                            let tangent = (s - s.dot(n) * n)
                                .normalize_or_zero()
                                .extend(n.cross(s).dot(t).signum());
                            a.tangent = tangent;
                            b.tangent = tangent;
                            c.tangent = tangent;
                        }
                    }
                    _ => unreachable!("safe because we know these are triangles"),
                });
            }

            let (vertex_start, vertex_count) = builder.add_meshlet(vertices);
            let material_index = primitive.material().index();
            let material_id = self.material_at(material_index, builder, document)?;
            let bounding_box = primitive.bounding_box().into();
            mesh_primitives.push(GltfMeshPrim {
                vertex_start,
                vertex_count,
                material_id,
                bounding_box,
            });
        }
        let _ = self
            .meshes
            .insert(mesh.index(), mesh.name().map(String::from), mesh_primitives);
        Ok(())
    }

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
        log::trace!("  position: {position:?}");
        log::trace!("  rotation: {rotation:?}");
        log::trace!("  scale: {scale:?}");

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
                log::trace!("    with only 1 primitive, so no children needed");
                (
                    parent
                        .with_starting_vertex_and_count(
                            prims[0].vertex_start,
                            prims[0].vertex_count,
                        )
                        .with_material(prims[0].material_id)
                        .build()
                        .id,
                    vec![],
                )
            } else {
                let parent = parent.build().id;
                log::trace!("    with {} child primitives", prims.len());
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
                            .with_parent(parent)
                            .build()
                            .id
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
            log::trace!("  node is {}", from_gltf_light_kind(light.kind()));
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
            GltfNode::Container(entity.id)
        };

        let may_parent_id = match &gltf_node {
            GltfNode::Camera(_) => None,
            GltfNode::Light(_) => None,
            GltfNode::Mesh(parent, _) => Some(parent),
            GltfNode::Container(parent) => Some(parent),
        };

        if let Some(parent) = may_parent_id {
            for child in node.children() {
                let child_node = self.load_node(child, builder)?;
                if let Some(child_id) = child_node.as_entity() {
                    let child_entity = builder
                        .entities
                        .get_mut(child_id.index())
                        .context(MissingEntitySnafu { id: child_id })?;
                    child_entity.parent = *parent;
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

    pub fn load_animation(
        &mut self,
        animation: gltf::Animation,
        buffers: &[gltf::buffer::Data],
    ) -> Result<(), GltfLoaderError> {
        let index = animation.index();
        let name = animation.name().map(String::from);
        log::trace!("loading animation {index} {name:?}");
        let mut r_animation = GltfAnimation::default();
        for (i, channel) in animation.channels().enumerate() {
            log::trace!("  channel {i}");
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let inputs = reader.read_inputs().context(MissingInputsSnafu)?;
            let outputs = reader.read_outputs().context(MissingOutputsSnafu)?;
            let keyframes = inputs.map(|t| Keyframe(t)).collect::<Vec<_>>();
            log::trace!("    with {} keyframes", keyframes.len());
            let interpolation = channel.sampler().interpolation().into();
            log::trace!("    using {interpolation} interpolation");
            let tween = Tween {
                keyframes,
                properties: match outputs {
                    gltf::animation::util::ReadOutputs::Translations(ts) => {
                        log::trace!("    tweens translations");
                        TweenProperties::Translations(ts.map(Vec3::from).collect())
                    }
                    gltf::animation::util::ReadOutputs::Rotations(rs) => {
                        log::trace!("    tweens rotations");
                        TweenProperties::Rotations(rs.into_f32().map(Quat::from_array).collect())
                    }
                    gltf::animation::util::ReadOutputs::Scales(ss) => {
                        log::trace!("    tweens scales");
                        TweenProperties::Scales(ss.map(Vec3::from).collect())
                    }
                    gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {
                        log::trace!("    tweens morph target weights");
                        todo!("morph targets unsupported")
                    }
                },
                interpolation,
                target_node_index: {
                    let index = channel.target().node().index();
                    log::trace!("    of node {index}");
                    index
                }
            };
            r_animation.tweens.push(tween);
        }

        self.animations.insert(
            animation.index(),
            animation.name().map(String::from),
            r_animation,
        );
        Ok(())
    }

    pub fn load_animations(
        &mut self,
        document: &gltf::Document,
        buffers: &[gltf::buffer::Data],
    ) -> Result<(), GltfLoaderError> {
        for animation in document.animations() {
            self.load_animation(animation, buffers)?;
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "gltf"))]
mod test {
    use glam::{Vec3, Vec4};

    use crate::{camera, test::_init_logging, GpuVertex, Id, LightingModel, Renderling};

    #[test]
    // tests importing a gltf file and rendering the first image as a 2d object
    // ensures we are decoding images correctly
    fn images() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(1.0));
        let mut builder = r.new_scene();
        let _loader = builder.gltf_load("../../gltf/cheetah_cone.glb").unwrap();
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        builder.set_camera(projection, view);
        let material_id = builder
            .new_unlit_material()
            .with_texture0(Id::new(0))
            .build();
        let _img = builder
            .new_entity()
            .with_meshlet({
                let vs = vec![
                    GpuVertex::default()
                        .with_position([0.0, 0.0, 0.0])
                        .with_uv0([0.0, 0.0]),
                    GpuVertex::default()
                        .with_position([1.0, 0.0, 0.0])
                        .with_uv0([1.0, 0.0]),
                    GpuVertex::default()
                        .with_position([1.0, 1.0, 0.0])
                        .with_uv0([1.0, 1.0]),
                    GpuVertex::default()
                        .with_position([0.0, 1.0, 0.0])
                        .with_uv0([0.0, 1.0]),
                ];
                [0, 3, 2, 0, 2, 1].map(|i| vs[i])
            })
            .with_material(material_id)
            .with_scale(Vec3::new(100.0, 100.0, 1.0))
            .build();
        let scene = builder.build().unwrap();
        let (device, queue) = r.get_device_and_queue();
        let texture = scene.textures.read(&device, &queue, 0, 1).unwrap()[0];
        println!("{texture:?}");
        crate::setup_scene_render_graph(scene, &mut r, true);
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_images.png", img);
    }

    #[test]
    // ensures we can read a minimal gltf file with a simple triangle mesh
    fn minimal_mesh() {
        let mut r = Renderling::headless(20, 20)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut builder = r.new_scene();
        let _loader = builder
            .gltf_load("../../gltf/gltfTutorial_003_MinimalGltfFile.gltf")
            .unwrap();
        let projection = camera::perspective(20.0, 20.0);
        let view = camera::look_at(Vec3::new(0.5, 0.5, 2.0), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        builder.set_camera(projection, view);
        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_minimal_mesh.png", img);
    }

    #[test]
    // ensures we can
    // * read simple meshes
    // * support multiple nodes that reference the same mesh
    // * support primitives w/ positions and normal attributes
    // * support transforming nodes (T * R * S)
    fn simple_meshes() {
        _init_logging();
        let mut r = Renderling::headless(100, 50)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut builder = r.new_scene();
        let _loader = builder
            .gltf_load("../../gltf/gltfTutorial_008_SimpleMeshes.gltf")
            .unwrap();
        let projection = camera::perspective(100.0, 50.0);
        let view = camera::look_at(Vec3::new(1.0, 0.5, 1.5), Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        builder.set_camera(projection, view);
        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_simple_meshes.png", img);
    }

    #[test]
    fn simple_texture() {
        _init_logging();
        let size = 100;
        let mut r = Renderling::headless(size, size)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut builder = r.new_scene();
        let _loader = builder
            .gltf_load("../../gltf/gltfTutorial_013_SimpleTexture.gltf")
            .unwrap();

        let projection = camera::perspective(size as f32, size as f32);
        let view = camera::look_at(Vec3::new(0.5, 0.5, 1.25), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        builder.set_camera(projection, view);

        // there are no lights in the scene and the material isn't marked as "unlit", so
        // let's force it to be unlit.
        let mut material = builder.materials.get(0).copied().unwrap();
        material.lighting_model = LightingModel::NO_LIGHTING;
        builder.materials[0] = material;

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_simple_texture.png", img);
    }

    #[test]
    fn normal_mapping_brick_sphere() {
        _init_logging();
        let size = 600;
        let mut r = Renderling::headless(size, size)
            .unwrap()
            .with_background_color(Vec3::splat(1.0).extend(1.0));
        let mut builder = r.new_scene();
        let loader = builder.gltf_load("../../gltf/red_brick_03_1k.glb").unwrap();
        let (projection, view) = loader.cameras.get(0).copied().unwrap();
        builder.set_camera(projection, view);

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        println!("saving frame");
        // crate::img_diff::save("gltf_normal_mapping_brick_sphere.png", img.clone());
        crate::img_diff::assert_img_eq("gltf_normal_mapping_brick_sphere.png", img);
    }

    #[test]
    // Tests that we can reuse the same builder for multiple loaders, building
    // up a scene of multiple gltf documents.
    fn can_load_multiple_gltfs_into_one_builder() {
        _init_logging();
        let size = 600;
        let mut r = Renderling::headless(size, size)
            .unwrap()
            .with_background_color(Vec3::splat(1.0).extend(1.0));
        let mut builder = r.new_scene();
        let brick_loader = builder.gltf_load("../../gltf/red_brick_03_1k.glb").unwrap();
        let (projection, view) = brick_loader.cameras.get(0).copied().unwrap();
        builder.set_camera(projection, view);

        let _another_sun = builder
            .new_directional_light()
            .with_ambient_color(Vec4::ONE)
            .with_direction(Vec3::NEG_Z)
            .build();

        let brick_sphere_id = brick_loader
            .nodes
            .get_by_name("Sphere")
            .next()
            .unwrap()
            .as_entity()
            .unwrap();
        {
            // move the sphere over so we can see both models
            let brick_sphere = builder.entities.get_mut(brick_sphere_id.index()).unwrap();
            brick_sphere.position = Vec4::new(-0.2, 0.0, 0.0, 0.0);
        }

        let bust_loader = builder.gltf_load("../../gltf/marble_bust_1k.glb").unwrap();
        let bust_id = bust_loader
            .nodes
            .get_by_name("marble_bust_01")
            .next()
            .unwrap()
            .as_entity()
            .unwrap();
        {
            // move the bust over too
            let bust = builder.entities.get_mut(bust_id.index()).unwrap();
            bust.position = Vec4::new(0.2, -0.1, 0.2, 0.0);
        }

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        println!("saving frame");
        crate::img_diff::save("gltf_can_load_multiple.png", img.clone());
    }

    #[cfg(feature = "gltf")]
    #[test]
    fn simple_animation() {
        use moongraph::{IsGraphNode, Read};

        use crate::{
            node::FrameTextureView, setup_scene_render_graph, BackgroundColor, DepthTexture,
            Device, Queue, WgpuStateError,
        };

        _init_logging();
        let mut r = Renderling::headless(50, 50)
            .unwrap()
            .with_background_color(Vec4::ONE);

        let projection = camera::perspective(50.0, 50.0);
        let view = camera::look_at(Vec3::Z * 3.0, Vec3::ZERO, Vec3::Y);
        let mut builder = r.new_scene().with_camera(projection, view);
        let default_material = builder
            .new_unlit_material()
            .with_base_color([0.0, 0.0, 0.0, 0.5])
            .build();

        let loader = builder
            .gltf_load("../../gltf/animated_triangle.gltf")
            .unwrap();
        let tri_id = loader.nodes.get(0).unwrap().as_entity().unwrap();
        {
            let entity = builder.entities.get_mut(tri_id.index()).unwrap();
            entity.material = default_material;
        }
        let mut entities = builder.entities.clone();
        let scene = builder.build().unwrap();
        setup_scene_render_graph(scene, &mut r, true);
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_simple_animation.png", img);

        assert_eq!(1, loader.animations.len());

        // after this point we don't want to clear the frame before every rendering
        // because we're going to composite different frames of an animation into one,
        // so we'll replace the clear_frame_and_depth node with our own node
        // that only clears the depth.
        let clear_frame_and_depth_node = r.graph.remove_node("clear_frame_and_depth").unwrap();
        pub fn clear_only_depth(
            (device, queue, _frame_view, depth, color): (
                Read<Device>,
                Read<Queue>,
                Read<FrameTextureView>,
                Read<DepthTexture>,
                Read<BackgroundColor>,
            ),
        ) -> Result<(), WgpuStateError> {
            let depth_view = &depth.view;
            let [r, g, b, a] = color.0.to_array();
            let color = wgpu::Color {
                r: r.into(),
                g: g.into(),
                b: b.into(),
                a: a.into(),
            };
            crate::node::conduct_clear_pass(
                &device,
                &queue,
                Some("clear_only_depth"),
                None,
                Some(&depth_view),
                color,
            );
            Ok(())
        }
        let clear_only_depth_node = clear_only_depth
            .into_node()
            .with_name("clear_only_depth_node")
            .runs_after_barrier(clear_frame_and_depth_node.get_barrier());
        r.graph.add_node(clear_only_depth_node);

        let anime = loader.animations.get(0).unwrap();
        println!("anime: {:?}", anime);
        assert_eq!(1.0, anime.tweens[0].length_in_seconds());

        let num = 8;
        for i in 0..8 {
            let t = i as f32 / num as f32;
            let transforms = anime.get_properties_at_time(&loader, t).unwrap();
            let scene = r.graph.get_resource_mut::<crate::Scene>().unwrap().unwrap();
            for (id, transform) in transforms.into_iter() {
                let entity = entities.get_mut(id.index()).unwrap();
                entity.position += transform.translate.extend(0.0);
                entity.scale *= transform.scale.extend(1.0);
                entity.rotation *= transform.rotate;
                scene.update_entity(*entity).unwrap();
            }
            drop(scene);
            r.render().unwrap();
        }

        {
            let entity = entities.get_mut(0).unwrap();
            entity.visible = 0;
            r.graph
                .get_resource_mut::<crate::Scene>()
                .unwrap()
                .unwrap()
                .update_entity(*entity)
                .unwrap();
        }
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gltf_simple_animation_after.png", img);
    }
}
