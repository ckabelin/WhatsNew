use reqwest::Client;
use sqlx::SqlitePool;

use crate::db::{articles, feeds, topics};
use crate::error::Result;
use crate::feeds::{fetch, parse};
use crate::models::Article;

/// The outcome of refreshing a single topic's feeds.
pub struct RefreshResult {
    pub topic_id: i64,
    /// Articles inserted during this refresh, across all of the topic's feeds.
    pub new_articles: Vec<Article>,
}

/// Fetches every feed linked to `topic_id`, stores any new articles, and updates
/// each feed's caching/error state. Marks the topic's initial refresh as done.
///
/// Callers (the scheduler, manual "refresh now") are responsible for deciding
/// whether `new_articles` should trigger a notification - in particular, a
/// topic's very first refresh should not notify even if `new_articles` is
/// non-empty, since every article is "new" on first fetch.
pub async fn refresh_topic(
    pool: &SqlitePool,
    client: &Client,
    topic_id: i64,
) -> Result<RefreshResult> {
    let topic_feeds = feeds::list_for_topic(pool, topic_id).await?;
    let mut new_articles = Vec::new();

    for feed in topic_feeds {
        match fetch::fetch_feed(
            client,
            &feed.url,
            feed.etag.as_deref(),
            feed.last_modified.as_deref(),
        )
        .await
        {
            Ok(fetched) => {
                if let Some(body) = fetched.body {
                    match parse::parse_feed(body.as_bytes()) {
                        Ok(raw) => {
                            let items = parse::to_new_articles(&raw);
                            let inserted = articles::insert_new(pool, feed.id, &items).await?;
                            new_articles.extend(inserted);
                        }
                        Err(e) => {
                            feeds::record_fetch_error(pool, feed.id, &e.to_string()).await?;
                            continue;
                        }
                    }
                }
                feeds::record_fetch_success(
                    pool,
                    feed.id,
                    fetched.etag.as_deref(),
                    fetched.last_modified.as_deref(),
                )
                .await?;
            }
            Err(e) => {
                feeds::record_fetch_error(pool, feed.id, &e.to_string()).await?;
            }
        }
    }

    topics::mark_initial_refresh_done(pool, topic_id).await?;

    Ok(RefreshResult {
        topic_id,
        new_articles,
    })
}
