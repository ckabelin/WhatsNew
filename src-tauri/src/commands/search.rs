use tauri::State;
use whatsnew_core::reader::{self, ReadableUrl};
use whatsnew_core::search::{self, SearchArticle};

use crate::state::AppState;

#[tauri::command]
pub async fn search_news(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<SearchArticle>, String> {
    search::search_news(&state.http, &query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_search_result(
    state: State<'_, AppState>,
    url: String,
    title: String,
) -> Result<ReadableUrl, String> {
    reader::read_url(&state.http, &url, &title)
        .await
        .map_err(|e| e.to_string())
}
