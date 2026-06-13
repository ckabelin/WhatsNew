use sqlx::SqlitePool;

use crate::error::Result;
use crate::models::Settings;

pub async fn get(pool: &SqlitePool) -> Result<Settings> {
    Ok(sqlx::query_as::<_, Settings>(
        "SELECT retention_days, max_articles_per_topic, max_cache_size_mb, \
         refresh_interval_minutes, notifications_enabled FROM settings WHERE id = 1",
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update(pool: &SqlitePool, settings: &Settings) -> Result<Settings> {
    sqlx::query(
        "UPDATE settings SET retention_days = ?, max_articles_per_topic = ?, \
         max_cache_size_mb = ?, refresh_interval_minutes = ?, notifications_enabled = ? \
         WHERE id = 1",
    )
    .bind(settings.retention_days)
    .bind(settings.max_articles_per_topic)
    .bind(settings.max_cache_size_mb)
    .bind(settings.refresh_interval_minutes)
    .bind(settings.notifications_enabled)
    .execute(pool)
    .await?;

    get(pool).await
}
