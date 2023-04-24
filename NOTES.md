# Notes

Just pro-cons on tech choices and little things I don't want to forget whil implementing `renderling`.

# rust-gpu

## pros

* sharing code on CPU and GPU
* it's Rust
  - using cargo and Rust module system
  - expressions!

## cons / limititions

* can't use enums (but you can't in glsl or hlsl or msl or wgsl either)
* struct layout size/alignment errors can be really tricky
* rust code must be no-std
* don't use `while let` or `while` loops
* for loops are hit or miss, sometimes they work and sometimes they don't
  - see [this rust-gpu issue](https://github.com/EmbarkStudios/rust-gpu/issues/739)
  - see [conversation with edyyb on discord](https://discord.com/channels/750717012564770887/750717499737243679/threads/1092283362217046066)
* meh, but no support for dynamically sized arrays (how would that work in no-std?)
  - see [conversation on discord](https://discord.com/channels/750717012564770887/750717499737243679/1091813590400516106)

# wgpu

## pros

* works on all platforms with the same API
* much more configurable than OpenGL
* much better error messages than OpenGL
* much less verbose than Vulkan
* the team is very responsive

## cons

* no support for arrays of textures on web, yet
* not yet 1.0 (on by default in chrome beta)
* what happens if WebGPU the standard fails? (everyone doubts it will)
* atomics are not supported in the Naga SPIRV frontend, which limits the capabilities of compute
  - see [the related Naga issue](https://github.com/gfx-rs/naga/issues/2301)

# more things to figure out

* bindless - wth exactly is it

# tips and gotchas

* `location[...] is provided by the previous stage output but is not consumed as input by this stage.`
  - rust-gpu has optimized away the shader input, you must use the input parameter in your downstream shader
  - sometimes the optimization is pretty agressive, so you really gotta _use_ the input

# links

- [Forward+ shading (as opposed to deferred)](https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf)
  **tl;dr**
  In a compute shader before the vertex pass:
  * break up the frame into tiles
  * for each tile compute which lights contribute to the pixels in the tile
  * during shading, iterate over the lights for each pixel according to its tile

Given:
```rust
/// A bundle of GPU components.
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(bytemuck::Pod, bytemuck::Zeroable, Debug)
)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct GpuEntity {
    // The id of this entity. `ID_NONE` means this entity is not in use.
    pub id: u32,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // The id of this entity's first texture in the atlas.
    pub texture0: u32,
    // The id of this entity's second texture in the atlas.
    pub texture1: u32,
    // The lighting model used for shading this object.
    pub lighting: u32,
    // The id of this entity's parent, if it exists. `ID_NONE` means "no parent".
    pub parent: u32,
    pub padding0: u32,
    // The local translation of this entity
    pub position: Vec4,
    // The local scale of this entity
    pub scale: Vec4,
    // The local rotation of this entity
    pub rotation: Quat,
}
```

Works:
```rust
    /// Return the position, rotation and scale that describe this entity's
    /// transform in world space.
    pub fn get_world_transform(&self, entities: &[GpuEntity]) -> (Vec3, Quat, Vec3) {
        let mut position = Vec3::ZERO;
        let mut scale = Vec3::ONE;
        let mut rotation = Quat::IDENTITY;
        let mut index = self.id as usize;
        loop {
            let entity = entities[index];
            position += entity.position.xyz();
            scale *= entity.scale.xyz();
            rotation = entity.rotation * rotation;
            index = entity.parent as usize;
            if index >= entities.len() {
                break;
            }
        }
        (position, rotation, scale)
    }
```

Doesn't work:
```rust
    /// Return the position, rotation and scale that describe this entity's
    /// transform in world space.
    pub fn get_world_transform(&self, entities: &[GpuEntity]) -> (Vec3, Quat, Vec3) {
        let mut position = self.position.xyz();
        let mut scale = self.scale.xyz();
        let mut rotation = self.rotation;

        let mut parent_index = self.parent as usize;
        loop {
            if parent_index >= entities.len() {
                break;
            }

            let parent = entities[parent_index];
            position += parent.position.xyz();
            scale *= parent.scale.xyz();
            rotation = parent.rotation * rotation;

            parent_index = parent.parent as usize;
        }

        (position, rotation, scale)
    }
```
