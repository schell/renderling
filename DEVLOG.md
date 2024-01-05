# devlog

## Fri Jan 5, 2024

The slab implementation in this repo has been spun off into its own thing.
[`crabslab`](https://github.com/schell/crabslab) is now live!

## Sometime around new years?

I removed the bloom implementation completely.
It will be reimplemented later as a physically-based bloom.

## Sat Dec 23, 2023

I've ported over a majority of the tests to the GLTF-on-the-slab implementation.
I'm currently working on the big PBR test and having trouble with the skybox, which
is rendering all black...

Debugging rabbit hole:
* So is it even running?
  - Yes, logging shows that it's running.
* Could it be it needs to be run in its own render pass?
* Before I even check that, I see that the skybox's vertex shader uses the `instance_index` as the `Id` of the camera, and I'm passing `0..1` as the instance range in the draw call.
  - So we need a way to pass the camera's `Id` to the skybox.
    - I just added it as a field on `Skybox`
    - Using that new field fixed that issue. Now I have an issue with bloom.

After fixing the skybox rendering it seems bloom isn't running.

Debugging rabbit hole:
* So is it even running?
  - Yes, logging shows that it's running.
* Is the result being used downstream during tonemapping?
  - It seems to be.
* Let's check to see that there isn't something funky when configuring the graph.
  - Nothing I can tell there.
* Maybe print out the brightness texture and make sure it's populated?
* Losing steam here, especially since bloom needs to be re-done as "physically based".

### Physically Based Bloom

## Thu Dec 21, 2023

It's the solstice! My Dad's birthday, and another bug hunt in `renderling`.

### Porting gltf_images test
The test `gltf_images` tests our image decoding by loading a GLTF file and then
creating a new staged object that uses the image's texture.

It's currently coming out all black, and it should come out like
![gltf_images test](test_img/gltf_images.png).

I recently got rid of the distinction between "native" vertex data and GLTF vertex
data. Now there is only GLTF vertex data and the "native" `Vertex` meshes can be
conveniently staged (marshalled to the GPU) using a helper function that creates
a `GltfPrimitive` complete with `GltfAccessors` etc.

Debbuging rabbit hole:
* Let's compare old vs new vertex shaders
  - It doesn't seem to be the vertices, because the staged vertices (read from the GPU) are equal to the original mesh.
  - The staged vertices are equal to the original CPU-side mesh, but the computed vertex values are different from legacy.
    - It looks like transforms on `RenderUnits` are not transforming their child primitive's geometry
      - Got it! It was because `GltfNode`'s `Default` instance was setting `scale` to `Vec3::ZERO`.

## Wed Dec 20, 2023

I think I'm going to keep going with this idea of making GLTF the internal representation of the
renderer.

## Tue Dec 19, 2023

### Thoughts on GLTF
GLTF on-the-slab has been a boon to this project and I'm tempted to make it the main way we do
rendering. I just want to write this down somewhere so I don't forget. Currently when loading
a GLTF file we traverse the GLTF document and store the whole thing on the GPU's slab. Then
the user has to specify which nodes (or a scene) to draw, which traverses one more time, linking
the `RenderUnit`s to the primitives within the GLTF. I _think_ it might be cognitively easier
to have GLTF nodes somehow be the base unit of rendering ... but I also have plans for supporting
SDFs and I'm not sure how that all fits together.

* [At least one other person is thinking about putting SDFs in GLTF using an extension](https://community.khronos.org/t/signed-distance-field-representation-of-geometry-extension/109575)

Anyway - I'll keep going with the momentum I have and think about refactoring towards this in the future.

## Mon Dec 18, 2023

### Simple Texture GLTF Example
* The `simple_texture` test is rendering the texture upside-down.
* There are _no rotation transformations_ in its node's hierarchy.
* What does the atlas look like?
  - It's not the atlas, the two tests (slabbed and the previous non-slabbed) have
    identical atlas images.
* So what about UV coords?
  - Comparing runs of the vertex shaders shows that the UV coords' Y components are flipped.
  - So, 0.0 is 1.0 and 1.0 is 0.0
* So is there something doing this intentionally?
  - Nothing that I can easily see in the `gltf_support` modules...
  - It has something to do with the accessor.
  - I can see in the GLTF file that the accessor's byte offset is 48, but somehow in
    my code it comes out 12...
  - It was because the accessor's offset was not being taken into account.

### Analytical Directional Lights
I got analytical lighting working (at least for directional lights) on the stage.
The problem I was having was that the shaders use `Camera.position` in lighting
equations, but that was defaulting to `Vec3::ZERO`. Previously in the "scene"
version of the renderer (which I'm porting over to "stage") the camera's position
was set automatically when setting the projection and/or view.
I had to run both versions of the vertex AND fragement shaders to track this down. Ugh!

## Fri Dec 8, 2023

I've been having trouble getting the new GLTF files on-the-slab method to pass my
previous tests. Mainly because of little things I had forgotten. Little bits of
state that need to be updated to run the shaders. The most recent was that the
size of the atlas needs to be updated on the GPU when the atlas changes.

I'm moving over tests from `renderling/scene/gltf_support.rs` to
`renderling/stage/gltf_support.rs` one at a time.

## Thu Dec 7, 2023

Ongoing work to get GLTF files on-the-slab working. When this work is done GLTF
file imports should be lightening fast.

## Wed Nov 15, 2023

I resubmitted the NLNet grant proposal with expanded scope to take care of [the
`naga` atomics issue](https://github.com/gfx-rs/naga/issues/2301).

## Sat Nov 11, 2023

### NLNet Grant Progress

I made a lot of progress on a grant from NLNet to work on renderling/naga.
Ultimately I missed the funding deadline after expanding the scope of work a bit,
but they encouraged me to apply for the December 1st 2023 round. I'll be working on
that over the next few weeks and hopefully can start diving into that work in
Q2 2024.

### Slab

I'm transitioning from using one GPU buffer for each array of items (Vertices, Entities, etc)
to using one or two for the whole system, based on a bespoke slab-allocator.

## Mon Sep 4, 2023

I bumped `rust-gpu` to 0.9.
There was an issue that was preventing me from doing this earlier and I was avoiding dealing with it.
It turned out to be a pretty simple fix, though I don't actually understand _why_ it fixed it.
See the [related issue](https://github.com/EmbarkStudios/rust-gpu/issues/1089) for more info.

Quite a big refactor looms overhead. I'm going to have to really think about how to represent the geometry on the GPU, as some of my earlier assumptions about nodes/entities doesn't hold.
Specifically it seems obvious to me now that I'd like to draw duplicate nodes without duplicating the data, and also that nodes/entities may be the child of more than one parent.

## Sat Sep 2, 2023

I added WASM support! Most of the work was ensuring that the shaders validate (see below).

## Fri Sep 1, 2023

While adding WASM support I found that my shaders were not validating in the browser.
Apparently this is because of an extra traspilation step from spv -> wgsl - because when targeting WebGPU in the browser, shaders must be written in WGSL, and naga's WGSL backend doesn't like infinities or NaNs.
Here's [the related ticket](https://github.com/gfx-rs/naga/issues/2461).
I ended up having to track down all the infinity and NaN comparisons and replace the functions that have those comparisons in their call trees. I then created a clippy lint to disallow those functions.

## Fri Aug 4, 2023

I tried to bump `rust-gpu` to 0.9 but ran into [an issue](https://github.com/EmbarkStudios/rust-gpu/issues/1089) where shaders that previously compiled no longer compile.
`spirv-opt` was erring because it didn't like _something_.
I'm working with @eddyb to figure out what the problem is.
Here's a link to the start of the [conversation](https://discord.com/channels/750717012564770887/750717499737243679/1136766077330796595).

I also fixed an issue where two versions of `glam` were being built - `0.22` by `spirv-std` and `0.24` by `renderling-shader`, which was causing CI to fail.

## Thur Aug 3, 2023

I greatly reduced the artifacts in the prefiltered environment cube used for specular highlights.
I did this by using a simplified `calc_lod` and also by generating enough mipmaps.
Previously I was only making 5 mip levels but the `calc_lod` was often requiring 21+!
Of course the environment cubemap's face size is only 512, which leads to 9 mip levels total - so now I'm providing 9 mips.

I also noticed that the IBL diffuse irradiance samples were not aligned! Now the normal's Y is flipped in the irradiance convolution.

## Wed Aug 2, 2023

When generating mipmaps I ran into a problem where sampling the original texture was always coming up [0.0, 0.0 0.0, 0.0]. It turns out that the sampler was trying to read from the mipmap at level 1, and of course it didn't exist yet as that was the one I was trying to generate. The fix was to sample a different texture - one without slots for the mipmaps, then throw away that texture.

I have to generate mipmaps to smooth out the irradiance and prefiltered cubemaps that we use for pbr shading.
