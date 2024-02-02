# devlog

## Thu Feb 1, 2024 

I've contributed to `rust-gpu`. 
Just a small little thing.
I added the ability to pass cargo features to the shader crate through `spirv-builder`.

## Tue Jan 27, 2024

### Raymarching!

Raymarching is totally cool and fun. I'm trying to set up an AST of SDF types but I'm
really battling the compile times. I have a theory that recursive enums slow down
compilation like crazy. Here's an example of my AST:

```rust
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct Translated {
    pub shape: Id<SdfShape>,
    pub translation: Vec3,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub enum SdfShape {
    #[default]
    None,
    Sphere(Id<Sphere>),
    Cuboid(Id<Cuboid>),
    Line(Id<Line>),
    Bezier(Id<Bezier>),
    Path(Id<Path>),
    Translated(Id<Translated>),
}

impl SdfShape {
    pub fn distance(&self, mut position: Vec3, slab: &[u32]) -> f32 {
        let mut shape = *self;
        loop {
            match shape {
                Self::None => return 0.0,
                Self::Sphere(id) => {
                    let circle = slab.read(id);
                    return circle.distance(position);
                }
                Self::Line(id) => {
                    let line = slab.read(id);
                    return line.distance(position);
                }
                Self::Bezier(id) => {
                    let bez = slab.read(id);
                    return bez.distance(position);
                }
                Self::Cuboid(id) => {
                    let rectangle = slab.read(id);
                    return rectangle.distance(position);
                }
                Self::Path(id) => {
                    let path = slab.read(id);
                    return path.distance(position, slab);
                }
                Self::Translated(id) => {
                    let translated = slab.read(id);
                    shape = slab.read(translated.shape);
                    position -= translated.translation;
                    continue;
                }
            };
        }
    }
}
```

The odd loop in `SdfShape::distance` is to avoid recursion. `rust-gpu` already complained about
that. This version took **2m 01s** to compile. I've seen it as high as **4m**. I'm going to
rewrite the AST to be a bit trickier and see how/if that helps.

If I change to this representation:
```rust
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct Translated {
    pub shape: Id<SdfShape>,
    pub translation: Vec3,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
#[repr(u32)]
pub enum ShapeType {
    #[default]
    None,
    Sphere,
    Cuboid,
    Line,
    Bezier,
    Path,
    Translated,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct SdfShape {
    pub shape_type: ShapeType,
    pub shape_id: u32,
}

impl SdfShape {
    pub fn from_sphere(id: Id<Sphere>) -> Self {
        Self {
            shape_type: ShapeType::Sphere,
            shape_id: id.inner(),
        }
    }

    pub fn from_cuboid(id: Id<Cuboid>) -> Self {
        Self {
            shape_type: ShapeType::Cuboid,
            shape_id: id.inner(),
        }
    }

    pub fn from_line(id: Id<Line>) -> Self {
        Self {
            shape_type: ShapeType::Line,
            shape_id: id.inner(),
        }
    }

    pub fn from_bezier(id: Id<Bezier>) -> Self {
        Self {
            shape_type: ShapeType::Bezier,
            shape_id: id.inner(),
        }
    }

    pub fn from_path(id: Id<Path>) -> Self {
        Self {
            shape_type: ShapeType::Path,
            shape_id: id.inner(),
        }
    }

    pub fn from_translated(id: Id<Translated>) -> Self {
        Self {
            shape_type: ShapeType::Translated,
            shape_id: id.inner(),
        }
    }

    pub fn distance(&self, mut position: Vec3, slab: &[u32]) -> f32 {
        let mut shape = *self;
        loop {
            match shape.shape_type {
                ShapeType::None => return 0.0,
                ShapeType::Sphere => {
                    let circle = slab.read(Id::<Sphere>::from(shape.shape_id));
                    return circle.distance(position);
                }
                ShapeType::Line => {
                    let id = Id::<Line>::from(shape.shape_id);
                    let line = slab.read(id);
                    return line.distance(position);
                }
                ShapeType::Bezier => {
                    let id = Id::<Bezier>::from(shape.shape_id);
                    let bez = slab.read(id);
                    return bez.distance(position);
                }
                ShapeType::Cuboid => {
                    let id = Id::<Cuboid>::from(shape.shape_id);
                    let rectangle = slab.read(id);
                    return rectangle.distance(position);
                }
                ShapeType::Path => {
                    let id = Id::<Path>::from(shape.shape_id);
                    let path = slab.read(id);
                    return path.distance(position, slab);
                }
                ShapeType::Translated => {
                    let id = Id::<Translated>::from(shape.shape_id);
                    let translated = slab.read(id);
                    shape = slab.read(translated.shape);
                    position -= translated.translation;
                    continue;
                }
            };
        }
    }
}
```

It compiles in **1m 37s**. That's an improvement, but it's still too long to be productive.

...le sigh.

### Compile times

I'm going to have to really dig into this soon as the times are just grueling. Here's a log of them:

- `1m 37s`
- `1m 37s`

## Tue Jan 23, 2024

I've been extending the use of SDFs. They are now in 3d.

Hit another weird snag last night where `rust-gpu` won't generate my PBR shader:

```
   Compiling renderling-shader v0.1.0 (/Users/schell/code/renderling/crates/renderling-shader)
error: cannot declare renderling_shader_pbr::pbr_fragment as an entry point
   --> /Users/schell/code/renderling/crates/renderling-shader-pbr/src/lib.rs:301:8
    |
301 | pub fn pbr_fragment(
    |        ^^^^^^^^^^^^
```

I just wish it would tell me _why_ it can't declare the function as an entry point.

Nobody is talking in the `rust-gpu` discord channel so to debug this I'll have to descend
into the depths of the compiler...

...I figured it out! The problem was that I was using my PBR shader entry point function
in my uber-shader entry point function. Apprently you **cannot** nest entry points within
each other.

## Fri Jan 19, 2024

Last night I successfully rendered a font face using 2d SDF path objects (lines and quadratic
Beziers):

!['!' char rendered with SDFs](test_img/sdf/filled_bez_path_font_face/!.png)
!['%' char rendered with SDFs](test_img/sdf/filled_bez_path_font_face/percent.png)
!['@' char rendered with SDFs](test_img/sdf/filled_bez_path_font_face/@.png)
!['a' char rendered with SDFs](test_img/sdf/filled_bez_path_font_face/a.png)
![closing bracket char rendered with SDFs](test_img/sdf/filled_bez_path_font_face/close_brace.png)

I'm not sure of the performance characteristics of the path shader yet, so we'll have to see
if it holds up well enough to render these paths at runtime or if they'll have to be cached
as textures.

### SDFs

SDFs have turned out to be rather magical and a lot of fun! I think I'll be using them more
often.

### Crabslab

I got my first pull request on `crabslab` yesterday from @cybersoulk (we both talk in Embark's
`rust-gpu` channel). Thanks for the PR!

I did notice while working on the SDF font rendering that resizing the slab between renders seemed
to cause issues - I'll have to look into it and write some tests.

## Sat Jan 13, 2024

`renderling` can now render 2d signed distance fields including circles/points, lines,
bezier curves, rectangles and paths of line+bezier items.

It's my plan to next use point-in-polygon (modified to include quadratic beziers) to
determine if a point lies inside or outside the path, which would allow us to properly
fill the path downstream.

Ultimately I'd like to be able to convert TTF/OTF fonts to path outlines for resolution
independent rendering.

Oh and [Inigo Quilez](https://iquilezles.org) is my new hero!

### Useful links

* https://iquilezles.org/articles/distfunctions2d/
* https://stackoverflow.com/questions/68178747/fast-2d-signed-distance


## Mon Jan 8, 2024

I added another ty var to `crabslab::Offset` to help with pointer math.

I've also added yet another layer of indirection around rendering.
Now the top level unit of rendering is `Rendering`, which is an enum of
`Id`s that point to different renderable things. There's an uber-vertex-shader
that tracks this and proxies to the correct sub-shader. This is in anticipation
of adding SDF rendering.

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
