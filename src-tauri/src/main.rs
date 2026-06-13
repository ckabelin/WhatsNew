// Prevents an additional console window from opening on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod notify;
mod scheduler;
mod state;
mod tray;

use std::sync::Arc;

use tauri::Manager;
use whatsnew_core::Db;

use crate::state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("app data dir should be resolvable");
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("whatsnew.db");

            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db = Db::connect(&db_path)
                    .await
                    .expect("failed to open database");
                let http = whatsnew_core::feeds::fetch::build_client()
                    .expect("failed to build http client");
                app_handle.manage(AppState {
                    db: Arc::new(db),
                    http,
                });
            });

            scheduler::spawn(app.handle().clone());
            tray::setup(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::topics::list_topics,
            commands::topics::create_topic,
            commands::topics::rename_topic,
            commands::topics::reorder_topics,
            commands::topics::delete_topic,
            commands::topics::set_topic_notifications,
            commands::articles::list_articles,
            commands::articles::read_article,
            commands::articles::refresh_topic_now,
            commands::feeds::list_topic_feeds,
            commands::feeds::add_feed_to_topic,
            commands::feeds::remove_feed_from_topic,
            commands::feeds::discover_feeds_for_site,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
