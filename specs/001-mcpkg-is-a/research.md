# Research: MCP Package Manager Implementation

**Feature**: 001-mcpkg-is-a  
**Date**: 2025-10-14  
**Phase**: 0 (Research & Technical Planning)

## Overview

This document captures research findings and technical decisions for implementing the MCP package manager. All "NEEDS CLARIFICATION" items from the Technical Context have been resolved through research.

## Functional Testing Approach

### Decision

Multi-layered testing approach using:
1. **Typer CliRunner** for CLI command testing
2. **MCP SDK stdio_client** for MCP protocol testing
3. **pytest fixtures with in-memory SQLite** for database testing
4. **pytest-asyncio** for async MCP operations

### Rationale

- **Native integration**: Leverages built-in testing capabilities of Typer and MCP SDK
- **Protocol correctness**: Tests actual stdio-based MCP protocol communication (not mocked)
- **Fast execution**: In-memory databases and CliRunner avoid subprocess overhead
- **TDD-friendly**: Quick feedback loop supports writing tests before implementation
- **Constitutional alignment**: Supports TDD requirement and 80% coverage threshold

### Alternatives Considered

1. **Manual subprocess invocation**: Rejected - too brittle, difficult to inspect internal state, slow execution
2. **HTTP-based testing only**: Rejected - MCP primary transport is stdio, not HTTP
3. **Mock-heavy approach**: Rejected - reduces confidence in real protocol interactions, doesn't validate MCP compliance
4. **End-to-end only with no unit tests**: Rejected - too slow for TDD workflow, harder to isolate failures

### Implementation Guidance

#### Testing Typer CLI Commands

Use `typer.testing.CliRunner` (built-in with Typer):

```python
from typer.testing import CliRunner
from mcpkg.cli import app

runner = CliRunner()

def test_workspace_create():
    result = runner.invoke(app, ["workspace", "create", "my-workspace"])
    assert result.exit_code == 0
    assert "Created workspace" in result.stdout
```

**Benefits**:
- No subprocess overhead
- Captures stdout/stderr and exit codes
- Direct invocation of Typer app
- Fast test execution

#### Testing MCP Server Operations

Use MCP SDK's `stdio_client` and `ClientSession`:

```python
import pytest
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

@pytest.fixture
async def mcp_session():
    server_params = StdioServerParameters(
        command="uv",
        args=["run", "mcpkg", "start", "--workspace", "test", "--project", "test-project"],
    )
    
    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            yield session

@pytest.mark.asyncio
async def test_list_prompts(mcp_session):
    prompts = await mcp_session.list_prompts()
    assert isinstance(prompts.prompts, list)
```

**Benefits**:
- Tests actual MCP protocol communication
- Validates stdio transport (primary MCP transport)
- Can test prompts, resources, and tools
- Ensures MCP specification compliance

#### Testing Database Operations

Use pytest fixtures with temporary SQLite databases:

```python
import pytest
import sqlite3
from pathlib import Path

@pytest.fixture
def db_path(tmp_path):
    """Provide temporary database path for each test"""
    return tmp_path / "test.db"

@pytest.fixture
def db_connection(db_path):
    """Provide clean database connection for each test"""
    conn = sqlite3.connect(db_path)
    # Run migrations to initialize schema
    initialize_database(conn)
    yield conn
    conn.close()

def test_workspace_operations(db_connection):
    cursor = db_connection.cursor()
    cursor.execute("INSERT INTO workspaces (name) VALUES (?)", ("test",))
    db_connection.commit()
    
    cursor.execute("SELECT name FROM workspaces")
    assert cursor.fetchone()[0] == "test"
```

**Benefits**:
- Each test gets isolated database state
- Uses pytest's `tmp_path` fixture for automatic cleanup
- Fast execution (no I/O overhead)
- Easy to inspect database state

#### Testing Concurrent MCP Server Instances

Test multiple servers running simultaneously:

```python
@pytest.mark.asyncio
async def test_concurrent_servers():
    server1_params = StdioServerParameters(
        command="uv",
        args=["run", "mcpkg", "start", "--workspace", "ws1", "--project", "p1"],
    )
    
    server2_params = StdioServerParameters(
        command="uv",
        args=["run", "mcpkg", "start", "--workspace", "ws2", "--project", "p2"],
    )
    
    async with stdio_client(server1_params) as (read1, write1):
        async with stdio_client(server2_params) as (read2, write2):
            async with ClientSession(read1, write1) as session1:
                async with ClientSession(read2, write2) as session2:
                    await asyncio.gather(
                        session1.initialize(),
                        session2.initialize(),
                    )
                    
                    # Verify both servers can operate independently
                    results = await asyncio.gather(
                        session1.list_prompts(),
                        session2.list_prompts(),
                    )
                    assert len(results) == 2
```

### Required Dependencies

Add to `pyproject.toml` dev dependencies:
- `pytest>=8.4.2` - Core testing framework
- `pytest-asyncio>=0.24.0` - Async test support for MCP tests

### Pytest Configuration

Create `pytest.ini`:
```ini
[pytest]
asyncio_mode = auto
testpaths = mcpkg/tests
python_files = test_*.py
python_classes = Test*
python_functions = test_*
```

### Test Organization

```
mcpkg/tests/
├── functional/              # End-to-end tests
│   ├── test_cli_workspace.py
│   ├── test_cli_project.py
│   ├── test_cli_prompt.py
│   ├── test_cli_resource.py
│   └── test_mcp_server.py
├── integration/             # Database + business logic
│   ├── test_database.py
│   ├── test_migrations.py
│   └── test_queries.py
└── unit/                    # Isolated logic
    ├── test_validators.py
    ├── test_storage.py
    └── test_models.py
```

### Best Practices

1. **Use fixtures for setup/teardown** - Ensures clean state per test
2. **Test both CLI and MCP layers** - CLI tests are faster; MCP tests validate protocol
3. **Use in-memory SQLite for unit tests** - `sqlite3.connect(":memory:")` for speed
4. **Use `tmp_path` for integration tests** - Pytest provides automatic cleanup
5. **Mark async tests** - Use `@pytest.mark.asyncio` for all async test functions
6. **Test stdio transport** - Primary MCP transport mechanism
7. **Follow TDD cycle** - Write test → verify failure → implement → refactor

## Database Migration Strategy

### Decision

Lightweight SQL-based migration system without rollback functionality:
- Plain `.sql` files in `mcpkg/migrations/` directory
- Numbered sequential naming: `001_initial_schema.sql`, `002_add_metadata.sql`
- Migration tracking table in each workspace database
- Automatic migration execution on database open
- No rollback support (forward-only migrations)

### Rationale

- **Simplicity**: No heavy migration framework overhead (Alembic, etc.)
- **Transparency**: Plain SQL is easy to understand and review
- **Constitutional alignment**: Minimal dependencies, maintainability over cleverness
- **Sufficient for use case**: Single-user local databases don't need complex migration orchestration
- **No rollback needed**: Breaking changes require new migration, not rollback (simpler mental model)

### Alternatives Considered

1. **Alembic**: Rejected - heavyweight dependency, overkill for single-user SQLite databases
2. **Flask-Migrate**: Rejected - requires Flask dependency, not using Flask
3. **Yoyo-migrations**: Rejected - adds external dependency when simple approach suffices
4. **No migrations**: Rejected - schema evolution is inevitable, need structured approach

### Implementation Guidance

#### Migration File Format

```sql
-- Migration: 001_initial_schema.sql
-- Description: Create initial workspace schema

CREATE TABLE workspaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (workspace_id) REFERENCES workspaces (id) ON DELETE CASCADE,
    UNIQUE (workspace_id, name)
);
```

#### Migration Tracking

Each database contains a `schema_migrations` table:

```sql
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

#### Migration Execution Logic

```python
def apply_migrations(conn: sqlite3.Connection, migrations_dir: Path) -> None:
    """Apply pending migrations to database"""
    cursor = conn.cursor()
    
    # Ensure migration tracking table exists
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    """)
    
    # Get current version
    cursor.execute("SELECT COALESCE(MAX(version), 0) FROM schema_migrations")
    current_version = cursor.fetchone()[0]
    
    # Find and apply pending migrations
    migration_files = sorted(migrations_dir.glob("*.sql"))
    for migration_file in migration_files:
        version = int(migration_file.stem.split("_")[0])
        
        if version > current_version:
            # Apply migration
            with open(migration_file) as f:
                sql = f.read()
            cursor.executescript(sql)
            
            # Record migration
            cursor.execute(
                "INSERT INTO schema_migrations (version) VALUES (?)",
                (version,)
            )
    
    conn.commit()
```

## OS Data Directory Location

### Decision

Use `platformdirs` library to get OS-appropriate data directory:
- **Linux**: `~/.local/share/mcpkg/`
- **macOS**: `~/Library/Application Support/mcpkg/`
- **Windows**: `%LOCALAPPDATA%\mcpkg\`

Database files stored as `{workspace_name}.mcpkg` in data directory.

### Rationale

- **Cross-platform**: Single API works on Linux/macOS/Windows
- **OS conventions**: Respects platform-specific standards
- **Standard library alternative**: Could use `os.path.expanduser()` but `platformdirs` handles edge cases
- **Lightweight dependency**: Small, well-maintained library

### Alternatives Considered

1. **Current directory**: Rejected - clutters user's working directory
2. **Home directory**: Rejected - doesn't follow OS conventions
3. **Manual platform detection**: Rejected - `platformdirs` handles this correctly

### Implementation Guidance

```python
from platformdirs import user_data_dir
from pathlib import Path

def get_data_dir() -> Path:
    """Get mcpkg data directory (creates if not exists)"""
    data_dir = Path(user_data_dir("mcpkg", "mcpkg"))
    data_dir.mkdir(parents=True, exist_ok=True)
    return data_dir

def get_workspace_db_path(workspace_name: str) -> Path:
    """Get path to workspace database file"""
    return get_data_dir() / f"{workspace_name}.mcpkg"
```

Add to dependencies: `platformdirs>=4.3.6`

## SQLite Concurrency Configuration

### Decision

Use SQLite WAL (Write-Ahead Logging) mode for better concurrency:
- Multiple readers can access database simultaneously
- Single writer can proceed while readers are active
- Better performance for concurrent MCP server instances

### Rationale

- **Improved concurrency**: WAL mode allows multiple processes to read while one writes
- **Constitutional alignment**: Supports requirement for 10+ concurrent MCP server instances
- **Standard SQLite feature**: No external dependencies
- **Safe default**: WAL mode is production-ready and widely used

### Implementation Guidance

```python
def open_database(db_path: Path) -> sqlite3.Connection:
    """Open database connection with WAL mode enabled"""
    conn = sqlite3.connect(db_path)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA foreign_keys=ON")
    return conn
```

## Summary

All NEEDS CLARIFICATION items have been resolved:

1. ✅ **Testing approach**: Multi-layered testing with CliRunner, MCP stdio_client, pytest fixtures
2. ✅ **Migration strategy**: Lightweight SQL-based migrations without rollback
3. ✅ **Data directory**: `platformdirs` library for OS-appropriate locations
4. ✅ **Concurrency**: SQLite WAL mode for better multi-process access

The research findings align with constitutional principles:
- Minimal dependencies (Typer, MCP SDK, platformdirs, pytest-asyncio)
- Plain SQL over ORM
- Simple, maintainable solutions over complex frameworks
- TDD-friendly testing approach
