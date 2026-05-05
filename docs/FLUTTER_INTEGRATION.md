# Flutter Integration Guide

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05
> **目标读者**：希望将本性能面板集成到 Flutter 桌面应用中的开发者。
> **当前状态**: Alpha — 所有 JSON contracts 已稳定，hardware-helper.exe 可独立使用。Flutter 项目本体尚未开发。

---

## 0. 当前可立即复用的接口

| 接口 | 状态 | 说明 |
|------|------|------|
| `hardware-helper.exe` | ✅ 独立可运行 | `Process.start()` + stdout JSON Lines |
| `HelperMessage` 协议 | ✅ JSON Schema | 逐行 `jsonDecode`, `type` 分发 |
| `HardwareSnapshot` / `Settings` / `PanelStatus` | ✅ JSON Schema | 可从 schema 生成 Dart 模型 |
| `packages/contracts/` | ✅ 稳定 | `contractVersion: "1.0.0"` |
| HTTP / WebSocket / Named Pipe / FFI | 🔮 预留 | 接口已规划，代码未实现 |

---

## 1. 集成级别概览

### Level 1：进程级复用（最完整）
- Flutter 启动编译好的 `desktop-performance-panel.exe`
- 面板作为独立悬浮窗口运行
- Flutter 通过未来实现的 WebSocket / Named Pipe 控制面板行为

```
┌──────────────────────────┐
│  Flutter App              │
│  ┌────────────────────┐   │
│  │  Process.start(     │   │
│  │    "perf-panel.exe" │   │
│  │  )                  │   │
│  └────────────────────┘   │
│           │               │
│           ▼               │
│  WebSocket / Named Pipe   │
│           │               │
│           ▼               │
│  ┌────────────────────┐   │
│  │  perf-panel.exe     │   │  ← Tauri + React 前端完整运行
│  │  + hardware-helper  │   │
│  └────────────────────┘   │
└──────────────────────────┘
```

**适用场景**：希望完整保留面板 UI，仅从 Flutter 侧控制显示/隐藏/位置。

### Level 2：数据层复用（推荐入门）
- Flutter 直接启动 `hardware-helper.exe`
- 读取 stdout JSON Lines (HelperMessage 协议)
- Flutter 自己渲染 UI + 管理悬浮窗口
- **React/Tauri 前端完全被替换，硬件采集层被复用**

```
┌──────────────────────────┐
│  Flutter App              │
│  ┌────────────────────┐   │
│  │  Process.start(     │   │
│  │    "helper.exe"     │   │
│  │  )                  │   │
│  │  stdout.listen(...) │   │
│  │  jsonDecode(line)   │   │
│  │  → 更新 UI           │   │
│  └────────────────────┘   │
│           │               │
│           ▼               │
│  ┌────────────────────┐   │
│  │  hardware-helper.exe│   │  ← 只有硬件采集层
│  │  + LibreHWM         │   │
│  └────────────────────┘   │
└──────────────────────────┘
```

**适用场景**：已有 Flutter UI 体系，只需要硬件数据。无需引入 Tauri/React 依赖。

### Level 3：核心库复用
- Rust 核心逻辑编译为 `cdylib` (C ABI 动态库)
- Flutter 通过 `dart:ffi` 直接调用 Rust 函数
- 或通过本地 HTTP / WebSocket 与 Rust core 通信

```
┌──────────────────────────┐
│  Flutter App              │
│  ┌────────────────────┐   │
│  │  dart:ffi           │   │
│  │  ↑                  │   │
│  │  extern "C" fn      │   │
│  └────────────────────┘   │
│           │               │
│           ▼               │
│  ┌────────────────────┐   │
│  │  perf_core.dll      │   │  ← Rust cdylib
│  │  + helper spawning  │   │
│  └────────────────────┘   │
│           │               │
│           ▼               │
│  ┌────────────────────┐   │
│  │  hardware-helper.exe│   │
│  └────────────────────┘   │
└──────────────────────────┘
```

**适用场景**：需要深度集成，避免进程间通信开销。

---

## 2. MVP 阶段已实现的可复用接口

| 接口 | 状态 | Flutter 消费方式 |
|------|------|-----------------|
| `hardware-helper.exe` | ✅ 独立可运行 | `Process.start()` + `stdout.listen()` |
| `HelperMessage` 协议 | ✅ JSON Schema 已定义 | 逐行 `jsonDecode`，match `type` |
| `HardwareSnapshot` 结构 | ✅ JSON Schema 已定义 | 根据 schema 生成 Dart model 类 |
| `Settings` 结构 | ✅ JSON Schema 已定义 | 根据 schema 生成 Dart model 类 |
| `PanelStatus` 结构 | ✅ JSON Schema 已定义 | 根据 schema 生成 Dart model 类 |
| `packages/contracts/` | ✅ 完整 JSON Schema | 可直接生成 Dart 代码 |

### 2.1 从 Flutter 消费 hardware-helper.exe

```dart
import 'dart:async';
import 'dart:convert';
import 'dart:io';

// Level 2 example: start helper and read HardwareSnapshot stream
Future<void> startHelper() async {
  final process = await Process.start(
    'hardware-helper.exe',
    ['--interval-ms', '1000'],
  );

  process.stdout
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .listen((line) {
    try {
      final msg = jsonDecode(line) as Map<String, dynamic>;
      if (msg['type'] == 'snapshot') {
        final snapshot = HardwareSnapshot.fromJson(msg['data']);
        // update your Flutter UI with snapshot
      } else if (msg['type'] == 'error') {
        print('Helper error: ${msg['message']}');
      }
    } catch (e) {
      print('Failed to parse helper message: $e');
    }
  });

  process.stderr
      .transform(utf8.decoder)
      .listen((data) => print('Helper stderr: $data'));
}
```

---

## 3. MVP 阶段不实现的接口（已预留）

| 接口 | 状态 | 说明 |
|------|------|------|
| HTTP REST API | 🔮 预留 | Rust 启动本地 HTTP server |
| WebSocket server | 🔮 预留 | Rust 启动本地 WS server（双向通信） |
| Named Pipe | 🔮 预留 | Windows named pipe IPC |
| Rust FFI (cdylib) | 🔮 预留 | Rust 编译为 C 动态库 |
| Dart shared-types 包 | 🔮 预留 | `packages/dart-types/` 目录 |

### 3.1 预留 HTTP REST 接口设计

```
GET  /api/status          → PanelStatus
GET  /api/snapshot        → HardwareSnapshot (latest)
POST /api/window/show     → void
POST /api/window/hide     → void
POST /api/settings        → Settings (current)
PUT  /api/settings        → Settings (updated)
WS   /api/ws              → HardwareSnapshot stream (push)
```

### 3.2 预留 Named Pipe 设计

```
Pipe name: \\.\pipe\perf-panel-{instance-id}
Protocol: JSON Lines (same as stdout protocol)
Direction: bidirectional
```

---

## 4. Transport 抽象说明

Rust 内部定义了 `HardwareTransport` trait（位于 `transport/mod.rs`），当前实现了两个具体 transport：

- `StdoutTransport` (`transport/stdout_transport.rs`)：管理 helper 进程生命周期，读取 stdout
- `TauriTransport` (`transport/tauri_transport.rs`)：通过 Tauri event system 将数据推送到 React 前端

未来可以新增 transport 实现（无需改动任何核心逻辑）：

| 新 Transport | 文件名 | 作用 |
|-------------|--------|------|
| `WebSocketTransport` | `transport/ws_transport.rs` | Rust 启动本地 WS server |
| `NamedPipeTransport` | `transport/pipe_transport.rs` | Windows named pipe |
| `FfiTransport` | `transport/ffi_transport.rs` | C ABI for FFI consumers |

**核心保证**：
- 所有 transport 实现相同的 trait 接口
- UI 层完全不感知 transport 类型
- 前端始终消费相同的 `HardwareSnapshot` 数据结构
- Flutter 可以选择直接复用 `hardware-helper.exe`，也可以通过新 transport 连接 Rust core

---

## 5. 对 Flutter 开发者的承诺

1. **`hardware-helper.exe` 是独立进程**：除 .NET 8 runtime 外无其他依赖。可直接 `Process.start()` 使用。
2. **HelperMessage 协议稳定**：JSON Schema 是唯一真相源，`type` 枚举只增不改，字段只增不减。
3. **JSON 对象字段顺序不是协议的一部分**：不要依赖 key 顺序。
4. **未来会提供 `packages/dart-types/`**：从 JSON Schema 自动生成的 Dart model 类。
5. **UI 不需要 React/Tauri**：Level 2 可直接替换前端，完全不依赖。
6. **所有命令和事件名记录在 `docs/API_CONTRACT.md`**，不会随意变更。
7. **`contractVersion` 管理兼容性**：SemVer 规则确保可预测的升级路径。

---

## 6. 推荐集成路径

```
Phase 1 (当前): Level 2 - 直接使用 hardware-helper.exe
  └── 最低成本，最短路径获得硬件数据

Phase 2 (未来): Level 1 - 接入完整 Tauri 面板
  └── 如需要面板的窗口管理、托盘、设置等完整功能

Phase 3 (未来): Level 3 - Rust FFI 深度集成
  └── 适合性能敏感场景或需要完全自定义 UI 的场景
```
