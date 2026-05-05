# Alpha Release Notes

> **Version**: 0.1.0-alpha
> **Date**: 2026-05-05
> **contractVersion**: 1.0.0

## Capabilities (What Works)

- [x] Transparent, frameless, always-on-top floating window (320×200)
- [x] Window drag to reposition
- [x] Real-time hardware monitoring:
  - [x] CPU usage %, temperature °C, frequency GHz
  - [x] GPU usage %, temperature °C, video memory GB
  - [x] RAM usage %, used/total GB
  - [x] Network download/upload B/s (auto-scaling)
  - [x] Disk read/write B/s
- [x] System tray with Show/Hide/Toggle Always on Top/Exit
- [x] Settings persistence (opacity, alwaysOnTop, clickThrough, pollingInterval)
- [x] Window opacity control (Windows layered window API)
- [x] Mouse click-through (Windows WS_EX_TRANSPARENT)
- [x] Settings validation with error messages
- [x] Close window → hide to tray (not exit)
- [x] Independent hardware-helper.exe (usable without React/Tauri)
- [x] Stable API contract (JSON Schemas in `packages/contracts/`)

## Not Yet Implemented

- [ ] Settings UI overlay (opacity slider, module toggles)
- [ ] History data / graphs
- [ ] Multiple monitor support (position save per monitor)
- [ ] Auto-start with Windows
- [ ] Flutter integration (contracts ready, code not built)
- [ ] macOS / Linux support
- [ ] Theme system
- [ ] FPS overlay
- [ ] Plugin system
- [ ] Cloud sync
- [ ] Auto-update

## P12 Additions (Alpha Stabilization)

- [x] One-command scripts (`check.ps1`, `build-helper.ps1`, `dev.ps1`, `clean.ps1`)
- [x] Tray "Toggle Click Through" menu item
- [x] Click-through synchronized between settings.json and tray
- [x] Helper path search includes `win-x64/publish/` for self-contained deployments
- [x] `#[allow(dead_code)]` on IPC contract constants (preserves naming convention)

## How to Run

```powershell
# Recommended: one-command start
.\scripts\dev.ps1

# Or manual:
cd services/hardware-helper && dotnet build -c Release
cd apps/desktop && npm install && npm run tauri dev
```

## Packaging (Production Build)

```powershell
# One-command alpha packaging
.\scripts\package-alpha.ps1
```

The script:
1. Builds `hardware-helper.exe` (Release)
2. Builds `desktop-performance-panel.exe` (Tauri release)
3. Creates `dist-alpha/` with all files

`dist-alpha/` structure:
```
dist-alpha/
├── desktop-performance-panel.exe    # Main application
├── resources/
│   └── hardware-helper.exe         # Hardware collector (auto-located)
└── docs/
    ├── README_DEV.md
    ├── RELEASE_ALPHA.md
    └── KNOWN_ISSUES.md
```

To run the portable alpha:
```powershell
.\dist-alpha\desktop-performance-panel.exe
```

The app automatically finds `resources/hardware-helper.exe` relative to its own
executable path. No installation required.

### Current limitations
- No code signing (Windows SmartScreen may warn)
- No installer (MSI/NSIS)
- No auto-update
- Requires .NET 8 runtime and WebView2 on the target machine
- Helper path is resolved relative to the main exe — keep the directory layout

## Next Steps (Post-Alpha)

| Priority | Task | Effort |
|----------|------|--------|
| P0 | Build script to bundle hardware-helper.exe | S |
| P0 | Settings UI overlay (React) | M |
| P1 | Tray click-through toggle | S |
| P1 | visibleModules/showTemperatures UI integration | M |
| P1 | Icon replacement | S |
| P2 | Auto-restart helper on pollingIntervalMs change | S |
| P2 | Position save/restore on startup | S |
| P2 | Flutter integration — Level 2 (helper-only) | M |
| P3 | History data + simple line charts | L |
| P3 | macOS/Linux window support | L |
| P3 | Flutter integration — Level 1 (full panel) | L |

## Flutter Integration Status

All JSON contracts are stable and versioned (`contractVersion: 1.0.0`).
`hardware-helper.exe` can be consumed independently.

- **Level 2 (recommended now)**: `Process.start('hardware-helper.exe')` + read stdout JSON Lines
- **Level 1 (future)**: Launch full Tauri panel as child process, control via future WebSocket/NamedPipe
- **Level 3 (future)**: Compile Rust core as cdylib for FFI

See `docs/FLUTTER_INTEGRATION.md` for details.
