# Data Model: MCP Package Manager

**Feature**: 001-mcpkg-is-a  
**Date**: 2025-10-14  
**Phase**: 1 (Design & Contracts)

## Overview

This document defines the data model for the MCP package manager. Each workspace is a separate SQLite database file containing projects, prompts, and resources. The model uses plain SQL (no ORM) with foreign key constraints for referential integrity.

## Entity Definitions

### Workspace

**Description**: An isolated container for projects, backed by a SQLite database file.

**Storage**: Each workspace is a separate `.mcpkg` file in the OS data directory.

**Fields**:

- **name**: Unique workspace identifier (stored as filename, not in database)
- **db_path**: Full path to SQLite database file (computed, not stored)

**Validation Rules**:

- Name must match pattern: `[a-zA-Z0-9._-]+`
- Name must be unique across all workspaces
- Default workspace named "default" is automatically created on first use

**State Transitions**: N/A (workspaces don't have state)

**Relationships**:

- One workspace contains zero or more projects

**Python Model**:

```python
from dataclasses import dataclass
from pathlib import Path

@dataclass
class Workspace:
    name: str
    db_path: Path
```

---

### Project

**Description**: A logical grouping of related prompts and resources within a workspace.

**Storage**: SQLite table in workspace database.

**Fields**:

- **id**: Integer primary key (auto-increment)
- **name**: Project name (TEXT, NOT NULL)
- **created_at**: Timestamp of creation (TEXT, ISO 8601 format, NOT NULL)

**Validation Rules**:

- Name must match pattern: `[a-zA-Z0-9._-]+`
- Name must be unique within workspace
- Name cannot be empty

**State Transitions**: N/A (projects don't have state)

**Relationships**:

- One project contains zero or more prompts
- One project contains zero or more resources
- Project is deleted when workspace is deleted (CASCADE)
- Prompts/resources are deleted when project is deleted (CASCADE)

**SQL Schema**:

```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_projects_name ON projects(name);
```

**Python Model**:

```python
from dataclasses import dataclass
from datetime import datetime

@dataclass
class Project:
    id: int | None
    name: str
    created_at: datetime
```

---

### Prompt

**Description**: A reusable text prompt for AI coding agents, exposed via MCP.

**Storage**: SQLite table in workspace database.

**Fields**:

- **id**: Integer primary key (auto-increment)
- **project_id**: Foreign key to projects table (INTEGER, NOT NULL)
- **name**: Prompt name (TEXT, NOT NULL)
- **content**: Prompt text content (TEXT, NOT NULL)
- **description**: Optional description (TEXT, NULL)
- **created_at**: Timestamp of creation (TEXT, ISO 8601 format, NOT NULL)
- **updated_at**: Timestamp of last update (TEXT, ISO 8601 format, NOT NULL)

**Validation Rules**:

- Name must match pattern: `[a-zA-Z0-9._-]+`
- Name must be unique within project
- Content cannot be empty
- Description is optional

**State Transitions**: N/A (prompts are static content)

**Relationships**:

- Each prompt belongs to exactly one project
- Prompt is deleted when project is deleted (CASCADE)

**SQL Schema**:

```sql
CREATE TABLE prompts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE (project_id, name)
);

CREATE INDEX idx_prompts_project_id ON prompts(project_id);
CREATE INDEX idx_prompts_name ON prompts(project_id, name);
```

**Python Model**:

```python
from dataclasses import dataclass
from datetime import datetime

@dataclass
class Prompt:
    id: int | None
    project_id: int
    name: str
    content: str
    description: str | None
    created_at: datetime
    updated_at: datetime
```

---

### Resource

**Description**: A file, documentation, or reference material for AI coding agents, exposed via MCP.

**Storage**: SQLite table in workspace database with content stored as BLOB or TEXT.

**Fields**:

- **id**: Integer primary key (auto-increment)
- **project_id**: Foreign key to projects table (INTEGER, NOT NULL)
- **name**: Resource name (TEXT, NOT NULL)
- **uri**: Resource URI for reference (TEXT, NOT NULL)
- **content**: Resource content stored in database (BLOB, NOT NULL)
- **mime_type**: MIME type of resource (TEXT, NULL)
- **description**: Optional description (TEXT, NULL)
- **created_at**: Timestamp of creation (TEXT, ISO 8601 format, NOT NULL)
- **updated_at**: Timestamp of last update (TEXT, ISO 8601 format, NOT NULL)

**Validation Rules**:

- Name must match pattern: `[a-zA-Z0-9._-]+`
- Name must be unique within project
- URI must be unique within project
- Content cannot be empty
- MIME type is optional (defaults to `text/plain` if not provided)
- Description is optional

**State Transitions**: N/A (resources are static content)

**Relationships**:

- Each resource belongs to exactly one project
- Resource is deleted when project is deleted (CASCADE)

**SQL Schema**:

```sql
CREATE TABLE resources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    uri TEXT NOT NULL,
    content BLOB NOT NULL,
    mime_type TEXT,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE (project_id, name),
    UNIQUE (project_id, uri)
);

CREATE INDEX idx_resources_project_id ON resources(project_id);
CREATE INDEX idx_resources_uri ON resources(project_id, uri);
```

**Python Model**:

```python
from dataclasses import dataclass
from datetime import datetime

@dataclass
class Resource:
    id: int | None
    project_id: int
    name: str
    uri: str
    content: bytes
    mime_type: str | None
    description: str | None
    created_at: datetime
    updated_at: datetime
```

---

### Schema Migrations

**Description**: Tracks applied database migrations for schema versioning.

**Storage**: SQLite table in workspace database.

**Fields**:

- **version**: Migration version number (INTEGER, PRIMARY KEY)
- **applied_at**: Timestamp when migration was applied (TEXT, ISO 8601 format, NOT NULL)

**SQL Schema**:

```sql
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Usage**:

- Created automatically when workspace database is initialized
- Migration files named `001_initial_schema.sql`, `002_add_metadata.sql`, etc.
- Migrations applied in order based on version number
- No rollback support (forward-only)

---

## Entity Relationship Diagram

```
Workspace (file-based)
    |
    | (contains)
    v
Project (table: projects)
    |
    +---(contains)---> Prompt (table: prompts)
    |
    +---(contains)---> Resource (table: resources)
```

**Cardinality**:

- One workspace → many projects (1:N)
- One project → many prompts (1:N)
- One project → many resources (1:N)

**Cascade Behavior**:

- Delete workspace → deletes database file (and all contained data)
- Delete project → CASCADE deletes all prompts and resources
- Delete prompt/resource → no cascade (leaf entities)

---

## Validation Summary

All entity names (workspace, project, prompt, resource) must:

- Match pattern: `[a-zA-Z0-9._-]+`
- Not be empty
- Be unique within their scope (workspace names globally, project/prompt/resource names within parent)

This is enforced in `validators.py` with a single function:

```python
import re

NAME_PATTERN = re.compile(r'^[a-zA-Z0-9._-]+$')

def validate_name(name: str, entity_type: str) -> None:
    """Validate entity name against allowed pattern"""
    if not name:
        raise ValueError(f"{entity_type} name cannot be empty")
    if not NAME_PATTERN.match(name):
        raise ValueError(
            f"{entity_type} name '{name}' contains invalid characters. "
            f"Allowed: a-z, A-Z, 0-9, -, _, ."
        )
```

---

## Database File Locations

Workspace databases stored in OS-specific data directory:

- **Linux**: `~/.local/share/mcpkg/{workspace_name}.mcpkg`
- **macOS**: `~/Library/Application Support/mcpkg/{workspace_name}.mcpkg`
- **Windows**: `%LOCALAPPDATA%\mcpkg\{workspace_name}.mcpkg`

Example:

- Default workspace: `~/.local/share/mcpkg/default.mcpkg`
- Custom workspace: `~/.local/share/mcpkg/my-project.mcpkg`

---

## Concurrency Considerations

- **SQLite WAL mode**: Enabled for better concurrency (multiple readers, single writer)
- **Foreign keys**: Enabled to maintain referential integrity
- **Locking**: SQLite built-in locking handles concurrent access
- **Multiple MCP servers**: Can run concurrently serving different projects from same workspace

Database connection configuration:

```python
conn = sqlite3.connect(db_path)
conn.execute("PRAGMA journal_mode=WAL")
conn.execute("PRAGMA foreign_keys=ON")
```

---

## Migration Strategy

Each workspace database tracks its schema version in `schema_migrations` table:

1. Check current version: `SELECT MAX(version) FROM schema_migrations`
2. Find pending migrations in `mcpkg/migrations/` directory
3. Apply migrations in order (001, 002, 003, ...)
4. Record each migration: `INSERT INTO schema_migrations (version) VALUES (?)`

Migrations are plain SQL files executed with `cursor.executescript()`.

---

## Summary

The data model provides:

- ✅ Clear entity hierarchy (workspace → project → prompts/resources)
- ✅ Strong validation rules with consistent naming constraints
- ✅ Referential integrity with foreign keys and cascade deletes
- ✅ Metadata tracking (timestamps for all entities)
- ✅ Concurrency support (SQLite WAL mode)
- ✅ Schema versioning (migration tracking table)
- ✅ Simple, maintainable structure (no ORM, plain SQL)
