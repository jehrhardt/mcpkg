from typer.testing import CliRunner

import pytest
from mcpkg.cli import app
from mcpkg.storage import get_workspace_db_path


@pytest.fixture
def runner() -> CliRunner:
    return CliRunner()


@pytest.fixture(autouse=True)
def setup_test_workspace(runner: CliRunner):
    runner.invoke(app, ["workspace", "create", "test-workspace"])
    yield
    db_path = get_workspace_db_path("test-workspace")
    if db_path.exists():
        db_path.unlink()


def test_project_create(runner: CliRunner) -> None:
    result = runner.invoke(
        app, ["project", "create", "test-project", "--workspace", "test-workspace"]
    )

    assert result.exit_code == 0
    assert "Created project 'test-project'" in result.stdout


def test_project_create_duplicate(runner: CliRunner) -> None:
    runner.invoke(
        app, ["project", "create", "test-project", "--workspace", "test-workspace"]
    )

    result = runner.invoke(
        app, ["project", "create", "test-project", "--workspace", "test-workspace"]
    )

    assert result.exit_code != 0
    assert "already exists" in result.stdout or "already exists" in result.stderr


def test_project_create_invalid_name(runner: CliRunner) -> None:
    result = runner.invoke(
        app, ["project", "create", "invalid project", "--workspace", "test-workspace"]
    )

    assert result.exit_code != 0
    assert (
        "invalid characters" in result.stdout or "invalid characters" in result.stderr
    )


def test_project_list_empty(runner: CliRunner) -> None:
    result = runner.invoke(app, ["project", "list", "--workspace", "test-workspace"])

    assert result.exit_code == 0
    assert "Projects in workspace 'test-workspace'" in result.stdout


def test_project_list_shows_projects(runner: CliRunner) -> None:
    runner.invoke(
        app, ["project", "create", "project1", "--workspace", "test-workspace"]
    )
    runner.invoke(
        app, ["project", "create", "project2", "--workspace", "test-workspace"]
    )

    result = runner.invoke(app, ["project", "list", "--workspace", "test-workspace"])

    assert result.exit_code == 0
    assert "project1" in result.stdout
    assert "project2" in result.stdout


def test_project_rename(runner: CliRunner) -> None:
    runner.invoke(
        app, ["project", "create", "old-name", "--workspace", "test-workspace"]
    )

    result = runner.invoke(
        app,
        [
            "project",
            "rename",
            "old-name",
            "new-name",
            "--workspace",
            "test-workspace",
        ],
    )

    assert result.exit_code == 0
    assert "Renamed project 'old-name' to 'new-name'" in result.stdout

    list_result = runner.invoke(
        app, ["project", "list", "--workspace", "test-workspace"]
    )
    assert "new-name" in list_result.stdout
    assert "old-name" not in list_result.stdout


def test_project_delete_with_force(runner: CliRunner) -> None:
    runner.invoke(
        app, ["project", "create", "test-project", "--workspace", "test-workspace"]
    )

    result = runner.invoke(
        app,
        [
            "project",
            "delete",
            "test-project",
            "--workspace",
            "test-workspace",
            "--force",
        ],
    )

    assert result.exit_code == 0
    assert "Deleted project 'test-project'" in result.stdout


def test_project_delete_nonexistent(runner: CliRunner) -> None:
    result = runner.invoke(
        app,
        [
            "project",
            "delete",
            "nonexistent",
            "--workspace",
            "test-workspace",
            "--force",
        ],
    )

    assert result.exit_code != 0
    assert "does not exist" in result.stdout or "does not exist" in result.stderr
