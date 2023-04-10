//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{Mat4, UVec3, Vec2, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

/// A vertex in a mesh.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable, Debug)
)]
#[derive(Clone, Copy, PartialEq)]
pub struct GpuVertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
        }
    }
}

/// A GPU mesh.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct GpuMeshlet {
    // index into a [GpuVertex] array
    pub first_vertex: u32,
    // number of vertices in this mesh
    pub vertex_count: u32,
}

/// A GPU camera.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable, Debug)
)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct GpuCamera {
    pub projection: Mat4,
    pub view: Mat4,
}

/// Calculate attenuation
///
/// attenuation.x: constant
/// attenuation.y: linear
/// attenuation.z: quadratic
pub fn attenuate(attenuation: Vec3, distance: f32) -> f32 {
    let level = attenuation.x + attenuation.y * distance + attenuation.z * (distance * distance);
    if level == 0.0 {
        // no attenuation
        1.0
    } else {
        1.0 / level
    }
}

#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Copy, Clone, Default)]
pub struct GpuPointLight {
    pub position: Vec4,
    pub attenuation: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl GpuPointLight {
    /// Calculate a point light's color contribution to a fragment.
    pub fn color_phong(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        let light_pos: Vec3 = (view * self.position().extend(1.0)).xyz();
        let vertex_to_light = light_pos - vertex_pos;
        let vertex_to_light_distance = vertex_to_light.length();

        let light_dir: Vec3 = vertex_to_light.normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = vertex_to_light_distance;
        let attenuation: f32 = attenuate(self.attenuation(), distance);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation;
        diffuse *= attenuation;
        specular *= attenuation;

        ambient + diffuse + specular
    }
}


}

#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Copy, Clone, Default)]
pub struct GpuSpotLight {
    pub position: Vec4,
    pub direction: Vec4,
    pub attenuation: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
    pub cutoff: Vec4,
}

impl GpuSpotLight {
    // Calculate a spotlight's color contribution to a fragment.
    pub fn color_phong(
        &self,
        vertex_pos: Vec3,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_pos: Vec3 = (view * self.position().extend(1.0)).xyz();
        let light_dir: Vec3 = (light_pos - vertex_pos).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // attenuation
        let distance: f32 = (light_pos - vertex_pos).length();
        let attenuation: f32 = attenuate(self.attenuation(), distance);
        // spotlight intensity
        let direction: Vec3 = (-(view * self.direction().extend(0.0)).xyz()).normalize();
        let theta: f32 = light_dir.dot(direction);
        let epsilon: f32 = self.inner_cutoff() - self.outer_cutoff();
        let intensity: f32 = ((theta - self.outer_cutoff()) / epsilon).clamp(0.0, 1.0);
        // combine results
        let mut ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let mut diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let mut specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient *= attenuation * intensity;
        diffuse *= attenuation * intensity;
        specular *= attenuation * intensity;

        ambient + diffuse + specular
    }
}

#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Copy, Clone, Default)]
pub struct GpuDirectionalLight {
    pub direction: Vec4,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl GpuDirectionalLight {
    // Calculate a directional light's color contribution to a fragment.
    pub fn color_phong(
        &self,
        view: Mat4,
        normal: Vec3,
        camera_to_frag_dir: Vec3,
        diffuse_color: Vec4,
        specular_color: Vec4,
        shininess: f32,
    ) -> Vec3 {
        if self.direction() == Vec3::ZERO {
            return Vec3::ZERO;
        }
        let light_dir: Vec3 = (-(view * self.direction().extend(0.0)).xyz()).normalize();
        // diffuse shading
        let diff: f32 = normal.dot(light_dir).max(0.0);
        // specular shading
        let halfway_dir: Vec3 = (light_dir + camera_to_frag_dir).normalize();
        let spec: f32 = normal.dot(halfway_dir).max(0.0).powf(shininess);
        // combine results
        let ambient: Vec3 = self.ambient_color.rgb() * diffuse_color.rgb();
        let diffuse: Vec3 = self.diffuse_color.rgb() * diff * diffuse_color.rgb();
        let specular: Vec3 = self.specular_color.rgb() * spec * specular_color.rgb();
        ambient + diffuse + specular
    }
}

/// A bundle of GPU components.
///
/// The fields of `GpuEntity` are all u32s that represent the
/// index of the property in a global buffer. [`u32::MAX`] is
/// used to specify that the entity **does not have that property**.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpuEntity {
    pub id: u32,
    pub mesh: u32,
    pub transform: u32,
    pub texture: u32,
}

impl Default for GpuEntity {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            mesh: u32::MAX,
            transform: u32::MAX,
            texture: 0,
        }
    }
}

impl GpuEntity {
    pub fn is_alive(&self) -> bool {
        self.id != u32::MAX
    }

    pub fn mesh_id(&self) -> Option<u32> {
        if self.mesh == u32::MAX {
            None
        } else {
            Some(self.mesh)
        }
    }

    pub fn transform_id(&self) -> Option<u32> {
        if self.transform == u32::MAX {
            None
        } else {
            Some(self.transform)
        }
    }

    pub fn texture_id(&self) -> Option<u32> {
        if self.texture == u32::MAX {
            None
        } else {
            Some(self.texture)
        }
    }
}

#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

/// Scene vertex shader.
pub fn main_vertex_scene(
    // which entity are we drawing
    instance_index: u32,
    // which vertex are we drawing
    vertex_index: u32,

    camera: &GpuCamera,
    _meshes: &[GpuMeshlet],
    vertices: &[GpuVertex],
    transforms: &[Mat4],
    entities: &[GpuEntity],

    out_color: &mut Vec4,
    out_uv: &mut Vec2,
    out_pos: &mut Vec4,
) {
    let entity = entities[instance_index as usize];
    let vertex = vertices[vertex_index as usize];
    let transform = transforms[entity.transform as usize];

    *out_color = vertex.color;
    *out_uv = vertex.uv.xy();
    *out_pos = camera.projection * camera.view * transform * vertex.position.xyz().extend(1.0);
}

/// Scene fragment shader.
pub fn main_fragment_scene(
    atlas: &Image2d,
    sampler: &Sampler,

    in_color: Vec4,
    in_uv: Vec2,

    output: &mut Vec4,
) {
    let uv_color: Vec4 = atlas.sample(*sampler, in_uv);
    *output = in_color * uv_color;
}

/// Compute the draw calls for this frame.
///
/// This should be called with `groupcount = (entities.len() / threads) + 1`.
pub fn compute_cull_entities(
    _camera: &GpuCamera,
    meshes: &[GpuMeshlet],
    _vertices: &[GpuVertex],
    _transforms: &[Mat4],
    entities: &[GpuEntity],

    draws: &mut [DrawIndirect],
    count: &mut u32,

    global_id: UVec3,
) {
    let i = global_id.x as usize;

    // this is a hack because we can't use atomics yet
    if i == 0 {
        *count = entities.len() as u32;
    }

    if i > entities.len() {
        return;
    }

    let mut call = DrawIndirect {
        vertex_count: 0,
        instance_count: 0,
        base_vertex: 0,
        base_instance: i as u32,
    };
    let entity = &entities[i];
    // at first we'll just draw everything into the draw indirect buffer
    let is_visible = true;
    if entity.is_alive() && is_visible {
        //// once naga supports atomics we can use this to compact the array
        // let index = unsafe {
        //    spirv_std::arch::atomic_i_increment::<
        //        u32,
        //        { spirv_std::memory::Scope::Device as u32 },
        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
        //    >(count)
        //};
        let mesh = meshes[entity.mesh as usize];
        call.instance_count = 1;
        call.base_vertex = mesh.first_vertex;
        call.vertex_count = mesh.vertex_count;
    }
    draws[i] = call;
}
