# Context

The first step of any `renderling` program starts with [`renderling::context::Context`][Context].

The `Context` is responsible for managing the underlying [`wgpu`][wgpu] runtime, including the
instance, adapter and queue.
It also sets up the [`RenderTarget`][RenderTarget], according to how the `Context` was created.

On that note, it's important to know that there are two main ways to create a `Context`:

1. A headless context, which renders to a texture, can be created with
  [`Context::headless`][Context#headless] or
  [`Context::try_new_headless`][Context#try_headless], depending on your error
  handling scenario.
2. A surface context, with a window (possibly from [`winit`][winit]) or a canvas
  from [`web-sys`][web-sys].

```rust, ignore
{{#include ../../crates/examples/src/context.rs:create}}
```

## Getting a frame

Another important concept is the [`Frame`][Frame]. Each time you'd like to present a new image
you must acquire a frame from the `Context` with [`Context::get_next_frame`][Context#get_next_frame]
and present it with [`Frame::present`][Frame#present].

### Presenting on WASM

When on WASM (aka running in a browser), [`Frame::present`][Frame#present] is a noop.
It's still a good idea to use it though, so you don't forget when programming in native.

### Saving the frame

You can also read out the frame to an image provided by the [`image`][image]
crate. See the [`Frame`][Frame] docs for help with the `read_*` functions.

### Frame example

```rust, ignore
{{#include ../../crates/examples/src/context.rs:frame}}
```

[Context]: {{DOCS_URL}}/renderling/context/struct.Context.html
[Context#headless]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.headless
[Context#try_headless]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.try_headless
[Context#get_next_frame]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.get_next_frame

[Frame]: {{DOCS_URL}}/renderling/context/struct.Frame.html
[Frame#present]: {{DOCS_URL}}/renderling/context/struct.Frame.html#method.present

[RenderTarget]: {{DOCS_URL}}/renderling/context/struct.RenderTarget.html

[image]: https://crates.io/crates/image
[wgpu]: https://crates.io/crates/wgpu
[winit]: https://crates.io/crates/winit
[web-sys]: https://crates.io/crates/web-sys
