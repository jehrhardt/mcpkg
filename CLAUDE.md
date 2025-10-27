# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Twig is an MCP (Model Context Protocol) server that helps coding agents work more effectively. The project provides prompts to coding agents via the MCP protocol.

## Development Setup

This project uses:
- **uv** for Python package management
- **mise** for task automation and environment management
- Python 3.13+

Environment variables are loaded from `.env` file (configured in mise.toml).

## Common Commands

### Running the MCP Server

```bash
# Run the server with MCP inspector (for development/debugging)
mise run dev

# Or run the server directly
uv run -m twig.main start
```

### Testing

```bash
# Run all tests
uv run pytest

# Run a specific test file
uv run pytest twig/test_main.py

# Run a specific test
uv run pytest twig/test_main.py::test_dummy
```

### Code Quality

```bash
# Run linter
uv run ruff check

# Auto-fix linting issues
uv run ruff check --fix

# Format code
uv run ruff format

# Type checking
uv run pyright
```

## Architecture

### Entry Points

- **twig/main.py**: Main entry point, delegates to CLI
- **twig/cli.py**: Typer-based CLI with two commands:
  - `start`: Starts the MCP server
  - `prompt`: Placeholder command (currently just prints "Hello")

### MCP Server Implementation

**twig/mcp.py** contains the core MCP server logic:

- Uses the `mcp` library's low-level server API
- Runs via stdio transport (stdin/stdout communication)
- Currently implements prompts functionality:
  - `list_prompts()`: Returns available prompt templates
  - `get_prompt()`: Returns specific prompt by name with argument interpolation
- Server name: "twig", version: "dev"

The MCP server follows a handler-based pattern where decorators register async functions to handle specific MCP protocol operations.

### Project Structure

```
twig/
├── __init__.py
├── main.py      # Entry point
├── cli.py       # CLI interface using Typer
├── mcp.py       # MCP server implementation
└── test_main.py # Tests
```

## Dependencies

- **mcp[cli]**: Model Context Protocol implementation
- **supabase**: Database client (included but not yet used in current implementation)
- **typer**: CLI framework

## Development Notes

- The project was recently renamed from "mcpkg" to "twig" (visible in git history)
- The MCP server currently has a basic example prompt implementation that should be extended
- The `prompt` CLI command is a placeholder and needs implementation
- Supabase is configured as a dependency but not yet integrated into the codebase
