use sqlx::SqlitePool;

use crate::error::{CoreError, Result};
use crate::models::Feed;

pub async fn get(pool: &SqlitePool, id: i64) -> Result<Feed> {
    sqlx::query_as::<_, Feed>("SELECT * FROM feeds WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(CoreError::FeedNotFound(id))
}

pub async fn find_by_url(pool: &SqlitePool, url: &str) -> Result<Option<Feed>> {
    Ok(
        sqlx::query_as::<_, Feed>("SELECT * FROM feeds WHERE url = ?")
            .bind(url)
            .fetch_optional(pool)
            .await?,
    )
}

/// Returns the feed for `url`, inserting it if it doesn't exist yet.
pub async fn get_or_create(
    pool: &SqlitePool,
    url: &str,
    title: Option<&str>,
    site_url: Option<&str>,
) -> Result<Feed> {
    if let Some(feed) = find_by_url(pool, url).await? {
        return Ok(feed);
    }

    let id = sqlx::query("INSERT INTO feeds (url, title, site_url) VALUES (?, ?, ?)")
        .bind(url)
        .bind(title)
        .bind(site_url)
        .execute(pool)
        .await?
        .last_insert_rowid();

    get(pool, id).await
}

pub async fn list_for_topic(pool: &SqlitePool, topic_id: i64) -> Result<Vec<Feed>> {
    Ok(sqlx::query_as::<_, Feed>(
        "SELECT f.* FROM feeds f \
         JOIN topic_feeds tf ON tf.feed_id = f.id \
         WHERE tf.topic_id = ? \
         ORDER BY f.title COLLATE NOCASE",
    )
    .bind(topic_id)
    .fetch_all(pool)
    .await?)
}

pub async fn link_to_topic(pool: &SqlitePool, topic_id: i64, feed_id: i64) -> Result<()> {
    sqlx::query("INSERT OR IGNORE INTO topic_feeds (topic_id, feed_id) VALUES (?, ?)")
        .bind(topic_id)
        .bind(feed_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn unlink_from_topic(pool: &SqlitePool, topic_id: i64, feed_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM topic_feeds WHERE topic_id = ? AND feed_id = ?")
        .bind(topic_id)
        .bind(feed_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Records a successful fetch, updating caching headers and clearing any prior error.
pub async fn record_fetch_success(
    pool: &SqlitePool,
    id: i64,
    etag: Option<&str>,
    last_modified: Option<&str>,
) -> Result<()> {
    sqlx::query(
        "UPDATE feeds SET last_fetched_at = datetime('now'), last_error = NULL, \
         etag = COALESCE(?, etag), last_modified = COALESCE(?, last_modified) WHERE id = ?",
    )
    .bind(etag)
    .bind(last_modified)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn record_fetch_error(pool: &SqlitePool, id: i64, error: &str) -> Result<()> {
    sqlx::query("UPDATE feeds SET last_fetched_at = datetime('now'), last_error = ? WHERE id = ?")
        .bind(error)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
