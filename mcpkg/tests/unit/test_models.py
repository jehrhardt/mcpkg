from datetime import datetime
from pathlib import Path

import pytest
from mcpkg.models import Project, Prompt, Resource, Workspace


def test_workspace_model() -> None:
    ws = Workspace(name="test", db_path=Path("/tmp/test.mcpkg"))
    assert ws.name == "test"
    assert ws.db_path == Path("/tmp/test.mcpkg")


def test_project_model() -> None:
    now = datetime.now()
    proj = Project(id=1, name="test-project", created_at=now)
    assert proj.id == 1
    assert proj.name == "test-project"
    assert proj.created_at == now


def test_project_model_without_id() -> None:
    now = datetime.now()
    proj = Project(id=None, name="test-project", created_at=now)
    assert proj.id is None
    assert proj.name == "test-project"


def test_prompt_model() -> None:
    now = datetime.now()
    prompt = Prompt(
        id=1,
        project_id=5,
        name="test-prompt",
        content="Test content",
        description="Test description",
        created_at=now,
        updated_at=now,
    )
    assert prompt.id == 1
    assert prompt.project_id == 5
    assert prompt.name == "test-prompt"
    assert prompt.content == "Test content"
    assert prompt.description == "Test description"
    assert prompt.created_at == now
    assert prompt.updated_at == now


def test_prompt_model_without_description() -> None:
    now = datetime.now()
    prompt = Prompt(
        id=None,
        project_id=5,
        name="test-prompt",
        content="Test content",
        description=None,
        created_at=now,
        updated_at=now,
    )
    assert prompt.description is None


def test_resource_model() -> None:
    now = datetime.now()
    resource = Resource(
        id=1,
        project_id=5,
        name="test-resource",
        uri="file:///test.md",
        content=b"Test content",
        mime_type="text/markdown",
        description="Test description",
        created_at=now,
        updated_at=now,
    )
    assert resource.id == 1
    assert resource.project_id == 5
    assert resource.name == "test-resource"
    assert resource.uri == "file:///test.md"
    assert resource.content == b"Test content"
    assert resource.mime_type == "text/markdown"
    assert resource.description == "Test description"
    assert resource.created_at == now
    assert resource.updated_at == now


def test_resource_model_without_optional_fields() -> None:
    now = datetime.now()
    resource = Resource(
        id=None,
        project_id=5,
        name="test-resource",
        uri="file:///test.md",
        content=b"Test content",
        mime_type=None,
        description=None,
        created_at=now,
        updated_at=now,
    )
    assert resource.mime_type is None
    assert resource.description is None
