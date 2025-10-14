# Quickstart Guide: mcpkg

**Feature**: 001-mcpkg-is-a  
**Date**: 2025-10-14  
**Phase**: 1 (Design & Contracts)

## Overview

This guide demonstrates how to use mcpkg to manage prompts and resources for AI coding agents. Follow these steps to get started quickly.

## Prerequisites

- Python 3.13 or higher
- `uv` package manager installed

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/mcpkg.git
cd mcpkg

# Install dependencies
uv sync

# Verify installation
uv run mcpkg --help
```

## Quick Start (5 minutes)

### Step 1: Create a Project

The default workspace is created automatically. Create your first project:

```bash
# Create a project for authentication work
uv run mcpkg project create auth-module

# Verify project was created
uv run mcpkg project list
```

**Output**:
```
Projects in workspace 'default':
  auth-module (created: 2025-10-14 10:30:00)
```

### Step 2: Add a Prompt

Add a prompt to help with code reviews:

```bash
uv run mcpkg prompt add code-review \
  "Review this authentication code for security issues. Focus on input validation, SQL injection risks, and password storage." \
  --project auth-module \
  --description "Security-focused code review prompt"
```

**Output**:
```
Added prompt 'code-review' to project 'auth-module' in workspace 'default'
```

### Step 3: Add a Resource

Add API documentation as a resource:

```bash
uv run mcpkg resource add api-docs \
  "file:///docs/api.md" \
  "# Authentication API

## POST /auth/login
Authenticate a user and return a JWT token.

**Request Body**:
- username (string, required)
- password (string, required)

**Response**:
- token (string)
- expires_at (ISO 8601 timestamp)" \
  --project auth-module \
  --mime-type text/markdown \
  --description "Authentication API documentation"
```

**Output**:
```
Added resource 'api-docs' to project 'auth-module' in workspace 'default'
```

### Step 4: Start the MCP Server

Start the MCP server to expose prompts and resources to AI agents:

```bash
uv run mcpkg start --project auth-module
```

**Output** (to stderr):
```
Starting MCP server for project 'auth-module' in workspace 'default'
Server ready on stdio
```

The server is now running and waiting for MCP client connections via stdin/stdout.

### Step 5: Connect Your AI Agent

Configure your AI coding agent (e.g., Claude Desktop) to connect to the MCP server:

**Claude Desktop Config** (`~/.config/Claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "mcpkg-auth": {
      "command": "uv",
      "args": ["run", "mcpkg", "start", "--project", "auth-module"],
      "cwd": "/path/to/mcpkg"
    }
  }
}
```

Restart Claude Desktop, and the prompts/resources from `auth-module` will be available.

## Working with Multiple Projects

### Create Multiple Projects

```bash
# Create projects for different areas
uv run mcpkg project create frontend
uv run mcpkg project create backend-api
uv run mcpkg project create database

# List all projects
uv run mcpkg project list
```

**Output**:
```
Projects in workspace 'default':
  auth-module (created: 2025-10-14 10:30:00)
  frontend (created: 2025-10-14 10:35:00)
  backend-api (created: 2025-10-14 10:35:01)
  database (created: 2025-10-14 10:35:02)
```

### Switch Between Projects

Start different MCP servers for different projects:

```bash
# Terminal 1: Serve frontend prompts/resources
uv run mcpkg start --project frontend

# Terminal 2: Serve backend prompts/resources
uv run mcpkg start --project backend-api
```

Each MCP server instance serves content from only one project at a time.

## Working with Multiple Workspaces

### Create a Custom Workspace

```bash
# Create a workspace for client work
uv run mcpkg workspace create client-acme

# Create projects in the new workspace
uv run mcpkg project create website --workspace client-acme
uv run mcpkg project create mobile-app --workspace client-acme

# List all workspaces
uv run mcpkg workspace list
```

**Output**:
```
Available workspaces:
  default
  client-acme
```

### Use a Custom Workspace

All commands support `--workspace` flag:

```bash
# Add prompt to client workspace project
uv run mcpkg prompt add ux-review \
  "Review this UI component for usability and accessibility" \
  --project website \
  --workspace client-acme

# Start MCP server for client workspace
uv run mcpkg start --project website --workspace client-acme
```

## Managing Prompts

### List Prompts

```bash
uv run mcpkg prompt list --project auth-module
```

**Output**:
```
Prompts in project 'auth-module' (workspace 'default'):
  code-review
    Description: Security-focused code review prompt
    Created: 2025-10-14 10:30:00
    Updated: 2025-10-14 10:30:00
```

### Update a Prompt

```bash
# Update prompt content
uv run mcpkg prompt update code-review \
  --content "Review this code for security issues. Focus on: input validation, SQL injection, XSS, authentication bypass, and password storage." \
  --project auth-module

# Update prompt description
uv run mcpkg prompt update code-review \
  --description "Comprehensive security code review checklist" \
  --project auth-module
```

### Delete a Prompt

```bash
uv run mcpkg prompt delete code-review --project auth-module
```

## Managing Resources

### List Resources

```bash
uv run mcpkg resource list --project auth-module
```

**Output**:
```
Resources in project 'auth-module' (workspace 'default'):
  api-docs
    URI: file:///docs/api.md
    MIME type: text/markdown
    Description: Authentication API documentation
    Created: 2025-10-14 10:30:00
    Updated: 2025-10-14 10:30:00
```

### Update a Resource

```bash
# Update resource content
uv run mcpkg resource update api-docs \
  --content "# Updated Authentication API..." \
  --project auth-module

# Update resource metadata
uv run mcpkg resource update api-docs \
  --mime-type text/markdown \
  --description "Updated API documentation with examples" \
  --project auth-module
```

### Delete a Resource

```bash
uv run mcpkg resource delete api-docs --project auth-module
```

## Using MCP Tools (AI Agent Operations)

AI agents can manage prompts and resources via MCP tools:

### Create Prompt via MCP Tool

The AI agent can call the `create_prompt` tool:

```json
{
  "name": "create_prompt",
  "arguments": {
    "name": "refactoring-guide",
    "content": "Refactor this code to improve readability and maintainability",
    "description": "Code refactoring guidelines"
  }
}
```

### Update Prompt via MCP Tool

```json
{
  "name": "update_prompt",
  "arguments": {
    "name": "refactoring-guide",
    "content": "Updated refactoring guidelines with examples"
  }
}
```

### Delete Prompt via MCP Tool

```json
{
  "name": "delete_prompt",
  "arguments": {
    "name": "refactoring-guide"
  }
}
```

The same pattern applies to `create_resource`, `update_resource`, and `delete_resource` tools.

## Project Management

### Rename a Project

```bash
uv run mcpkg project rename auth-module authentication
```

**Output**:
```
Renamed project 'auth-module' to 'authentication' in workspace 'default'
```

All prompts and resources remain intact after renaming.

### Delete a Project

```bash
# Delete with confirmation
uv run mcpkg project delete authentication

# Delete without confirmation
uv run mcpkg project delete authentication --force
```

**Output** (with confirmation):
```
Warning: This will permanently delete project 'authentication' and all its prompts and resources.
Continue? [y/N]: y
Deleted project 'authentication' from workspace 'default'
```

## Workspace Management

### Delete a Workspace

```bash
# Delete with confirmation
uv run mcpkg workspace delete client-acme

# Delete without confirmation
uv run mcpkg workspace delete client-acme --force
```

**Output** (with confirmation):
```
Warning: This will permanently delete workspace 'client-acme' and all its projects, prompts, and resources.
Continue? [y/N]: y
Deleted workspace 'client-acme'
```

## Common Workflows

### Workflow 1: Set Up a New Project

```bash
# 1. Create project
uv run mcpkg project create my-app

# 2. Add initial prompts
uv run mcpkg prompt add code-review "Review code for quality" --project my-app
uv run mcpkg prompt add debug-help "Help debug this issue" --project my-app

# 3. Add resources
uv run mcpkg resource add readme file:///README.md "$(cat README.md)" --project my-app
uv run mcpkg resource add architecture file:///docs/arch.md "$(cat docs/architecture.md)" --project my-app --mime-type text/markdown

# 4. Start MCP server
uv run mcpkg start --project my-app
```

### Workflow 2: Organize by Client

```bash
# Create workspace per client
uv run mcpkg workspace create acme-corp
uv run mcpkg workspace create globex-inc

# Create projects for each client
uv run mcpkg project create website --workspace acme-corp
uv run mcpkg project create api --workspace acme-corp

uv run mcpkg project create mobile-app --workspace globex-inc

# Add client-specific prompts and resources
uv run mcpkg prompt add brand-guidelines "Follow ACME brand guidelines" --project website --workspace acme-corp
```

### Workflow 3: Run Multiple Servers

```bash
# Terminal 1: Frontend development
uv run mcpkg start --project frontend

# Terminal 2: Backend development
uv run mcpkg start --project backend-api

# Terminal 3: Database work
uv run mcpkg start --project database
```

Each terminal serves a different project context to different AI agent sessions.

## Tips and Best Practices

### Naming Conventions

- Use kebab-case for names: `auth-module`, `api-docs`, `code-review`
- Keep names short and descriptive
- Allowed characters: `a-z`, `A-Z`, `0-9`, `-`, `_`, `.`

### Project Organization

- One project per logical area (e.g., `frontend`, `backend`, `database`)
- Use workspaces to separate unrelated work (e.g., different clients or products)
- Default workspace is fine for personal projects

### Prompt Best Practices

- Make prompts specific and actionable
- Include clear instructions and focus areas
- Use descriptions to explain when to use each prompt
- Update prompts as you learn better prompting techniques

### Resource Best Practices

- Store frequently-referenced documentation as resources
- Use descriptive URIs: `file:///docs/api.md`, `https://example.com/guide`
- Set correct MIME types for better AI agent handling
- Keep resources up-to-date with your codebase

## Troubleshooting

### Server Won't Start

**Problem**: `Error: Project 'my-project' does not exist in workspace 'default'`

**Solution**: Create the project first:
```bash
uv run mcpkg project create my-project
uv run mcpkg start --project my-project
```

### Cannot Create Prompt

**Problem**: `Error: Prompt name 'my prompt' contains invalid characters`

**Solution**: Use only allowed characters:
```bash
uv run mcpkg prompt add my-prompt "Content" --project my-project
```

### Workspace Not Found

**Problem**: `Error: Workspace 'my-workspace' does not exist`

**Solution**: Create the workspace first:
```bash
uv run mcpkg workspace create my-workspace
```

Or omit `--workspace` flag to use default workspace.

## Next Steps

- **Read the contracts**: See `contracts/cli-commands.md` and `contracts/mcp-api.md` for full API reference
- **Explore the data model**: See `data-model.md` for database schema details
- **Configure your AI agent**: Add mcpkg to your AI agent's MCP server configuration
- **Create your first prompts**: Add prompts that help with your daily coding tasks
- **Organize your resources**: Store documentation, schemas, and references for AI access

## Summary

Key commands to remember:

```bash
# Workspace management
mcpkg workspace list
mcpkg workspace create <name>
mcpkg workspace delete <name>

# Project management
mcpkg project list [--workspace <name>]
mcpkg project create <name> [--workspace <name>]
mcpkg project delete <name> [--workspace <name>]

# Prompt management
mcpkg prompt list --project <name> [--workspace <name>]
mcpkg prompt add <name> <content> --project <name> [--workspace <name>]
mcpkg prompt update <name> --content <content> --project <name>
mcpkg prompt delete <name> --project <name>

# Resource management
mcpkg resource list --project <name> [--workspace <name>]
mcpkg resource add <name> <uri> <content> --project <name> [--workspace <name>]
mcpkg resource update <name> --content <content> --project <name>
mcpkg resource delete <name> --project <name>

# Start MCP server
mcpkg start --project <name> [--workspace <name>]
```

All commands support `--help` for detailed usage information.
