import sqlite3
from pathlib import Path


def open_database(db_path: Path) -> sqlite3.Connection:
    conn = sqlite3.connect(db_path)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA foreign_keys=ON")
    return conn


def initialize_database(db_path: Path) -> sqlite3.Connection:
    conn = open_database(db_path)
    return conn


def close_database(conn: sqlite3.Connection) -> None:
    conn.close()
