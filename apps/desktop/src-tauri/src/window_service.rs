use tauri::Manager;

pub fn set_always_on_top(app_handle: &tauri::AppHandle, value: bool) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window
        .set_always_on_top(value)
        .map_err(|e| format!("Failed to set always on top: {}", e))
}

pub fn set_opacity(app_handle: &tauri::AppHandle, value: f64) -> Result<(), String> {
    set_opacity_impl(app_handle, value)
}

pub fn set_click_through(app_handle: &tauri::AppHandle, enabled: bool) -> Result<(), String> {
    set_click_through_impl(app_handle, enabled)
}

// ─── Windows implementations ──────────────────────────────

#[cfg(target_os = "windows")]
fn set_opacity_impl(app_handle: &tauri::AppHandle, value: f64) -> Result<(), String> {
    let hwnd_raw = get_hwnd(app_handle)?;
    let alpha: u8 = (value.clamp(0.1, 1.0) * 255.0).round() as u8;

    const GWL_EXSTYLE: i32 = -20;
    const WS_EX_LAYERED: isize = 0x80000;
    const LWA_ALPHA: u32 = 0x2;

    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd_raw, GWL_EXSTYLE);
        if (ex_style & WS_EX_LAYERED) == 0 {
            SetWindowLongPtrW(hwnd_raw, GWL_EXSTYLE, ex_style | WS_EX_LAYERED);
        }
        SetLayeredWindowAttributes(hwnd_raw, 0, alpha as u8, LWA_ALPHA);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_click_through_impl(app_handle: &tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let hwnd_raw = get_hwnd(app_handle)?;

    const GWL_EXSTYLE: i32 = -20;
    const WS_EX_TRANSPARENT: isize = 0x20;
    const WS_EX_LAYERED: isize = 0x80000;

    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd_raw, GWL_EXSTYLE);

        if enabled {
            // Add transparent + layered flags
            SetWindowLongPtrW(
                hwnd_raw,
                GWL_EXSTYLE,
                ex_style | WS_EX_TRANSPARENT | WS_EX_LAYERED,
            );
        } else {
            // Remove transparent, keep layered (needed for opacity)
            SetWindowLongPtrW(
                hwnd_raw,
                GWL_EXSTYLE,
                ex_style & !WS_EX_TRANSPARENT,
            );
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_hwnd(app_handle: &tauri::AppHandle) -> Result<isize, String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let hwnd = window
        .hwnd()
        .map_err(|e| format!("hwnd error: {}", e))?;
    Ok(hwnd.0 as isize)
}

// ─── Non-Windows stubs ───────────────────────────────────

#[cfg(not(target_os = "windows"))]
fn set_opacity_impl(_app_handle: &tauri::AppHandle, _value: f64) -> Result<(), String> {
    Err("Window opacity is only supported on Windows".into())
}

#[cfg(not(target_os = "windows"))]
fn set_click_through_impl(_app_handle: &tauri::AppHandle, _enabled: bool) -> Result<(), String> {
    Err("Click-through is only supported on Windows".into())
}

// ─── Windows FFI ─────────────────────────────────────────

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn GetWindowLongPtrW(hwnd: isize, nIndex: i32) -> isize;
    fn SetWindowLongPtrW(hwnd: isize, nIndex: i32, dwNewLong: isize) -> isize;
    fn SetLayeredWindowAttributes(hwnd: isize, crKey: u32, bAlpha: u8, dwFlags: u32) -> i32;
}
