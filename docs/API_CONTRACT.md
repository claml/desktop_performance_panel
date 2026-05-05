# API Contract

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05
> **本文档是性能面板所有跨层、跨进程通信的唯一契约。所有组件必须严格遵循。**

---

## 目录

1. [通信模型](#1-通信模型)
2. [数据协议](#2-数据协议)
3. [命令 (Commands)](#3-命令-commands)
4. [事件 (Events)](#4-事件-events)
5. [Hardware Helper stdout 协议](#5-hardware-helper-stdout-协议)
6. [Transport 抽象](#6-transport-抽象)
7. [版本管理规则](#7-版本管理规则)

---

## 1. 通信模型

```
┌──────────────────────────────────────────────────────────────────┐
│  React 前端 (UI Layer)                                           │
│                                                                  │
│  只有 tauriBridge.ts 可以 invoke / listen                         │
│  组件 → store → tauriBridge → Rust                                │
│                              ← events ← store ← 组件              │
└───────────────┬──────────────────────────────────────────────────┘
                │ invoke("settings_get")       listen("hardware:snapshot")
                ▼                                        ▲
┌───────────────────────────────────────────────────────────────┐
│  Rust 后端 (Shell Layer - Tauri v2)                           │
│                                                               │
│  集中注册所有命令     hardware_service   window_service        │
│                     settings_service    tray_service           │
│                     ipc_contract.rs                           │
│                                                               │
│  Transport 层:                                                 │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │  Transport Trait (transport/mod.rs)                       │ │
│  │  ├── TauriTransport      (当前: emit events to React)     │ │
│  │  ├── StdoutTransport     (当前: read helper stdout)       │ │
│  │  ├── WebSocketTransport  (预留: 本地 WS server)           │ │
│  │  ├── NamedPipeTransport  (预留: Windows named pipe)       │ │
│  │  └── FfiTransport        (预留: C ABI for Flutter)        │ │
│  └──────────────────────────────────────────────────────────┘ │
└───────────────┬───────────────────────────────────────────────┘
                │ spawn("hardware-helper.exe") → stdout JSON Lines
                ▼
┌───────────────────────────────────────────────────────────────┐
│  C# Hardware Helper (Hardware Layer)                           │
│                                                               │
│  LibreHardwareMonitorLib → HardwareSnapshot                   │
│  Console.WriteLine(HelperMessage JSON)                        │
│                                                               │
│  独立运行、独立测试、不依赖 Tauri / React                        │
└───────────────────────────────────────────────────────────────┘
```

### 核心规则

1. **React 前端只能消费统一数据结构（HardwareSnapshot / Settings / PanelStatus / HelperMessage）**
2. **React 不能直接 `import { invoke }`，只能通过 `tauriBridge.ts` 调用**
3. **Rust 只暴露 snake_case 命令名**
4. **C# helper 只输出 HelperMessage 协议的 JSON Lines**
5. **所有跨语言/跨进程数据格式从 `packages/contracts/` JSON Schema 派生**
6. **Transport 层将数据流向与传输介质解耦**
7. **JSON 对象字段顺序不被视为协议的一部分。消费者不得依赖 key 顺序。**

---

## 2. 数据协议

### 2.1 HardwareSnapshot

定义源：`packages/contracts/hardware_snapshot.schema.json`

```json
{
  "timestamp": 1700000000000,
  "cpu": {
    "usagePercent": 35.2,
    "temperatureC": 58.0,
    "frequencyMhz": 3600,
    "powerW": 65.0
  },
  "gpu": {
    "name": "NVIDIA GeForce RTX 3060",
    "usagePercent": 42.0,
    "temperatureC": 72.0,
    "memoryUsedMb": 4096,
    "memoryTotalMb": 12288,
    "powerW": 120.0,
    "fanRpm": 1800
  },
  "memory": {
    "usedGb": 8.5,
    "totalGb": 16.0,
    "usagePercent": 53.1
  },
  "network": {
    "downloadBps": 1048576,
    "uploadBps": 524288
  },
  "disk": {
    "readBps": 2097152,
    "writeBps": 1048576
  },
  "battery": {
    "percent": 85.0,
    "charging": false
  },
  "error": null
}
```

**字段语义**：

- 所有数值字段类型为 `number | null`
- `null` = 传感器不可用 / 该机器不支持
- 数字 `0` = 数值确为 0，不是"不可用"
- 字符串字段 `null` = 信息不可用（不要用空字符串 `""`）
- `error` 为 `null` 表示整次采集无异常；否则为错误描述字符串

### 2.2 Settings

定义源：`packages/contracts/settings.schema.json`

```json
{
  "opacity": 0.85,
  "alwaysOnTop": true,
  "clickThrough": false,
  "position": { "x": 100, "y": 100 },
  "pollingIntervalMs": 1000,
  "visibleModules": ["cpu", "memory", "network", "gpu"],
  "showTemperatures": true
}
```

**字段约束**：

| 字段 | 类型 | 默认值 | 约束 |
|------|------|--------|------|
| `opacity` | `number` | `0.85` | `0.1 ~ 1.0` |
| `alwaysOnTop` | `boolean` | `true` | — |
| `clickThrough` | `boolean` | `false` | — |
| `position` | `{x:number, y:number}` | `{x:0, y:0}` | `x >= 0, y >= 0` |
| `pollingIntervalMs` | `number` | `1000` | `500 ~ 10000` |
| `visibleModules` | `string[]` | `["cpu","memory","network","gpu"]` | 枚举值：`cpu`, `memory`, `network`, `gpu`, `disk`, `battery` |
| `showTemperatures` | `boolean` | `true` | — |

### 2.3 PanelStatus

定义源：`packages/contracts/panel_status.schema.json`

```json
{
  "windowVisible": true,
  "alwaysOnTop": true,
  "clickThrough": false,
  "opacity": 0.85,
  "helperRunning": true,
  "helperPid": 12345,
  "latestSnapshot": null
}
```

- `latestSnapshot` 为完整 `HardwareSnapshot` 对象，或 `null`（尚未采集到任何数据）。
- 不可以返回 `{}` 或其他非标准值。

---

## 3. 命令 (Commands)

### 命名规范

所有 Tauri invoke 命令名使用 **snake_case**。逻辑 API 名称使用 **colon namespace** 约定。

| 逻辑名称 (API Contract) | 说明 | Tauri 命令名 (snake_case) |
|--------------------------|------|---------------------------|
| `settings:get` | 读取全部设置 | `settings_get` |
| `settings:update` | 更新设置（Partial patch） | `settings_update` |
| `settings:reset` | 恢复默认设置 | `settings_reset` |
| `window:show` | 显示窗口 | `window_show` |
| `window:hide` | 隐藏窗口 | `window_hide` |
| `window:set_always_on_top` | 设置置顶 | `window_set_always_on_top` |
| `window:set_click_through` | 设置鼠标穿透 | `window_set_click_through` |
| `window:set_opacity` | 设置透明度 | `window_set_opacity` |
| `window:set_position` | 设置窗口位置 | `window_set_position` |
| `window:get_position` | 获取窗口位置 | `window_get_position` |
| `hardware:start` | 启动 helper | `hardware_start` |
| `hardware:stop` | 停止 helper | `hardware_stop` |
| `hardware:restart` | 重启 helper | `hardware_restart` |
| `hardware:get_latest_snapshot` | 获取最新快照 | `hardware_get_latest_snapshot` |
| `panel:get_status` | 获取面板完整状态 | `panel_get_status` |

- `services/tauriBridge.ts` 负责将逻辑名映射为 snake_case 命令名。
- React 组件不得直接 `invoke`，必须通过 `tauriBridge.ts`。
- 映射关系同时记录在 `packages/contracts/commands.schema.json`。

### 3.1 `settings:get` → `settings_get`

**参数**：无

**返回**：`Settings`

### 3.2 `settings:update` → `settings_update`

**参数**：
```json
{
  "patch": {
    "opacity": 0.85,
    "alwaysOnTop": true
  }
}
```

- `patch` 为 `Partial<Settings>`，只传需要修改的字段
- Rust 端对每个字段做合法性校验：
  - `opacity`: 必须在 `0.1 ~ 1.0` 范围内
  - `pollingIntervalMs`: 必须在 `500 ~ 10000` 范围内
  - `visibleModules`: 每个元素必须是有效枚举值
  - `position.x`, `position.y`: 必须 `>= 0`
- 校验失败返回 `Err(String)`，前端展示错误信息
- 校验通过后合并到当前 settings，写入 `%APPDATA%/perf-panel/settings.json`

**返回**：完整 `Settings` 对象（patch 合并后）

**副作用**：
- 修改 `opacity` → 立即调用 `window_set_opacity`
- 修改 `pollingIntervalMs` → Rust 重启 helper 进程，传入新的 `--interval-ms` 参数（见方案 A）
- 修改其他 display 设置 → 通过 `settings:changed` 事件通知前端

### 3.3 `settings:reset` → `settings_reset`

**参数**：无

**返回**：默认 `Settings`（见 settings.schema.json defaults）

### 3.4 `hardware:start` → `hardware_start`

**参数**：无

**返回**：`{ success: boolean, message: string }`

### 3.5 `hardware:stop` → `hardware_stop`

**参数**：无

**返回**：`{ success: boolean, message: string }`

### 3.6 `hardware:restart` → `hardware_restart`

**参数**：无  
**行为**：先 stop，再 start

**返回**：`{ success: boolean, message: string }`

### 3.7 `hardware:get_latest_snapshot` → `hardware_get_latest_snapshot`

**参数**：无

**返回**：`HardwareSnapshot`（Rust 内存中缓存的最后快照；如无数据则返回全 null 结构）

### 3.8 `panel:get_status` → `panel_get_status`

**参数**：无

**返回**：`PanelStatus`

### 3.9 窗口控制命令

| 命令 (snake_case) | 参数 | 返回 |
|-------------------|------|------|
| `window_show` | 无 | `void` |
| `window_hide` | 无 | `void` |
| `window_set_always_on_top` | `{ value: boolean }` | `void` |
| `window_set_click_through` | `{ value: boolean }` | `void` (非 Windows: 返回错误) |
| `window_set_opacity` | `{ value: number }` | `void` |
| `window_set_position` | `{ x: number, y: number }` | `void` |
| `window_get_position` | 无 | `{ x: number, y: number }` |

---

## 4. 事件 (Events)

事件名使用逻辑名格式（`hardware:snapshot`），不做 snake_case 转换。

| 事件名 | 方向 | Payload | 频率 |
|--------|------|---------|------|
| `hardware:snapshot` | Rust → React | `HardwareSnapshot` | 每秒 1 次 |
| `hardware:status` | Rust → React | `{ status: "running"\|"stopped"\|"error", pid: number\|null, message: string\|null }` | 状态变化时 |
| `settings:changed` | Rust → React | `Settings` | 设置变化时 |
| `helper:message` | Rust → React | `HelperMessage` (原样转发) | helper 每次输出时 (debug) |

### 4.1 `hardware:snapshot`

前端通过此事件获得实时硬件数据：

```typescript
import { listen } from '@tauri-apps/api/event';
import type { HardwareSnapshot } from 'shared-types/hardware';

listen<HardwareSnapshot>('hardware:snapshot', (event) => {
  performanceStore.setSnapshot(event.payload);
});
```

### 4.2 `hardware:status`

```json
{
  "status": "running",
  "pid": 12345,
  "message": null
}
```

- `status`: `"running"` | `"stopped"` | `"error"`
- `pid`: helper 进程 ID，`null` 表示未运行
- `message`: 附加信息（错误原因等）

### 4.3 `settings:changed`

settings.json 被修改后，Rust 主动推送最新 Settings 给前端。

### 4.4 `helper:message`

原样转发硬件 helper 的 stdout 消息到前端。默认不渲染，仅供开发调试。

---

## 5. Hardware Helper stdout 协议

### 5.1 HelperMessage Envelope

定义源：`packages/contracts/helper_message.schema.json`

硬件 helper 的标准输出 (`stdout`) 每行是一个完整的 JSON 对象（JSON Lines），包装为 **HelperMessage**。

**消息类型**：

| type | 必含字段 | 说明 |
|------|----------|------|
| `init` | `version`, `timestamp` | helper 启动后首行，声明协议版本 |
| `snapshot` | `version`, `timestamp`, `data` | 一次完整硬件采集，`data` 为 `HardwareSnapshot` |
| `error` | `version`, `timestamp`, `message`, `recoverable` | 采集错误。`recoverable=true` 继续采集；`false` 进程即将退出 |
| `status` | `version`, `timestamp`, `message` | 预留：helper 内部状态变化 |

**处理流程**：

```
hardware-helper.exe stdout
          │
          ▼
┌──────────────────────────────┐
│  Rust: BufReader 逐行读取    │
│  ┌────────────────────────┐  │
│  │ try JSON::from_str()   │  │
│  │                         │  │
│  │ match msg.type {        │  │
│  │   "init"     → 校验版本, │  │
│  │                emit hardware:status │
│  │   "snapshot" → 提取 data,│  │
│  │                存入缓存, │  │
│  │                emit hardware:snapshot │
│  │   "error"    → 记录日志, │  │
│  │                emit helper:message   │
│  │                if !recoverable:      │
│  │                  标记 helper 异常   │
│  │   "status"   → 记录日志, │  │
│  │                emit helper:message  │
│  │   _          → 跳过 (向前兼容)       │  │
│  │ }                       │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
```

**示例输出流**：
```
{"type":"init","version":"1.0.0","timestamp":1700000000000}
{"type":"snapshot","version":"1.0.0","timestamp":1700000001000,"data":{"timestamp":1700000001000,"cpu":{"usagePercent":35.2,"temperatureC":58.0,"frequencyMhz":3600,"powerW":65.0},"gpu":{"name":"NVIDIA GeForce RTX 3060","usagePercent":42.0,"temperatureC":72.0,"memoryUsedMb":4096,"memoryTotalMb":12288,"powerW":120.0,"fanRpm":1800},"memory":{"usedGb":8.5,"totalGb":16.0,"usagePercent":53.1},"network":{"downloadBps":1048576,"uploadBps":524288},"disk":{"readBps":2097152,"writeBps":1048576},"battery":{"percent":85.0,"charging":false},"error":null}}
{"type":"error","version":"1.0.0","timestamp":1700000003000,"message":"GPU temperature sensor not available","recoverable":true}
```

### 5.2 pollingIntervalMs 运行时变更 (方案 A)

- hardware-helper 支持启动参数：`hardware-helper.exe --interval-ms 1000`
- 用户修改 `pollingIntervalMs` 后，Rust 调用 `hardware_restart`，内部 stop 旧进程 + 以新 `--interval-ms` 启动 helper
- Rust 从 settings 中读取当前 `pollingIntervalMs`，拼接启动参数
- helper 内部从命令行参数解析轮询间隔，不使用 stdin 协议

---

## 6. Transport 抽象

### 6.1 当前实现 (MVP)

```
┌──────────────────────────────────────┐
│  Transport / Data Flow               │
│                                      │
│  StdoutTransport                     │
│  ├── spawn hardware-helper.exe       │
│  ├── --interval-ms {N}               │
│  ├── 逐行读取 stdout                 │
│  ├── 解析 HelperMessage              │
│  └── 缓存 latestSnapshot             │
│           │                          │
│           ▼                          │
│  TauriTransport                      │
│  ├── emit("hardware:snapshot", ...)  │
│  ├── emit("hardware:status", ...)    │
│  ├── emit("settings:changed", ...)   │
│  ├── emit("helper:message", ...)     │
│  └── register all commands           │
│           │                          │
│           ▼                          │
│  React 前端                          │
│  ├── tauriBridge.ts (invoke/listen)  │
│  └── performanceStore.ts             │
└──────────────────────────────────────┘
```

### 6.2 Transport Trait 定义 (Rust 预留)

位于 `apps/desktop/src-tauri/src/transport/mod.rs`：

```rust
/// Data transport from hardware helper to consumers (React frontend, external programs, etc.)
pub trait HardwareTransport: Send + Sync {
    fn start(&mut self, interval_ms: u64) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn latest_snapshot(&self) -> Option<HardwareSnapshot>;
    fn set_snapshot_callback(&mut self, callback: Box<dyn Fn(HardwareSnapshot) + Send + Sync>);
}
```

### 6.3 未来可扩展

| Transport | 文件名 | 当前状态 | 说明 |
|-----------|--------|---------|------|
| `StdoutTransport` | `transport/stdout_transport.rs` | ✅ 实现 | 通过 stdout 读取 helper 数据 |
| `TauriTransport` | `transport/tauri_transport.rs` | ✅ 实现 | 通过 Tauri event system 推送前端 |
| `WebSocketTransport` | `transport/ws_transport.rs` (预留) | 🔮 预留 | Rust 启动本地 WS server |
| `NamedPipeTransport` | `transport/pipe_transport.rs` (预留) | 🔮 预留 | Windows named pipe IPC |
| `FfiTransport` | `transport/ffi_transport.rs` (预留) | 🔮 预留 | Rust cdylib, C ABI |

**核心保证**：无论使用哪种 transport，前端始终消费相同的 `HardwareSnapshot`。UI 层不感知 transport 类型。

---

## 7. 版本管理规则

### 7.1 contractVersion

- 所有 schema 和协议共享同一个 `contractVersion`，当前为 `1.0.0`
- 记录于 `packages/contracts/header.json`

### 7.2 兼容性承诺

| 操作 | 规则 |
|------|------|
| 新增 HardwareSnapshot 字段 | ✅ 允许，放在末尾，默认值 `null` |
| 删除 HardwareSnapshot 字段 | ❌ 不允许。需先标记 `deprecated` 3 个小版本 |
| 修改字段类型 | ❌ 不允许 |
| 新增 HelperMessage.type | ✅ 允许 |
| 修改/删除已有 type | ❌ 不允许 |
| 新增命令 | ✅ 允许 |
| 改名已有命令 | ❌ 不允许（可新增别名，保留旧命令 3 个小版本） |
| 废弃命令 | 保留 3 个小版本后移除 |
| 新增事件 | ✅ 允许 |
| 改名已有事件 | ❌ 不允许（同命令规则） |
| JSON 字段顺序 | **不视为协议的一部分，消费者不得依赖顺序** |

### 7.3 跨语言类型同步

| 语言 | 位置 | 同步方式 |
|------|------|----------|
| TypeScript | `packages/shared-types/` | 手工维护，与 schema 同步 |
| Rust | `apps/desktop/src-tauri/src/` | `serde` struct 手工定义，与 schema 同步 |
| C# | `services/hardware-helper/Models/` | 手工定义，与 schema 同步 |
| Dart (未来) | `packages/dart-types/` (计划中) | 从 JSON Schema 自动生成 |

---

## 8. tauriBridge.ts 映射表

```typescript
// apps/desktop/src/services/tauriBridge.ts

const COMMAND_MAP: Record<string, string> = {
  'settings:get':                  'settings_get',
  'settings:update':               'settings_update',
  'settings:reset':                'settings_reset',
  'window:show':                   'window_show',
  'window:hide':                   'window_hide',
  'window:set_always_on_top':      'window_set_always_on_top',
  'window:set_click_through':      'window_set_click_through',
  'window:set_opacity':            'window_set_opacity',
  'window:set_position':           'window_set_position',
  'window:get_position':           'window_get_position',
  'hardware:start':                'hardware_start',
  'hardware:stop':                 'hardware_stop',
  'hardware:restart':              'hardware_restart',
  'hardware:get_latest_snapshot':  'hardware_get_latest_snapshot',
  'panel:get_status':              'panel_get_status',
};
```
