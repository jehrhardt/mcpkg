# Agent Guidelines for mcpkg

## Build & Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format check: `cargo fmt -- --check`
- Format: `cargo fmt`

## Code Style
- Edition: Rust 2024
- Format: Use `rustfmt` defaults (run `cargo fmt` before committing)
- Linting: All clippy warnings must be treated as errors (`-D warnings`)
- Imports: Group std, external crates, then local modules; alphabetical within groups
- Naming: snake_case for functions/variables, PascalCase for types/traits
- Error handling: Use `Result<T, E>` with proper error types, avoid unwrap in library code
- Documentation: Add doc comments (`///`) for public APIs

## CI Requirements
All PRs must pass:
1. `cargo fmt -- --check` (formatting)
2. `cargo clippy -- -D warnings` (linting)
3. `cargo test` (tests)
