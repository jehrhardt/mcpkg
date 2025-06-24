# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

mcpkg is a model context package manager for developers, built in Rust. This is an early-stage project with basic CLI scaffolding implemented using the clap crate.

## Build and Development Commands

- `cargo build` - Build the project
- `cargo run` - Run the application (currently only supports `cargo run login`)
- `cargo test` - Run tests
- `cargo check` - Check code without building
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

## Architecture

This is a modular Rust CLI application with the following structure:

- `src/main.rs` - Entry point that delegates to the CLI module
- `src/cli.rs` - CLI interface using clap with derive macros, contains the main application logic
- Uses Rust edition 2024
- Single external dependency: clap v4.5.40 with derive features

The application follows a command-pattern architecture where:
1. `main()` calls `cli::run()`
2. `cli::run()` parses command-line arguments using clap
3. Commands are handled via an enum-based dispatch system
4. Currently only implements a placeholder `login` command

The project is in its initial scaffolding phase with the CLI framework established but no actual package management functionality implemented yet.