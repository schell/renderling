<div align="center">
    <img src="crabslab.png" alt="slabcraft for crabs" width="512" />
</div>

## what
`crabslab` is a slab implementation focused on marshalling data from CPUs to GPUs.

## why
### Opinion
Working with shaders is much easier using a slab.

### rust-gpu
This crate was made to work with [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu/).
Specifically, using this crate it is possible to pack your types into a buffer on the CPU
and then read your types from the slab on the GPU (in Rust).

### Other no-std platforms
Even though this crate was written with `rust-gpu` in mind, it should work in other `no-std`
contexts.

## how
`crabslab` includes:
* a few traits:
  - `Slab`
  - `GrowableSlab`
  - `SlabItem`
* a derive macro for `SlabItem`
* a few structs for working with various slabs
  - `Id`
  - `Array`
  - `Offset`
* a helper struct `CpuSlab`
* a feature-gated helper for using slabs with `wgpu` - `WgpuBuffer`
  - [example](src/wgpu_slab.rs#L344)
