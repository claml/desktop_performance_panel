export function formatPercent(value: number | null): string {
  if (value === null) return "--";
  return `${Math.round(value)}%`;
}

export function formatTemperature(value: number | null): string {
  if (value === null) return "--";
  return `${Math.round(value)}°C`;
}

export function formatBytes(value: number | null): string {
  if (value === null) return "--";
  if (value < 0) return "--";
  if (value >= 1048576) return `${(value / 1048576).toFixed(1)} MB/s`;
  if (value >= 1024) return `${(value / 1024).toFixed(1)} KB/s`;
  return `${Math.round(value)} B/s`;
}

export function formatMemory(value: number | null): string {
  if (value === null) return "--";
  return `${value.toFixed(1)} GB`;
}

export function formatVideoMemory(usedMb: number | null, totalMb: number | null): string {
  if (usedMb === null || totalMb === null) return "--";
  const u = usedMb / 1024;
  const t = totalMb / 1024;
  if (t >= 1) return `${u.toFixed(1)}/${t.toFixed(1)} GB`;
  return `${Math.round(usedMb)}/${Math.round(totalMb)} MB`;
}

export function formatFrequency(value: number | null): string {
  if (value === null) return "--";
  if (value >= 1000) return `${(value / 1000).toFixed(1)} GHz`;
  return `${Math.round(value)} MHz`;
}
