import sqlite3
from pathlib import Path


def get_current_version(conn: sqlite3.Connection) -> int:
    cursor = conn.cursor()

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
    """)

    cursor.execute("SELECT COALESCE(MAX(version), 0) FROM schema_migrations")
    result = cursor.fetchone()
    return result[0] if result else 0


def apply_migrations(conn: sqlite3.Connection, migrations_dir: Path) -> None:
    current_version = get_current_version(conn)

    migration_files = sorted(migrations_dir.glob("*.sql"))

    for migration_file in migration_files:
        version = int(migration_file.stem.split("_")[0])

        if version > current_version:
            with open(migration_file) as f:
                sql = f.read()

            cursor = conn.cursor()
            cursor.executescript(sql)

            cursor.execute(
                "INSERT INTO schema_migrations (version) VALUES (?)", (version,)
            )

    conn.commit()
