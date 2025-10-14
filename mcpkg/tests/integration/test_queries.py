import sqlite3
from datetime import datetime
from pathlib import Path

import pytest
from mcpkg.database import initialize_database
from mcpkg.migrations import apply_migrations
from mcpkg.models import Project, Prompt, Resource
from mcpkg.queries import (
    create_project,
    create_prompt,
    create_resource,
    delete_project,
    delete_prompt,
    delete_resource,
    get_project_by_name,
    get_prompt_by_name,
    get_resource_by_name,
    get_resource_by_uri,
    list_projects,
    list_prompts,
    list_resources,
    rename_project,
    update_prompt,
    update_resource,
)


@pytest.fixture
def db_path(tmp_path: Path) -> Path:
    return tmp_path / "test.mcpkg"


@pytest.fixture
def db_connection(db_path: Path):  # type: ignore[misc]
    conn = initialize_database(db_path)
    migrations_dir = Path(__file__).parent.parent.parent / "migrations"
    apply_migrations(conn, migrations_dir)
    yield conn
    conn.close()


def test_create_project(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")

    assert project.id is not None
    assert project.name == "test-project"
    assert isinstance(project.created_at, datetime)


def test_list_projects_empty(db_connection: sqlite3.Connection) -> None:
    projects = list_projects(db_connection)
    assert projects == []


def test_list_projects_returns_all(db_connection: sqlite3.Connection) -> None:
    create_project(db_connection, "project1")
    create_project(db_connection, "project2")

    projects = list_projects(db_connection)

    assert len(projects) == 2
    assert projects[0].name == "project1"
    assert projects[1].name == "project2"


def test_get_project_by_name(db_connection: sqlite3.Connection) -> None:
    create_project(db_connection, "test-project")

    project = get_project_by_name(db_connection, "test-project")

    assert project is not None
    assert project.name == "test-project"


def test_get_project_by_name_not_found(db_connection: sqlite3.Connection) -> None:
    project = get_project_by_name(db_connection, "nonexistent")
    assert project is None


def test_rename_project(db_connection: sqlite3.Connection) -> None:
    create_project(db_connection, "old-name")

    rename_project(db_connection, "old-name", "new-name")

    project = get_project_by_name(db_connection, "new-name")
    assert project is not None
    assert project.name == "new-name"

    old_project = get_project_by_name(db_connection, "old-name")
    assert old_project is None


def test_delete_project(db_connection: sqlite3.Connection) -> None:
    create_project(db_connection, "test-project")

    delete_project(db_connection, "test-project")

    project = get_project_by_name(db_connection, "test-project")
    assert project is None


def test_create_prompt(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")

    prompt = create_prompt(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-prompt",
        "Test content",
        "Test description",
    )

    assert prompt.id is not None
    assert prompt.project_id == project.id
    assert prompt.name == "test-prompt"
    assert prompt.content == "Test content"
    assert prompt.description == "Test description"


def test_list_prompts(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")

    create_prompt(db_connection, project.id, "prompt1", "Content1", None)  # type: ignore[arg-type]
    create_prompt(db_connection, project.id, "prompt2", "Content2", None)  # type: ignore[arg-type]

    prompts = list_prompts(db_connection, project.id)  # type: ignore[arg-type]

    assert len(prompts) == 2
    assert prompts[0].name == "prompt1"
    assert prompts[1].name == "prompt2"


def test_get_prompt_by_name(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_prompt(db_connection, project.id, "test-prompt", "Content", None)  # type: ignore[arg-type]

    prompt = get_prompt_by_name(db_connection, project.id, "test-prompt")  # type: ignore[arg-type]

    assert prompt is not None
    assert prompt.name == "test-prompt"


def test_update_prompt(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_prompt(db_connection, project.id, "test-prompt", "Old content", None)  # type: ignore[arg-type]

    update_prompt(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-prompt",
        content="New content",
        description="New description",
    )

    prompt = get_prompt_by_name(db_connection, project.id, "test-prompt")  # type: ignore[arg-type]
    assert prompt is not None
    assert prompt.content == "New content"
    assert prompt.description == "New description"


def test_delete_prompt(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_prompt(db_connection, project.id, "test-prompt", "Content", None)  # type: ignore[arg-type]

    delete_prompt(db_connection, project.id, "test-prompt")  # type: ignore[arg-type]

    prompt = get_prompt_by_name(db_connection, project.id, "test-prompt")  # type: ignore[arg-type]
    assert prompt is None


def test_create_resource(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")

    resource = create_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        "file:///test.md",
        b"Test content",
        "text/markdown",
        "Test description",
    )

    assert resource.id is not None
    assert resource.project_id == project.id
    assert resource.name == "test-resource"
    assert resource.uri == "file:///test.md"
    assert resource.content == b"Test content"
    assert resource.mime_type == "text/markdown"
    assert resource.description == "Test description"


def test_list_resources(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")

    create_resource(
        db_connection, project.id, "resource1", "file:///r1.md", b"Content1", None, None  # type: ignore[arg-type]
    )
    create_resource(
        db_connection, project.id, "resource2", "file:///r2.md", b"Content2", None, None  # type: ignore[arg-type]
    )

    resources = list_resources(db_connection, project.id)  # type: ignore[arg-type]

    assert len(resources) == 2
    assert resources[0].name == "resource1"
    assert resources[1].name == "resource2"


def test_get_resource_by_name(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        "file:///test.md",
        b"Content",
        None,
        None,
    )

    resource = get_resource_by_name(db_connection, project.id, "test-resource")  # type: ignore[arg-type]

    assert resource is not None
    assert resource.name == "test-resource"


def test_get_resource_by_uri(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        "file:///test.md",
        b"Content",
        None,
        None,
    )

    resource = get_resource_by_uri(db_connection, project.id, "file:///test.md")  # type: ignore[arg-type]

    assert resource is not None
    assert resource.uri == "file:///test.md"


def test_update_resource(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        "file:///test.md",
        b"Old content",
        None,
        None,
    )

    update_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        content=b"New content",
        mime_type="text/markdown",
        description="New description",
    )

    resource = get_resource_by_name(db_connection, project.id, "test-resource")  # type: ignore[arg-type]
    assert resource is not None
    assert resource.content == b"New content"
    assert resource.mime_type == "text/markdown"
    assert resource.description == "New description"


def test_delete_resource(db_connection: sqlite3.Connection) -> None:
    project = create_project(db_connection, "test-project")
    create_resource(
        db_connection,
        project.id,  # type: ignore[arg-type]
        "test-resource",
        "file:///test.md",
        b"Content",
        None,
        None,
    )

    delete_resource(db_connection, project.id, "test-resource")  # type: ignore[arg-type]

    resource = get_resource_by_name(db_connection, project.id, "test-resource")  # type: ignore[arg-type]
    assert resource is None
