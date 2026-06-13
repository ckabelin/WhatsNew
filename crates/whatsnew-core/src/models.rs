use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user-defined, free-text topic to aggregate news for (e.g. "rust programming").
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: i64,
    pub name: String,
    pub notifications_enabled: bool,
    pub initial_refresh_done: bool,
    pub sort_order: i64,
    pub created_at: DateTime<Utc>,
}

/// An RSS/Atom feed, shared across any topics it has been linked to.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Feed {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub site_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub last_fetched_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

/// A single article/entry pulled from a feed.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    pub id: i64,
    pub feed_id: i64,
    pub guid: String,
    pub title: String,
    pub link: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub fetched_at: DateTime<Utc>,
    pub is_favorite: bool,
}

/// Global application settings (single row).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Settings {
    pub retention_days: i64,
    pub max_articles_per_topic: i64,
    pub max_cache_size_mb: i64,
    pub refresh_interval_minutes: i64,
    pub notifications_enabled: bool,
}
