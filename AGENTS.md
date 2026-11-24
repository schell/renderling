# AGENTS.md

## Build & Test
- Build: `cargo build -p renderling`
- Test all: `cargo nextest run -j 1` or `cargo test`
- Single test: `cargo test <test_name>` or `cargo nextest run <test_name>`
- Lint: `cargo clippy`
- Shaders: `cargo shaders` (compile), `cargo linkage` (generate WGSL)

## Code Style
- Max line width: 100 chars
- Imports: group by crate (`imports_granularity = "crate"`), std → external → internal
- Error handling: use `snafu` crate with `#[derive(Debug, Snafu)]`
- Naming: `snake_case` functions/modules, `PascalCase` types, `with_*` builder methods
- CPU-only code: wrap with `#[cfg(cpu)]`

Always format with `cargo +nightly fmt`.

The fmt toolchain is pinned to `nightly-2026-10-10` via `RUST_NIGHTLY_TOOLCHAIN` in `.github/workflows/push.yaml`. Use that toolchain locally too; bump it deliberately (reformat + regenerate shaders in the same PR).

## Disallowed Methods (clippy.toml)
Avoid: `Vec{2,3,4}::normalize_or_zero`, `Mat4::to_scale_rotation_translation`, `f32::signum`

## Beads

The bead prefix for this project is `rling`.

## Testing

This repo does a lot of generating images and comparing them to saved "golden" images.
To do this, use the `img_diff::assert_*` functions. They'll generate convenient diffs
in failure cases, which you can then inspect from the `test_output` directory.
