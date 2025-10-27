# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build/Test/Lint Commands

- **Run server**: `uv run mcpkg start` or `mise run dev` (with MCP inspector)
- **Format code**: `uv run ruff format`
- **Check formatting**: `uv run ruff format --check`
- **Lint**: `uv run ruff check`
- **Type check**: `uv run pyright`
- **Run all tests**: `uv run pytest`
- **Run single test**: `uv run pytest path/to/test_file.py::test_name`
- **Install dependencies**: `uv sync`
- **CI verification**: Run format check, lint, type check, and pytest before committing

## Code Style

- **Language**: Python 3.13 (leverage modern Python features)
- **Package manager**: `uv` (installed via mise)
- **Formatter**: Ruff (used in CI)
- **Linter**: Ruff (used in CI)
- **Type checker**: Pyright with strict configuration
- **Imports**: Standard library first, then third-party (mcp, typer), sort with ruff
- **Type hints**: Required on all functions and variables; project uses pyright
- **Docstrings**: Triple-quoted strings for all public functions (see main.py examples)
- **Naming**: snake_case for functions/variables, descriptive names (e.g., `get_greeting`, `greet_user`)
- **Decorators**: Use low-level MCP server decorators (@server.list_prompts(), @server.get_prompt())
- **Error handling**: Use standard Python exceptions; add specific error types as needed

## Project Context

- **Purpose**: Model Context Protocol (MCP) package manager
- **Framework**: Low-level MCP server implementation, Typer for CLI
- **Structure**: CLI commands defined via Typer, MCP handlers defined via server decorators
- **Environment**: Virtual environment in `.venv`, Python 3.13 in `.python-version`

## Architecture

The project follows a modular CLI-based architecture:

- **main.py**: Entry point that launches the CLI via `run()` function
- **cli.py**: Typer-based CLI with commands:
  - `mcp`: Starts the MCP server
- **mcp.py**: MCP server implementation using low-level MCP protocol
  - Prompt handlers for listing and getting prompts
  - Server runs via stdio_server for MCP communication
- **test_main.py**: Pytest test suite
- **.mcp.json**: MCP client configuration for connecting to the server
- **CI/CD**: GitHub Actions workflow validates formatting, linting, type checking, and tests

The server is currently a demonstration/quickstart example and will be extended with package management functionality.
