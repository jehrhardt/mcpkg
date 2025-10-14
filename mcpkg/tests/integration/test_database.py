import sqlite3
from pathlib import Path

import pytest
from mcpkg.database import close_database, initialize_database, open_database


@pytest.fixture
def db_path(tmp_path: Path) -> Path:
    return tmp_path / "test.mcpkg"


def test_initialize_database_creates_file(db_path: Path) -> None:
    assert not db_path.exists()

    conn = initialize_database(db_path)
    assert db_path.exists()

    conn.close()


def test_initialize_database_enables_wal_mode(db_path: Path) -> None:
    conn = initialize_database(db_path)

    cursor = conn.cursor()
    cursor.execute("PRAGMA journal_mode")
    journal_mode = cursor.fetchone()[0]

    assert journal_mode.upper() == "WAL"

    conn.close()


def test_initialize_database_enables_foreign_keys(db_path: Path) -> None:
    conn = initialize_database(db_path)

    cursor = conn.cursor()
    cursor.execute("PRAGMA foreign_keys")
    foreign_keys = cursor.fetchone()[0]

    assert foreign_keys == 1

    conn.close()


def test_open_database_returns_connection(db_path: Path) -> None:
    initialize_database(db_path)

    conn = open_database(db_path)
    assert isinstance(conn, sqlite3.Connection)

    conn.close()


def test_open_database_enables_wal_mode(db_path: Path) -> None:
    initialize_database(db_path)

    conn = open_database(db_path)

    cursor = conn.cursor()
    cursor.execute("PRAGMA journal_mode")
    journal_mode = cursor.fetchone()[0]

    assert journal_mode.upper() == "WAL"

    conn.close()


def test_open_database_enables_foreign_keys(db_path: Path) -> None:
    initialize_database(db_path)

    conn = open_database(db_path)

    cursor = conn.cursor()
    cursor.execute("PRAGMA foreign_keys")
    foreign_keys = cursor.fetchone()[0]

    assert foreign_keys == 1

    conn.close()


def test_close_database(db_path: Path) -> None:
    conn = initialize_database(db_path)

    close_database(conn)

    with pytest.raises(sqlite3.ProgrammingError):
        conn.execute("SELECT 1")
