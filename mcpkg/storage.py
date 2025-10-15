from pathlib import Path

from platformdirs import user_data_dir


def get_data_dir() -> Path:
    data_dir = Path(user_data_dir("mcpkg", "mcpkg"))
    data_dir.mkdir(parents=True, exist_ok=True)
    return data_dir


def get_workspace_db_path(workspace_name: str) -> Path:
    return get_data_dir() / f"{workspace_name}.mcpkg"
