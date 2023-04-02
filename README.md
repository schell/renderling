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

## Project Organization
* crates/renderling-shader

  Contains Rust shader code that can be shared on CPU and GPU.
  Most of the shader code is here!
  This crate is a member of the workspace so you get nice editor tooling while writing shaders in Rust.
  You can also write sanity tests that run with `cargo test`.
  Things just work like BAU.

* shaders

  Contains a thin `rust-gpu` wrapper around `renderling-shader`.
  Provides the GPU annotations needed to bind `renderling` and `renderling-shader`.
  Contains a program that compiles and copies **.spv** files into the main `renderling` crate.

* crates/renderling

  The main crate.
  Contains CPU Rust code for creating pipelines and managing resources, making render passes, etc.

* crates/example

  Contains an example of using the `renderling` crate to make an application.

## Tests

Tests use `renderling` in headless mode and generate images that are compared to expected output.

### Running tests

```
cargo test
```

## Building the shaders

The `shaders/` folder is a crate that is excluded from the cargo workspace.
It compiles into a program that can be run to generate the shaders:

```
cd shaders/ && cargo run --release
```

Currently they all compile into one monolithic `.spv` file, but that may change in the future.

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
  - [ ] scenes, nodes
  - [x] cameras
  - [x] meshes
  - [x] materials
  - [x] textures, images, samplers
  - [ ] skins
  - [ ] animations
- [ ] convert shaders to [rust-gpu](https://github.com/EmbarkStudios/rust-gpu) - maybe?
- [ ] deferred shading pipeline
- [ ] physically based rendering pipeline
- [ ] render graph?
- [ ] wireframe shader pipeline - maybe?
