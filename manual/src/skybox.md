# Skybox 🌌

One of the most striking effects we can provide is a
[skybox](https://en.wikipedia.org/wiki/Skybox_(video_games)).

Using a skybox is an easy way to improve immersion, and with
`renderling` your skyboxes also illuminate the scene.

## Building on the stage example 

We'll start out this example by extending the example from the
[stage](./stage). In that example we created a cube with colored
vertices:

```rust,ignore
{{#include ../../crates/examples/src/skybox.rs:setup}}
```

Then we rendered:

```rust,ignore
{{#include ../../crates/examples/src/skybox.rs:render_cube}}
```

Which gave us this scene:

![image of a unit cube with colored vertices](assets/stage-example.png)

## Adding the skybox

In `renderling`, skyboxes get their background from an "HDR" image.
These are typically large three dimensional images. You can find
free HDR images [here](https://polyhaven.com/hdris) and other
places around the web. 

For this example we'll be using this HDR:

![Qwantani Dusk 2 (Pure Sky)](assets/qwantani_dusk_2_puresky.webp)

```rust,ignore
{{#include ../../crates/examples/src/skybox.rs:skybox}}
```

Then we render:

```rust,ignore
{{#include ../../crates/examples/src/skybox.rs:render_skybox}}
```


## Results

And there we go!

![renderling skybox](assets/skybox.png)
