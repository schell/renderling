# Analytical lights

Analytical lighting in real-time rendering refers to the use of mathematical
models to simulate the effects of light on surfaces. 

What that means in `renderling` is that analytical lights are the lights that
you create, configure and place programmatically, one by one, into the scene.

To do any lighting, though, we have to turn lighting back on in the stage:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:lighting_on}}
```

![image of a marble bust in shadow](/assets/lighting/no-lights.png)

As we talked about in [the GLTF example](/gltf.html#render), with no lights
on the stage, the bust renders in shadow.

Now we're ready to add some lights.

## Directional lights

Directional lights simulate light coming from a specific direction, like
sunlight. They affect all objects in the scene equally, regardless of their
position, and do not diminish with distance. This makes them ideal for
simulating large-scale lighting effects.

Let's create a directional light:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:directional}}
```

![image of a marble bust lit by a single directional light](/assets/lighting/directional.png)

Not bad!

Before moving on we'll remove the directional light:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:remove_directional}}
```

Dropping the light isn't strictly necessary, except to reclaim the resources.

## Point lights

Point lights emit light equally in all directions from a single point in space,
similar to a light bulb. They are ideal for simulating localized light sources
and their intensity diminishes with distance, following the inverse square law.
This makes them suitable for creating realistic lighting effects in small areas.

Let's create a point light:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:point}}
```

![image of a marble bust lit by a single point light](/assets/lighting/point.png)

Similarly we'll remove the point light before moving on:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:remove_point}}
```

## Spot lights

Spot lights emit a cone of light from a single point, with a specified direction
and angle. They are useful for highlighting specific areas or objects in a
scene, such as a spotlight on a stage. The intensity of a spot light diminishes
with distance and is also affected by the angle of the cone, allowing for
precise control over the lighting effect.

Let's create a spotlight. One thing about spotlights though, they can be a bit fiddly
due to having a position, direction _and_ inner and outer cutoff values. For this reason
we'll place the spotlight at the camera's position and point it in the same direction, so
you can see the effect:

```rust,ignore
{{#include ../../../crates/examples/src/lighting.rs:spot}}
```

![image of a marble bust lit by a single spot light](/assets/lighting/spot.png)

Good enough! Now on to image based lighting.
