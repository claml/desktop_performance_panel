# Tasks

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05
> **Status**: P1 进行中

---

## 阶段概览

| 阶段 | 名称 | 状态 | 产出物 |
|------|------|------|--------|
| **P1** | Contract 与文档 | 🔄 进行中 | 所有 JSON Schema、shared-types、文档 |
| **P2** | Tauri + React + Tailwind 骨架 | ⬜ 待开始 | 项目骨架，能启动 |
| **P3** | 透明无边框置顶窗口 | ⬜ 待开始 | tauri.conf.json + window_service + CSS |
| **P4** | React Mock 性能面板 | ⬜ 待开始 | 组件、store、假数据渲染 |
| **P5** | C# hardware-helper | ⬜ 待开始 | 独立运行，stdout HelperMessage JSON Lines |
| **P6** | Rust 读取 helper stdout | ⬜ 待开始 | spawn、解析 HelperMessage、缓存 snapshot |
| **P7** | Rust emit + React 真实数据 | ⬜ 待开始 | 替换 mock，面板实时显示 |
| **P8** | settings.json + settings_update | ⬜ 待开始 | settings_service + SettingsOverlay |
| **P9** | 托盘菜单 | ⬜ 待开始 | tray_service + tray-icon feature |
| **P10** | Windows 鼠标穿透 | ⬜ 待开始 | WS_EX_TRANSPARENT + WS_EX_LAYERED |
| **P11** | 文档与验收 | ⬜ 待开始 | 最终一致性检查 |

---

## P1：Contract 与文档

### 产出文件

| 文件 | 状态 |
|------|------|
| `packages/contracts/header.json` | ✅ |
| `packages/contracts/hardware_snapshot.schema.json` | ✅ |
| `packages/contracts/helper_message.schema.json` | ✅ |
| `packages/contracts/settings.schema.json` | ✅ |
| `packages/contracts/panel_status.schema.json` | ✅ |
| `packages/contracts/commands.schema.json` | ✅ |
| `packages/contracts/events.schema.json` | ✅ |
| `packages/contracts/README.md` | ✅ |
| `packages/shared-types/hardware.ts` | ✅ |
| `packages/shared-types/helper_message.ts` | ✅ |
| `packages/shared-types/settings.ts` | ✅ |
| `packages/shared-types/panel_status.ts` | ✅ |
| `docs/API_CONTRACT.md` | ✅ |
| `docs/FLUTTER_INTEGRATION.md` | ✅ |
| `docs/ARCHITECTURE.md` | ✅ |
| `docs/TASKS.md` | ✅ |

### 验收标准

- [ ] `packages/contracts/header.json` 含 `contractVersion: "1.0.0"`
- [ ] `hardware_snapshot.schema.json` 通过 JSON Schema 校验 (`additionalProperties: false`, `required` 明确, nullable 用 `type: ["number", "null"]`)
- [ ] `helper_message.schema.json` 通过 JSON Schema 校验，含 `init`/`snapshot`/`error`/`status` 四种 `oneOf`
- [ ] `settings.schema.json` 通过 JSON Schema 校验，含 `minimum`/`maximum` 约束
- [ ] `panel_status.schema.json` 通过 JSON Schema 校验，`latestSnapshot` 为 `HardwareSnapshot | null`
- [ ] `commands.schema.json` 每个命令含 `logicalName`、`tauriCommandName`、`paramsSchema`、`returnsSchema`、`description`
- [ ] `events.schema.json` 含所有事件定义
- [ ] `README.md` 含版本管理规则
- [ ] 所有 `shared-types/*.ts` 类型与对应 schema 一致
- [ ] `docs/API_CONTRACT.md` 含逻辑名/snake_case 映射、HelperMessage 协议、版本规则、tauriBridge 映射表
- [ ] `docs/FLUTTER_INTEGRATION.md` 含 Level 1/2/3 说明、Transport 抽象、Dart 示例代码
- [ ] `docs/ARCHITECTURE.md` 含总体架构图、分层图、数据流图、Tauri v2 注意事项、Transport 抽象
- [ ] `docs/TASKS.md` 含完整任务清单和当前进度

---

## P2：Tauri + React + Tailwind 骨架

### 验收标准

- [ ] `npm create tauri-app` 成功创建项目于 `apps/desktop/`
- [ ] 项目名称为 `desktop-performance-panel`
- [ ] Tauri v2（最新稳定版）
- [ ] React + TypeScript 模板
- [ ] `npm run tauri dev` 能启动默认窗口
- [ ] Tailwind CSS v3 配置完成
- [ ] `tsconfig.json` `strict: true`
- [ ] `package.json` scripts 完整（`dev` / `build` / `preview`）

---

## P3：透明无边框置顶窗口

### 验收标准

- [ ] `tauri.conf.json` 窗口配置：
  - `decorations: false`（无边框）
  - `alwaysOnTop: true`（置顶）
  - `transparent: true`（透明）
  - `resizable: false`（固定尺寸）
  - 窗口大小 320×200
- [ ] Rust `window_service.rs` 实现窗口属性读写
- [ ] `index.css` 设置 `html, body` 背景透明
- [ ] 窗口通过 `data-tauri-drag-region` 可拖拽
- [ ] 窗口在桌面其他窗口上方显示

---

## P4：React Mock 性能面板

### 验收标准

- [ ] `PerformancePanel.tsx` 根组件，从 store 读取 snapshot
- [ ] `PanelRow.tsx` 渲染指标行
- [ ] `MetricDisplay.tsx` 渲染单个指标（标签 + 数值 + 单位）
- [ ] `DragHandle.tsx` 提供 `data-tauri-drag-region` 拖拽区域
- [ ] `performanceStore.ts` (Zustand) 管理 snapshot 和 settings
- [ ] `useHardwareSnapshot.ts` 使用 setInterval mock 假数据（HardwareSnapshot 格式，每秒更新）
- [ ] 面板可拖拽移动
- [ ] 右键弹出上下文菜单
- [ ] `useSettings.ts` 已创建，暂用于 mock
- [ ] `services/tauriBridge.ts` 已创建但暂不使用

---

## P5：C# hardware-helper

### 验收标准

- [ ] `dotnet run` 可独立运行
- [ ] 启动后首行输出 `{"type":"init","version":"1.0.0","timestamp":...}`
- [ ] 每 `--interval-ms`（默认 1000）输出一行 snapshot
- [ ] snapshot 的 `data` 字段完全符合 HardwareSnapshot JSON Schema
- [ ] **能读取到的传感器返回真实值，读取不到的返回 `null`**
- [ ] **不因为缺少某个传感器而崩溃**
- [ ] `error` 字段（snapshot 内）或 error 消息中说明缺失原因
- [ ] 普通权限下能运行
- [ ] 需要管理员权限的传感器：输出 error 提示但不强制，继续运行
- [ ] GPU 信息在 NVIDIA/AMD/Intel 不同设备上允许字段缺失（`name` 可为 `null`）
- [ ] CPU 使用率：通过性能计数器或 LHM 获取
- [ ] 内存使用率：通过 `GlobalMemoryStatusEx` 获取，必须真实
- [ ] 网络速率：通过性能计数器获取（可能为 0）
- [ ] `LibreHardwareMonitorLib` NuGet 引用正确
- [ ] `HelperMessage` 包装正确（`type`/`version`/`timestamp`）
- [ ] 编译输出为 `hardware-helper.exe`（单文件/自包含发布）

---

## P6：Rust 读取 helper stdout

### 验收标准

- [ ] Rust 能通过相对路径或固定路径找到 `hardware-helper.exe`
- [ ] Rust spawn 进程时传入正确的 `--interval-ms` 参数
- [ ] 逐行读取 stdout（`BufReader::read_line`）
- [ ] 正确解析 `HelperMessage`：按 `type` 分发处理
- [ ] `type=init` → 校验 version，emit `hardware:status`
- [ ] `type=snapshot` → 提取 `data`，缓存到 `AppState.latest_snapshot`
- [ ] `type=error` → 记录日志；`recoverable=false` 时标记 helper 异常
- [ ] `type=status` → 记录日志
- [ ] 解析失败的行（非 JSON）→ 记录错误日志，跳过
- [ ] helper 崩溃/退出时：检测退出码，尝试重启（最多 3 次）
- [ ] `hardware_start`/`hardware_stop`/`hardware_restart` 命令能控制 helper
- [ ] `hardware_get_latest_snapshot` 返回缓存的 snapshot（无数据时返回全 null 结构）

---

## P7：Rust emit + React 真实数据

### 验收标准

- [ ] Rust 在收到 `type=snapshot` 后 emit `hardware:snapshot`
- [ ] Rust 在 helper 启动/停止/崩溃时 emit `hardware:status`
- [ ] React `useHardwareSnapshot.ts` 改为 `listen("hardware:snapshot")`（移除 mock）
- [ ] `performanceStore.ts` 实时更新真实数据
- [ ] 面板显示真实 CPU 使用率
- [ ] 面板显示真实内存使用率
- [ ] 面板显示真实网络速率
- [ ] 面板显示 GPU 使用率（如机器有 GPU）
- [ ] 面板显示 CPU 温度（如传感器可用）
- [ ] 面板显示 GPU 温度（如传感器可用）
- [ ] `null` 值显示为 `--` 或 `N/A`
- [ ] 数据缺失不导致面板崩溃
- [ ] `tauriBridge.ts` 的映射正常工作

---

## P8：settings.json + settings_update

### 验收标准

- [ ] Rust 启动时读取 `%APPDATA%/perf-panel/settings.json`
- [ ] 文件不存在时创建默认 settings.json
- [ ] `settings_get` 返回当前 Settings
- [ ] `settings_update` 接受 `{ patch: Partial<Settings> }` 参数
- [ ] `settings_update` 对 `opacity` 做 `0.1 ~ 1.0` 校验
- [ ] `settings_update` 对 `pollingIntervalMs` 做 `500 ~ 10000` 校验
- [ ] 校验失败返回错误字符串，不修改文件
- [ ] 校验通过后 merge 并写回文件
- [ ] `settings_update` 返回合并后的 Settings
- [ ] `settings_reset` 恢复默认并写回文件
- [ ] `opacity` 更改即时生效（调用 `window_set_opacity`）
- [ ] `pollingIntervalMs` 更改：Rust 重启 helper 并传入新 `--interval-ms`（方案 A）
- [ ] `settings:changed` 事件在 settings 变更后 emit
- [ ] React `SettingsOverlay.tsx`：透明度滑块、模块开关、穿透开关
- [ ] 设置面板可通过右键菜单打开

---

## P9：托盘菜单

### 验收标准

- [ ] Cargo.toml 启用 `tray-icon` feature
- [ ] 系统托盘显示图标
- [ ] 托盘菜单项：
  - "显示面板" → `window_show`
  - "隐藏面板" → `window_hide`
  - "置顶" (Check) → `window_set_always_on_top`
  - "鼠标穿透" (Check) → `window_set_click_through`
  - "退出" → 终止 helper + 退出应用
- [ ] 双击托盘图标切换显示/隐藏
- [ ] 菜单项状态与 settings 同步
- [ ] 关闭窗口不退出应用，仅隐藏（托盘持续运行）

---

## P10：Windows 鼠标穿透

### 验收标准

- [ ] 仅 Windows 平台实现
- [ ] `window_set_click_through({ value: true })` 设置 `WS_EX_TRANSPARENT | WS_EX_LAYERED`
- [ ] 穿透状态下鼠标事件完全透传
- [ ] `window_set_click_through({ value: false })` 恢复正常交互
- [ ] 通过托盘菜单切换
- [ ] 通过设置面板切换
- [ ] 穿透状态持久化到 `settings.clickThrough`
- [ ] 非 Windows 平台：命令返回 `Err("unsupported platform")`
- [ ] 穿透模式下自动禁用窗口拖拽，反之恢复

---

## P11：文档与验收

### 验收标准

- [ ] `README.md` 含项目简介、架构图、运行步骤、技术栈
- [ ] `API_CONTRACT.md` 与所有 schema 和代码实现一致
- [ ] `FLUTTER_INTEGRATION.md` 含 Level 1/2/3 说明和 transport 抽象
- [ ] `ARCHITECTURE.md` 含总体架构图、Tauri v2 注意事项、transport 说明
- [ ] `TASKS.md` 含完整任务清单和当前进度
- [ ] 所有 command 和事件名在文档中与代码一致
- [ ] 所有 JSON 字段与 schema 一致
- [ ] `packages/contracts/README.md` 版本管理规则完整
