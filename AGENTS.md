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
- Tests: inline `#[cfg(test)] mod test { ... }` within modules
- CPU-only code: wrap with `#[cfg(cpu)]`

## Disallowed Methods (clippy.toml)
Avoid: `Vec{2,3,4}::normalize_or_zero`, `Mat4::to_scale_rotation_translation`, `f32::signum`
