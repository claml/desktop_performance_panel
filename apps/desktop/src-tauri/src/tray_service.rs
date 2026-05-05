use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

use crate::settings_service;
use crate::window_service;

const ID_SHOW: &str = "tray_show";
const ID_HIDE: &str = "tray_hide";
const ID_TOGGLE_TOP: &str = "tray_toggle_top";
const ID_TOGGLE_CLICK: &str = "tray_toggle_click";
const ID_QUIT: &str = "tray_quit";

pub fn create_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, ID_SHOW, "Show Panel", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, ID_HIDE, "Hide Panel", true, None::<&str>)?;
    let toggle_top =
        MenuItem::with_id(app, ID_TOGGLE_TOP, "Toggle Always on Top", true, None::<&str>)?;
    let toggle_click =
        MenuItem::with_id(app, ID_TOGGLE_CLICK, "Toggle Click Through", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, ID_QUIT, "Exit", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .item(&show)
        .item(&hide)
        .separator()
        .item(&toggle_top)
        .item(&toggle_click)
        .separator()
        .item(&quit)
        .build()?;

    let icon = app
        .default_window_icon()
        .cloned()
        .unwrap_or_else(|| tauri::image::Image::new_owned(vec![0, 0, 0, 0], 1, 1));

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(icon)
        .on_menu_event(move |app_handle, event| {
            let id = event.id.as_ref();
            match id {
                ID_SHOW => {
                    if let Some(w) = app_handle.get_webview_window("main") {
                        let _ = w.show();
                        let _ = w.set_focus();
                    }
                }
                ID_HIDE => {
                    if let Some(w) = app_handle.get_webview_window("main") {
                        let _ = w.hide();
                    }
                }
                ID_TOGGLE_TOP => {
                    let state = app_handle.state::<crate::AppState>();
                    let mut guard = state.settings.lock().unwrap();
                    let new_top = !guard.always_on_top;
                    guard.always_on_top = new_top;
                    let _ = window_service::set_always_on_top(app_handle, new_top);
                    let _ = settings_service::save_settings(&guard);
                    drop(guard);
                    let _ = app_handle.emit(
                        "settings:changed",
                        state.settings.lock().unwrap().clone(),
                    );
                }
                ID_TOGGLE_CLICK => {
                    let state = app_handle.state::<crate::AppState>();
                    let mut guard = state.settings.lock().unwrap();
                    let new_click = !guard.click_through;
                    guard.click_through = new_click;
                    let _ = window_service::set_click_through(app_handle, new_click);
                    let _ = settings_service::save_settings(&guard);
                    drop(guard);
                    let _ = app_handle.emit(
                        "settings:changed",
                        state.settings.lock().unwrap().clone(),
                    );
                }
                ID_QUIT => {
                    let _ = app_handle.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
