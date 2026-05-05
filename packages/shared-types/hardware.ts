// packages/shared-types/hardware.ts
// Contract version: 1.0.0
// Source schema: packages/contracts/hardware_snapshot.schema.json
//
// NOTE: All numeric fields are `number | null`.
// `null` means the sensor is unavailable on this machine.
// `0` means the value is actually zero — do NOT treat 0 as unavailable.
// Field order is not significant; consumers must not rely on key ordering.

export interface HardwareSnapshot {
  timestamp: number;
  cpu: CpuSnapshot;
  gpu: GpuSnapshot;
  memory: MemorySnapshot;
  network: NetworkSnapshot;
  disk: DiskSnapshot;
  battery: BatterySnapshot;
  error: string | null;
}

export interface CpuSnapshot {
  usagePercent: number | null;
  temperatureC: number | null;
  frequencyMhz: number | null;
  powerW: number | null;
}

export interface GpuSnapshot {
  name: string | null;
  usagePercent: number | null;
  temperatureC: number | null;
  memoryUsedMb: number | null;
  memoryTotalMb: number | null;
  powerW: number | null;
  fanRpm: number | null;
}

export interface MemorySnapshot {
  usedGb: number | null;
  totalGb: number | null;
  usagePercent: number | null;
}

export interface NetworkSnapshot {
  downloadBps: number | null;
  uploadBps: number | null;
}

export interface DiskSnapshot {
  readBps: number | null;
  writeBps: number | null;
}

export interface BatterySnapshot {
  percent: number | null;
  charging: boolean | null;
}

/** Returns a fully-null HardwareSnapshot (used as default/initial state before real data arrives) */
export function createEmptySnapshot(timestamp = Date.now()): HardwareSnapshot {
  return {
    timestamp,
    cpu: {
      usagePercent: null,
      temperatureC: null,
      frequencyMhz: null,
      powerW: null,
    },
    gpu: {
      name: null,
      usagePercent: null,
      temperatureC: null,
      memoryUsedMb: null,
      memoryTotalMb: null,
      powerW: null,
      fanRpm: null,
    },
    memory: {
      usedGb: null,
      totalGb: null,
      usagePercent: null,
    },
    network: {
      downloadBps: null,
      uploadBps: null,
    },
    disk: {
      readBps: null,
      writeBps: null,
    },
    battery: {
      percent: null,
      charging: null,
    },
    error: null,
  };
}
