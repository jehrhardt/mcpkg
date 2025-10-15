import asyncio
from pathlib import Path

import typer
from mcpkg.database import initialize_database, open_database
from mcpkg.migrations import apply_migrations
from mcpkg.queries import (
    create_project,
    delete_project,
    get_project_by_name,
    list_projects,
    rename_project,
)
from mcpkg.storage import get_data_dir, get_workspace_db_path
from mcpkg.validators import validate_name

app = typer.Typer(no_args_is_help=True)
workspace_app = typer.Typer(help="Manage workspaces")
project_app = typer.Typer(help="Manage projects")
app.add_typer(workspace_app, name="workspace")
app.add_typer(project_app, name="project")


def ensure_default_workspace() -> None:
    default_db_path = get_workspace_db_path("default")
    if not default_db_path.exists():
        conn = initialize_database(default_db_path)
        migrations_dir = Path(__file__).parent / "migrations"
        apply_migrations(conn, migrations_dir)
        conn.close()


@workspace_app.command("list")
def workspace_list() -> None:
    """List all available workspaces."""
    ensure_default_workspace()

    data_dir = get_data_dir()
    workspace_files = sorted(data_dir.glob("*.mcpkg"))

    typer.echo("Available workspaces:")
    for ws_file in workspace_files:
        workspace_name = ws_file.stem
        typer.echo(f"  {workspace_name}")


@workspace_app.command("create")
def workspace_create(name: str) -> None:
    """Create a new workspace."""
    try:
        validate_name(name, "Workspace")
    except ValueError as e:
        typer.echo(f"Error: {e}", err=True)
        raise typer.Exit(1)

    db_path = get_workspace_db_path(name)

    if db_path.exists():
        typer.echo(f"Error: Workspace '{name}' already exists", err=True)
        raise typer.Exit(1)

    try:
        conn = initialize_database(db_path)
        migrations_dir = Path(__file__).parent / "migrations"
        apply_migrations(conn, migrations_dir)
        conn.close()
    except Exception as e:
        typer.echo(f"Error: Failed to create workspace database: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(f"Created workspace '{name}' at {db_path}")


@workspace_app.command("delete")
def workspace_delete(
    name: str, force: bool = typer.Option(False, "--force", "-f")
) -> None:
    """Delete a workspace and its database file."""
    db_path = get_workspace_db_path(name)

    if not db_path.exists():
        typer.echo(f"Error: Workspace '{name}' does not exist", err=True)
        raise typer.Exit(1)

    if not force:
        typer.echo(
            f"Warning: This will permanently delete workspace '{name}' and all its projects, prompts, and resources."
        )
        confirm = typer.confirm("Continue?", default=False)
        if not confirm:
            raise typer.Exit(1)

    try:
        db_path.unlink()
    except Exception as e:
        typer.echo(f"Error: Failed to delete workspace database: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(f"Deleted workspace '{name}'")


@project_app.command("list")
def project_list(workspace: str = typer.Option("default", "--workspace", "-w")) -> None:
    """List all projects in a workspace."""
    db_path = get_workspace_db_path(workspace)

    if not db_path.exists():
        typer.echo(f"Error: Workspace '{workspace}' does not exist", err=True)
        raise typer.Exit(1)

    try:
        conn = open_database(db_path)
        projects = list_projects(conn)
        conn.close()
    except Exception as e:
        typer.echo(f"Error: Failed to read projects: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(f"Projects in workspace '{workspace}':")
    for project in projects:
        typer.echo(f"  {project.name} (created: {project.created_at})")


@project_app.command("create")
def project_create(
    name: str, workspace: str = typer.Option("default", "--workspace", "-w")
) -> None:
    """Create a new project in a workspace."""
    try:
        validate_name(name, "Project")
    except ValueError as e:
        typer.echo(f"Error: {e}", err=True)
        raise typer.Exit(1)

    db_path = get_workspace_db_path(workspace)

    if not db_path.exists():
        typer.echo(f"Error: Workspace '{workspace}' does not exist", err=True)
        raise typer.Exit(1)

    try:
        conn = open_database(db_path)

        existing = get_project_by_name(conn, name)
        if existing is not None:
            conn.close()
            typer.echo(
                f"Error: Project '{name}' already exists in workspace '{workspace}'",
                err=True,
            )
            raise typer.Exit(1)

        create_project(conn, name)
        conn.close()
    except typer.Exit:
        raise
    except Exception as e:
        typer.echo(f"Error: Failed to create project: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(f"Created project '{name}' in workspace '{workspace}'")


@project_app.command("rename")
def project_rename_cmd(
    old_name: str,
    new_name: str,
    workspace: str = typer.Option("default", "--workspace", "-w"),
) -> None:
    """Rename an existing project."""
    try:
        validate_name(new_name, "Project")
    except ValueError as e:
        typer.echo(f"Error: {e}", err=True)
        raise typer.Exit(1)

    db_path = get_workspace_db_path(workspace)

    if not db_path.exists():
        typer.echo(f"Error: Workspace '{workspace}' does not exist", err=True)
        raise typer.Exit(1)

    try:
        conn = open_database(db_path)

        existing = get_project_by_name(conn, old_name)
        if existing is None:
            conn.close()
            typer.echo(
                f"Error: Project '{old_name}' does not exist in workspace '{workspace}'",
                err=True,
            )
            raise typer.Exit(1)

        new_existing = get_project_by_name(conn, new_name)
        if new_existing is not None:
            conn.close()
            typer.echo(
                f"Error: Project '{new_name}' already exists in workspace '{workspace}'",
                err=True,
            )
            raise typer.Exit(1)

        rename_project(conn, old_name, new_name)
        conn.close()
    except typer.Exit:
        raise
    except Exception as e:
        typer.echo(f"Error: Failed to rename project: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(
        f"Renamed project '{old_name}' to '{new_name}' in workspace '{workspace}'"
    )


@project_app.command("delete")
def project_delete(
    name: str,
    workspace: str = typer.Option("default", "--workspace", "-w"),
    force: bool = typer.Option(False, "--force", "-f"),
) -> None:
    """Delete a project and all its prompts/resources."""
    db_path = get_workspace_db_path(workspace)

    if not db_path.exists():
        typer.echo(f"Error: Workspace '{workspace}' does not exist", err=True)
        raise typer.Exit(1)

    try:
        conn = open_database(db_path)

        existing = get_project_by_name(conn, name)
        if existing is None:
            conn.close()
            typer.echo(
                f"Error: Project '{name}' does not exist in workspace '{workspace}'",
                err=True,
            )
            raise typer.Exit(1)

        if not force:
            conn.close()
            typer.echo(
                f"Warning: This will permanently delete project '{name}' and all its prompts and resources."
            )
            confirm = typer.confirm("Continue?", default=False)
            if not confirm:
                raise typer.Exit(1)
            conn = open_database(db_path)

        delete_project(conn, name)
        conn.close()
    except typer.Exit:
        raise
    except Exception as e:
        typer.echo(f"Error: Failed to delete project: {e}", err=True)
        raise typer.Exit(1)

    typer.echo(f"Deleted project '{name}' from workspace '{workspace}'")


@app.command()
def start() -> None:
    """Start the MCP server."""
    import mcpkg.mcp

    asyncio.run(mcpkg.mcp.run())


@app.command()
def prompt() -> None:
    print("Hello")


def run() -> None:
    """Run the CLI application."""
    app()
