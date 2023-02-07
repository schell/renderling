# renderling 🍖
This is a collection of WGPU renderers wrapped in a convenient interface.

## Features

* builder pattern for lights, cameras and objects
* automatic resource management (objects and cameras are removed from the scene and gpu resources released on drop)
* headless rendering support
  - rendering to texture and saving via `image` crate
* text rendering support (cargo feature `text` - on by default)
* nested nodes with local transforms
* gltf support

Shaders are written in GLSL. **shaderc** is used to compile shaders to SPIR-V.

## Definition
**renderling** noun

A wrapper around a WGPU graphics pipeline, along with simple types used to marshal data to the GPU.

## Pipelines

### Forward
A blinn-phong material forward shader.

![renderling forward shader pipeline](https://raw.githubusercontent.com/schell/renderling/main/img/forward.png "renderling forward pipeline")

### UI
A simple forward shader that supports vertices with colors and/or textures. It has a special
blending uniform that determines how vertex colors should be blended with vertex UV texture
coords. This enables support for colorful text.

## License
Renderling is free and open source. All code in this repository is dual-licensed under either:

    MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach
is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

## Roadmap

- [x] ui shader
- [x] blinn-phong forward shader
- [x] builder pattern for lights, camera and objects
- [x] automatic resource management
- [x] headless rendering
- [x] object nesting / parenting / local transforms
- [ ] gltf support
- [ ] wireframe shader pipeline
- [ ] deferred shading pipeline
- [ ] physically based rendering pipeline
- [ ] convert shaders to [rust-gpu](https://github.com/EmbarkStudios/rust-gpu) - maybe?
