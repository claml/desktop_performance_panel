# Architecture

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05

## 1. 总体架构

```
┌──────────────────────────────────────────────────────────────────────┐
│                        桌面性能监控面板                                │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │  apps/desktop/src/                  (React Frontend)          │    │
│  │  ┌────────────┐ ┌────────────┐ ┌────────────────────────┐    │    │
│  │  │ Components │ │  Zustand   │ │ services/tauriBridge.ts│    │    │
│  │  │ (Panel,    │ │  Store     │ │ (唯一 IPC 入口)        │    │    │
│  │  │  Settings) │ │            │ │                        │    │    │
│  │  └─────┬──────┘ └─────┬──────┘ └───────────┬────────────┘    │    │
│  └────────┼──────────────┼────────────────────┼──────────────────┘    │
│           │              │                    │                        │
│           │     read     │          invoke / listen                   │
│           ▼              ▼                    │                        │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │  apps/desktop/src-tauri/src/     (Rust Backend - Tauri v2)    │   │
│  │                                                               │   │
│  │  main.rs         集中注册命令/事件/插件                         │   │
│  │  hardware_service.rs   管理 helper 进程                        │   │
│  │  window_service.rs     窗口属性控制 (含 WS_EX_TRANSPARENT)     │   │
│  │  settings_service.rs   settings.json 读写+校验                 │   │
│  │  tray_service.rs       托盘图标 + 右键菜单                     │   │
│  │  ipc_contract.rs       命令名/事件名常量                        │   │
│  │                                                               │   │
│  │  transport/                   (Transport 抽象层)              │   │
│  │  ├── mod.rs                  HardwareTransport trait          │   │
│  │  ├── stdout_transport.rs     spawn helper + read stdout       │   │
│  │  └── tauri_transport.rs      emit events to React             │   │
│  └────────────┬──────────────────────────────────────────────────┘   │
│               │ spawn("hardware-helper.exe --interval-ms 1000")       │
│               ▼                                                      │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │  services/hardware-helper/     (C# Hardware Collector)        │    │
│  │                                                               │    │
│  │  Program.cs              入口: 循环采集 → stdout JSON Lines   │    │
│  │  Services/LibreHardwareReader.cs     LHM 传感器封装            │    │
│  │  Models/HardwareSnapshot.cs          数据模型                  │    │
│  │  Models/HelperMessage.cs             Envelope 包装             │    │
│  │                                                               │    │
│  │  依赖: LibreHardwareMonitorLib (NuGet)                         │    │
│  │  独立可运行、独立测试、不依赖 Tauri / React                     │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │  packages/                       (Cross-cutting Contracts)    │    │
│  │  ├── contracts/        JSON Schemas (唯一真相源)              │    │
│  │  └── shared-types/     TypeScript 类型定义                     │    │
│  └──────────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────────┘
```

## 2. 分层架构

```
┌──────────────────┐ ┌──────────────────┐
│  Component Tree  │ │  Zustand Store   │  ← UI Layer
│  (React)         │ │  (state)         │
├──────────────────┴─┴──────────────────┤
│  services/tauriBridge.ts               │  ← IPC Boundary
│  (invoke / listen 唯一入口)             │
├────────────────────────────────────────┤
│  Tauri Shell (Rust)                    │  ← Shell Layer
│  - Window, Tray, Settings, IPC         │
├────────────────────────────────────────┤
│  Transport (Rust trait)                │  ← Transport Layer
│  - StdoutTransport                    │
│  - TauriTransport                     │
├────────────────────────────────────────┤
│  Hardware Helper (C#)                  │  ← Hardware Layer
│  - LibreHardwareMonitorLib             │
│  - Windows Perf Counters               │
└────────────────────────────────────────┘
```

### 硬约束

1. **UI 层不得直接读取硬件**：React 只能通过 Rust events 获取 `HardwareSnapshot`
2. **UI 层不得直接 invoke**：必须通过 `tauriBridge.ts` 统一入口
3. **Rust 不得渲染 UI**：Rust 只负责窗口管理、数据转发、配置读写
4. **C# helper 不得依赖 Tauri/React**：独立编译、独立运行
5. **所有跨层通信使用稳定 JSON contract**
6. **业务逻辑集中在 store 和 Rust services，不散落在组件**

## 3. 数据流

```
hardware-helper.exe
    │ stdout JSON Line (HelperMessage)
    ▼
Rust hardware_service / stdout_transport
    │ 1. BufReader 逐行读取
    │ 2. 解析 HelperMessage.type
    │ 3. 提取 snapshot → 缓存
    │ 4. 通过 tauri_transport emit
    ▼
Tauri Event System
    │ emit("hardware:snapshot", HardwareSnapshot)
    ▼
React tauriBridge.ts → listen()
    │
    ▼
Zustand performanceStore
    │ setSnapshot(payload)
    ▼
React Components (PerformancePanel → PanelRow → MetricDisplay)
```

## 4. 技术栈

| 层 | 技术 | 版本 |
|----|------|------|
| UI Framework | React | 18+ |
| Language | TypeScript | 5+ (strict) |
| CSS | TailwindCSS | 3.4+ |
| State | Zustand | 5+ |
| Build | Vite | 5+ |
| Shell | Tauri | v2 (stable) |
| Backend | Rust | 1.78+ |
| Serialization | serde / serde_json | — |
| Hardware | C# / .NET 8 | net8.0 |
| Sensors | LibreHardwareMonitorLib | NuGet latest |

## 5. Tauri v2 实现注意事项

### 5.1 Cargo Features
- 托盘功能需要启用 `tray-icon` feature
- 窗口透明需要 `transparent` 配置项
- 推荐启用的 features: `tray-icon`

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
```

### 5.2 窗口配置 (tauri.conf.json)

```json
{
  "app": {
    "windows": [
      {
        "label": "main",
        "width": 320,
        "height": 200,
        "decorations": false,
        "alwaysOnTop": true,
        "transparent": true,
        "resizable": false,
        "center": true
      }
    ]
  }
}
```

### 5.3 Windows 鼠标穿透

需要通过 Windows API 设置窗口扩展样式：

- `WS_EX_TRANSPARENT` = `0x00000020` — 鼠标事件穿透
- `WS_EX_LAYERED`   = `0x00080000` — 分层窗口（配合透明使用）
- 使用 `SetWindowLongPtrW` + `GWL_EXSTYLE`
- 仅 Windows 实现；非 Windows 平台返回 `Err("unsupported platform")`
- 穿透模式下自动禁用窗口拖拽

### 5.4 命令集中注册

- 所有 Tauri invoke 命令必须在 `main.rs` 中集中注册
- 使用 `tauri::generate_handler![]` 宏
- **禁止在模块内部注册命令**
- 命令名必须与 `ipc_contract.rs` 中常量一致

### 5.5 前端通信限制

- 组件**不允许**直接 `import { invoke } from '@tauri-apps/api/core'`
- 所有 invoke 调用**必须**经过 `services/tauriBridge.ts`
- `tauriBridge.ts` 负责逻辑命令名 → snake_case 命令名映射
- 只有 `tauriBridge.ts` 可以 `import { invoke }`

## 6. Transport 抽象

### 6.1 Trait 定义

```rust
// transport/mod.rs
pub trait HardwareTransport: Send + Sync {
    fn start(&mut self, interval_ms: u64) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn latest_snapshot(&self) -> Option<HardwareSnapshot>;
    fn set_snapshot_callback(&mut self, callback: Box<dyn Fn(HardwareSnapshot) + Send + Sync>);
}
```

### 6.2 当前实现

| Transport | 文件 | 作用 |
|-----------|------|------|
| `StdoutTransport` | `transport/stdout_transport.rs` | spawn helper process, read stdout JSON Lines |
| `TauriTransport` | `transport/tauri_transport.rs` | emit events to React via Tauri event system |

### 6.3 未来扩展

| Transport | 文件 (预留) | 说明 |
|-----------|------------|------|
| `WebSocketTransport` | `transport/ws_transport.rs` | Rust 启动本地 WS server |
| `NamedPipeTransport` | `transport/pipe_transport.rs` | Windows named pipe |
| `FfiTransport` | `transport/ffi_transport.rs` | Rust cdylib C ABI |

## 7. pollingIntervalMs 运行时变更

采用 **方案 A**：通过命令行参数传递，重启 helper 进程切换。

- helper 启动格式：`hardware-helper.exe --interval-ms 1000`
- `settings_update` 修改 `pollingIntervalMs` 后，Rust 调用 `hardware_restart`
- `hardware_restart` 内部：kill 旧进程 → 以新 `--interval-ms` 启动新进程
- helper 从 `args` 解析 `--interval-ms`，默认 1000
- 不使用 stdin 协议

## 8. 文件系统

```
%APPDATA%/perf-panel/
├── settings.json          # 用户配置
└── logs/                  # (预留) 运行日志
```

Windows 上 `%APPDATA%` 展开为 `C:\Users\{Username}\AppData\Roaming`。

## 9. 构建产物

```
target/release/
├── desktop-performance-panel.exe    # Tauri 主程序
└── hardware-helper.exe              # C# helper (复制到同目录)
```

`hardware-helper.exe` 由 C# 项目独立编译，通过构建脚本复制到 Tauri 输出目录。
