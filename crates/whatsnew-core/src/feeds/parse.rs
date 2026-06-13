use feed_rs::model::Feed as RawFeed;

use crate::db::articles::NewArticle;
use crate::error::Result;

/// Parses raw RSS/Atom/JSON-Feed bytes into `feed-rs`'s unified feed model.
pub fn parse_feed(bytes: &[u8]) -> Result<RawFeed> {
    Ok(feed_rs::parser::parse(bytes)?)
}

/// Returns the feed's display title, if present.
pub fn feed_title(feed: &RawFeed) -> Option<String> {
    feed.title.as_ref().map(|t| t.content.clone())
}

/// Converts a parsed feed's entries into rows ready for `articles::insert_new`.
pub fn to_new_articles(feed: &RawFeed) -> Vec<NewArticle> {
    feed.entries
        .iter()
        .map(|entry| {
            let title = entry
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_else(|| "(untitled)".to_string());

            let link = entry.links.first().map(|l| l.href.clone());

            let summary = entry
                .summary
                .as_ref()
                .map(|s| s.content.clone())
                .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()));

            let published_at = entry.published.or(entry.updated);

            NewArticle {
                guid: entry.id.clone(),
                title,
                link,
                summary,
                published_at,
            }
        })
        .collect()
}
