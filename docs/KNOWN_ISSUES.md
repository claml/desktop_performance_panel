# Known Issues

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05

## Sensor Data

### CPU / GPU temperatures may be null
Different hardware, drivers, and privilege levels affect sensor availability.
Non-admin users may see `null` for temperature sensors that require ring-0 access.
This is expected — the panel shows `--` for unavailable values and does not crash.

### GPU sensors vary by vendor
NVIDIA / AMD / Intel GPUs expose different sensor sets. Some fields (powerW,
fanRpm) may be null depending on the GPU model and driver version.

### Network rates start at 0
On first launch, network rates may show 0 B/s until the first delta can be
calculated (1–2 seconds).

### Disk rates may be null
`PhysicalDisk\Disk Read Bytes/sec` PerformanceCounter may be unavailable on
some systems (e.g. certain SSDs, RAID configurations). Falls back to null.

## Window Behavior

### clickThrough=true prevents window dragging
When mouse penetration is enabled, all mouse events pass through the window.
To move the panel, open the tray menu and click "Toggle Click Through" to
disable it, reposition the window, then re-enable via the tray.

## Settings

### pollingIntervalMs change does not restart helper
Changing `pollingIntervalMs` saves to settings.json but does not automatically
restart the hardware-helper process. The new interval takes effect on the next
app restart. (Planned for future phase.)

### visibleModules / showTemperatures not consumed by UI
These settings are saved and validated but the React UI does not yet filter
modules or hide temperatures based on them. (Planned for future phase.)

## Tray Icon

### Default icon is minimal
If no custom tray icon is provided, a fallback icon is used. Replace
`apps/desktop/src-tauri/icons/icon.ico` with a custom icon for a better appearance.

## Build

### helper.exe path is development-relative
The Rust code searches for `hardware-helper.exe` relative to the workspace root.
For a production build, the helper exe must be copied next to the main
executable. (Planned for build script.)

### Ctrl+C on dev exit
Pressing Ctrl+C in the `npm run tauri dev` terminal shows
`STATUS_CONTROL_C_EXIT` in stderr. This is normal behavior for Tauri dev
mode — the process is terminated by the signal.

## Unused Rust imports / dead code warnings

Some `ipc_contract::cmd::*` constants are defined but not directly referenced
by match arms (commands are registered by the `generate_handler!` macro).
These are harmless and will be cleaned up in a future refactor pass.
