# Notes

Just pro-cons on tech choices and little things I don't want to forget whil implementing `renderling`.

# gltf

* why are there repeats of nodes in document.nodes?

# rust-gpu

## pros

* sharing code on CPU and GPU
  - sanity testing GPU code on CPU using regular tests
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
  - see [conversation with eddyb on discord](https://discord.com/channels/750717012564770887/750717499737243679/threads/1092283362217046066)
* meh, but no support for dynamically sized arrays (how would that work in no-std?)
  - see [conversation on discord](https://discord.com/channels/750717012564770887/750717499737243679/1091813590400516106)
* can't use bitwise rotate_left or rotate_right
  - see [the issue on github](https://github.com/EmbarkStudios/rust-gpu/issues/1062)
* sometimes things like indexing are just funky-joe-monkey:
  - see [this comment on discord](https://discord.com/channels/750717012564770887/750717499737243679/1131395331368693770)

# wgpu

## pros

* works on all platforms with the same API
* much more configurable than OpenGL
* much better error messages than OpenGL
* less verbose than Vulkan
* the team is very responsive

## cons

* no support for arrays of textures on web, yet
* not yet 1.0 (on by default in chrome beta)
* ~~what happens if WebGPU the standard fails? (everyone doubts it will)~~
* atomics are not supported in the Naga SPIRV frontend, which limits the capabilities of compute
  - see [the related Naga issue](https://github.com/gfx-rs/naga/issues/2301)

# more things to figure out

* bindless - wth exactly is it

# tips, gotchas, links and further reading

* `location[...] is provided by the previous stage output but is not consumed as input by this stage.`
  - rust-gpu has optimized away the shader input, you must use the input parameter in your downstream shader
  - sometimes the optimization is pretty agressive, so you really gotta _use_ the input
* [Forward+ shading (as opposed to deferred)](https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf)
  **tl;dr**
  In a compute shader before the vertex pass:
  * break up the frame into tiles
  * for each tile compute which lights' contribution to the pixels in the tile
  * during shading, iterate over _only_ the lights for each pixel according to its tile
* [**Help inspecting buffers in Xcode** ](https://developer.apple.com/documentation/xcode/inspecting-buffers?changes=__9)

## PBR reference implementations
* [khronos sample viewer](https://github.khronos.org/glTF-Sample-Viewer-Release/)
  - [vertex shader code](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/main/source/Renderer/shaders/primitive.vert)
  - [fragment shader code](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/main/source/Renderer/shaders/pbr.frag)
* [babylonjs](https://sandbox.babylonjs.com/)
  - [vertex shader code](https://github.com/BabylonJS/Babylon.js/blob/master/packages/dev/core/src/Shaders/pbr.vertex.fx)
  - [fragment shader code](https://github.com/BabylonJS/Babylon.js/blob/master/packages/dev/core/src/Shaders/pbr.fragment.fx)
