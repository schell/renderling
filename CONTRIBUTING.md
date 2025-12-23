# Contributing to Renderling

Thank you for your interest in contributing to Renderling!

## Code of Conduct

This project follows the [Zcash Code of Conduct](https://github.com/zcash/zcash/blob/master/code_of_conduct.md).
We are committed to providing a welcoming and harassment-free experience for everyone.

## NLnet Generative AI Policy

This is an NLnet-funded project. We adhere to the [NLnet Generative AI Policy](https://nlnet.nl/foundation/policies/generativeAI/).
If you use generative AI tools like LLMs, code assistants, etc. in your contributions, you must:
- Disclose any substantive use
- Maintain a prompt provenance log for material contributions
- Ensure outputs can be legally published under FLOS licenses
- Not present AI-generated content as your own human-authored work

### When it comes to AI - use your best judgment

I use AI to help plan a strategy and then make changes by hand.

What I want to avoid is a situation where copyrighted material from an LLM's training corpus
makes it into the codebase.

So please disclose if the _outputs_ of generative AI is what is being committed.

## Getting Started

1. Fork and clone the repository
2. Install Rust via [rustup](https://rustup.rs)
3. Install `cargo-gpu`: `cargo install --git https://github.com/rust-gpu/cargo-gpu cargo-gpu`
4. Optionally install `cargo-nextest`: `cargo install cargo-nextest`

## Development Workflow

1. Create a branch for your changes
2. Follow the code style guidelines in [AGENTS.md](AGENTS.md)
3. Run tests: `cargo nextest run -j 1` or `cargo test`
4. Run lints: `cargo clippy`
5. If modifying shaders: `cargo shaders && cargo linkage`
6. Ensure there are no unexpected diffs: `git diff`
7. Submit a pull request

## Discussions

Questions, ideas, and general discussion should happen on
[GitHub Discussions](https://github.com/schell/renderling/discussions).

## Testing

Tests render images in headless mode and compare against reference images in `test_img/`.
New visual features should include image comparison tests where applicable.

## License

By contributing, you agree that your contributions will be dual-licensed under
the MIT and Apache 2.0 licenses. See [LICENSE](LICENSE) for details.
