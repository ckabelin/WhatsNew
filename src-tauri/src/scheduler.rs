use std::time::Duration;

use tauri::{AppHandle, Manager};
use whatsnew_core::db::{settings, topics};

use crate::notify;
use crate::state::AppState;

/// Spawns the background task that periodically refreshes every topic's feeds,
/// prunes old articles per the retention settings, and shows notifications for
/// topics that received new articles (skipping each topic's initial backfill).
pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            run_refresh_cycle(&app).await;

            let interval_minutes = {
                let state = app.state::<AppState>();
                settings::get(&state.db.pool)
                    .await
                    .map(|s| s.refresh_interval_minutes)
                    .unwrap_or(60)
                    .max(1)
            };

            tokio::time::sleep(Duration::from_secs(interval_minutes as u64 * 60)).await;
        }
    });
}

async fn run_refresh_cycle(app: &AppHandle) {
    let state = app.state::<AppState>();

    let Ok(all_topics) = topics::list(&state.db.pool).await else {
        return;
    };
    let Ok(current_settings) = settings::get(&state.db.pool).await else {
        return;
    };

    for topic in &all_topics {
        let was_initial_refresh = !topic.initial_refresh_done;

        match whatsnew_core::refresh::refresh_topic(&state.db.pool, &state.http, topic.id).await {
            Ok(result) => {
                if !was_initial_refresh
                    && !result.new_articles.is_empty()
                    && topic.notifications_enabled
                    && current_settings.notifications_enabled
                {
                    notify::notify_new_articles(app, &topic.name, result.new_articles.len());
                }
            }
            Err(e) => {
                eprintln!("refresh failed for topic {}: {e}", topic.id);
            }
        }
    }

    if let Err(e) = whatsnew_core::retention::prune(&state.db.pool, &current_settings).await {
        eprintln!("retention prune failed: {e}");
    }
}
