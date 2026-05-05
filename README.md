# Desktop Performance Panel

> **版本**: 0.1.0-alpha**contractVersion**: 1.0.0**平台**: Windows

透明悬浮、无边框、始终置顶的 Windows 性能监控面板。实时显示 CPU、GPU、内存、网络、磁盘指标。

* * *

## 功能

* 320×200 透明无边框悬浮窗口，始终置顶
* 实时监控：CPU、GPU、内存、网络、磁盘
* 每秒刷新，数据来自硬件传感器
* 系统托盘：显示/隐藏/切换置顶/切换鼠标穿透/退出
* 鼠标穿透模式（Windows `WS_EX_TRANSPARENT`）
* 窗口透明度调节
* 持久化配置（`%APPDATA%\desktop-performance-panel\settings.json`）
* 硬件采集器（`hardware-helper.exe`）可脱离 UI 独立使用
* 稳定的 API 契约（JSON Schema），预留 Flutter / 外部集成接口

## 快速开始

### 环境要求

* Windows 10/11
* [Node.js](https://nodejs.org/) ≥ 20
* [Rust](https://rustup.rs/) ≥ 1.78
* [.NET SDK](https://dotnet.microsoft.com/) ≥ 8.0
* [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)（C++ 工作负载）
* WebView2 Runtime（Windows 11 已预装）

### 运行

    # 1. 检查环境
    .\scripts\check.ps1
    
    # 2. 构建硬件采集器 + 启动面板
    .\scripts\dev.ps1

面板窗口自动弹出。右键托盘图标可操作菜单。点击窗口关闭按钮（X）为隐藏，托盘持续运行。

### 打包（便携 Alpha）

    .\scripts\package-alpha.ps1
    .\dist-alpha\desktop-performance-panel.exe

## 架构

    ┌─────────────────────────────────────────┐
    │  React + TypeScript + TailwindCSS       │  界面层
    │  ┌─────────────────────────────────┐    │
    │  │  tauriBridge.ts  (IPC 入口)     │    │
    │  └─────────────────────────────────┘    │
    ├─────────────────────────────────────────┤
    │  Rust (Tauri v2)                        │  壳层
    │  hardware_service / settings / window   │
    ├─────────────────────────────────────────┤
    │  C# .NET 8 (hardware-helper.exe)        │  硬件层
    │  LibreHardwareMonitorLib                │
    └─────────────────────────────────────────┘

所有跨层数据通过 `packages/contracts/` 中的 JSON Schema 契约传递。详见 `docs/ARCHITECTURE.md`。

## 目录结构

    desktop-performance-panel/
    ├── apps/desktop/                  # Tauri + React 应用
    │   ├── src/                       # React 前端
    │   └── src-tauri/                 # Rust 后端
    ├── services/hardware-helper/      # C# 独立采集器
    ├── packages/
    │   ├── contracts/                 # JSON Schema（唯一真相源）
    │   └── shared-types/              # TypeScript 类型
    ├── scripts/                       # 构建与开发脚本
    │   ├── check.ps1                  # 环境检查
    │   ├── build-helper.ps1           # 构建 hardware-helper
    │   ├── dev.ps1                    # 一键启动开发环境
    │   ├── clean.ps1                  # 清理构建产物
    │   └── package-alpha.ps1          # 打包便携 Alpha
    ├── docs/                          # 文档
    └── assets/                        # 图标、截图

## 脚本速查

| 脚本  | 用途  |
| --- | --- |
| `.\scripts\check.ps1` | 检查所有前置依赖 |
| `.\scripts\build-helper.ps1` | 构建 hardware-helper.exe |
| `.\scripts\dev.ps1` | 构建 helper + 启动 Tauri 开发模式 |
| `.\scripts\clean.ps1` | 清理编译产物 |
| `.\scripts\clean.ps1 -All` | 同时清理 helper 编译目录 |
| `.\scripts\package-alpha.ps1` | 打包便携 `dist-alpha/` |

## 配置文件

配置保存在 `%APPDATA%\desktop-performance-panel\settings.json`：

    {
      "opacity": 0.85,
      "alwaysOnTop": true,
      "clickThrough": false,
      "position": { "x": 0, "y": 0 },
      "pollingIntervalMs": 1000,
      "visibleModules": ["cpu", "memory", "network", "gpu"],
      "showTemperatures": true
    }

可通过托盘菜单或直接修改文件来切换穿透、置顶等。修改后重启生效（置顶和穿透可通过托盘即时切换）。

## Flutter 集成

`hardware-helper.exe` 是独立进程，通过 stdout 输出 JSON Lines 格式的硬件数据。Flutter 应用可直接消费：

    final process = await Process.start('hardware-helper.exe', ['--interval-ms', '1000']);
    process.stdout
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .listen((line) => /* jsonDecode(line) → HardwareSnapshot */);

详见 `docs/FLUTTER_INTEGRATION.md`。

## 文档

| 文档  | 说明  |
| --- | --- |
| `docs/ARCHITECTURE.md` | 完整架构图与分层说明 |
| `docs/API_CONTRACT.md` | 所有命令、事件、数据结构定义 |
| `docs/FLUTTER_INTEGRATION.md` | Flutter 集成方案与预留接口 |
| `docs/README_DEV.md` | 开发者指南 |
| `docs/ACCEPTANCE.md` | P1–P10 逐阶段验收清单 |
| `docs/KNOWN_ISSUES.md` | 已知问题与解决方式 |
| `docs/RELEASE_ALPHA.md` | Alpha 版本能力与后续路线 |

## 技术栈

| 层   | 技术  |
| --- | --- |
| 界面  | React 19, TypeScript 5, TailwindCSS 4, Zustand 5 |
| 壳   | Tauri v2, Rust 1.78+, serde |
| 硬件采集 | C# .NET 8, LibreHardwareMonitorLib |
| 构建  | Vite 6 |
| 契约  | JSON Schema (Draft 2020-12) |

## 已知问题

* 非管理员权限下 CPU/GPU 温度可能为 `null`（显示 `--`）
* `clickThrough: true` 时无法拖拽窗口，需通过托盘关闭穿透后再移动
* 首次启动网络速率为 0 B/s（1-2 秒后正常）
* 详见 `docs/KNOWN_ISSUES.md`

## 许可

MIT
