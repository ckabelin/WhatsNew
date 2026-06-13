use tauri::State;
use whatsnew_core::db::articles;
use whatsnew_core::models::Article;
use whatsnew_core::reader::ReadableArticle;

use crate::state::AppState;

#[tauri::command]
pub async fn list_articles(
    state: State<'_, AppState>,
    topic_id: i64,
    limit: Option<i64>,
) -> Result<Vec<Article>, String> {
    articles::list_for_topic(&state.db.pool, topic_id, limit.unwrap_or(100))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_article(
    state: State<'_, AppState>,
    article_id: i64,
) -> Result<ReadableArticle, String> {
    whatsnew_core::reader::read_article(&state.db.pool, &state.http, article_id)
        .await
        .map_err(|e| e.to_string())
}

/// Triggers an immediate refresh of a topic's feeds (outside the periodic
/// scheduler) and returns the number of new articles found.
#[tauri::command]
pub async fn refresh_topic_now(state: State<'_, AppState>, topic_id: i64) -> Result<usize, String> {
    let result = whatsnew_core::refresh::refresh_topic(&state.db.pool, &state.http, topic_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.new_articles.len())
}

#[tauri::command]
pub async fn set_article_favorite(
    state: State<'_, AppState>,
    article_id: i64,
    favorite: bool,
) -> Result<Article, String> {
    articles::set_favorite(&state.db.pool, article_id, favorite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_favorite_articles(state: State<'_, AppState>) -> Result<Vec<Article>, String> {
    articles::list_favorites(&state.db.pool)
        .await
        .map_err(|e| e.to_string())
}
