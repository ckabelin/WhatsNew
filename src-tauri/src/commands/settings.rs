use tauri::State;
use whatsnew_core::db::settings as settings_db;
use whatsnew_core::models::Settings;

use crate::state::AppState;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    settings_db::get(&state.db.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings, String> {
    settings_db::update(&state.db.pool, &settings)
        .await
        .map_err(|e| e.to_string())
}
