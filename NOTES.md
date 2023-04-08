# Notes

Just pro-cons on tech choices and little things I don't want to forget.

# rust-gpu

## pros
* sharing code on CPU and GPU
* it's Rust
  - using cargo and Rust module system

## limititions of rust-gpu
* rust code must be no-std
* no support for dynamically sized arrays (how would that work in no-std?)
  - see [conversation on discord](https://discord.com/channels/750717012564770887/750717499737243679/1091813590400516106)
* for loops are hit or miss, sometimes they work and sometimes they don't
  - see [this rust-gpu issue](https://github.com/EmbarkStudios/rust-gpu/issues/739)
  - see [conversation with edyyb on discord](https://discord.com/channels/750717012564770887/750717499737243679/threads/1092283362217046066)

# wgpu

## pros
* works on all platforms with the same API
* much more configurable than OpenGL
* much better error messages than OpenGL
* less verbose than Vulkan
* the team is very responsive

## cons
* no support for arrays of textures on web, yet
* not yet 1.0
* what happens if WebGPU the standard fails? (everyone doubts it will)
* atomics are not supported in the Naga SPIRV frontend

# more things to figure out
* bindless - what exactly is it
