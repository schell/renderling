# Setup

`renderling` is a Rust library, so first you'll need to get familiar with the
language. Visit <https://www.rust-lang.org/learn/get-started> if you're not
already familiar.

Once you're ready, start a new project with `cargo new`.
Then `cd` into your project directory and add `renderling` as a dependency:

```
cargo add --git https://github.com/schell/renderling.git --branch main
```

## patch crates.io

`renderling` is special in that all the shaders are written in Rust using
[Rust-GPU](https://rust-gpu.github.io/), which is currently between
releases. For this reason we need to add an entry to the `[patch.crates-io]`
section of our `Cargo.toml`:

```toml
[patch.crates-io]
spirv-std = { git = "https://github.com/rust-gpu/rust-gpu.git", rev = "05b34493ce661dccd6694cf58afc13e3c8f7a7e0" }  
```

This is a temporary workaround that will be resolved after the next Rust-GPU
release.

The rest is Rust business as usual.

## WASM

TODO: write about setting up a WASM project.

## Re-exports

`renderling` **re-exports** [`glam`][glam] from its top level module,
because it provides the underlying mathematical types used throughout the API.

[glam]: https://crates.io/crates/glam
