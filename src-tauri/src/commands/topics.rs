use tauri::State;
use whatsnew_core::db::topics;
use whatsnew_core::matching;
use whatsnew_core::models::Topic;

use crate::state::AppState;

#[tauri::command]
pub async fn list_topics(state: State<'_, AppState>) -> Result<Vec<Topic>, String> {
    topics::list(&state.db.pool)
        .await
        .map_err(|e| e.to_string())
}

/// Creates a topic and links any feeds from the curated directory whose keywords
/// match the topic's free text, plus a per-topic Google News search feed as a
/// catch-all. Returns the created topic; linked feeds can be fetched via
/// `list_topic_feeds`.
#[tauri::command]
pub async fn create_topic(state: State<'_, AppState>, name: String) -> Result<Topic, String> {
    let topic = topics::create(&state.db.pool, &name)
        .await
        .map_err(|e| e.to_string())?;

    matching::ensure_feeds_for_topic(&state.db.pool, &topic)
        .await
        .map_err(|e| e.to_string())?;

    Ok(topic)
}

#[tauri::command]
pub async fn rename_topic(
    state: State<'_, AppState>,
    id: i64,
    name: String,
) -> Result<Topic, String> {
    topics::rename(&state.db.pool, id, &name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reorder_topics(
    state: State<'_, AppState>,
    topic_ids: Vec<i64>,
) -> Result<Vec<Topic>, String> {
    topics::reorder(&state.db.pool, &topic_ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_topic(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    topics::delete(&state.db.pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_topic_notifications(
    state: State<'_, AppState>,
    id: i64,
    enabled: bool,
) -> Result<Topic, String> {
    topics::set_notifications_enabled(&state.db.pool, id, enabled)
        .await
        .map_err(|e| e.to_string())
}
