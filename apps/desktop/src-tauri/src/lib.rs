mod hardware_service;
mod ipc_contract;
mod settings_service;
mod tray_service;
mod window_service;

use std::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;

use hardware_service::{HardwareService, HelperStatusPayload};
use ipc_contract::event;
use settings_service::{Settings, SettingsUpdateArgs};

pub struct AppState {
    pub hardware_service: Mutex<Option<HardwareService>>,
    pub settings: Mutex<Settings>,
}

// ─── Hardware commands ───────────────────────────────────

#[tauri::command]
fn hardware_start(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let mut svc = HardwareService::new(app_handle);
    svc.start(1000)?;
    let mut guard = state.hardware_service.lock().unwrap();
    *guard = Some(svc);
    Ok("hardware-helper started".to_string())
}

#[tauri::command]
fn hardware_stop(
    state: tauri::State<AppState>,
) -> Result<String, String> {
    let mut guard = state.hardware_service.lock().unwrap();
    if let Some(ref mut svc) = *guard {
        svc.stop()?;
    }
    *guard = None;
    Ok("hardware-helper stopped".to_string())
}

#[tauri::command]
fn hardware_restart(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let mut guard = state.hardware_service.lock().unwrap();
    if let Some(ref mut svc) = *guard {
        svc.restart(1000)?;
        return Ok("hardware-helper restarted".to_string());
    }
    drop(guard);
    let mut svc = HardwareService::new(app_handle);
    svc.start(1000)?;
    let mut guard = state.hardware_service.lock().unwrap();
    *guard = Some(svc);
    Ok("hardware-helper started (was not running)".to_string())
}

#[tauri::command]
fn hardware_get_latest_snapshot(
    state: tauri::State<AppState>,
) -> Result<serde_json::Value, String> {
    let guard = state.hardware_service.lock().unwrap();
    match &*guard {
        Some(svc) => match svc.get_latest_snapshot() {
            Some(snapshot) => Ok(serde_json::to_value(snapshot)
                .map_err(|e| format!("Serialization error: {}", e))?),
            None => Err("No snapshot available yet".to_string()),
        },
        None => Err("Hardware service is not running".to_string()),
    }
}

// ─── Settings commands ───────────────────────────────────

#[tauri::command]
fn settings_get(
    state: tauri::State<AppState>,
) -> Result<Settings, String> {
    let guard = state.settings.lock().unwrap();
    Ok(guard.clone())
}

#[tauri::command]
fn settings_update(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    args: SettingsUpdateArgs,
) -> Result<Settings, String> {
    let mut guard = state.settings.lock().unwrap();

    let updated = settings_service::update_settings(&guard, &args.patch)?;
    settings_service::save_settings(&updated)?;

    if args.patch.opacity.is_some() {
        let _ = window_service::set_opacity(&app_handle, updated.opacity);
    }
    if args.patch.always_on_top.is_some() {
        let _ = window_service::set_always_on_top(&app_handle, updated.always_on_top);
    }

    *guard = updated.clone();

    let _ = app_handle.emit(event::SETTINGS_CHANGED, updated.clone());

    Ok(updated)
}

#[tauri::command]
fn settings_reset(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Settings, String> {
    let defaults = Settings::default();
    settings_service::save_settings(&defaults)?;

    let mut guard = state.settings.lock().unwrap();
    *guard = defaults.clone();

    let _ = window_service::set_opacity(&app_handle, defaults.opacity);
    let _ = window_service::set_always_on_top(&app_handle, defaults.always_on_top);

    let _ = app_handle.emit(event::SETTINGS_CHANGED, defaults.clone());

    Ok(defaults)
}

// ─── Window commands ─────────────────────────────────────

#[tauri::command]
fn window_set_always_on_top(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    value: bool,
) -> Result<(), String> {
    window_service::set_always_on_top(&app_handle, value)?;
    let mut guard = state.settings.lock().unwrap();
    guard.always_on_top = value;
    let _ = settings_service::save_settings(&guard);
    let _ = app_handle.emit(event::SETTINGS_CHANGED, guard.clone());
    Ok(())
}

#[tauri::command]
fn window_set_opacity(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    value: f64,
) -> Result<(), String> {
    if value < 0.1 || value > 1.0 {
        return Err("opacity must be between 0.1 and 1.0".into());
    }
    window_service::set_opacity(&app_handle, value)?;
    let mut guard = state.settings.lock().unwrap();
    guard.opacity = value;
    let _ = settings_service::save_settings(&guard);
    let _ = app_handle.emit(event::SETTINGS_CHANGED, guard.clone());
    Ok(())
}

#[tauri::command]
fn window_set_click_through(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    value: bool,
) -> Result<(), String> {
    window_service::set_click_through(&app_handle, value)?;
    let mut guard = state.settings.lock().unwrap();
    guard.click_through = value;
    let _ = settings_service::save_settings(&guard);
    let _ = app_handle.emit(event::SETTINGS_CHANGED, guard.clone());
    Ok(())
}

// ─── Application entry ───────────────────────────────────

pub fn run() {
    let settings = settings_service::load_settings().unwrap_or_else(|e| {
        eprintln!("[settings] Failed to load settings, using defaults: {}", e);
        Settings::default()
    });

    let app_state = AppState {
        hardware_service: Mutex::new(None),
        settings: Mutex::new(settings.clone()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            hardware_start,
            hardware_stop,
            hardware_restart,
            hardware_get_latest_snapshot,
            settings_get,
            settings_update,
            settings_reset,
            window_set_always_on_top,
            window_set_opacity,
            window_set_click_through,
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            let state = app.state::<AppState>();

            // Apply saved window properties
            {
                let s = state.settings.lock().unwrap();
                let _ = window_service::set_always_on_top(&handle, s.always_on_top);
                let _ = window_service::set_opacity(&handle, s.opacity);
                let _ = window_service::set_click_through(&handle, s.click_through);
            }

            // Emit initial settings
            let _ = handle.emit(event::SETTINGS_CHANGED, settings.clone());

            // Create system tray
            if let Err(e) = tray_service::create_tray(&handle) {
                eprintln!("[tray] Failed to create tray icon: {}", e);
            }

            // Close → hide instead of exit
            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w.hide();
                    }
                });
            }

            // Auto-start hardware helper
            let mut svc = HardwareService::new(handle.clone());
            match svc.start(1000) {
                Ok(()) => {
                    *state.hardware_service.lock().unwrap() = Some(svc);
                }
                Err(e) => {
                    eprintln!("[hardware-service] Auto-start failed: {}", e);
                    let _ = handle.emit(
                        event::HARDWARE_STATUS,
                        HelperStatusPayload {
                            status: "error".to_string(),
                            pid: None,
                            message: Some(e),
                        },
                    );
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
