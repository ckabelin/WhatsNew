use sqlx::SqlitePool;

use crate::error::{CoreError, Result};
use crate::models::Topic;

pub async fn list(pool: &SqlitePool) -> Result<Vec<Topic>> {
    Ok(sqlx::query_as::<_, Topic>(
        "SELECT * FROM topics ORDER BY sort_order ASC, name COLLATE NOCASE ASC",
    )
    .fetch_all(pool)
    .await?)
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

    let sort_order: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sort_order), 0) + 1 FROM topics")
        .fetch_one(pool)
        .await?;

    let id = sqlx::query("INSERT INTO topics (name, sort_order) VALUES (?, ?)")
        .bind(name)
        .bind(sort_order)
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

pub async fn reorder(pool: &SqlitePool, topic_ids: &[i64]) -> Result<Vec<Topic>> {
    let existing = list(pool).await?;
    if existing.len() != topic_ids.len() {
        return Err(CoreError::InvalidInput(
            "topic order must include every topic exactly once".into(),
        ));
    }

    let mut expected: Vec<_> = existing.iter().map(|topic| topic.id).collect();
    let mut provided = topic_ids.to_vec();
    expected.sort_unstable();
    provided.sort_unstable();
    if expected != provided {
        return Err(CoreError::InvalidInput(
            "topic order must include every topic exactly once".into(),
        ));
    }

    let mut tx = pool.begin().await?;
    for (index, topic_id) in topic_ids.iter().enumerate() {
        sqlx::query("UPDATE topics SET sort_order = ? WHERE id = ?")
            .bind(index as i64)
            .bind(topic_id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;

    list(pool).await
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
