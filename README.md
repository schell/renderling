
# renderling 🍖

Renderling is an innovative, GPU-driven renderer designed for efficient scene rendering with a focus on leveraging 
GPU capabilities for nearly all rendering operations. 
Utilizing Rust for shader development, it ensures memory safety and cross-platform compatibility, including web platforms. 
The project, currently in the alpha stage, aims for rapid loading of GLTF files, handling large scenes, and supporting numerous lights. 
Development emphasizes performance, configurability, observability and the use of modern rendering techniques like forward+ rendering and 
physically based shading.

<img width="912" alt="ibl_environment_test" src="https://github.com/schell/renderling/assets/24942/297d6150-64b2-45b8-9760-12b27dc8cc3e">

This project is sponsored by [NLnet](https://nlnet.nl/project/Renderling/) as part of [NGI Zero Core](https://nlnet.nl/core/).

<img src="https://nlnet.nl/logo/banner.svg" width="200" />

## What

`renderling` holds entire scenes of geometry, textures, materials, lighting, even the scene graph itself - in GPU buffers.
All but a few of the rendering operations happen on the GPU.
The CPU is used to interact with the filesystem to marshall data to the GPU and to update transforms.

Shaders are written in Rust, via `rust-gpu`.

## Why

This makes `renderling` very effective at rendering certain types of scenes.
Specifically `renderling` aims to be good at rendering scenes with a moderate level of unique geometry,
(possibly a large amount of repeated geometry), with a small number of large textures (or large number of small textures),
and lots of lighting effects.

## General Aspirations

- Very fast loading times for GLTF files
- Support for very large scenes
- Support for many lights

## API Features

* builder pattern for scenes, entities (scene nodes), materials and lights
* headless rendering support
  - rendering to texture and saving via `image` crate
* text rendering support (cargo feature `text` - on by default)
* nested nodes with local transforms
* tight support for loading scenes through `gltf` (cargo feature `gltf` - on by default)

Shaders are written in Rust via `rust-gpu` where possible, falling back to `wgsl` where needed.

## Rendering Features / Roadmap

Renderling takes a [forward+](https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf) approach to rendering.

By default it uses a single uber-shader for rendering.

- [ ] frustum culling
- [ ] occlusion culling
- [ ] light tiling
- 3d
  - [x] Built-in support for common lighting/material workflows
    - [x] physically based shading
    - [x] unlit
  - [x] high dynamic range
  - [x] skybox
  - image based lighting
    - [x] diffuse
    - [x] specular
  - [ ] msaa
  - [x] bloom "physically based" up+downsampling blur
  - [ ] ssao
  - [ ] depth of field
  - gltf support
    - [ ] scenes
    - [x] nodes
    - [x] cameras
    - [x] meshes
    - materials
      - [x] pbr metallic roughness (factors + textures)
      - [x] normal mapping
      - [x] occlusion textures
      - [ ] pbr specular glosiness
      - [ ] parallax mapping
    - [x] textures, images, samplers
    - animation
      - [x] interpolation
      - [x] morph targets
      - [x] skinning

## Definition
**renderling** noun

A small beast that looks cute up close, ready to do your graphics bidding.

## Haiku

> Ghost in the machine,
> lighting your scene with magic.
> Cute technology.

## Project Organization
* crates/renderling-shader

  Contains Rust shader code that can be shared on CPU and GPU (using `rust-gpu` to compile to SPIR-V).
  Most of the shader code is here!
  Certain tasks require atomics which doesn't work from `rust-gpu` to `wgpu` yet. See [NOTES.md](NOTES.md).
  This crate is a member of the workspace so you get nice editor tooling while writing shaders in Rust.
  You can also write sanity tests that run with `cargo test`.
  Things just work like BAU.

* shaders

  Contains a thin crate wrapper around `renderling-shader`.
  Provides the spirv annotations for shaders.
  Contains a program that compiles Rust into SPIR-V and copies **.spv** files into the main `renderling` crate.

* crates/renderling

  The main crate.
  Contains CPU Rust code for creating pipelines and managing resources, making render passes, etc.
  Contains tests, some using image comparison of actual frame renders for consistency and backwards compatibility.

* img

  Image assets for tests (textures, etc.)

* test_img

  Reference images to use for testing.

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

## Building on WASM

```
RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build crates/example-wasm/index.html && basic-http-server -a 127.0.0.1:8080 crates/example-wasm/dist
```

## 🫶 Sponsor this!

This work will always be free and open source. 
If you use it (outright or for inspiration), please consider donating.

[💰 Sponsor 💝](https://github.com/sponsors/schell)

### Related work & spin-off projects 

Many projects were born from first solving a need within `renderling`. 
Some of these solutions were then spun off into their own projects.

- [`crabslab`](https://github.com/schell/crabslab)
  A slab allocator for working across CPU/GPU boundaries.
- [`loading-bytes`](crates/loading-bytes)
  A cross-platform (including the web) way of loading files to bytes.
- [`moongraph`](https://github.com/schell/moongraph)
  A DAG and resource graph runner.
- Contributions to [`naga`](https://github.com/gfx-rs/wgpu/issues/4489)
  * Adding atomics support to the SPIR-V frontend (in progress)

Sponsoring this project contributes to the ecosystem. 

## License
Renderling is free and open source. All code in this repository is dual-licensed under either:

    MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach
is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

## Notes & Devlog
I keep a list of (un)organized notes about this project [here](NOTES.md).
I keep a devlog [here](DEVLOG.md).
