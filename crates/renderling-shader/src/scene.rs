//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{Mat4, UVec3, Vec4, Vec4Swizzles};

/// A vertex in a mesh.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable, Debug)
)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct GpuVertex {
    pub position: Vec4,
    pub color: Vec4,
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

//impl core::fmt::Debug for GpuCamera {
//    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//        use std::string::ToString;
//        f.debug_struct("GpuCamera")
//            .field("projection", &self.projection.to_string())
//            .field("view", &self.view.to_string())
//            .finish()
//    }
//}

/// A bundle of GPU components.
#[cfg_attr(
    not(target_arch = "spirv"),
    repr(C),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpuEntity {
    pub id: u32,
    pub mesh: u32,
    pub transform: u32,
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
    // which mesh
    instance_id: u32,
    // which vertex on that mesh
    vertex_id: u32,

    camera: &GpuCamera,
    meshes: &[GpuMeshlet],
    vertices: &[GpuVertex],
    transforms: &[Mat4],
    entities: &[GpuEntity],

    out_color: &mut Vec4,
    out_pos: &mut Vec4,
) {
    let entity: &GpuEntity = &entities[instance_id as usize];
    let mesh: &GpuMeshlet = &meshes[entity.mesh as usize];
    let vertex_index = mesh.first_vertex + vertex_id;
    let vertex: &GpuVertex = &vertices[vertex_index as usize];
    let transform = transforms[entity.transform as usize];

    *out_color = vertex.color;
    *out_pos = camera.projection * transform * vertex.position.xyz().extend(1.0);
}

/// Scene fragment shader.
pub fn main_fragment_scene(in_color: Vec4, output: &mut Vec4) {
    *output = in_color;
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

    let entity = &entities[i];
    let mesh = meshes[entity.mesh as usize];
    let call = DrawIndirect {
        vertex_count: mesh.vertex_count,
        instance_count: 1,
        base_vertex: mesh.first_vertex,
        base_instance: entity.id,
    };

    // at first we'll just draw everything into the draw indirect buffer
    let should_cull = false;
    if !should_cull {
        //// once naga supports atomics we can use this to compact the array
        // let index = unsafe {
        //    spirv_std::arch::atomic_i_increment::<
        //        u32,
        //        { spirv_std::memory::Scope::Device as u32 },
        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
        //    >(count)
        //};
        let index = i;
        draws[index as usize] = call;
    } else {
        draws[i] = DrawIndirect::default();
    }
}
