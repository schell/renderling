# devlog

## Wed Apr 24, 2024 🎉

NLnet is officially sponsoring the development of `renderling`! 

In fact, the project was specifically mentioned in 
[their announcement](https://nlnet.nl/news/2024/20240417-announcing-projects.html), 
which feels good.

[Here is the renderling project overview on NLnet](https://nlnet.nl/project/Renderling/).

Now I've got to get on my project organization and write up some documents, etc, then I 
can get started adding atomics to `naga`, and unblock `renderling`'s occlusion culling 
and light tiling steps (they will be `rust-gpu` compiled compute shaders, but they require 
support for atomics, which `wgpu` currently lacks).

## Tue Apr 9, 2024

### Better debugging

Debugging on the CPU is great - it finds a lot of bugs relatively quickly.
It's no silver bullet though, because often the size of types are different on the GPU,
and the implementations of functions are different as well. 

To some bugs, debugging on the GPU is necessary - but without special features and some 
Vulkan layer magic (that are unavailable to `wgpu` at the time of this writing), 
debugging is pretty hard.

So I'm experimenting with writing my shaders to take an extra `debug: &mut [u32]` buffer 
that it can use to write messages into. So far it works great in my vertex shader, but 
the same setup (with a separate buffer) doesn't work on my fragment shader. I still don't 
know why. So now I'm debugging my debugging. 

For help I've posted on: 
- [GP (graphics programming) discord](https://discord.com/channels/318590007881236480/591343919598534681/1227041127899729970) 
- [rust-gpu discord](https://discord.com/channels/750717012564770887/750717499737243679/122701598544219355) for help...

#### ...

It seems that in fact, the values are being written, but when I read them out - I only 
get a few...

Wait.

Oh wait.

_smh_

The vertex shader only covers _certain fragments_. 

The fragment shader would only evaluate those pixels covered by the vertex shader.

🤦

So everything is as it should be.

...Hooray! Sheesh.

Ok, debugging messages work.

Now - if I had atomics I could make this pretty ergonomic.

## Sat Apr 6, 2024 

### Finishing the debugging session

It WAS `crabslab`! Specifically it was `Slab::contains`, which is used to check that 
a type with an `Id` _can be read_. 

Previously the definition was: 

```rust 
fn contains<T: SlabItem>(&self, id: Id<T>) -> bool {
    id.index() + T::SLAB_SIZE <= self.len()
}
```

Which seems correct, and it functions correctly on the CPU. 
But on the GPU (meaning `target_arch = "spirv"``) `usize` is a 32bit `u32`, 
and so the `id.index() + T::SLAB_SIZE` will overflow if the id is `Id::NONE`, 
because `Id::NONE = u32::MAX;`. 

Indeed, the id is often `Id::NONE`, as that is the default!
This was causing a silent panic in my shader, which then produced no output.

Now the definition is this: 
```rust
fn contains<T: SlabItem>(&self, id: Id<T>) -> bool {
    self.len() >= T::SLAB_SIZE && id.index() <= self.len() - T::SLAB_SIZE
}
```

What a hard-to-diagnose bug! I really need trace statements on GPU.

## Fri Apr 5, 2024 

I have bugs after removing the SDF raymarching stuff. 

Primarily I can't get any of my `stage_vertex`+`stage_fragment` tests passing. 
Everything is blank. 

### Debug with me!

* it's not crabslab: I fixed some bugs in it and after testing through the `tutorial`
  shaders I'm 80% certain it's not a (de)serialization problem.
* NOTHING is being written to the depth texture...
  - depth is cleared to 1.0 
  - pipeline depth function is set to ALWAYS (always succeed) and still nothing is written
  - face culling is off and still nothing is written
  - running the vertex shader on CPU and printing out clip positions shows:
    ```
    clips: [
        Vec4(
            -1.0,
            1.0,
            0.25,
            1.0,
        ),
        Vec4(
            -1.0,
            -1.0,
            0.25,
            1.0,
        ),
        Vec4(
            1.0,
            1.0,
            0.25,
            1.0,
        ),
    ]
    ```
    Which is a CCW triangle up in the top left of the clip space. 
    So we should see SOMETHING in the depth texture at least, but we don't.
    Why do we not? Is the render even happening on the GPU? Let's check logging
    to see if we're issuing the calls..
      - `stage_render` prints `drawing vertices 0..3 and instances 147..148`
        so I'm certain we're actually rendering.

At this point I'm a bit at a loss. The difference between the tutorial shaders (which are working) 
and my stage shader is mainly that the stage shader first writes to an HDR surface and then 
runs tonemapping and writes the result to the frame surface. I can't see any other course of action 
than removing HDR and tonemapping to see if that works.

I'm going to try rendering the `gltf_cmy_tri` slab with the `tutorial` shaders. 
We'll see what happens.

NOTHING! No rendering. No depth values. So this must have something to do with the data.

### What to do about the wacky GLTF stage shaders

I'm going to go back to a much simpler "renderlet" pbr shader program. The entire GLTF document can 
still live on the GPU, but GLTF is too complicated a structure to use for the internal representation.

## Mon Feb 26, 2024 

### SDF stack lang on GPU considered harmful

I think anyone with a good working knowledge of GPUs could have predicted that evaluating 
a stack language on a GPU would turn out poorly. 

Of course I don't quite fit into that aformentioned group, yet, and so I had to find this 
out for myself. 

I think the problem is roughly that: 

* The SDF raymarching shader performs raymarching until an SDF is hit
  - includes evaluating the stack of each SDF in the scene and taking the min (obviously could 
    use some kind of BVH)
    - stack evaluation is a loop with branching
* Because of this, there's no real coherence between operations in a warp

So I think what I'll try next is completely ditching the stack lang and representing my SDFs 
analytically on the CPU, and then "serializing" them to the GPU as pre-baked distance volumes. 

[There's at least a little prior art.](https://gpuopen.com/gdc-presentations/2023/GDC-2023-Sparse-Distance-Fields-For-Games.pdf)

Or maybe I'll just park SDFs for now and get back to rasterization...


### SDFs going forward+

I still think there's a way to make SDFs work well in _this_ project. Consider this rasterization factory:

1. break down triangle geometry into meshlets 
2. determine draw calls for meshlets
3. draw calls run vertex shader for meshlets
4. fragment shader writes to gbuffer, which might include
   - u32 id of object
   - vec3 position 
   - vec3 normal  
   - vec4 albedo 
   - f32 metallic 
   - f32 roughness 
   - vec3 emissive 
   - vec3 irradiance 
   - vec3 prefiltered 
   - vec2 brdf
   - f32 depth
5. break down SDFs into volumes 
6. determine draw calls for SDF volumes 
7. draw calls run vertex shader for volumes
8. SDF volume fragment shader writes to gbuffer
9. do a _raymarching_ pass over the gbuffer, discarding fragments not drawn to, using the depth as the first already-known hit point
   1. march till bounce...
   2. accumulate 
   3. goto 1. or die

But I guess step `9` can be omitted until a later date. Support for rasterizing SDFs is the main point. 

I like the idea of raymarching for shading, though. It seems cognitively simpler than the current pile of tricks...


### Wgsly

After hitting those exponential compile times with `rust-gpu` 
(and also thinking ahead to `naga`'s lack of atomics support), I made a quick foray into embedding 
WGSL into Rust using procedural macros.

There's no quick and easy way to mate `naga`'s IR with `syn`'s AST parsing, so I stopped once I 
realized I would have to implement `syn::parse::Parse` for the entirety of WGSL by hand. 

It's not an insane amount of work though, and it would give nice editor tooling for any IDE that
has it for Rust. Plus you could use macros to implement imports for WGSL....

Anyway - I'm going to pull it out because it's not really on topic.

### Crabslab update 

I bumped `crabslab` after updating that library to use an associated constant for the slab size.

The file sizes are a tad bit smaller now, but only by at most 100 bytes.

## Fri Feb 23, 2024

### Wavefront path tracing 
@eddyb recommended I read  [Megakernels Considered Harmful: Wavefront Path Tracing on GPUs](convolution__fragment_generate_mipmap).
It's a cool paper about breaking up monolithic ray tracing shaders into microkernal steps.

There are also some breakdown posts about it: 

- [https://jacco.ompf2.com/2019/07/18/wavefront-path-tracing/](https://jacco.ompf2.com/2019/07/18/wavefront-path-tracing/)

## Thu Feb 22, 2024

### NLNet progress 

Michiel Leenaars reached out from NLNet on the 17th about my proposal. 
It's been selected to enter the second round of the December 2023 call. 🤞

### Exponentials 

@eddyb has been drilling down into the exponential compile-time and file sizes caused by certain type-nesting scenarios in `rust-gpu`.
It seems like he's found the cause(s) and has a couple ideas on how to fix it.
[Get up to speed on the discord thread here](https://discord.com/channels/750717012564770887/1204616011475849296/1209826103502315520).

### Feature gate the shaders

I'm feature gating all the shaders, that way I can build only specific shaders by using `--no-default-features` + `--features {the_shader}`.

## Wed Feb 7, 2024

### Filesize and compile time woes 

Lots of discussions about file sizes on the `rust-gpu` discord [starting here](https://discord.com/channels/750717012564770887/750717499737243679/1204153056191848618).
Long story short (go read that thread if you want the long story), inlining happens in a big way in the `rust-gpu` compiler, and my code got hit hard. 
I was able to reduce the `.spv` filesize of one of my shaders over 50% (from 731kb to 304kb) and the compile time by 85% (266s to 40s) simply by converting six calls of one function into a for loop 6 times over one function call.

I'm also going to audit the `crabslab` API to attempt to reduce filesizes. 

### SlabItem `read_slab` audit

I have a minimal `crabslab` based shader that reads some structs off a the slab. 
It clocks in at 9756 bytes. 

I also have a baseline shader that does the same by hand, without the `SlabItem` trait.
It weighs in at 4352 bytes. 

So - just including `crabslab` here increases the `.spv` filesize by 124%!

#### Rundown

* including `Id` and `Array` doesn't change the filesize
* including `SlabItem` increases it to 4688 bytes, a 7% increase.
  - using `fn read_slab(&mut self, id: u32, slab: &[u32]) -> u32` is how we get to 4688 bytes
  - using `fn read_slab(id: u32, slab: &[u32]) -> (u32, Self);` increases it to 4884 bytes
  - using `fn read_slab(id: u32, slab: &[u32]) -> Self;` reduces it to 4628 bytes

After rewriting the `read_slab` fn to `fn read_slab(id: u32, slab: &[u32]) -> Self;` the minimal 
`crabslab` based shader is only 4576 bytes, which is only 5% larger than the baseline and 53% 
smaller than the previous. We'll see how much smaller my shaders get as a result.

### Filesize / compilation time audit result 

After changing the slab reading API, bumping crabslab in `renderling` and recompiling my shader 
the filesize was further reduced another 40% - from 304kb to 182kb.
Compilation time reduced a further 35% - from 40s to 26s!

So the total reduction in filesize is 75% - from 731kb to 182kb.
Total reduction in compilation time is 90% - from 266s to 26s!

What a difference a little tangential work and profiling can make!

## Sun Feb 4, 2024 

Oof, I miss recursion.

In the absence of recursion I'm working on a little stack language evaluator that will 
evaluate the distance to surfaces using signed distance functions. I figure if it works 
well I could use it for both raymarching the distance and evaluating the color/material 
of the object.

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
