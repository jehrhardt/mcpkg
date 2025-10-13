# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build/Test/Lint Commands

- **Run server**: `uv run main.py` or `mise run dev` (with MCP inspector)
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
- **Decorators**: Use FastMCP decorators (@mcp.tool(), @mcp.resource(), @mcp.prompt())
- **Error handling**: Use standard Python exceptions; add specific error types as needed

## Project Context

- **Purpose**: Model Context Protocol (MCP) package manager
- **Framework**: FastMCP for MCP server implementation
- **Structure**: Tools, resources, and prompts defined via decorators
- **Environment**: Virtual environment in `.venv`, Python 3.13 in `.python-version`

## Architecture

The project follows a simple, single-file MCP server architecture:

- **main.py**: MCP server implementation using FastMCP decorators
  - `@mcp.tool()`: Exposes functions as MCP tools
  - `@mcp.resource()`: Defines dynamic resources with URI templates
  - `@mcp.prompt()`: Creates prompt templates
- **test_main.py**: Pytest test suite
- **.mcp.json**: MCP client configuration for connecting to the server
- **CI/CD**: GitHub Actions workflow validates formatting, linting, type checking, and tests

The server is currently a demonstration/quickstart example and will be extended with package management functionality.
