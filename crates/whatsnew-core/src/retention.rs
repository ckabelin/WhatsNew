use chrono::Utc;
use sqlx::SqlitePool;

use crate::db::{articles, topics};
use crate::error::Result;
use crate::models::Settings;

/// Deletes articles older than `settings.retention_days`, then trims each topic's
/// articles down to `settings.max_articles_per_topic`, keeping the newest.
pub async fn prune(pool: &SqlitePool, settings: &Settings) -> Result<()> {
    let cutoff = Utc::now() - chrono::Duration::days(settings.retention_days);
    articles::delete_older_than(pool, cutoff).await?;

    for topic in topics::list(pool).await? {
        articles::prune_excess_for_topic(pool, topic.id, settings.max_articles_per_topic).await?;
    }

    Ok(())
}
