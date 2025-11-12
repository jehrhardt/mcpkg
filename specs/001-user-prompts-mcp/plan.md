# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

**Language/Version**: Rust 2024 Edition  
**Primary Dependencies**: 
  - `rmcp`: MCP server framework (stdio transport)
  - `minijinja`: Jinja2 template rendering for prompt content
  - `dirs`: Platform-specific data directory resolution
  - `toml`: TOML parsing for twig.toml library config files
  - `tokio`: Async runtime with "full" features
  
**Storage**: File-based (user data directory with TOML config + markdown prompts)  
**Testing**: 
  - Unit tests: `cargo test` as inline submodules in source files (library.rs, prompt.rs, data_dir.rs)
  - Integration tests: MCP server via stdio transport with client connection in `tests/` directory
  - Environment variable `TWIG_DATA_DIR` for test temp directories
  
**Target Platform**: Linux, macOS, Windows (cross-platform via dirs crate)  
**Project Type**: Single CLI application with MCP server capability  
**Performance Goals**: List prompts within 2s, retrieve content within 500ms (per spec SC-001, SC-002)  
**Constraints**: 100% library discovery success, clear error messages for configuration issues  
**Scale/Scope**: Support unlimited number of prompt libraries without degradation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Principle I - Zero-Tolerance Code Quality**: ✅ PASS
- All code will pass `cargo clippy -- -D warnings`, `cargo fmt --check`, and full test suite
- Release build with `--release` will succeed
- No exceptions for MCP handlers or library scanning code

**Principle II - Test-First Development**: ✅ PASS
- Unit tests as inline submodules in tested Rust files (library.rs, prompt.rs, data_dir.rs)
- Integration tests in `tests/` directory for MCP server startup via `cli::run()` with stdio client
- Tests must independently verify each requirement (list, get, error handling)
- Test naming follows `test_<function>_<scenario>_<expected_outcome>` pattern

**Principle III - Explicit Over Implicit**: ✅ PASS
- All MCP handler signatures use explicit types with `Result<T, ErrorData>`
- Library discovery returns structured error messages
- Avoid unwrap() in favor of proper error handling
- Document template rendering behavior and library normalization rules

**Principle IV - Developer Experience Consistency**: ✅ PASS
- Use `mise` as defined in `mise.toml` for environment management
- Commands documented in `AGENTS.md`
- Standard project structure: src/ for source, tests/ for integration tests
- Dependencies locked in Cargo.lock

**Principle V - Small, Focused Modules**: ✅ PASS
- Module structure: `cli` (command parsing), `mcp` (MCP server), separate module for library discovery
- Use `pub(crate)` for internal APIs
- File length limits respected (500 lines max)
- Single responsibility: library loading, template rendering, MCP handlers kept separate

**All principles satisfied. Proceed to Phase 0.**

## Project Structure

### Documentation (this feature)

```text
specs/001-user-prompts-mcp/
├── plan.md              # This file (implementation plan)
├── research.md          # Phase 0 output (research findings)
├── data-model.md        # Phase 1 output (data entities & validation)
├── quickstart.md        # Phase 1 output (usage examples)
├── contracts/           # Phase 1 output (API contracts)
└── tasks.md             # Phase 2 output (task breakdown - NOT created by plan)
```

### Source Code (Rust single-project structure)

```text
src/
├── main.rs              # Entry point, tokio runtime setup
├── cli.rs               # Command parsing (clap v4)
├── mcp.rs               # MCP server handler (ServerHandler impl)
└── lib.rs (or module)   # Core logic modules:
    ├── library.rs       # Library discovery, TOML parsing
    ├── prompt.rs        # Prompt loading and template rendering
    └── data_dir.rs      # Platform data directory resolution

tests/
├── integration_test.rs  # MCP server stdio integration tests
└── [other integration test files]

Unit tests (inline):
- tests module at bottom of library.rs, prompt.rs, data_dir.rs
```

**Structure Decision**: Single Rust project (no workspace needed). Core logic modularized:
- `library.rs`: Scan data directory, parse twig.toml, normalize library names
- `prompt.rs`: Load markdown files, render with minijinja templates, handle arguments
- `data_dir.rs`: Resolve platform-specific directory via dirs crate, respect TWIG_DATA_DIR env
- `mcp.rs`: MCP ServerHandler implementation for list_prompts/get_prompt
- Integration tests in `tests/` connect via stdio and verify full server behavior

## Clarifications

### Session 2025-11-12

- Q: twig.toml schema: array-of-tables `[[prompts]]` vs nested tables `[prompts.code_review]`? → A: Use nested table format `[prompts.prompt_name]` (Option B - minimal, prompt-focused)
- Q: Should `required` field default to false for optional arguments? → A: Yes, omit `required` field to default to `false` (Option A)
- Q: Should argument `description` be required or optional? → A: Optional - can be omitted and treated as None/null (Option B)
- Q: Support both simple-list and nested-table formats, or only nested? → A: Only nested tables - single consistent schema (Option B)

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
