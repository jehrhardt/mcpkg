import sqlite3
from pathlib import Path

import pytest
from mcpkg.database import initialize_database
from mcpkg.migrations import apply_migrations, get_current_version


@pytest.fixture
def db_path(tmp_path: Path) -> Path:
    return tmp_path / "test.mcpkg"


@pytest.fixture
def db_connection(db_path: Path):  # type: ignore[misc]
    conn = initialize_database(db_path)
    yield conn
    conn.close()


@pytest.fixture
def migrations_dir() -> Path:
    return Path(__file__).parent.parent.parent / "migrations"


def test_apply_migrations_creates_schema_migrations_table(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)

    cursor = db_connection.cursor()
    cursor.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_migrations'"
    )
    result = cursor.fetchone()

    assert result is not None
    assert result[0] == "schema_migrations"


def test_apply_migrations_applies_initial_schema(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)

    cursor = db_connection.cursor()

    cursor.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='projects'"
    )
    assert cursor.fetchone() is not None

    cursor.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='prompts'"
    )
    assert cursor.fetchone() is not None

    cursor.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='resources'"
    )
    assert cursor.fetchone() is not None


def test_apply_migrations_records_version(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)

    cursor = db_connection.cursor()
    cursor.execute("SELECT version FROM schema_migrations WHERE version = 1")
    result = cursor.fetchone()

    assert result is not None
    assert result[0] == 1


def test_apply_migrations_is_idempotent(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)
    apply_migrations(db_connection, migrations_dir)

    cursor = db_connection.cursor()
    cursor.execute("SELECT COUNT(*) FROM schema_migrations WHERE version = 1")
    count = cursor.fetchone()[0]

    assert count == 1


def test_get_current_version_returns_zero_for_new_database(
    db_connection: sqlite3.Connection
) -> None:
    version = get_current_version(db_connection)
    assert version == 0


def test_get_current_version_returns_highest_version(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)

    version = get_current_version(db_connection)
    assert version >= 1


def test_apply_migrations_skips_already_applied(
    db_connection: sqlite3.Connection, migrations_dir: Path
) -> None:
    apply_migrations(db_connection, migrations_dir)

    cursor = db_connection.cursor()
    cursor.execute("SELECT COUNT(*) FROM schema_migrations")
    count_before = cursor.fetchone()[0]

    apply_migrations(db_connection, migrations_dir)

    cursor.execute("SELECT COUNT(*) FROM schema_migrations")
    count_after = cursor.fetchone()[0]

    assert count_before == count_after
