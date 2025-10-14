import sqlite3
from datetime import datetime

from mcpkg.models import Project, Prompt, Resource


def create_project(conn: sqlite3.Connection, name: str) -> Project:
    cursor = conn.cursor()
    cursor.execute(
        "INSERT INTO projects (name) VALUES (?)",
        (name,),
    )
    conn.commit()

    project_id = cursor.lastrowid

    cursor.execute(
        "SELECT id, name, created_at FROM projects WHERE id = ?", (project_id,)
    )
    row = cursor.fetchone()

    return Project(
        id=row[0], name=row[1], created_at=datetime.fromisoformat(row[2])
    )


def list_projects(conn: sqlite3.Connection) -> list[Project]:
    cursor = conn.cursor()
    cursor.execute("SELECT id, name, created_at FROM projects ORDER BY created_at")
    rows = cursor.fetchall()

    return [
        Project(id=row[0], name=row[1], created_at=datetime.fromisoformat(row[2]))
        for row in rows
    ]


def get_project_by_name(conn: sqlite3.Connection, name: str) -> Project | None:
    cursor = conn.cursor()
    cursor.execute("SELECT id, name, created_at FROM projects WHERE name = ?", (name,))
    row = cursor.fetchone()

    if row is None:
        return None

    return Project(id=row[0], name=row[1], created_at=datetime.fromisoformat(row[2]))


def rename_project(conn: sqlite3.Connection, old_name: str, new_name: str) -> None:
    cursor = conn.cursor()
    cursor.execute("UPDATE projects SET name = ? WHERE name = ?", (new_name, old_name))
    conn.commit()


def delete_project(conn: sqlite3.Connection, name: str) -> None:
    cursor = conn.cursor()
    cursor.execute("DELETE FROM projects WHERE name = ?", (name,))
    conn.commit()


def create_prompt(
    conn: sqlite3.Connection,
    project_id: int,
    name: str,
    content: str,
    description: str | None,
) -> Prompt:
    cursor = conn.cursor()
    cursor.execute(
        "INSERT INTO prompts (project_id, name, content, description) VALUES (?, ?, ?, ?)",
        (project_id, name, content, description),
    )
    conn.commit()

    prompt_id = cursor.lastrowid

    cursor.execute(
        "SELECT id, project_id, name, content, description, created_at, updated_at FROM prompts WHERE id = ?",
        (prompt_id,),
    )
    row = cursor.fetchone()

    return Prompt(
        id=row[0],
        project_id=row[1],
        name=row[2],
        content=row[3],
        description=row[4],
        created_at=datetime.fromisoformat(row[5]),
        updated_at=datetime.fromisoformat(row[6]),
    )


def list_prompts(conn: sqlite3.Connection, project_id: int) -> list[Prompt]:
    cursor = conn.cursor()
    cursor.execute(
        "SELECT id, project_id, name, content, description, created_at, updated_at FROM prompts WHERE project_id = ? ORDER BY created_at",
        (project_id,),
    )
    rows = cursor.fetchall()

    return [
        Prompt(
            id=row[0],
            project_id=row[1],
            name=row[2],
            content=row[3],
            description=row[4],
            created_at=datetime.fromisoformat(row[5]),
            updated_at=datetime.fromisoformat(row[6]),
        )
        for row in rows
    ]


def get_prompt_by_name(
    conn: sqlite3.Connection, project_id: int, name: str
) -> Prompt | None:
    cursor = conn.cursor()
    cursor.execute(
        "SELECT id, project_id, name, content, description, created_at, updated_at FROM prompts WHERE project_id = ? AND name = ?",
        (project_id, name),
    )
    row = cursor.fetchone()

    if row is None:
        return None

    return Prompt(
        id=row[0],
        project_id=row[1],
        name=row[2],
        content=row[3],
        description=row[4],
        created_at=datetime.fromisoformat(row[5]),
        updated_at=datetime.fromisoformat(row[6]),
    )


def update_prompt(
    conn: sqlite3.Connection,
    project_id: int,
    name: str,
    content: str | None = None,
    description: str | None = None,
) -> None:
    updates = []
    params = []

    if content is not None:
        updates.append("content = ?")
        params.append(content)

    if description is not None:
        updates.append("description = ?")
        params.append(description)

    updates.append("updated_at = datetime('now')")

    params.extend([project_id, name])

    cursor = conn.cursor()
    cursor.execute(
        f"UPDATE prompts SET {', '.join(updates)} WHERE project_id = ? AND name = ?",
        params,
    )
    conn.commit()


def delete_prompt(conn: sqlite3.Connection, project_id: int, name: str) -> None:
    cursor = conn.cursor()
    cursor.execute(
        "DELETE FROM prompts WHERE project_id = ? AND name = ?", (project_id, name)
    )
    conn.commit()


def create_resource(
    conn: sqlite3.Connection,
    project_id: int,
    name: str,
    uri: str,
    content: bytes,
    mime_type: str | None,
    description: str | None,
) -> Resource:
    cursor = conn.cursor()
    cursor.execute(
        "INSERT INTO resources (project_id, name, uri, content, mime_type, description) VALUES (?, ?, ?, ?, ?, ?)",
        (project_id, name, uri, content, mime_type, description),
    )
    conn.commit()

    resource_id = cursor.lastrowid

    cursor.execute(
        "SELECT id, project_id, name, uri, content, mime_type, description, created_at, updated_at FROM resources WHERE id = ?",
        (resource_id,),
    )
    row = cursor.fetchone()

    return Resource(
        id=row[0],
        project_id=row[1],
        name=row[2],
        uri=row[3],
        content=row[4],
        mime_type=row[5],
        description=row[6],
        created_at=datetime.fromisoformat(row[7]),
        updated_at=datetime.fromisoformat(row[8]),
    )


def list_resources(conn: sqlite3.Connection, project_id: int) -> list[Resource]:
    cursor = conn.cursor()
    cursor.execute(
        "SELECT id, project_id, name, uri, content, mime_type, description, created_at, updated_at FROM resources WHERE project_id = ? ORDER BY created_at",
        (project_id,),
    )
    rows = cursor.fetchall()

    return [
        Resource(
            id=row[0],
            project_id=row[1],
            name=row[2],
            uri=row[3],
            content=row[4],
            mime_type=row[5],
            description=row[6],
            created_at=datetime.fromisoformat(row[7]),
            updated_at=datetime.fromisoformat(row[8]),
        )
        for row in rows
    ]


def get_resource_by_name(
    conn: sqlite3.Connection, project_id: int, name: str
) -> Resource | None:
    cursor = conn.cursor()
    cursor.execute(
        "SELECT id, project_id, name, uri, content, mime_type, description, created_at, updated_at FROM resources WHERE project_id = ? AND name = ?",
        (project_id, name),
    )
    row = cursor.fetchone()

    if row is None:
        return None

    return Resource(
        id=row[0],
        project_id=row[1],
        name=row[2],
        uri=row[3],
        content=row[4],
        mime_type=row[5],
        description=row[6],
        created_at=datetime.fromisoformat(row[7]),
        updated_at=datetime.fromisoformat(row[8]),
    )


def get_resource_by_uri(
    conn: sqlite3.Connection, project_id: int, uri: str
) -> Resource | None:
    cursor = conn.cursor()
    cursor.execute(
        "SELECT id, project_id, name, uri, content, mime_type, description, created_at, updated_at FROM resources WHERE project_id = ? AND uri = ?",
        (project_id, uri),
    )
    row = cursor.fetchone()

    if row is None:
        return None

    return Resource(
        id=row[0],
        project_id=row[1],
        name=row[2],
        uri=row[3],
        content=row[4],
        mime_type=row[5],
        description=row[6],
        created_at=datetime.fromisoformat(row[7]),
        updated_at=datetime.fromisoformat(row[8]),
    )


def update_resource(
    conn: sqlite3.Connection,
    project_id: int,
    name: str,
    content: bytes | None = None,
    mime_type: str | None = None,
    description: str | None = None,
) -> None:
    updates = []
    params = []

    if content is not None:
        updates.append("content = ?")
        params.append(content)

    if mime_type is not None:
        updates.append("mime_type = ?")
        params.append(mime_type)

    if description is not None:
        updates.append("description = ?")
        params.append(description)

    updates.append("updated_at = datetime('now')")

    params.extend([project_id, name])

    cursor = conn.cursor()
    cursor.execute(
        f"UPDATE resources SET {', '.join(updates)} WHERE project_id = ? AND name = ?",
        params,
    )
    conn.commit()


def delete_resource(conn: sqlite3.Connection, project_id: int, name: str) -> None:
    cursor = conn.cursor()
    cursor.execute(
        "DELETE FROM resources WHERE project_id = ? AND name = ?", (project_id, name)
    )
    conn.commit()
