# Agent Guidelines for Twig

## Build & Test Commands
- **Build**: `cargo build` or `cargo build --release`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_name` (supports partial matching)
- **Lint**: `cargo clippy -- -D warnings` (warnings treated as errors in CI)
- **Format**: `cargo fmt` (check: `cargo fmt -- --check`)
- **Run**: `cargo run -- start` (MCP server via stdio)
- **Dev tools**: `mise run dev:mcp` (MCP inspector), `mise run dev:page` (docs site)

## Code Style & Conventions
- **Edition**: Rust 2024
- **Imports**: Group stdlib, external crates, then local modules; use `use crate::` for internal imports
- **Formatting**: Use `cargo fmt` - follows default rustfmt rules
- **Naming**: snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
- **Error handling**: Use `Result<T, ErrorData>` for MCP handlers; `.expect()` with descriptive messages for setup code
- **Async**: Use `#[tokio::main]` and `async fn` for async entry points; tokio runtime with "full" features
- **Visibility**: Use `pub(crate)` for internal APIs, avoid `pub` unless truly public
- **Types**: Prefer explicit types in function signatures; leverage type inference in function bodies

## Architecture
- **MCP server**: Uses `rmcp` crate with stdio transport for Model Context Protocol
- **CLI**: clap v4 with derive macros for command parsing
- **Modules**: `cli` (command parsing), `mcp` (MCP server logic), `main` (entry point)

## Active Technologies
- Rust 2024 Edition (001-user-prompts-mcp)
- File-based (user data directory with TOML config + markdown prompts) (001-user-prompts-mcp)

## Recent Changes
- 001-user-prompts-mcp: Added Rust 2024 Edition
