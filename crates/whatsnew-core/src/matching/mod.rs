pub mod directory;

use sqlx::SqlitePool;

use crate::db::feeds;
use crate::error::Result;
use crate::models::Topic;

/// Links `topic` to its curated directory matches (if any) plus a per-topic
/// Google News search feed, which acts as a catch-all so that any free-text
/// topic - even ones the curated directory has no keywords for - still
/// surfaces relevant, recent articles.
pub async fn ensure_feeds_for_topic(pool: &SqlitePool, topic: &Topic) -> Result<()> {
    let dir = directory::load_directory();
    for feed in directory::match_topic(&dir, &topic.name) {
        let f = feeds::get_or_create(pool, &feed.url, Some(&feed.title), Some(&feed.site)).await?;
        feeds::link_to_topic(pool, topic.id, f.id).await?;
    }

    let news = directory::google_news_feed(&topic.name);
    let f = feeds::get_or_create(pool, &news.url, Some(&news.title), Some(&news.site)).await?;
    feeds::link_to_topic(pool, topic.id, f.id).await?;

    Ok(())
}
