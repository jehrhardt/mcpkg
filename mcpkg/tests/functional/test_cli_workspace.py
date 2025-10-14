from pathlib import Path
from typer.testing import CliRunner

import pytest
from mcpkg.cli import app
from mcpkg.storage import get_workspace_db_path


@pytest.fixture
def runner() -> CliRunner:
    return CliRunner()


@pytest.fixture(autouse=True)
def clean_test_workspaces():
    yield

    test_workspaces = ["test-workspace", "my-project", "client-work"]
    for ws_name in test_workspaces:
        db_path = get_workspace_db_path(ws_name)
        if db_path.exists():
            db_path.unlink()


def test_workspace_list_empty(runner: CliRunner) -> None:
    result = runner.invoke(app, ["workspace", "list"])

    assert result.exit_code == 0
    assert "Available workspaces:" in result.stdout


def test_workspace_create(runner: CliRunner) -> None:
    result = runner.invoke(app, ["workspace", "create", "test-workspace"])

    assert result.exit_code == 0
    assert "Created workspace 'test-workspace'" in result.stdout

    db_path = get_workspace_db_path("test-workspace")
    assert db_path.exists()


def test_workspace_create_duplicate(runner: CliRunner) -> None:
    runner.invoke(app, ["workspace", "create", "test-workspace"])

    result = runner.invoke(app, ["workspace", "create", "test-workspace"])

    assert result.exit_code != 0
    assert "already exists" in result.stdout or "already exists" in result.stderr


def test_workspace_create_invalid_name(runner: CliRunner) -> None:
    result = runner.invoke(app, ["workspace", "create", "invalid workspace"])

    assert result.exit_code != 0
    assert "invalid characters" in result.stdout or "invalid characters" in result.stderr


def test_workspace_list_shows_workspaces(runner: CliRunner) -> None:
    runner.invoke(app, ["workspace", "create", "test-workspace"])
    runner.invoke(app, ["workspace", "create", "my-project"])

    result = runner.invoke(app, ["workspace", "list"])

    assert result.exit_code == 0
    assert "test-workspace" in result.stdout
    assert "my-project" in result.stdout


def test_workspace_delete_with_force(runner: CliRunner) -> None:
    runner.invoke(app, ["workspace", "create", "test-workspace"])

    result = runner.invoke(app, ["workspace", "delete", "test-workspace", "--force"])

    assert result.exit_code == 0
    assert "Deleted workspace 'test-workspace'" in result.stdout

    db_path = get_workspace_db_path("test-workspace")
    assert not db_path.exists()


def test_workspace_delete_nonexistent(runner: CliRunner) -> None:
    result = runner.invoke(app, ["workspace", "delete", "nonexistent", "--force"])

    assert result.exit_code != 0
    assert "does not exist" in result.stdout or "does not exist" in result.stderr


def test_default_workspace_auto_creation(runner: CliRunner) -> None:
    result = runner.invoke(app, ["workspace", "list"])

    assert result.exit_code == 0
    assert "default" in result.stdout

    db_path = get_workspace_db_path("default")
    assert db_path.exists()
