use anyhow::{Context, Result};
use std::fs::{DirEntry, read_dir, read_to_string};
use tracing::info;
use turso::{Connection, Database};

pub async fn run_migrations(db: &Database) -> Result<Connection> {
    let mut conn = db
        .connect()
        .context("Failed to load connection with database")?;

    // Create a migrations tracking table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .await?;

    // Read migration files from disk (or embed them)
    let mut migration_files: Vec<DirEntry> = read_dir("../migrations")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();

    // Sort by filename to ensure order
    migration_files.sort_by_key(|e| e.file_name());

    for entry in migration_files {
        let name = entry.file_name().to_string_lossy().to_string();
        let sql = read_to_string(entry.path())?;

        // Check if already applied
        let already_applied: bool = conn
            .query(
                "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?1)",
                [name.as_str()],
            )
            .await?
            .next()
            .await?
            .map(|row| row.get::<bool>(0).unwrap_or(false))
            .unwrap_or(false);

        if already_applied {
            info!("Skipping already applied migration: {name}");
            continue;
        }

        // Execute the migration in a transaction
        let tx = conn.transaction().await?;
        tx.execute_batch(&sql).await?;
        tx.execute(
            "INSERT INTO _migrations (name) VALUES (?1)",
            [name.as_str()],
        )
        .await?;
        tx.commit().await?;

        info!("Applied migration: {name}");
    }

    Ok(conn)
}
