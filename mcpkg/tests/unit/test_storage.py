from pathlib import Path

from mcpkg.storage import get_data_dir, get_workspace_db_path


def test_get_data_dir_returns_path() -> None:
    data_dir = get_data_dir()
    assert isinstance(data_dir, Path)
    assert "mcpkg" in str(data_dir)


def test_get_workspace_db_path() -> None:
    db_path = get_workspace_db_path("test-workspace")
    assert isinstance(db_path, Path)
    assert db_path.name == "test-workspace.mcpkg"
    assert "mcpkg" in str(db_path.parent)


def test_get_workspace_db_path_with_special_name() -> None:
    db_path = get_workspace_db_path("my_project.2024")
    assert db_path.name == "my_project.2024.mcpkg"
