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
