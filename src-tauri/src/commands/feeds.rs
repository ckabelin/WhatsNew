use tauri::State;
use whatsnew_core::db::feeds;
use whatsnew_core::feeds::{discovery, fetch};
use whatsnew_core::models::Feed;

use crate::state::AppState;

#[tauri::command]
pub async fn list_topic_feeds(
    state: State<'_, AppState>,
    topic_id: i64,
) -> Result<Vec<Feed>, String> {
    feeds::list_for_topic(&state.db.pool, topic_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_feed_to_topic(
    state: State<'_, AppState>,
    topic_id: i64,
    url: String,
) -> Result<Feed, String> {
    let feed = feeds::get_or_create(&state.db.pool, &url, None, None)
        .await
        .map_err(|e| e.to_string())?;
    feeds::link_to_topic(&state.db.pool, topic_id, feed.id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(feed)
}

#[tauri::command]
pub async fn remove_feed_from_topic(
    state: State<'_, AppState>,
    topic_id: i64,
    feed_id: i64,
) -> Result<(), String> {
    feeds::unlink_from_topic(&state.db.pool, topic_id, feed_id)
        .await
        .map_err(|e| e.to_string())
}

/// Fetches `site_url` and scans it for RSS/Atom `<link rel="alternate">` tags,
/// returning any feeds found for the user to add manually.
#[tauri::command]
pub async fn discover_feeds_for_site(
    state: State<'_, AppState>,
    site_url: String,
) -> Result<Vec<discovery::DiscoveredFeed>, String> {
    let html = fetch::fetch_html(&state.http, &site_url)
        .await
        .map_err(|e| e.to_string())?;
    discovery::discover_feeds(&site_url, &html).map_err(|e| e.to_string())
}
