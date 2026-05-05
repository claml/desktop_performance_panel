// types/index.ts
// Re-export 所有 shared-types
// P2 路径解析说明:
//   - vite.config.ts 已配置 alias: "@shared-types" → "../../packages/shared-types"
//   - tsconfig.json 已配置 paths: "@shared-types/*" → "../packages/shared-types/*"
//   - 项目使用 ES module 时, TS 编译由 Vite 处理, paths 需配合 vite alias 一致
//
// P4 阶段将实际使用这些类型

export type {
  HardwareSnapshot,
  CpuSnapshot,
  GpuSnapshot,
  MemorySnapshot,
  NetworkSnapshot,
  DiskSnapshot,
  BatterySnapshot,
} from "@shared-types/hardware";

export { createEmptySnapshot } from "@shared-types/hardware";

export type {
  HelperMessage,
  HelperMessageType,
  HelperMessageInit,
  HelperMessageSnapshot,
  HelperMessageError,
  HelperMessageStatus,
} from "@shared-types/helper_message";

export {
  isSnapshotMessage,
  isRecoverableError,
} from "@shared-types/helper_message";

export type {
  Settings,
  SettingsPatch,
  VisibleModule,
  Position,
} from "@shared-types/settings";

export {
  DEFAULT_SETTINGS,
  validateOpacity,
  validatePollingIntervalMs,
  validateVisibleModules,
} from "@shared-types/settings";

export type { PanelStatus } from "@shared-types/panel_status";
export { DEFAULT_PANEL_STATUS } from "@shared-types/panel_status";
