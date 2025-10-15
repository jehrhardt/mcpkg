from dataclasses import dataclass
from datetime import datetime
from pathlib import Path


@dataclass
class Workspace:
    name: str
    db_path: Path


@dataclass
class Project:
    id: int | None
    name: str
    created_at: datetime


@dataclass
class Prompt:
    id: int | None
    project_id: int
    name: str
    content: str
    description: str | None
    created_at: datetime
    updated_at: datetime


@dataclass
class Resource:
    id: int | None
    project_id: int
    name: str
    uri: str
    content: bytes
    mime_type: str | None
    description: str | None
    created_at: datetime
    updated_at: datetime
