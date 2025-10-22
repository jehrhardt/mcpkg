<!--
Sync Impact Report:
- Version change: N/A (initial) → 1.0.0
- New principles added:
  * I. Code Quality Excellence
  * II. Testing Standards
  * III. User Experience Consistency
  * IV. Performance Requirements
- New sections added:
  * Development Standards
  * Quality Gates
  * Governance
- Templates requiring updates:
  ✅ plan-template.md - Constitution Check section aligns with principles
  ✅ spec-template.md - Success criteria align with performance requirements
  ✅ tasks-template.md - Task categorization supports testing discipline
- Follow-up TODOs: None
-->

# mcpkg Constitution

## Core Principles

### I. Code Quality Excellence

All code MUST meet rigorous quality standards before merge:

- **Zero warnings tolerance**: All clippy warnings treated as errors (`-D warnings`)
- **Consistent formatting**: All code formatted with `rustfmt` defaults
- **Proper error handling**: Use `Result<T, E>` with domain-specific error types; MUST NOT use `unwrap()` or `expect()` in library code
- **Import organization**: Group imports as std, external crates, then local modules; alphabetical within groups
- **Naming conventions**: snake_case for functions/variables, PascalCase for types/traits
- **Documentation**: Public APIs MUST have doc comments (`///`)

**Rationale**: Rust's type system and tooling enable catching errors at compile time; leveraging these tools maximally reduces runtime defects and maintenance burden.

### II. Testing Standards

Testing is NON-NEGOTIABLE and follows strict discipline:

- **Test coverage**: All public APIs MUST have unit tests
- **Test clarity**: Tests MUST have clear Given-When-Then structure or equivalent
- **Integration tests**: New features involving CLI interaction or MCP protocol MUST include integration tests
- **Test location**: Unit tests in module or `tests/unit/`, integration tests in `tests/integration/`, contract tests for MCP protocol in `tests/contract/`
- **No flaky tests**: Tests MUST be deterministic; randomness/timing dependencies require justification
- **Tests as documentation**: Test names MUST clearly describe the scenario being tested

**Rationale**: mcpkg is infrastructure tooling managing model context packages; failures impact developer workflows directly. Comprehensive testing ensures reliability and enables confident refactoring.

### III. User Experience Consistency

User-facing behavior MUST be predictable and consistent:

- **Error messages**: MUST be actionable, include context, and suggest resolution steps
- **CLI patterns**: Follow established CLI conventions (flags, subcommands, help text)
- **Output formats**: Support both human-readable and machine-parseable (JSON) output where applicable
- **Stdin/stdout protocol**: Read from stdin/args, write results to stdout, errors to stderr
- **Progress feedback**: Long-running operations MUST provide progress indication
- **Exit codes**: Use conventional exit codes (0 success, 1 general error, specific codes for specific failures)

**Rationale**: As a package manager, mcpkg integrates into automation and developer workflows; consistency reduces friction and enables reliable scripting.

### IV. Performance Requirements

Performance MUST meet user expectations for a package manager:

- **Response time**: Interactive commands MUST respond within 100ms for local operations
- **Throughput**: Package operations MUST handle batches efficiently (target: 50+ packages/sec for metadata reads)
- **Memory usage**: Peak memory MUST stay under 100MB for typical operations (up to 1000 packages)
- **Startup time**: CLI startup overhead MUST be under 50ms
- **Network efficiency**: Remote operations MUST use connection pooling and implement request batching where applicable

**Rationale**: Package managers are invoked frequently during development; performance directly impacts developer productivity and satisfaction.

## Development Standards

**Build verification**: All changes MUST pass before commit:

1. `cargo fmt -- --check` (formatting)
2. `cargo clippy -- -D warnings` (linting)
3. `cargo test` (all tests)

**Dependency management**:

- New dependencies require justification (avoid duplication, evaluate maintenance status)
- Use minimal feature flags to reduce compilation time and binary size
- Document why each major dependency was chosen in project documentation

**Edition policy**: Use latest stable Rust edition (currently 2024); edition upgrades require validation across CI

## Quality Gates

All pull requests MUST satisfy:

1. **CI passes**: All automated checks green (format, lint, test)
2. **Review approval**: At least one maintainer approval
3. **Documentation**: Public API changes include doc updates
4. **Changelog**: User-visible changes documented in changelog
5. **Test coverage**: New functionality includes tests

Breaking changes MUST additionally include:

- Migration guide for users
- Version bump rationale (semantic versioning: MAJOR.MINOR.PATCH)
- Deprecation warnings in prior minor version when feasible

## Governance

**Authority**: This constitution supersedes all other development practices and guidelines.

**Compliance**:

- All code reviews MUST verify compliance with principles
- Violations require explicit justification and documentation
- Complexity beyond these principles MUST be defended with clear rationale

**Amendments**:

- Amendments require documentation of motivation and impact analysis
- Breaking changes to constitution require MAJOR version bump
- New principles or significant expansions require MINOR version bump
- Clarifications and refinements require PATCH version bump

**Runtime guidance**: Day-to-day development guidance for agents is maintained in `AGENTS.md`; constitution principles take precedence over AGENTS.md in case of conflict.

**Version**: 1.0.0 | **Ratified**: 2025-10-22 | **Last Amended**: 2025-10-22
