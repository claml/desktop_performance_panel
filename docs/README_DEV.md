# README — Developer Guide

> **contractVersion**: 1.0.0
> **Status**: Alpha

## Project Goal

A Windows floating performance monitor panel that displays real-time CPU, GPU, RAM,
network, and disk metrics in a transparent, always-on-top, frameless window.

Built as a standalone Tauri + React desktop application, with a reusable C#
hardware collector (`hardware-helper.exe`) that can also be consumed independently
(e.g. by future Flutter projects).

## Architecture

```
React (TypeScript + TailwindCSS)   ← UI Layer
    ↕ tauriBridge (invoke / listen)
Rust (Tauri v2)                    ← Shell Layer
    ↕ stdout JSON Lines
C# (.NET 8) hardware-helper.exe    ← Hardware Layer
```

See `docs/ARCHITECTURE.md` for the full architecture diagram.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| UI | React 19 + TypeScript 5 + TailwindCSS 4 + Zustand 5 |
| Shell | Tauri v2 + Rust 1.78+ |
| Hardware | C# .NET 8 + LibreHardwareMonitorLib |
| Build | Vite 6 |
| Config | settings.json (%APPDATA%\\desktop-performance-panel\\) |

## Directory Structure

```
desktop-performance-panel/
├── apps/desktop/                   # Tauri + React application
│   ├── src/                        # React frontend
│   │   ├── components/panel/       # PerformancePanel, PanelRow, MetricDisplay
│   │   ├── hooks/                  # useHardwareSnapshot
│   │   ├── services/               # tauriBridge (IPC entry)
│   │   ├── stores/                 # Zustand store
│   │   ├── types/                  # Re-exports from shared-types
│   │   └── utils/                  # formatters, mockData
│   └── src-tauri/                  # Rust backend
│       └── src/
│           ├── main.rs, lib.rs     # Entry, commands, setup
│           ├── hardware_service.rs # Helper process management
│           ├── settings_service.rs # Settings CRUD + validation
│           ├── window_service.rs   # Window properties (Windows FFI)
│           ├── tray_service.rs     # System tray
│           └── ipc_contract.rs     # Command/event name constants
├── services/hardware-helper/       # C# standalone collector
│   ├── Program.cs                  # Entry point, JSON Lines output
│   ├── Models/                     # HardwareSnapshot, HelperMessage
│   └── Services/                   # LibreHardwareReader, SystemMetricsReader
├── packages/
│   ├── contracts/                  # JSON Schemas (single source of truth)
│   └── shared-types/               # TypeScript types
└── docs/                           # Documentation
```

## Prerequisites

- **Node.js** ≥ 20
- **npm** ≥ 10
- **Rust** ≥ 1.78 (install via `winget install Rustlang.Rustup && rustup default stable`)
- **.NET SDK** ≥ 8.0
- **Visual Studio Build Tools** (C++ build tools — for Tauri on Windows)
- **WebView2 Runtime** (preinstalled on Windows 11; download for Windows 10)

## Setup & Run

### Quick start (recommended)

```powershell
# Check your environment
.\scripts\check.ps1

# Build the hardware helper
.\scripts\build-helper.ps1

# Start the panel
.\scripts\dev.ps1
```

### Manual steps

```powershell
# 1. Build the hardware-helper
cd services/hardware-helper
dotnet restore
dotnet build -c Release

# 2. Install frontend dependencies
cd apps/desktop
npm install

# 3. Start the panel
npm run tauri dev
```

### Scripts reference

| Script | Purpose |
|--------|---------|
| `.\scripts\check.ps1` | Verify all prerequisites (dotnet, rustc, cargo, node, npm, node_modules, helper exe) |
| `.\scripts\build-helper.ps1` | Restore + build hardware-helper in Release config |
| `.\scripts\dev.ps1` | Build helper, then start `npm run tauri dev` |
| `.\scripts\clean.ps1` | Remove Rust target, Vite dist, .vite cache |
| `.\scripts\clean.ps1 -All` | Also remove `services/hardware-helper/bin` |
| `.\scripts\package-alpha.ps1` | Build helper + Tauri release → `dist-alpha/` portable package |

## Packaging

```powershell
# Create portable alpha package
.\scripts\package-alpha.ps1
```

This produces `dist-alpha/` containing:
```
dist-alpha/
├── desktop-performance-panel.exe
├── resources/hardware-helper.exe
└── docs/
```

Run the portable exe directly — no installation needed.  
Note: .NET 8 runtime and WebView2 must be installed on the target machine.

The panel window appears automatically. A system tray icon is created.
Close the window to hide it (the tray keeps running). Use the tray menu to
show/hide/toggle/exit.

## Settings

Settings are stored in `%APPDATA%\desktop-performance-panel\settings.json`.

Default:
```json
{
  "opacity": 0.85,
  "alwaysOnTop": true,
  "clickThrough": false,
  "position": { "x": 0, "y": 0 },
  "pollingIntervalMs": 1000,
  "visibleModules": ["cpu", "memory", "network", "gpu"],
  "showTemperatures": true
}
```

### Changing click-through

- **Via tray menu**: Right-click tray → "Toggle Click Through"
- **Via settings file**: Set `clickThrough: true` or `false`, then restart
- **Via Tauri invoke** (from devtools):
  ```js
  await __TAURI__.core.invoke('window_set_click_through', { value: true });
  ```

To disable, set `clickThrough: false` (via tray or settings).

## Common Issues

| Problem | Solution |
|---------|----------|
| `hardware-helper.exe not found` | Run `dotnet build -c Release` in `services/hardware-helper/` |
| Rust doesn't compile | Ensure `cargo --version` ≥ 1.78 |
| Temperature shows `--` | Normal — sensors may require admin privileges or may not exist on your hardware |
| Window can't be dragged | `clickThrough` is enabled; set it to `false` |
| Tray icon looks wrong | We use the window icon; replace `icons/icon.ico` with a custom icon |

## API Contract

All cross-layer data structures are defined in `packages/contracts/` as JSON Schemas.
See `docs/API_CONTRACT.md` for the full specification.
