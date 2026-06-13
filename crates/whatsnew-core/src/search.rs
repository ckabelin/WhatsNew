use chrono::{DateTime, Utc};
use feed_rs::model::Entry;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};
use crate::feeds::{fetch, parse};
use crate::matching::directory::google_news_feed;

/// A single ad-hoc news search result, not persisted to the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArticle {
    pub title: String,
    pub link: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Runs an ad-hoc news search for `query` via Google News RSS and returns the
/// resulting articles without storing anything in the database.
pub async fn search_news(client: &Client, query: &str) -> Result<Vec<SearchArticle>> {
    let query = query.trim();
    if query.is_empty() {
        return Err(CoreError::InvalidInput(
            "search query must not be empty".to_string(),
        ));
    }

    let feed = google_news_feed(query);
    let fetched = fetch::fetch_feed(client, &feed.url, None, None).await?;
    let raw = parse::parse_feed(fetched.body.unwrap_or_default().as_bytes())?;

    Ok(raw.entries.iter().map(to_search_article).collect())
}

fn to_search_article(entry: &Entry) -> SearchArticle {
    let title = entry
        .title
        .as_ref()
        .map(|t| t.content.clone())
        .unwrap_or_else(|| "(untitled)".to_string());

    let link = entry.links.first().map(|l| l.href.clone());

    let raw_summary = entry
        .summary
        .as_ref()
        .map(|s| s.content.clone())
        .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()));

    let link = parse::resolve_link(link, raw_summary.as_deref());
    let summary = parse::clean_summary(&title, raw_summary);

    let published_at = entry.published.or(entry.updated);

    SearchArticle {
        title,
        link,
        summary,
        published_at,
    }
}
