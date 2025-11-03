# Twig Constitution

<!--
═════════════════════════════════════════════════════════════════════════════
SYNC IMPACT REPORT
═════════════════════════════════════════════════════════════════════════════
Version Change: [NONE] → 1.0.0

Modified Principles: N/A (initial version)

Added Sections:
  - Core Principles (5 principles focused on code quality, testing, DX)
  - Development Standards (code style, error handling, async patterns)
  - Quality Gates (CI requirements and enforcement)
  - Governance (amendment procedures and compliance)

Removed Sections: N/A (initial version)

Templates Requiring Updates:
  ✅ .specify/templates/plan-template.md - Constitution Check gate already present
  ✅ .specify/templates/spec-template.md - Acceptance criteria align with testing principle
  ✅ .specify/templates/tasks-template.md - Test-first approach matches tasks structure
  ✅ .specify/templates/agent-file-template.md - Generic template, no changes needed
  ✅ .specify/templates/checklist-template.md - Generic template, no changes needed

Follow-up TODOs: None - all placeholders filled

Rationale for 1.0.0:
  - Initial constitution establishing governance framework
  - Defines non-negotiable principles for code quality and testing
  - Sets baseline for all future development
═════════════════════════════════════════════════════════════════════════════
-->

## Core Principles

### I. Zero-Tolerance Code Quality

All code MUST pass automated quality gates before merge. No exceptions.

**Rules:**
- `cargo clippy -- -D warnings`: Warnings are treated as errors
- `cargo fmt -- --check`: Code must be formatted before commit
- `cargo test`: All tests must pass in CI
- `cargo build --release`: Release builds must succeed

**Rationale:** Automated enforcement prevents quality erosion and ensures
consistent codebase standards. Manual review cannot catch formatting and
lint issues as reliably as tooling.

### II. Test-First Development (NON-NEGOTIABLE)

Tests MUST be written before implementation. No feature ships without tests.

**Rules:**
- Write test → Verify test fails → Implement → Verify test passes
- Unit tests for business logic and utilities
- Integration tests for MCP protocol handlers and tool interactions
- Contract tests for external interfaces (stdio transport, tool schemas)
- Test naming: `test_<function>_<scenario>_<expected_outcome>`
- Tests must be independently runnable: `cargo test test_name`

**Rationale:** Test-first development catches bugs early, documents behavior,
and ensures features work as specified. Retrofitting tests after
implementation often results in tests that match the implementation rather
than the requirements.

### III. Explicit Over Implicit

Code clarity beats cleverness. Prefer explicit types and error handling.

**Rules:**
- Function signatures MUST use explicit types (no type inference)
- Function bodies MAY use type inference for local variables
- Error handling: `Result<T, ErrorData>` for MCP handlers, `.expect()` with
  descriptive messages for setup code
- Avoid `unwrap()` except in tests or prototypes
- Document non-obvious behavior with inline comments

**Rationale:** Rust's type system is a powerful tool for correctness. Explicit
signatures serve as living documentation and help catch errors at compile
time. Clear error messages reduce debugging time.

### IV. Developer Experience Consistency

Tooling and workflows MUST be consistent across all developer environments.

**Rules:**
- Use `mise` for environment management (defined in `mise.toml`)
- Commands documented in `AGENTS.md` and verified in CI
- Dev tools: `mise run dev:mcp` (MCP inspector), `mise run dev:page` (docs)
- Standard project structure: `src/` for source, `tests/` for tests,
  `website/docs/` for documentation
- Dependencies locked in `Cargo.lock` (committed to repo)

**Rationale:** Inconsistent environments lead to "works on my machine"
problems. Standardizing on `mise` and documented commands reduces onboarding
friction and environment-specific bugs.

### V. Small, Focused Modules

Each module has a single, clear responsibility. Complexity requires
justification.

**Rules:**
- Modules: `cli` (command parsing), `mcp` (MCP server logic), `main` (entry)
- Use `pub(crate)` for internal APIs, `pub` only for public interfaces
- Group imports: stdlib → external crates → `use crate::`
- Maximum file length: 500 lines (exceptions require justification in PR)
- Async runtime: `tokio` with "full" features (declared once in `main.rs`)

**Rationale:** Small modules are easier to understand, test, and maintain.
Clear boundaries prevent coupling and make refactoring safer.

## Development Standards

### Code Style & Conventions

**Rust Edition:** 2024

**Naming:**
- `snake_case` for functions, variables, modules
- `PascalCase` for types, structs, enums, traits
- `SCREAMING_SNAKE_CASE` for constants

**Imports:**
1. Standard library (`use std::*`)
2. External crates (`use clap::*`, `use rmcp::*`, `use tokio::*`)
3. Internal modules (`use crate::cli::*`, `use crate::mcp::*`)

**Formatting:** Enforced by `cargo fmt` (default rustfmt rules)

### Error Handling

- MCP handlers: Return `Result<T, ErrorData>` (from `rmcp` crate)
- Setup code: Use `.expect("descriptive message")` for panics
- Avoid `unwrap()` in production code paths
- Log errors before returning them to callers

### Async Patterns

- Use `#[tokio::main]` for async entry points
- Use `async fn` for async functions
- Prefer structured concurrency (spawn with join handles)
- Document blocking operations in async contexts

### Visibility

- Default to private (`fn`, `struct`)
- Use `pub(crate)` for internal APIs
- Use `pub` only for APIs exposed to external consumers
- Document all `pub` items with `///` doc comments

### Type Inference

- MUST: Explicit types in function signatures
- MAY: Type inference in function bodies
- Document complex type inferences with inline comments

## Quality Gates

### Pre-Commit

Developers SHOULD run these locally before committing:
- `cargo fmt` - Auto-format code
- `cargo clippy -- -D warnings` - Lint with warnings as errors
- `cargo test` - Run all tests

### Continuous Integration

CI MUST enforce these on every push:
- `cargo fmt -- --check` - Verify formatting
- `cargo clippy -- -D warnings` - Verify no warnings
- `cargo test` - Verify all tests pass
- `cargo build --release` - Verify release build succeeds

**Failure Mode:** Any gate failure blocks merge. No exceptions.

### Pre-Release

Before tagging a release:
- All CI gates MUST pass
- Documentation MUST be updated (`website/docs/`)
- `AGENTS.md` MUST reflect current commands and conventions
- Version MUST be bumped in `Cargo.toml`

## Governance

### Amendment Procedure

1. Propose amendment via pull request to `constitution.md`
2. Document rationale in PR description
3. Update version number:
   - MAJOR: Backward-incompatible principle changes (remove/redefine)
   - MINOR: New principle or materially expanded guidance
   - PATCH: Clarifications, wording fixes, non-semantic changes
4. Update Sync Impact Report (HTML comment at top of file)
5. Propagate changes to dependent templates in `.specify/templates/`
6. Require approval from project maintainers
7. Merge only after all affected documentation updated

### Versioning Policy

This constitution follows semantic versioning:
- Version format: `MAJOR.MINOR.PATCH`
- MAJOR: Breaking changes to governance or principles
- MINOR: New principles or sections added
- PATCH: Clarifications and refinements

### Compliance Review

- All pull requests MUST reference relevant principles
- Complexity exceptions MUST be justified in PR (see plan-template.md
  "Complexity Tracking" section)
- Templates in `.specify/templates/` define how principles are applied to
  feature planning and task breakdown
- `AGENTS.md` serves as runtime development guidance for agents and developers

### Template Alignment

- **plan-template.md**: Constitution Check gate validates compliance before
  Phase 0 research
- **spec-template.md**: Acceptance scenarios align with test-first principle
- **tasks-template.md**: Test-first task ordering (tests → implementation)
- **agent-file-template.md**: Generic template, no constitution-specific
  constraints
- **checklist-template.md**: Generic template, no constitution-specific
  constraints

**Version**: 1.0.0 | **Ratified**: 2025-11-03 | **Last Amended**: 2025-11-03
