import type { HardwareSnapshot } from "../types";

function randAround(center: number, spread: number): number {
  return center + (Math.random() - 0.5) * spread * 2;
}

function clamp(v: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, v));
}

export function generateMockSnapshot(): HardwareSnapshot {
  const cpuUsage = clamp(randAround(35, 12), 0, 100);
  const cpuTemp  = randAround(58, 8);
  const gpuUsage = clamp(randAround(42, 15), 0, 100);
  const gpuTemp  = randAround(72, 6);

  return {
    timestamp: Date.now(),
    cpu: {
      usagePercent: Math.round(cpuUsage * 10) / 10,
      temperatureC: Math.round(cpuTemp * 10) / 10,
      frequencyMhz: Math.round(randAround(3600, 200)),
      powerW: null,
    },
    gpu: {
      name: "NVIDIA GeForce RTX 3060",
      usagePercent: Math.round(gpuUsage * 10) / 10,
      temperatureC: Math.round(gpuTemp * 10) / 10,
      memoryUsedMb: Math.round(randAround(4096, 512)),
      memoryTotalMb: 12288,
      powerW: null,
      fanRpm: null,
    },
    memory: {
      usedGb: Math.round(randAround(8.5, 0.4) * 10) / 10,
      totalGb: 16,
      usagePercent: Math.round(randAround(53, 4) * 10) / 10,
    },
    network: {
      downloadBps: Math.round(Math.max(0, randAround(1048576, 512000))),
      uploadBps: Math.round(Math.max(0, randAround(262144, 200000))),
    },
    disk: {
      readBps: Math.round(Math.max(0, randAround(524288, 400000))),
      writeBps: Math.round(Math.max(0, randAround(262144, 200000))),
    },
    battery: {
      percent: null,
      charging: null,
    },
    error: null,
  };
}
