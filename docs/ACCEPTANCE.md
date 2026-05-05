# Acceptance Checklist

> **contractVersion**: 1.0.0
> **Status**: Alpha — all P1–P10 passed

---

## P1 — Contract & Documentation

| Check | Status |
|-------|--------|
| `packages/contracts/header.json` with `contractVersion: "1.0.0"` | ✅ |
| `hardware_snapshot.schema.json` validates with standard JSON Schema | ✅ |
| `helper_message.schema.json` with init/snapshot/error/status | ✅ |
| `settings.schema.json` with field constraints | ✅ |
| `panel_status.schema.json` with `latestSnapshot: HardwareSnapshot \| null` | ✅ |
| `commands.schema.json` with logicalName + tauriCommandName mapping | ✅ |
| `events.schema.json` with all event definitions | ✅ |
| `packages/contracts/README.md` with version rules | ✅ |
| `packages/shared-types/*.ts` match schemas | ✅ |
| `docs/API_CONTRACT.md` complete | ✅ |
| `docs/FLUTTER_INTEGRATION.md` complete | ✅ |
| `docs/ARCHITECTURE.md` complete | ✅ |
| `docs/TASKS.md` complete | ✅ |

---

## P2 — Tauri + React + Tailwind Skeleton

| Check | Status |
|-------|--------|
| `npm install` succeeds | ✅ |
| `npm run tauri dev` launches window | ✅ |
| TailwindCSS v4 active | ✅ |
| TypeScript strict mode, `npx tsc --noEmit` passes | ✅ |
| No business logic in App.tsx | ✅ |

---

## P3 — Transparent Frameless Always-on-Top Window

| Check | Status |
|-------|--------|
| `decorations: false` — no title bar | ✅ |
| `transparent: true` — see-through | ✅ |
| `alwaysOnTop: true` — stays on top | ✅ |
| `resizable: false` — fixed 320×200 | ✅ |
| `data-tauri-drag-region` + `startDragging()` works | ✅ |

---

## P4 — React Mock Performance Panel

| Check | Status |
|-------|--------|
| CPU / GPU / RAM / NET / DSK rows displayed | ✅ |
| Per-second mock data update | ✅ |
| Null values show `--` | ✅ |
| Formatters: percent, temperature, bytes, GB | ✅ |
| Frosted glass card (`bg-gray-900/60` + `backdrop-blur-md`) | ✅ |
| Zustand store functional | ✅ |

---

## P5 — C# hardware-helper

| Check | Status |
|-------|--------|
| `dotnet restore && dotnet build -c Release` succeeds | ✅ |
| `dotnet run -- --interval-ms 1000` produces JSON Lines | ✅ |
| First line is `{"type":"init",...}` | ✅ |
| Subsequent lines are `{"type":"snapshot",...}` with full HardwareSnapshot | ✅ |
| Missing sensors → `null`, no crash | ✅ |
| `--interval-ms` CLI parameter accepted, clamped 500–10000 | ✅ |
| Ctrl+C graceful shutdown | ✅ |
| Non-admin: recoverable error message emitted | ✅ |

---

## P6 — Rust helper bridge

| Check | Status |
|-------|--------|
| `cargo build` succeeds | ✅ |
| Spawns `hardware-helper.exe` on app startup | ✅ |
| Reads stdout line-by-line via `BufReader` | ✅ |
| Parses `HelperMessage` and dispatches by type | ✅ |
| `hardware_start` / `hardware_stop` / `hardware_restart` commands work | ✅ |
| `hardware_get_latest_snapshot` returns cached snapshot | ✅ |
| Emits `hardware:snapshot` and `hardware:status` events | ✅ |

---

## P7 — React real data

| Check | Status |
|-------|--------|
| `npx tsc --noEmit` passes | ✅ |
| Replaced mock `setInterval` with `bridgeListen` | ✅ |
| Real `hardware:snapshot` → Zustand store → UI | ✅ |
| CPU/GPU/RAM/NET/DSK rows display real values | ✅ |
| Null sensors display `--` | ✅ |
| Helper status messages shown on error | ✅ |

---

## P8 — settings.json + settings_update patch

| Check | Status |
|-------|--------|
| `settings.json` created at `%APPDATA%/desktop-performance-panel/` | ✅ |
| `settings_get` returns current settings | ✅ |
| `settings_update` merges partial patch and validates | ✅ |
| `opacity` 0.1–1.0 range check enforced | ✅ |
| `pollingIntervalMs` 500–10000 range check enforced | ✅ |
| `visibleModules` enum check enforced | ✅ |
| `alwaysOnTop` applied immediately | ✅ |
| `opacity` applied immediately (Windows `SetLayeredWindowAttributes`) | ✅ |
| `settings_reset` restores defaults | ✅ |
| `settings:changed` emitted on every change | ✅ |

---

## P9 — System tray

| Check | Status |
|-------|--------|
| Tray icon appears on startup | ✅ |
| Right-click menu: Show / Hide / Toggle Always on Top / Exit | ✅ |
| Show — window displayed and focused | ✅ |
| Hide — window hidden | ✅ |
| Toggle Always on Top — setting flipped and saved | ✅ |
| Exit — helper killed, app exits cleanly | ✅ |
| Close window (X) → hide, not exit | ✅ |

---

## P10 — Click-through (mouse penetration)

| Check | Status |
|-------|--------|
| `window_set_click_through({ value: true })` enables `WS_EX_TRANSPARENT` | ✅ |
| Mouse passes through window to underlying applications | ✅ |
| `window_set_click_through({ value: false })` restores interactivity | ✅ |
| Startup reads `settings.clickThrough` and applies | ✅ |
| Opacity + click-through coexist correctly | ✅ |
| Non-Windows returns "unsupported" | ✅ |

---

## Final Result

**All 10 stages passed. Alpha ready.**
