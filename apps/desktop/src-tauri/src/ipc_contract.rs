/// IPC contract constants for Tauri commands and events.
///
/// Command names use snake_case (registered with Tauri).
/// Event names use colon notation (emitted to React).

// ─── Commands ────────────────────────────────────────────

#[allow(dead_code)]
pub mod cmd {
    // Hardware
    pub const HARDWARE_START: &str = "hardware_start";
    pub const HARDWARE_STOP: &str = "hardware_stop";
    pub const HARDWARE_RESTART: &str = "hardware_restart";
    pub const HARDWARE_GET_LATEST_SNAPSHOT: &str = "hardware_get_latest_snapshot";

    // Settings
    pub const SETTINGS_GET: &str = "settings_get";
    pub const SETTINGS_UPDATE: &str = "settings_update";
    pub const SETTINGS_RESET: &str = "settings_reset";

    // Window
    pub const WINDOW_SET_ALWAYS_ON_TOP: &str = "window_set_always_on_top";
    pub const WINDOW_SET_OPACITY: &str = "window_set_opacity";
    pub const WINDOW_SET_CLICK_THROUGH: &str = "window_set_click_through";
}

// ─── Events ──────────────────────────────────────────────

pub mod event {
    pub const HARDWARE_SNAPSHOT: &str = "hardware:snapshot";
    pub const HARDWARE_STATUS: &str = "hardware:status";
    pub const HELPER_MESSAGE: &str = "helper:message";
    pub const SETTINGS_CHANGED: &str = "settings:changed";
}
