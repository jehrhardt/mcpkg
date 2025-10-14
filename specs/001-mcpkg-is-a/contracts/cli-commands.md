# CLI Commands Contract

**Feature**: 001-mcpkg-is-a  
**Date**: 2025-10-14  
**Phase**: 1 (Design & Contracts)

## Overview

This document specifies all CLI commands for the mcpkg tool. Commands follow a noun-verb structure (e.g., `mcpkg workspace create`) and use Typer for implementation.

## Command Structure

All commands support:
- `--help` flag for command-specific help
- `--workspace` flag (optional, defaults to "default") for workspace-specific commands
- Consistent error handling with actionable error messages
- Exit code 0 for success, non-zero for errors

## Workspace Commands

### `mcpkg workspace list`

**Purpose**: List all available workspaces.

**Usage**:
```bash
mcpkg workspace list
```

**Arguments**: None

**Flags**: None

**Output** (stdout):
```
Available workspaces:
  default
  my-project
  client-work
```

**Exit Codes**:
- 0: Success
- 1: I/O error reading data directory

**Errors**:
- If data directory is not accessible: "Error: Cannot access data directory at {path}"

---

### `mcpkg workspace create <name>`

**Purpose**: Create a new workspace.

**Usage**:
```bash
mcpkg workspace create my-project
```

**Arguments**:
- `name` (required): Workspace name

**Flags**: None

**Output** (stdout):
```
Created workspace 'my-project' at ~/.local/share/mcpkg/my-project.mcpkg
```

**Exit Codes**:
- 0: Success
- 1: Validation error (invalid name)
- 1: Workspace already exists
- 1: I/O error creating database

**Errors**:
- Invalid name: "Error: Workspace name 'my project' contains invalid characters. Allowed: a-z, A-Z, 0-9, -, _, ."
- Duplicate: "Error: Workspace 'my-project' already exists"
- I/O error: "Error: Failed to create workspace database at {path}: {reason}"

---

### `mcpkg workspace delete <name>`

**Purpose**: Delete a workspace and its database file.

**Usage**:
```bash
mcpkg workspace delete my-project
mcpkg workspace delete my-project --force  # Skip confirmation
```

**Arguments**:
- `name` (required): Workspace name

**Flags**:
- `--force` / `-f`: Skip confirmation prompt

**Output** (stdout):
```
Warning: This will permanently delete workspace 'my-project' and all its projects, prompts, and resources.
Continue? [y/N]: y
Deleted workspace 'my-project'
```

**Exit Codes**:
- 0: Success
- 1: Workspace not found
- 1: User cancelled deletion
- 1: I/O error deleting file

**Errors**:
- Not found: "Error: Workspace 'my-project' does not exist"
- I/O error: "Error: Failed to delete workspace database: {reason}"

---

## Project Commands

### `mcpkg project list`

**Purpose**: List all projects in a workspace.

**Usage**:
```bash
mcpkg project list
mcpkg project list --workspace my-project
```

**Arguments**: None

**Flags**:
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Projects in workspace 'default':
  auth-module (created: 2025-10-14 10:30:00)
  payment-service (created: 2025-10-14 11:45:00)
```

**Exit Codes**:
- 0: Success
- 1: Workspace not found
- 1: Database error

**Errors**:
- Workspace not found: "Error: Workspace 'my-project' does not exist"
- Database error: "Error: Failed to read projects: {reason}"

---

### `mcpkg project create <name>`

**Purpose**: Create a new project in a workspace.

**Usage**:
```bash
mcpkg project create auth-module
mcpkg project create payment-service --workspace my-project
```

**Arguments**:
- `name` (required): Project name

**Flags**:
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Created project 'auth-module' in workspace 'default'
```

**Exit Codes**:
- 0: Success
- 1: Validation error (invalid name)
- 1: Project already exists
- 1: Workspace not found
- 1: Database error

**Errors**:
- Invalid name: "Error: Project name 'auth module' contains invalid characters. Allowed: a-z, A-Z, 0-9, -, _, ."
- Duplicate: "Error: Project 'auth-module' already exists in workspace 'default'"
- Workspace not found: "Error: Workspace 'my-project' does not exist"
- Database error: "Error: Failed to create project: {reason}"

---

### `mcpkg project rename <old_name> <new_name>`

**Purpose**: Rename an existing project.

**Usage**:
```bash
mcpkg project rename auth-module authentication
mcpkg project rename old-name new-name --workspace my-project
```

**Arguments**:
- `old_name` (required): Current project name
- `new_name` (required): New project name

**Flags**:
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Renamed project 'auth-module' to 'authentication' in workspace 'default'
```

**Exit Codes**:
- 0: Success
- 1: Validation error (invalid name)
- 1: Project not found
- 1: New name already exists
- 1: Database error

**Errors**:
- Not found: "Error: Project 'auth-module' does not exist in workspace 'default'"
- Duplicate: "Error: Project 'authentication' already exists in workspace 'default'"
- Database error: "Error: Failed to rename project: {reason}"

---

### `mcpkg project delete <name>`

**Purpose**: Delete a project and all its prompts/resources.

**Usage**:
```bash
mcpkg project delete auth-module
mcpkg project delete auth-module --force  # Skip confirmation
mcpkg project delete auth-module --workspace my-project
```

**Arguments**:
- `name` (required): Project name

**Flags**:
- `--workspace` / `-w`: Workspace name (default: "default")
- `--force` / `-f`: Skip confirmation prompt

**Output** (stdout):
```
Warning: This will permanently delete project 'auth-module' and all its prompts and resources.
Continue? [y/N]: y
Deleted project 'auth-module' from workspace 'default'
```

**Exit Codes**:
- 0: Success
- 1: Project not found
- 1: User cancelled deletion
- 1: Database error

**Errors**:
- Not found: "Error: Project 'auth-module' does not exist in workspace 'default'"
- Database error: "Error: Failed to delete project: {reason}"

---

## Prompt Commands

### `mcpkg prompt list`

**Purpose**: List all prompts in a project.

**Usage**:
```bash
mcpkg prompt list --project auth-module
mcpkg prompt list --project auth-module --workspace my-project
```

**Arguments**: None

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Prompts in project 'auth-module' (workspace 'default'):
  code-review
    Description: Review authentication code for security issues
    Created: 2025-10-14 10:30:00
    Updated: 2025-10-14 10:30:00
  
  refactor-guide
    Description: Guide for refactoring auth module
    Created: 2025-10-14 11:00:00
    Updated: 2025-10-14 12:15:00
```

**Exit Codes**:
- 0: Success
- 1: Workspace not found
- 1: Project not found
- 1: Database error

**Errors**:
- Workspace not found: "Error: Workspace 'my-project' does not exist"
- Project not found: "Error: Project 'auth-module' does not exist in workspace 'default'"
- Database error: "Error: Failed to read prompts: {reason}"

---

### `mcpkg prompt add <name> <content>`

**Purpose**: Add a new prompt to a project.

**Usage**:
```bash
mcpkg prompt add code-review "Review this code for security issues" --project auth-module
mcpkg prompt add refactor-guide "Refactoring guidelines" --project auth-module --description "Guide for safe refactoring"
```

**Arguments**:
- `name` (required): Prompt name
- `content` (required): Prompt text content

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")
- `--description` / `-d`: Optional description

**Output** (stdout):
```
Added prompt 'code-review' to project 'auth-module' in workspace 'default'
```

**Exit Codes**:
- 0: Success
- 1: Validation error (invalid name or empty content)
- 1: Prompt already exists
- 1: Project not found
- 1: Database error

**Errors**:
- Invalid name: "Error: Prompt name 'code review' contains invalid characters. Allowed: a-z, A-Z, 0-9, -, _, ."
- Empty content: "Error: Prompt content cannot be empty"
- Duplicate: "Error: Prompt 'code-review' already exists in project 'auth-module'"
- Project not found: "Error: Project 'auth-module' does not exist in workspace 'default'"
- Database error: "Error: Failed to add prompt: {reason}"

---

### `mcpkg prompt update <name>`

**Purpose**: Update an existing prompt's content or description.

**Usage**:
```bash
mcpkg prompt update code-review --content "Updated review guidelines" --project auth-module
mcpkg prompt update code-review --description "New description" --project auth-module
```

**Arguments**:
- `name` (required): Prompt name

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")
- `--content` / `-c`: New prompt content (optional)
- `--description` / `-d`: New description (optional)

**Output** (stdout):
```
Updated prompt 'code-review' in project 'auth-module' (workspace 'default')
```

**Exit Codes**:
- 0: Success
- 1: Prompt not found
- 1: No update fields provided
- 1: Database error

**Errors**:
- Not found: "Error: Prompt 'code-review' does not exist in project 'auth-module'"
- No fields: "Error: At least one of --content or --description must be provided"
- Database error: "Error: Failed to update prompt: {reason}"

---

### `mcpkg prompt delete <name>`

**Purpose**: Delete a prompt from a project.

**Usage**:
```bash
mcpkg prompt delete code-review --project auth-module
mcpkg prompt delete code-review --project auth-module --workspace my-project
```

**Arguments**:
- `name` (required): Prompt name

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Deleted prompt 'code-review' from project 'auth-module' (workspace 'default')
```

**Exit Codes**:
- 0: Success
- 1: Prompt not found
- 1: Database error

**Errors**:
- Not found: "Error: Prompt 'code-review' does not exist in project 'auth-module'"
- Database error: "Error: Failed to delete prompt: {reason}"

---

## Resource Commands

### `mcpkg resource list`

**Purpose**: List all resources in a project.

**Usage**:
```bash
mcpkg resource list --project auth-module
mcpkg resource list --project auth-module --workspace my-project
```

**Arguments**: None

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Resources in project 'auth-module' (workspace 'default'):
  api-docs
    URI: file:///docs/api.md
    MIME type: text/markdown
    Description: API documentation
    Created: 2025-10-14 10:30:00
    Updated: 2025-10-14 10:30:00
  
  schema
    URI: file:///schema.sql
    MIME type: application/sql
    Description: Database schema
    Created: 2025-10-14 11:00:00
    Updated: 2025-10-14 12:15:00
```

**Exit Codes**:
- 0: Success
- 1: Workspace not found
- 1: Project not found
- 1: Database error

---

### `mcpkg resource add <name> <uri> <content>`

**Purpose**: Add a new resource to a project.

**Usage**:
```bash
mcpkg resource add api-docs file:///docs/api.md "API documentation content" --project auth-module
mcpkg resource add schema file:///schema.sql "CREATE TABLE..." --project auth-module --mime-type application/sql --description "Database schema"
```

**Arguments**:
- `name` (required): Resource name
- `uri` (required): Resource URI
- `content` (required): Resource content (text or file path with @)

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")
- `--mime-type` / `-m`: MIME type (optional, default: text/plain)
- `--description` / `-d`: Optional description

**Output** (stdout):
```
Added resource 'api-docs' to project 'auth-module' in workspace 'default'
```

**Exit Codes**:
- 0: Success
- 1: Validation error (invalid name, empty content)
- 1: Resource already exists (name or URI)
- 1: Project not found
- 1: Database error

**Errors**:
- Invalid name: "Error: Resource name 'api docs' contains invalid characters. Allowed: a-z, A-Z, 0-9, -, _, ."
- Empty content: "Error: Resource content cannot be empty"
- Duplicate name: "Error: Resource 'api-docs' already exists in project 'auth-module'"
- Duplicate URI: "Error: Resource with URI 'file:///docs/api.md' already exists in project 'auth-module'"
- Project not found: "Error: Project 'auth-module' does not exist in workspace 'default'"

---

### `mcpkg resource update <name>`

**Purpose**: Update an existing resource's content or metadata.

**Usage**:
```bash
mcpkg resource update api-docs --content "Updated documentation" --project auth-module
mcpkg resource update api-docs --mime-type text/markdown --description "New description" --project auth-module
```

**Arguments**:
- `name` (required): Resource name

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")
- `--content` / `-c`: New resource content (optional)
- `--mime-type` / `-m`: New MIME type (optional)
- `--description` / `-d`: New description (optional)

**Output** (stdout):
```
Updated resource 'api-docs' in project 'auth-module' (workspace 'default')
```

**Exit Codes**:
- 0: Success
- 1: Resource not found
- 1: No update fields provided
- 1: Database error

---

### `mcpkg resource delete <name>`

**Purpose**: Delete a resource from a project.

**Usage**:
```bash
mcpkg resource delete api-docs --project auth-module
mcpkg resource delete api-docs --project auth-module --workspace my-project
```

**Arguments**:
- `name` (required): Resource name

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stdout):
```
Deleted resource 'api-docs' from project 'auth-module' (workspace 'default')
```

**Exit Codes**:
- 0: Success
- 1: Resource not found
- 1: Database error

---

## MCP Server Command

### `mcpkg start`

**Purpose**: Start the MCP server for a specific project.

**Usage**:
```bash
mcpkg start --project auth-module
mcpkg start --project auth-module --workspace my-project
```

**Arguments**: None

**Flags**:
- `--project` / `-p`: Project name (required)
- `--workspace` / `-w`: Workspace name (default: "default")

**Output** (stderr - for logging):
```
Starting MCP server for project 'auth-module' in workspace 'default'
Server ready on stdio
```

**Exit Codes**:
- 0: Success (server runs until killed)
- 1: Project not found
- 1: Workspace not found
- 1: Database error

**Errors**:
- Missing flag: "Error: --project flag is required"
- Project not found: "Error: Project 'auth-module' does not exist in workspace 'default'"
- Workspace not found: "Error: Workspace 'my-project' does not exist"
- Database error: "Error: Failed to connect to database: {reason}"

**Behavior**:
- Server runs indefinitely, communicating via stdin/stdout
- Logging goes to stderr
- Server exposes prompts, resources, and tools for the specified project
- Multiple servers can run concurrently

---

## Summary

All CLI commands:
- Follow consistent naming and flag conventions
- Provide actionable error messages
- Support `--help` for documentation
- Use exit code 0 for success, non-zero for errors
- Default to "default" workspace when `--workspace` is omitted
- Require explicit confirmation for destructive operations (unless `--force` is used)
