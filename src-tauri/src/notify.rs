use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// Shows a toast notification for `count` new articles in `topic_name`. No-op if
/// `count` is zero.
pub fn notify_new_articles(app: &AppHandle, topic_name: &str, count: usize) {
    if count == 0 {
        return;
    }

    let body = if count == 1 {
        "1 new article".to_string()
    } else {
        format!("{count} new articles")
    };

    let _ = app
        .notification()
        .builder()
        .title(topic_name)
        .body(body)
        .show();
}
