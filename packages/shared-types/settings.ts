// packages/shared-types/settings.ts
// Contract version: 1.0.0
// Source schema: packages/contracts/settings.schema.json

export type VisibleModule = 'cpu' | 'memory' | 'network' | 'gpu' | 'disk' | 'battery';

export interface Position {
  x: number;
  y: number;
}

export interface Settings {
  opacity: number;                  // 0.1 ~ 1.0
  alwaysOnTop: boolean;
  clickThrough: boolean;
  position: Position;
  pollingIntervalMs: number;        // 500 ~ 10000
  visibleModules: VisibleModule[];
  showTemperatures: boolean;
}

/** Partial<Settings> used as patch payload for settings_update command */
export type SettingsPatch = Partial<Settings>;

/** Default settings used on first launch or after settings_reset */
export const DEFAULT_SETTINGS: Settings = {
  opacity: 0.85,
  alwaysOnTop: true,
  clickThrough: false,
  position: { x: 0, y: 0 },
  pollingIntervalMs: 1000,
  visibleModules: ['cpu', 'memory', 'network', 'gpu'],
  showTemperatures: true,
};

/** Validates that a value falls within the allowed range */
export function validateOpacity(value: number): boolean {
  return value >= 0.1 && value <= 1.0;
}

export function validatePollingIntervalMs(value: number): boolean {
  return value >= 500 && value <= 10000;
}

export function validateVisibleModules(modules: string[]): modules is VisibleModule[] {
  const valid: VisibleModule[] = ['cpu', 'memory', 'network', 'gpu', 'disk', 'battery'];
  return modules.every((m) => valid.includes(m as VisibleModule));
}
