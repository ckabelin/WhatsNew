use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::Result;
use crate::models::Article;

/// An article parsed from a feed, not yet persisted.
pub struct NewArticle {
    pub guid: String,
    pub title: String,
    pub link: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Inserts `items` for `feed_id`, skipping any that already exist (matched by
/// `(feed_id, guid)`). Returns only the articles that were newly inserted.
pub async fn insert_new(
    pool: &SqlitePool,
    feed_id: i64,
    items: &[NewArticle],
) -> Result<Vec<Article>> {
    let mut inserted = Vec::with_capacity(items.len());

    for item in items {
        let result = sqlx::query(
            "INSERT INTO articles (feed_id, guid, title, link, summary, published_at) \
             VALUES (?, ?, ?, ?, ?, ?) \
             ON CONFLICT (feed_id, guid) DO NOTHING",
        )
        .bind(feed_id)
        .bind(&item.guid)
        .bind(&item.title)
        .bind(&item.link)
        .bind(&item.summary)
        .bind(item.published_at)
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            let article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
                .bind(result.last_insert_rowid())
                .fetch_one(pool)
                .await?;
            inserted.push(article);
        }
    }

    Ok(inserted)
}

pub async fn list_for_topic(pool: &SqlitePool, topic_id: i64, limit: i64) -> Result<Vec<Article>> {
    Ok(sqlx::query_as::<_, Article>(
        "SELECT a.* FROM articles a \
         JOIN topic_feeds tf ON tf.feed_id = a.feed_id \
         WHERE tf.topic_id = ? \
         GROUP BY a.id \
         ORDER BY COALESCE(a.published_at, a.fetched_at) DESC \
         LIMIT ?",
    )
    .bind(topic_id)
    .bind(limit)
    .fetch_all(pool)
    .await?)
}

pub async fn get(pool: &SqlitePool, article_id: i64) -> Result<Article> {
    Ok(
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(article_id)
            .fetch_one(pool)
            .await?,
    )
}

/// Sets `article_id`'s favorite/bookmark flag and returns the updated article.
pub async fn set_favorite(pool: &SqlitePool, article_id: i64, favorite: bool) -> Result<Article> {
    sqlx::query("UPDATE articles SET is_favorite = ? WHERE id = ?")
        .bind(favorite)
        .bind(article_id)
        .execute(pool)
        .await?;
    get(pool, article_id).await
}

/// Lists every bookmarked article across all topics/feeds, most recent first.
pub async fn list_favorites(pool: &SqlitePool) -> Result<Vec<Article>> {
    Ok(sqlx::query_as::<_, Article>(
        "SELECT * FROM articles \
         WHERE is_favorite = 1 \
         ORDER BY COALESCE(published_at, fetched_at) DESC",
    )
    .fetch_all(pool)
    .await?)
}

/// Deletes all articles older than `cutoff` (by published date, falling back to
/// fetched date). Returns the number of rows deleted.
pub async fn delete_older_than(pool: &SqlitePool, cutoff: DateTime<Utc>) -> Result<u64> {
    let result = sqlx::query("DELETE FROM articles WHERE COALESCE(published_at, fetched_at) < ?")
        .bind(cutoff)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

/// Deletes the oldest articles for `topic_id` beyond `max_articles`, keeping the
/// newest. Returns the number of rows deleted.
pub async fn prune_excess_for_topic(
    pool: &SqlitePool,
    topic_id: i64,
    max_articles: i64,
) -> Result<u64> {
    let result = sqlx::query(
        "DELETE FROM articles WHERE id IN ( \
            SELECT a.id FROM articles a \
            JOIN topic_feeds tf ON tf.feed_id = a.feed_id \
            WHERE tf.topic_id = ? \
            ORDER BY COALESCE(a.published_at, a.fetched_at) DESC \
            LIMIT -1 OFFSET ? \
         )",
    )
    .bind(topic_id)
    .bind(max_articles)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
