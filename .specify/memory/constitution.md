<!--
SYNC IMPACT REPORT
==================
Version Change: [None] → 1.0.0
Constitution Type: MINOR (initial constitution establishment)

Modified Principles: N/A (initial creation)

Added Sections:
  - Core Principles (5 principles)
    1. Type Safety & Code Quality
    2. Test-Driven Development (NON-NEGOTIABLE)
    3. User Experience Consistency
    4. Performance Requirements
    5. Maintainability & Simplicity
  - Development Standards
  - Quality Gates
  - Governance

Removed Sections: N/A (initial creation)

Templates Requiring Updates:
  ✅ plan-template.md - Constitution Check section aligns with new principles
  ✅ spec-template.md - Requirements section aligns with functional/measurable requirements
  ✅ tasks-template.md - Phase structure supports TDD and quality gates
  
Follow-up TODOs:
  - RATIFICATION_DATE set to creation date (2025-10-14)
  - Monitor constitution effectiveness during first feature implementation cycle
  - Review performance thresholds after initial benchmarking
-->

# mcpkg Constitution

## Core Principles

### I. Type Safety & Code Quality

All code MUST be fully type-hinted and pass strict type checking. Code quality is non-negotiable and enforced through automated tooling:

- **Type hints required**: Every function parameter, return value, and variable must have explicit type annotations
- **Strict type checking**: Pyright strict mode must pass with zero errors
- **Formatting**: Ruff formatting must be applied before commit (no exceptions)
- **Linting**: Ruff linting must pass with zero violations
- **No commented code**: Remove dead code; use version control for history
- **Import ordering**: Standard library first, then third-party, then local; sorted with ruff

**Rationale**: Type safety catches bugs at development time, reduces cognitive load during code review, and enables safe refactoring. Consistent formatting eliminates style debates and improves readability.

### II. Test-Driven Development (NON-NEGOTIABLE)

Tests MUST be written before implementation. The TDD cycle is strictly enforced:

1. **Write test**: Define behavior through test cases
2. **User approval**: Review test coverage and scenarios with stakeholder
3. **Verify failure**: Confirm test fails for the right reason
4. **Implement**: Write minimal code to pass test
5. **Refactor**: Improve code while keeping tests green

**Test requirements**:
- **Unit tests**: Required for all business logic and data transformations
- **Integration tests**: Required for MCP protocol handlers, CLI commands, and cross-boundary interactions
- **Contract tests**: Required for API endpoints and external integrations
- **Coverage minimum**: 80% line coverage; 100% for critical paths (security, data integrity)

**Rationale**: TDD ensures specifications are testable, reduces defects, enables confident refactoring, and serves as living documentation.

### III. User Experience Consistency

User-facing interfaces MUST be consistent, predictable, and well-documented:

- **CLI conventions**: Follow Typer best practices; use consistent flag naming (--verbose, --format, etc.)
- **Output formats**: Support both human-readable and machine-parseable (JSON) output
- **Error messages**: Must be actionable (state problem + suggested fix) and user-friendly
- **MCP protocol compliance**: Strict adherence to Model Context Protocol specification
- **Documentation**: Every user-facing command must have docstring with examples
- **Backwards compatibility**: Breaking changes require MAJOR version bump and migration guide

**Rationale**: Consistent UX reduces learning curve, prevents user frustration, and enables automation. MCP compliance ensures interoperability with Claude and other MCP clients.

### IV. Performance Requirements

System MUST meet performance thresholds appropriate for a package manager CLI tool:

- **Command startup**: < 200ms for simple commands (list, version)
- **Package operations**: < 2s for install/update of typical package
- **Memory footprint**: < 100MB resident memory during normal operation
- **Concurrent operations**: Support at least 10 parallel package operations without degradation
- **Response streaming**: Long-running operations must stream progress updates

**Performance validation**:
- Critical paths must be benchmarked in CI
- Regressions > 20% block merge
- Profiling data collected for operations > 1s

**Rationale**: CLI tools must feel instant to maintain developer flow. Package managers are in the critical path of development workflows and cannot be sluggish.

### V. Maintainability & Simplicity

Code MUST prioritize clarity and simplicity over cleverness:

- **YAGNI principle**: Implement only what is needed now; resist speculative features
- **Function length**: Prefer functions < 50 lines; extract helpers if exceeding
- **Module cohesion**: Group related functionality; avoid god modules
- **Dependency minimization**: Justify every new dependency; prefer standard library
- **Security by default**: Never log secrets; validate all external input; use parameterized queries

**Rationale**: Simple code is easier to understand, test, debug, and modify. As a package manager handling code execution, security cannot be an afterthought.

## Development Standards

### Code Review Requirements

- All changes require review by at least one maintainer
- Reviews MUST verify:
  - Type checking passes
  - Tests written before implementation (TDD)
  - Test coverage meets thresholds
  - Documentation updated
  - No security vulnerabilities introduced
  - Performance implications assessed

### Commit Standards

- Commits MUST be atomic and follow conventional commits format
- Each commit MUST pass all quality gates (format, lint, typecheck, tests)
- Commit messages MUST explain "why" not "what" (code shows "what")

### Branch Strategy

- Feature branches: `###-feature-name` format
- Direct commits to main prohibited
- Squash merge preferred for clean history

## Quality Gates

Every pull request MUST pass these gates before merge:

1. **Formatting**: `uv run ruff format --check` (zero issues)
2. **Linting**: `uv run ruff check` (zero issues)
3. **Type Checking**: `uv run pyright` (zero errors)
4. **Tests**: `uv run pytest` (all passing, coverage ≥ 80%)
5. **Performance**: Benchmark suite passes (no regressions > 20%)
6. **Security**: No secrets committed, input validation present

**CI enforcement**: GitHub Actions runs all gates; failures block merge.

## Governance

This constitution is the highest authority for all development decisions in the mcpkg project. When in doubt, constitution principles override individual preferences.

### Amendment Process

1. Propose amendment with rationale in GitHub issue
2. Require consensus among maintainers
3. Update constitution with version bump:
   - **MAJOR**: Principle removal or incompatible redefinition
   - **MINOR**: New principle or section added
   - **PATCH**: Clarifications, wording improvements
4. Update all dependent templates and documentation
5. Create migration guide if changes affect workflow

### Compliance

- All code reviews MUST explicitly verify constitution compliance
- Template updates MUST reference constitution principles
- New contributors MUST read constitution before first contribution
- Constitution reviewed quarterly for relevance

### Complexity Justification

Any violation of constitutional principles MUST be justified in writing:
- Why the principle cannot be followed
- Why simpler alternatives are insufficient
- Technical debt tracking for future remediation

**Version**: 1.0.0 | **Ratified**: 2025-10-14 | **Last Amended**: 2025-10-14
