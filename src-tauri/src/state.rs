use std::sync::Arc;

use whatsnew_core::Db;

/// Shared application state managed by Tauri and accessed from commands and the
/// background refresh scheduler.
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Db>,
    pub http: reqwest::Client,
}
