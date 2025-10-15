# Implementation Plan: MCP Package Manager for AI Coding Agent Prompts and Resources

**Branch**: `001-mcpkg-is-a` | **Date**: 2025-10-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-mcpkg-is-a/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

mcpkg is a CLI tool and MCP server for managing project-specific prompts and resources for AI coding agents. Users can create multiple workspaces (SQLite databases), with multiple projects per workspace. Prompts and resources belong to projects and are exposed via Model Context Protocol. The system uses Python 3.13 with Typer for CLI, low-level MCP SDK for server implementation, and SQLite for data persistence with lightweight SQL-based migrations.

## Technical Context

**Language/Version**: Python 3.13  
**Primary Dependencies**: Typer (CLI), MCP SDK (low-level MCP server), SQLite3 (built-in)  
**Storage**: SQLite 3 databases (one per workspace) stored in OS data directory  
**Testing**: pytest for unit/integration tests, NEEDS CLARIFICATION (functional testing for CLI/MCP)  
**Target Platform**: Linux/macOS/Windows (cross-platform CLI)
**Project Type**: single (CLI + MCP server in one package)  
**Performance Goals**: CLI commands <200ms startup, MCP queries <100ms response time  
**Constraints**: <100MB memory footprint, support 10+ concurrent MCP server instances, database migrations must run automatically  
**Scale/Scope**: Support 10 workspaces with 20 projects each, 50 prompts/resources per project without performance degradation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Quality Gates (from Constitution)

- ✅ **Type Safety & Code Quality**: All code will use Python 3.13 type hints, pass Pyright strict mode, and follow Ruff formatting/linting standards
- ✅ **Test-Driven Development**: Tests will be written before implementation for all CLI commands, MCP handlers, database operations, and migrations logic
- ✅ **User Experience Consistency**: CLI uses Typer best practices with consistent flag naming (--workspace, --project); MCP follows protocol specification; error messages will be actionable
- ✅ **Performance Requirements**: Target <200ms CLI startup, <100ms MCP queries, <100MB memory footprint - aligns with constitution thresholds
- ✅ **Maintainability & Simplicity**: Using built-in SQLite (no ORM), plain SQL migrations, standard library where possible; minimal external dependencies (Typer, MCP SDK only)

### Constitution Compliance Assessment

**Status**: ✅ PASS - No violations

This feature aligns with all constitutional principles:
- Type hints required on all functions (Python 3.13)
- TDD enforced: tests written before implementation
- CLI follows Typer conventions, MCP follows protocol spec
- Performance targets within constitutional limits
- Minimal dependencies, plain SQL instead of ORM, no speculative features

## Project Structure

### Documentation (this feature)

```
specs/001-mcpkg-is-a/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── cli-commands.md  # CLI command specifications
│   └── mcp-api.md       # MCP protocol operations and tools
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
mcpkg/
├── __init__.py          # Package initialization
├── main.py              # Entry point (runs CLI)
├── cli.py               # Typer CLI commands (workspace, project, prompt, resource, start)
├── mcp.py               # MCP server implementation (prompts, resources, tools)
├── database.py          # Database connection, initialization, migration logic
├── migrations.py        # Migration management and execution
├── models.py            # Data models (Workspace, Project, Prompt, Resource) - dataclasses
├── queries.py           # SQL query functions (no ORM, plain SQL)
├── storage.py           # File path management (OS data directory, workspace DB files)
└── validators.py        # Name validation and constraint checking

mcpkg/migrations/
├── 001_initial_schema.sql
├── 002_add_metadata_fields.sql
└── [future migrations...]

mcpkg/tests/
├── functional/          # End-to-end tests for CLI and MCP
│   ├── test_cli_workspace.py
│   ├── test_cli_project.py
│   ├── test_cli_prompt.py
│   ├── test_cli_resource.py
│   └── test_mcp_server.py
├── integration/         # Integration tests (DB + business logic)
│   ├── test_database.py
│   ├── test_migrations.py
│   └── test_queries.py
└── unit/                # Unit tests (isolated logic)
    ├── test_validators.py
    ├── test_storage.py
    └── test_models.py
```

**Structure Decision**: Single project structure (Option 1) is selected as mcpkg is a single Python package containing both CLI and MCP server functionality. The structure separates concerns by layer: CLI interface (cli.py), MCP server (mcp.py), database operations (database.py, queries.py, migrations.py), domain models (models.py), and infrastructure (storage.py, validators.py). Tests are organized by testing level (functional, integration, unit) to support TDD at appropriate granularity.

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

**Status**: No violations - Complexity Tracking not needed

## Post-Design Constitution Check

**Re-evaluated**: 2025-10-14 (after Phase 1 design completion)

### Design Validation

✅ **Type Safety & Code Quality**: 
- All data models defined with Python dataclasses and type hints
- SQL queries use parameterized queries (security best practice)
- All functions will have full type annotations

✅ **Test-Driven Development**: 
- Test structure defined: functional, integration, and unit tests
- CLI testing with Typer CliRunner (fast, isolated)
- MCP testing with stdio_client (protocol-correct)
- Database testing with pytest fixtures and tmp_path

✅ **User Experience Consistency**: 
- CLI commands follow noun-verb structure consistently
- All commands support --help
- Error messages are actionable (see contracts/cli-commands.md)
- MCP API follows protocol specification exactly

✅ **Performance Requirements**: 
- SQLite WAL mode for concurrency
- No ORM overhead (plain SQL)
- Simple migration system (no complex framework)
- Design supports 10+ concurrent servers

✅ **Maintainability & Simplicity**: 
- Minimal dependencies: Typer, MCP SDK, platformdirs, pytest-asyncio
- Plain SQL instead of ORM
- Lightweight migration system (no Alembic)
- Clear separation of concerns (see Project Structure)
- Each module has single responsibility

### Design Alignment Summary

The Phase 1 design maintains full constitutional compliance:
- **Dependencies added**: platformdirs (for OS data directory) - justified as cross-platform standard
- **No complexity introduced**: Single project structure, no layers beyond necessary
- **Security considered**: Parameterized SQL queries, no secrets in code, input validation
- **TDD-ready**: Complete test strategy with appropriate tools for each layer

**Final Status**: ✅ PASS - No violations after design phase
