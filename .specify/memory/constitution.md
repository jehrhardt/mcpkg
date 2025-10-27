# Twig Project Constitution

<!--
SYNC IMPACT REPORT:
Version change: N/A (initial) → 1.0.0
Modified principles: N/A (initial version)
Added sections:
  - I. Type Safety & Code Quality
  - II. Test-First Development (NON-NEGOTIABLE)
  - III. Developer Experience Consistency
  - IV. Simple, Maintainable Architecture
  - V. Tooling Standardization
  - Development Workflow
  - Quality Gates
Removed sections: None (initial version)

Templates requiring updates:
  ✅ .specify/templates/plan-template.md - Constitution Check section aligns with new principles
  ✅ .specify/templates/spec-template.md - Requirements and acceptance scenarios support testability
  ✅ .specify/templates/tasks-template.md - Task structure supports test-first workflow and code quality gates
  ⚠ No command files found in .specify/templates/commands/ - no updates needed

Follow-up TODOs: None - all placeholders filled
-->

## Core Principles

### I. Type Safety & Code Quality

All code MUST be type-checked and linted before commit. This is enforced through:

- **Pyright** for static type checking with strict mode enabled
- **Ruff** for linting and formatting with zero tolerance for errors
- All functions MUST have type annotations for parameters and return values
- No `Any` types unless explicitly justified in code comments
- All code MUST pass `uv run ruff check` and `uv run pyright` with zero errors

**Rationale**: Type safety catches bugs at development time, improves IDE support, and serves as living documentation. Consistent code quality reduces cognitive load during code review and maintenance.

### II. Test-First Development (NON-NEGOTIABLE)

Test-Driven Development (TDD) is MANDATORY for all features:

- Tests MUST be written before implementation code
- Tests MUST fail initially (red phase)
- Implementation proceeds only after tests are failing correctly
- Red-Green-Refactor cycle is strictly enforced
- All tests MUST pass before merging: `uv run pytest` returns zero failures

**Test Coverage Requirements**:
- **Unit tests**: Required for all business logic, services, and utilities
- **Integration tests**: Required for MCP protocol handlers, CLI commands, and external integrations
- **Contract tests**: Required when defining or modifying MCP protocol interfaces

**Rationale**: TDD ensures requirements are testable, reduces debugging time, provides executable specifications, and enables confident refactoring. Tests written first prevent implementation-driven test design that misses edge cases.

### III. Developer Experience Consistency

Developer tooling and workflows MUST be consistent and documented:

- **Single source of truth**: CLAUDE.md documents all development commands and workflows
- **Tool standardization**: `uv` for dependencies, `mise` for task automation, `pytest` for testing
- **Zero configuration drift**: All developers run identical tooling versions via `pyproject.toml` and `mise.toml`
- **Fast feedback loops**: Local testing MUST complete in under 60 seconds for rapid iteration
- **Clear error messages**: All validation failures MUST provide actionable fix instructions

**Onboarding requirement**: A new developer MUST be able to run tests successfully within 5 minutes of cloning the repository using only CLAUDE.md instructions.

**Rationale**: Consistency eliminates "works on my machine" issues, reduces onboarding friction, and ensures all team members (human or AI) follow identical quality standards.

### IV. Simple, Maintainable Architecture

Simplicity is a core architectural constraint:

- **YAGNI principle**: Implement only what is needed now, not what might be needed later
- **Flat structure**: Minimize abstraction layers - prefer straightforward, readable code over clever patterns
- **Explicit over implicit**: No magic - all behavior should be traceable through clear function calls
- **Minimal dependencies**: Every new dependency MUST be justified with a specific use case
- **No premature optimization**: Optimize only when profiling identifies actual bottlenecks

**Complexity justification**: Any abstraction beyond a direct implementation (e.g., repository patterns, factory patterns, custom decorators) MUST document:
- The specific problem it solves
- Why a simpler alternative is insufficient
- The maintenance cost being accepted

**Rationale**: The MCP server is a protocol adapter - its job is clarity and correctness, not architectural sophistication. Simple code is easier to test, debug, and modify.

### V. Tooling Standardization

All development tools MUST be pinned and version-controlled:

- **Python**: 3.13+ specified in `pyproject.toml`
- **Package manager**: `uv` exclusively - no pip, poetry, or conda
- **Task runner**: `mise` for all scripted operations (dev, test, lint, format)
- **Linter**: `ruff` configured in `pyproject.toml`
- **Type checker**: `pyright` configured in `pyproject.toml`
- **Testing**: `pytest` with no plugins unless explicitly required

**Lock file discipline**: `uv.lock` MUST be committed and updated atomically with dependency changes.

**Rationale**: Tool version inconsistencies cause non-deterministic behavior. Pinning versions ensures reproducible builds and consistent developer experience across environments.

## Development Workflow

### Code Change Process

1. **Branch naming**: Use descriptive names (e.g., `add-prompt-validation`, `fix-mcp-error-handling`)
2. **Write tests first**: Create failing tests that define the expected behavior
3. **Implement code**: Make tests pass with minimal sufficient code
4. **Run quality checks**: Execute `uv run ruff check`, `uv run ruff format`, `uv run pyright`, `uv run pytest`
5. **Commit atomically**: Each commit should represent a single logical change with all tests passing
6. **Pull request**: Reference tests that validate the change in PR description

### Code Review Standards

All pull requests MUST:
- Pass all automated quality gates (linting, type checking, tests)
- Include tests that validate the changes
- Update CLAUDE.md if new commands or workflows are introduced
- Have clear commit messages explaining the "why" not just the "what"

Reviewers MUST verify:
- Tests were written first (check git history if unclear)
- No unjustified complexity was added
- Type annotations are complete and accurate
- Error handling covers expected failure modes

## Quality Gates

### Pre-Commit Requirements (MUST pass locally)

```bash
uv run ruff check      # Zero linting errors
uv run ruff format     # Code is formatted
uv run pyright         # Zero type errors
uv run pytest          # All tests pass
```

### CI/CD Requirements (MUST pass before merge)

- All pre-commit checks in CI environment
- Test coverage MUST NOT decrease (tracked via pytest-cov if enabled)
- No new dependencies without corresponding justification in PR description

### Breaking the Build

If main branch is broken (tests failing, type errors, etc.):
- **Priority 1**: Fix immediately - all other work stops
- **Communication**: Notify team in commit message and PR
- **Root cause**: Document what quality gate failed to catch the issue

## Governance

This constitution supersedes all informal practices and tribal knowledge. When in doubt, the constitution defines the correct behavior.

### Amendment Process

Constitution changes require:
1. **Proposal**: Document the specific principle change and rationale
2. **Impact analysis**: Identify affected templates, workflows, and existing code
3. **Version bump**: Follow semantic versioning (MAJOR for breaking governance changes, MINOR for new principles, PATCH for clarifications)
4. **Sync propagation**: Update all dependent templates and documentation before merging
5. **Team approval**: Constitution changes require explicit review (not auto-merge)

### Compliance Verification

All pull requests MUST verify compliance with this constitution:
- Test-first discipline followed (check git history)
- All quality gates passed
- No unjustified complexity added
- CLAUDE.md updated if workflows changed

### Runtime Development Guidance

For day-to-day development guidance beyond governance, refer to **CLAUDE.md**. The constitution defines "what" and "why"; CLAUDE.md defines "how".

**Version**: 1.0.0 | **Ratified**: 2025-10-27 | **Last Amended**: 2025-10-27
