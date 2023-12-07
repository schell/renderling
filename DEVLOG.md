# devlog

## Fri Dec 8, 2023

I've been having trouble getting the new GLTF files on-the-slab method to pass my
previous tests. Mainly because of little things I had forgotten. Little bits of
state that need to be updated to run the shaders. The most recent was that the
size of the atlas needs to be updated on the GPU when the atlas changes.

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
