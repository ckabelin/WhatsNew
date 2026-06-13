use sqlx::SqlitePool;

use crate::error::{CoreError, Result};
use crate::models::Topic;

pub async fn list(pool: &SqlitePool) -> Result<Vec<Topic>> {
    Ok(
        sqlx::query_as::<_, Topic>("SELECT * FROM topics ORDER BY name COLLATE NOCASE")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get(pool: &SqlitePool, id: i64) -> Result<Topic> {
    sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(CoreError::TopicNotFound(id))
}

/// Creates a new topic. `name` is trimmed; an empty name is rejected.
pub async fn create(pool: &SqlitePool, name: &str) -> Result<Topic> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CoreError::InvalidInput(
            "topic name must not be empty".into(),
        ));
    }

    let id = sqlx::query("INSERT INTO topics (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?
        .last_insert_rowid();

    get(pool, id).await
}

pub async fn rename(pool: &SqlitePool, id: i64, name: &str) -> Result<Topic> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CoreError::InvalidInput(
            "topic name must not be empty".into(),
        ));
    }

    sqlx::query("UPDATE topics SET name = ? WHERE id = ?")
        .bind(name)
        .bind(id)
        .execute(pool)
        .await?;

    get(pool, id).await
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query("DELETE FROM topics WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_notifications_enabled(pool: &SqlitePool, id: i64, enabled: bool) -> Result<Topic> {
    sqlx::query("UPDATE topics SET notifications_enabled = ? WHERE id = ?")
        .bind(enabled)
        .bind(id)
        .execute(pool)
        .await?;
    get(pool, id).await
}

/// Marks a topic as having completed its first refresh, so future refreshes are
/// eligible to trigger notifications (the initial backfill never notifies).
pub async fn mark_initial_refresh_done(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query("UPDATE topics SET initial_refresh_done = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
