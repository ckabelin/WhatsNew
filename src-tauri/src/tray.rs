use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::menu::MenuBuilder;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager, WebviewWindow, WindowEvent};

const SHOW_MENU_ID: &str = "show";
const QUIT_MENU_ID: &str = "quit";

#[derive(Clone)]
pub struct TrayState {
    quitting: Arc<AtomicBool>,
}

impl TrayState {
    fn new() -> Self {
        Self {
            quitting: Arc::new(AtomicBool::new(false)),
        }
    }

    fn is_quitting(&self) -> bool {
        self.quitting.load(Ordering::SeqCst)
    }

    fn quit(&self, app: &AppHandle) {
        self.quitting.store(true, Ordering::SeqCst);
        app.exit(0);
    }
}

pub fn setup(app: &mut App) -> tauri::Result<()> {
    let state = TrayState::new();
    app.manage(state.clone());

    let menu = MenuBuilder::new(app)
        .text(SHOW_MENU_ID, "Show WhatsNew")
        .separator()
        .text(QUIT_MENU_ID, "Quit")
        .build()?;

    let tray_state = state.clone();
    TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .cloned()
                .expect("default app icon"),
        )
        .tooltip("WhatsNew")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            SHOW_MENU_ID => show_main_window(app),
            QUIT_MENU_ID => tray_state.quit(app),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    if let Some(window) = app.get_webview_window("main") {
        register_close_to_tray(window);
    }

    Ok(())
}

fn register_close_to_tray(window: WebviewWindow) {
    window.clone().on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            let app = window.app_handle();
            let quitting = app
                .try_state::<TrayState>()
                .map(|state| state.is_quitting())
                .unwrap_or(false);

            if !quitting {
                api.prevent_close();
                let _ = window.hide();
            }
        }
    });
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}
