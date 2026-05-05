// services/tauriBridge.ts
// ★ 唯一 IPC 入口 — 所有 invoke / listen 必须经过此文件
// 组件不允许直接 import { invoke } from '@tauri-apps/api/core'

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

/**
 * logicalName → tauriCommandName 映射表
 * 对应 packages/contracts/commands.schema.json 中的定义
 */
export const COMMAND_MAP: Record<string, string> = {
  "settings:get":     "settings_get",
  "settings:update":  "settings_update",
  "settings:reset":   "settings_reset",
  "window:show":                 "window_show",
  "window:hide":                 "window_hide",
  "window:set_always_on_top":    "window_set_always_on_top",
  "window:set_click_through":    "window_set_click_through",
  "window:set_opacity":          "window_set_opacity",
  "window:set_position":         "window_set_position",
  "window:get_position":         "window_get_position",
  "hardware:start":               "hardware_start",
  "hardware:stop":                "hardware_stop",
  "hardware:restart":             "hardware_restart",
  "hardware:get_latest_snapshot": "hardware_get_latest_snapshot",
  "panel:get_status": "panel_get_status",
} as const;

export const EVENTS = {
  HARDWARE_SNAPSHOT: "hardware:snapshot",
  HARDWARE_STATUS:   "hardware:status",
  SETTINGS_CHANGED:  "settings:changed",
  HELPER_MESSAGE:    "helper:message",
} as const;

/**
 * Invoke a Tauri command by logical name.
 * Maps logicalName → snake_case command name via COMMAND_MAP.
 */
export async function bridgeInvoke<T>(
  logicalName: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const cmd = COMMAND_MAP[logicalName];
  if (!cmd) throw new Error(`Unknown command: ${logicalName}`);
  return invoke<T>(cmd, args);
}

/**
 * Listen to a Tauri event. Returns an unlisten function for cleanup.
 */
export async function bridgeListen<T>(
  event: string,
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(event, (e) => handler(e.payload));
}
