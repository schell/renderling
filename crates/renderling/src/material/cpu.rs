//! CPU side of materials.

use std::sync::{Arc, Mutex};

use craballoc::{
    // Craballoc is used for memory allocation and management.
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
    value::Hybrid,
};
use crabslab::Id;
use glam::{Vec3, Vec4};

use crate::{
    atlas::{Atlas, AtlasTexture},
    material::MaterialDescriptor,
};

/// Wrapper around the materials slab, which holds material textures in an atlas.
#[derive(Clone)]
pub struct Materials {
    slab: SlabAllocator<WgpuRuntime>,
    atlas: Atlas,
    default_material: Material,
}

impl AsRef<WgpuRuntime> for Materials {
    fn as_ref(&self) -> &WgpuRuntime {
        self.slab.runtime()
    }
}

impl Materials {
    /// Creates a new `Materials` instance with the specified runtime and atlas size.
    ///
    /// # Arguments
    ///
    /// * `runtime` - A reference to the WgpuRuntime.
    /// * `atlas_size` - The size of the atlas texture.
    pub fn new(runtime: impl AsRef<WgpuRuntime>, atlas_size: wgpu::Extent3d) -> Self {
        let slab = SlabAllocator::new(runtime, "materials", wgpu::BufferUsages::empty());
        let atlas = Atlas::new(&slab, atlas_size, None, Some("materials-atlas"), None);
        let default_material = Material {
            descriptor: slab.new_value(Default::default()),
            albedo_texture: Default::default(),
            metallic_roughness_texture: Default::default(),
            normal_mapping_texture: Default::default(),
            ao_texture: Default::default(),
            emissive_texture: Default::default(),
        };
        Self {
            slab,
            atlas,
            default_material,
        }
    }

    /// Returns a reference to the WgpuRuntime.
    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    /// Returns a reference to the slab allocator.
    pub fn slab_allocator(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.slab
    }

    /// Returns a reference to the atlas.
    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    /// Returns the default material.
    pub fn default_material(&self) -> &Material {
        &self.default_material
    }

    /// Runs atlas upkeep and commits all changes to the GPU.
    ///
    /// Returns `true` if the atlas texture was recreated.
    #[must_use]
    pub fn commit(&self) -> (bool, SlabBuffer<wgpu::Buffer>) {
        // Atlas upkeep must be called first because it generates updates into the slab
        (self.atlas.upkeep(self.runtime()), self.slab.commit())
    }

    /// Returns a [`MaterialBuilder`] for building a new [`Material`].
    pub fn new_material(&self) -> Material {
        let descriptor = self.slab.new_value(MaterialDescriptor::default());
        Material {
            descriptor,
            albedo_texture: Default::default(),
            metallic_roughness_texture: Default::default(),
            normal_mapping_texture: Default::default(),
            ao_texture: Default::default(),
            emissive_texture: Default::default(),
        }
    }
}

/// A material staged on the GPU.
///
/// Internally a `Material` holds references to:
/// * its descriptor, [`MaterialDescriptor`], which lives on the GPU
/// * [`AtlasTexture`]s that determine how the material presents:
///   * albedo color
///   * metallic roughness
///   * normal mapping
///   * ambient occlusion
///   * emissive
///
/// ## Note
///
/// Clones of `Material` all point to the same underlying data.
/// Changing a value on one `Material` will change that value for all clones as well.
#[derive(Clone)]
pub struct Material {
    descriptor: Hybrid<MaterialDescriptor>,

    albedo_texture: Arc<Mutex<Option<AtlasTexture>>>,
    metallic_roughness_texture: Arc<Mutex<Option<AtlasTexture>>>,
    normal_mapping_texture: Arc<Mutex<Option<AtlasTexture>>>,
    ao_texture: Arc<Mutex<Option<AtlasTexture>>>,
    emissive_texture: Arc<Mutex<Option<AtlasTexture>>>,
}

impl From<&Material> for Material {
    fn from(value: &Material) -> Self {
        value.clone()
    }
}

impl Material {
    /// Returns the unique identifier of the material descriptor.
    pub fn id(&self) -> Id<MaterialDescriptor> {
        self.descriptor.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> MaterialDescriptor {
        self.descriptor.get()
    }

    /// Sets the emissive factor of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The emissive factor as a `Vec3`.
    pub fn set_emissive_factor(&self, param: Vec3) -> &Self {
        self.descriptor.modify(|d| d.emissive_factor = param);
        self
    }
    /// Sets the emissive factor.
    ///
    /// # Arguments
    ///
    /// * `param` - The emissive factor as a `Vec3`.
    pub fn with_emissive_factor(self, param: Vec3) -> Self {
        self.set_emissive_factor(param);
        self
    }
    /// Sets the emissive strength multiplier of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The emissive strength multiplier as a `f32`.
    pub fn set_emissive_strength_multiplier(&self, param: f32) -> &Self {
        self.descriptor
            .modify(|d| d.emissive_strength_multiplier = param);
        self
    }
    /// Sets the emissive strength multiplier.
    ///
    /// # Arguments
    ///
    /// * `param` - The emissive strength multiplier as a `f32`.
    pub fn with_emissive_strength_multiplier(self, param: f32) -> Self {
        self.set_emissive_strength_multiplier(param);
        self
    }
    /// Sets the albedo factor of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The albedo factor as a `Vec4`.
    pub fn set_albedo_factor(&self, param: Vec4) -> &Self {
        self.descriptor.modify(|d| d.albedo_factor = param);
        self
    }
    /// Sets the albedo factor.
    ///
    /// # Arguments
    ///
    /// * `param` - The albedo factor as a `Vec4`.
    pub fn with_albedo_factor(self, param: Vec4) -> Self {
        self.set_albedo_factor(param);
        self
    }
    /// Sets the metallic factor of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The metallic factor as a `f32`.
    pub fn set_metallic_factor(&self, param: f32) -> &Self {
        self.descriptor.modify(|d| d.metallic_factor = param);
        self
    }
    /// Sets the metallic factor.
    ///
    /// # Arguments
    ///
    /// * `param` - The metallic factor as a `f32`.
    pub fn with_metallic_factor(self, param: f32) -> Self {
        self.set_metallic_factor(param);
        self
    }
    /// Sets the roughness factor of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The roughness factor as a `f32`.
    pub fn set_roughness_factor(&self, param: f32) -> &Self {
        self.descriptor.modify(|d| d.roughness_factor = param);
        self
    }
    /// Sets the roughness factor.
    ///
    /// # Arguments
    ///
    /// * `param` - The roughness factor as a `f32`.
    pub fn with_roughness_factor(self, param: f32) -> Self {
        self.set_roughness_factor(param);
        self
    }
    /// Sets the albedo texture coordinate of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn set_albedo_tex_coord(&self, param: u32) -> &Self {
        self.descriptor.modify(|d| d.albedo_tex_coord = param);
        self
    }
    /// Sets the albedo texture coordinate.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn with_albedo_tex_coord(self, param: u32) -> Self {
        self.set_albedo_tex_coord(param);
        self
    }
    /// Sets the metallic roughness texture coordinate of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn set_metallic_roughness_tex_coord(&self, param: u32) -> &Self {
        self.descriptor
            .modify(|d| d.metallic_roughness_tex_coord = param);
        self
    }
    /// Sets the metallic roughness texture coordinate.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn with_metallic_roughness_tex_coord(self, param: u32) -> Self {
        self.set_metallic_roughness_tex_coord(param);
        self
    }
    /// Sets the normal texture coordinate of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn set_normal_tex_coord(&self, param: u32) -> &Self {
        self.descriptor.modify(|d| d.normal_tex_coord = param);
        self
    }
    /// Sets the normal texture coordinate.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn with_normal_tex_coord(self, param: u32) -> Self {
        self.set_normal_tex_coord(param);
        self
    }
    /// Sets the ambient occlusion texture coordinate of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn set_ambient_occlusion_tex_coord(&self, param: u32) -> &Self {
        self.descriptor.modify(|d| d.ao_tex_coord = param);
        self
    }
    /// Sets the ambient occlusion texture coordinate.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn with_ambient_occlusion_tex_coord(self, param: u32) -> Self {
        self.set_ambient_occlusion_tex_coord(param);
        self
    }
    /// Sets the emissive texture coordinate of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn set_emissive_tex_coord(&self, param: u32) -> &Self {
        self.descriptor.modify(|d| d.emissive_tex_coord = param);
        self
    }
    /// Sets the emissive texture coordinate.
    ///
    /// # Arguments
    ///
    /// * `param` - The texture coordinate as a `u32`.
    pub fn with_emissive_tex_coord(self, param: u32) -> Self {
        self.set_emissive_tex_coord(param);
        self
    }
    /// Sets whether the material has lighting.
    ///
    /// # Arguments
    ///
    /// * `param` - A boolean indicating if the material has lighting.
    pub fn set_has_lighting(&self, param: bool) -> &Self {
        self.descriptor.modify(|d| d.has_lighting = param);
        self
    }
    /// Sets whether the material has lighting.
    ///
    /// # Arguments
    ///
    /// * `param` - A boolean indicating if the material has lighting.
    pub fn with_has_lighting(self, param: bool) -> Self {
        self.set_has_lighting(param);
        self
    }
    /// Sets the ambient occlusion strength of the material.
    ///
    /// # Arguments
    ///
    /// * `param` - The ambient occlusion strength as a `f32`.
    pub fn set_ambient_occlusion_strength(&self, param: f32) -> &Self {
        self.descriptor.modify(|d| d.ao_strength = param);
        self
    }
    /// Sets the ambient occlusion strength.
    ///
    /// # Arguments
    ///
    /// * `param` - The ambient occlusion strength as a `f32`.
    pub fn with_ambient_occlusion_strength(self, param: f32) -> Self {
        self.set_ambient_occlusion_strength(param);
        self
    }

    /// Remove the albedo texture.
    ///
    /// This causes any `[Renderlet]` that references this material to fall back to
    /// using the albedo factor for color.
    pub fn remove_albedo_texture(&self) {
        self.descriptor.modify(|d| d.albedo_texture_id = Id::NONE);
        self.albedo_texture.lock().unwrap().take();
    }

    /// Sets the albedo color texture.
    pub fn set_albedo_texture(&self, texture: &AtlasTexture) -> &Self {
        self.descriptor
            .modify(|d| d.albedo_texture_id = texture.id());
        *self.albedo_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Replace the albedo texture.
    pub fn with_albedo_texture(self, texture: &AtlasTexture) -> Self {
        self.descriptor
            .modify(|d| d.albedo_texture_id = texture.id());
        *self.albedo_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Remove the metallic roughness texture.
    ///
    /// This causes any `[Renderlet]` that references this material to fall back to
    /// using the metallic and roughness factors for appearance.
    pub fn remove_metallic_roughness_texture(&self) {
        self.descriptor
            .modify(|d| d.metallic_roughness_texture_id = Id::NONE);
        self.metallic_roughness_texture.lock().unwrap().take();
    }

    /// Sets the metallic roughness texture of the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the metallic roughness `AtlasTexture`.
    pub fn set_metallic_roughness_texture(&self, texture: &AtlasTexture) -> &Self {
        self.descriptor
            .modify(|d| d.metallic_roughness_texture_id = texture.id());
        *self.metallic_roughness_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Sets the metallic roughness texture and returns the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the metallic roughness `AtlasTexture`.
    pub fn with_metallic_roughness_texture(self, texture: &AtlasTexture) -> Self {
        self.set_metallic_roughness_texture(texture);
        self
    }

    /// Remove the normal texture.
    ///
    /// This causes any `[Renderlet]` that references this material to fall back to
    /// using the default normal mapping.
    pub fn remove_normal_texture(&self) {
        self.descriptor.modify(|d| d.normal_texture_id = Id::NONE);
        self.normal_mapping_texture.lock().unwrap().take();
    }

    /// Sets the normal texture of the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the normal `AtlasTexture`.
    pub fn set_normal_texture(&self, texture: &AtlasTexture) -> &Self {
        self.descriptor
            .modify(|d| d.normal_texture_id = texture.id());
        *self.normal_mapping_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Sets the normal texture and returns the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the normal `AtlasTexture`.
    pub fn with_normal_texture(self, texture: &AtlasTexture) -> Self {
        self.set_normal_texture(texture);
        self
    }

    /// Remove the ambient occlusion texture.
    ///
    /// This causes any `[Renderlet]` that references this material to fall back to
    /// using the default ambient occlusion.
    pub fn remove_ambient_occlusion_texture(&self) {
        self.descriptor.modify(|d| d.ao_texture_id = Id::NONE);
        self.ao_texture.lock().unwrap().take();
    }

    /// Sets the ambient occlusion texture of the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the ambient occlusion `AtlasTexture`.
    pub fn set_ambient_occlusion_texture(&self, texture: &AtlasTexture) -> &Self {
        self.descriptor.modify(|d| d.ao_texture_id = texture.id());
        *self.ao_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Sets the ambient occlusion texture and returns the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the ambient occlusion `AtlasTexture`.
    pub fn with_ambient_occlusion_texture(self, texture: &AtlasTexture) -> Self {
        self.set_ambient_occlusion_texture(texture);
        self
    }

    /// Remove the emissive texture.
    ///
    /// This causes any `[Renderlet]` that references this material to fall back to
    /// using the emissive factor for appearance.
    pub fn remove_emissive_texture(&self) {
        self.descriptor.modify(|d| d.emissive_texture_id = Id::NONE);
        self.emissive_texture.lock().unwrap().take();
    }

    /// Sets the emissive texture of the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the emissive `AtlasTexture`.
    pub fn set_emissive_texture(&self, texture: &AtlasTexture) -> &Self {
        self.descriptor
            .modify(|d| d.emissive_texture_id = texture.id());
        *self.emissive_texture.lock().unwrap() = Some(texture.clone());
        self
    }

    /// Sets the emissive texture and returns the material.
    ///
    /// # Arguments
    ///
    /// * `texture` - A reference to the emissive `AtlasTexture`.
    pub fn with_emissive_texture(self, texture: &AtlasTexture) -> Self {
        self.set_emissive_texture(texture);
        self
    }
}
