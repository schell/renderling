# renderling 🍖

Renderling is an innovative, GPU-driven renderer designed for efficient scene rendering with a focus on leveraging 
GPU capabilities for nearly all rendering operations. 
Utilizing Rust for shader development, it ensures memory safety and cross-platform compatibility, including web platforms. 
The project, currently in the alpha stage, aims for rapid loading of GLTF files, handling large scenes, and supporting numerous lights. 
Development emphasizes performance, ergonomics, observability and the use of modern rendering techniques like forward+ rendering and 
physically based shading.

<img width="912" alt="ibl_environment_test" src="https://github.com/schell/renderling/assets/24942/297d6150-64b2-45b8-9760-12b27dc8cc3e">

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) 
with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. 
Learn more at the [NLnet project page](https://nlnet.nl/project/Renderling).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)

[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)


## Warning

This is very much a work in progress.

## What

`renderling` holds entire scenes of geometry, textures, materials, lighting, even the scene graph itself - in GPU buffers.
All but a few of the rendering operations happen on the GPU.
The CPU is used to interact with the filesystem to marshall data to the GPU and to update transforms.

Shaders are written in Rust, via `rust-gpu`.

## Why should I use `renderling`

* Data is easily staged on the GPU using an automatically reference counted slab allocator that 
  provides access from the CPU.

  Your scene geometry, materials, animations - all of it - live on the GPU, while the CPU has easy access
  to read and modify that data, without borrowing - allowing you to send your data through threads to anything 
  that needs it. 

* Having everything on the GPU makes `renderling` very effective at rendering certain types of scenes.
  
  Specifically `renderling` aims to be good at rendering scenes with a moderate level of unique geometry,
  (possibly a large amount of repeated geometry), with a small number of large textures (or large number of small textures),
  and lots of lighting effects.

* Tight integration with GLTF:
  - Loading scenes, nodes, animations etc
  - Includes tools for controlling animations
  - Supported extensions:
    * KHR_lights_punctual
    * KHR_materials_unlit
    * KHR_materials_emissive_strength
* Image based lighting + analytical lighting

* Good documentation

## API Features

* simple structs represent nodes, meshes, materials and lights
* seamless GPU / CPU syncronization
* headless rendering support
  - rendering to texture and saving via `image` crate
* text and user interface rendering support
* nested nodes with local transforms
* tight integration with glTF (cargo feature `gltf` - on by default)

Shaders are written in Rust via `rust-gpu` where possible, falling back to `wgsl` where needed.

## Rendering Features / Roadmap

Renderling takes a [forward+](https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf) approach to rendering.

By default it uses a single uber-shader for rendering.

- [x] texture atlas
  - [x] automatic resource management (Arc/drop based)
- [x] GPU slab allocator
  - [x] automatic resource management (Arc/drop based)
- [x] frustum culling
- [ ] occlusion culling
- [ ] light tiling
- [ ] shadow mapping
- 3d
  - [x] Built-in support for common lighting/material workflows
    - [x] physically based shading
    - [x] unlit
  - [x] high dynamic range
  - [x] skybox
  - image based lighting
    - [x] diffuse
    - [x] specular
  - [x] msaa (easy because of forward+)
  - [x] bloom "physically based" up+downsampling blur
  - [ ] ssao
  - [ ] depth of field
  - gltf support
    - [x] scenes
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
      - [x] skinning
      - [x] morph targets 
- 2d (renderling-ui)
  - [x] text
  - [x] stroked and filled paths
    - [x] circles
    - [x] rectangles
    - [x] cubic beziers
    - [x] quadratic beziers
    - [x] arbitrary polygons
    - [x] fill w/ image

## Definition
**renderling** noun

A small beast that looks cute up close, ready to do your graphics bidding.

## Haiku

> Ghost in the machine,
> lighting your scene with magic.
> Cute technology.

## Project Organization

* crates/renderling

  Main library crate.
  Contains CPU Rust code for creating pipelines and managing resources, making render passes, etc.
  Contains GPU Rust code of the shader operations themselves.
  Contains tests, some using image comparison of actual frame renders for consistency and backwards compatibility.

* crates/renderling/shaders

  Contains **.spv** and **.wgsl** files generated by [`cargo-gpu`](https://github.com/rust-gpu/cargo-gpu).

* crates/renderling/src/linkage*

  Contains autogenerated `wgpu` linkage for the generated shaders.

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

The `crates/renderling/shaders/` folder contains the generated SPIR-V files.

To regenerate the shaders, run:

```
cargo shaders
```

There is a `.cargo/config.toml` alias for `cargo shaders` that expands into a larger 
shader compilation command.

## Building on WASM

```
RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build crates/example-wasm/index.html && basic-http-server -a 127.0.0.1:8080 crates/example-wasm/dist
```

## 🫶 Sponsor this!

This work will always be free and open source. 
If you use it (outright or for inspiration), please consider donating.

[💰 Sponsor 💝](https://github.com/sponsors/schell)

### Special thanks 

- James Harton ([@jimsynz](https://github.com/jimsynz/)) for donating multiple linux CI runners with 
  physical GPUs!

### Related work & spin-off projects 

Many projects were born from first solving a need within `renderling`. 
Some of these solutions were then spun off into their own projects.

- [`cargo-gpu`](https://githu.com/rust-gpu/cargo-gpu)
  A shader compilation cli tool.
- [`crabslab`](https://github.com/schell/crabslab)
  A slab allocator for working across CPU/GPU boundaries.
- [`loading-bytes`](crates/loading-bytes)
  A cross-platform (including the web) and comedically tiny way of loading files to bytes.
- [`moongraph`](https://github.com/schell/moongraph)
  A DAG and resource graph runner.
- Contributions to [`naga`](https://github.com/gfx-rs/wgpu/issues/4489)
  * Adding atomics support to the SPIR-V frontend (in progress)
- Contributions to [`gltf`](https://github.com/gltf-rs/gltf/pull/419)

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
