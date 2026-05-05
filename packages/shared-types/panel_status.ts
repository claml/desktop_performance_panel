// packages/shared-types/panel_status.ts
// Contract version: 1.0.0
// Source schema: packages/contracts/panel_status.schema.json

import type { HardwareSnapshot } from './hardware';

export interface PanelStatus {
  windowVisible: boolean;
  alwaysOnTop: boolean;
  clickThrough: boolean;
  opacity: number;
  helperRunning: boolean;
  helperPid: number | null;
  latestSnapshot: HardwareSnapshot | null;
}

/** Default panel status before backend is initialized */
export const DEFAULT_PANEL_STATUS: PanelStatus = {
  windowVisible: true,
  alwaysOnTop: true,
  clickThrough: false,
  opacity: 0.85,
  helperRunning: false,
  helperPid: null,
  latestSnapshot: null,
};
