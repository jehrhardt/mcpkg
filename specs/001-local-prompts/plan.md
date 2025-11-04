# Implementation Plan: Local Prompts Support

**Branch**: `001-local-prompts` | **Date**: 2025-11-04 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-local-prompts/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement local prompts feature to enable users to create reusable prompt templates stored in `.twig/prompts/` directory. Prompts are Markdown files with YAML frontmatter containing metadata and support Jinja template syntax for parameter substitution. The feature exposes prompts via MCP's `prompts/list` and `prompts/get` endpoints with automatic file watching to notify clients of changes.

## Technical Context

**Language/Version**: Rust 2024 edition  
**Primary Dependencies**: rmcp 0.8.2 (MCP server), NEEDS CLARIFICATION (YAML parsing, Jinja templating, file watching)  
**Storage**: File system (`.twig/prompts/` directory with Markdown files)  
**Testing**: cargo test with integration tests using rmcp client  
**Target Platform**: Cross-platform (Linux, macOS, Windows) - stdio-based MCP server  
**Project Type**: Single binary CLI application  
**Performance Goals**: 
  - Prompt discovery <100ms on server startup (SC-001)
  - Template rendering <50ms per prompt (SC-002)
  - File change detection and notification <2 seconds (SC-003)
  - Handle 100+ prompt files without degradation (SC-005)  
**Constraints**: 
  - Synchronous file I/O acceptable for prompt loading
  - File watching must work across all target platforms
  - Jinja template errors must be caught and reported gracefully
  - UTF-8 encoding required for prompt files  
**Scale/Scope**: 
  - 100+ prompt files per directory
  - Flat directory structure (no nested subdirectories)
  - Single `.twig/prompts/` directory per server instance

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Zero-Tolerance Code Quality ✅
- **Status**: PASS
- All code will be validated with `cargo clippy -- -D warnings`, `cargo fmt -- --check`, `cargo test`, and `cargo build --release`
- CI gates enforce these automatically

### II. Test-First Development ✅
- **Status**: PASS  
- User explicitly requested: "Automated integration tests should use rmcp's client to launch the MCP server and interact with it to test the proper scenarios"
- Test strategy:
  - **Integration tests**: Use rmcp client to test MCP protocol handlers (prompts/list, prompts/get, notifications)
  - **Unit tests**: Test prompt file parsing, YAML frontmatter extraction, Jinja template rendering
  - **Contract tests**: Validate MCP protocol compliance for prompt-related endpoints
- Tests will be written before implementation per Constitution Principle II

### III. Explicit Over Implicit ✅
- **Status**: PASS
- All public APIs will use explicit `Result<T, ErrorData>` for error handling
- Function signatures will declare explicit types
- No unwrap() in production code paths (only in tests)

### IV. Developer Experience Consistency ✅
- **Status**: PASS
- No new tooling required beyond existing `mise` setup
- Standard project structure maintained (`src/`, `tests/`)
- Commands remain consistent with `AGENTS.md`

### V. Small, Focused Modules ✅
- **Status**: PASS
- New functionality contained within existing `mcp` module
- Potential new internal modules: `prompts` (parsing), `templates` (Jinja rendering), `watcher` (file watching)
- All modules will use `pub(crate)` for internal APIs

**GATE RESULT**: ✅ PASS - All constitutional principles satisfied. Proceed to Phase 0 research.

---

## Post-Design Constitution Re-evaluation

*After Phase 1 design completion*

### Design Review Against Constitution

#### Module Structure ✅
- **prompts/** module with 6 files (mod, types, parser, renderer, registry, watcher)
- Each file has single responsibility (Constitution V)
- Uses `pub(crate)` for internal APIs (Constitution V)
- No files exceed 500 lines (estimated ~100-200 lines each)

#### Testing Strategy ✅
- Integration tests defined in quickstart.md using rmcp client (Constitution II)
- Unit tests for parser, renderer modules
- Test-first approach documented in quickstart
- User explicitly requested integration tests with rmcp client

#### Dependencies ✅
- **gray_matter** 0.3: YAML parsing (researched, mature)
- **minijinja** 2.12: Jinja templates (researched, minimal deps)
- **notify** 8.2 + **notify-debouncer-full** 0.6: File watching (researched, cross-platform)
- All dependencies are actively maintained and production-tested

#### Error Handling ✅
- `PromptError` enum with `thiserror` (Constitution III)
- Explicit `Result<T, PromptError>` return types
- Conversion to `rmcp::ErrorData` for MCP protocol
- No `unwrap()` in production code paths

#### Concurrency Safety ✅
- `Arc<RwLock<HashMap>>` for thread-safe registry
- Read-heavy optimization (Constitution principle on performance)
- Immutable template environment wrapped in Arc

#### Complexity Assessment ✅
- No violations of complexity principles
- Standard module structure
- No repository patterns or unnecessary abstractions
- Straightforward data flow: File → Parse → Store → Render

**POST-DESIGN GATE RESULT**: ✅ PASS - Design maintains constitutional compliance. Ready for Phase 2 implementation.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── cli.rs               # CLI command parsing (existing)
├── main.rs              # Entry point (existing)
├── mcp.rs               # MCP server logic (existing - will extend)
├── prompts/             # New: Prompt management module
│   ├── mod.rs           # Public API for prompt operations
│   ├── parser.rs        # Parse Markdown + YAML frontmatter
│   ├── renderer.rs      # Jinja template rendering
│   ├── watcher.rs       # File system watching
│   └── types.rs         # Prompt data structures
└── lib.rs               # Module exports (if needed)

tests/
├── integration/         # New: Integration tests using rmcp client
│   ├── mod.rs
│   ├── prompts_list.rs      # Test prompts/list endpoint
│   ├── prompts_get.rs       # Test prompts/get endpoint
│   └── prompts_notify.rs    # Test list_changed notifications
└── unit/                # New: Unit tests for prompt logic
    ├── mod.rs
    ├── parser_tests.rs      # Test YAML/Markdown parsing
    └── renderer_tests.rs    # Test Jinja rendering

website/docs/
└── local-prompts.md     # New: User-facing documentation
```

**Structure Decision**: Single project structure (Option 1) is appropriate. This is a CLI application with a single binary. The new `prompts/` module will be added to `src/` to encapsulate all prompt-related functionality (parsing, rendering, watching). Integration tests will use rmcp's client to launch and interact with the MCP server, as specified by the user.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
