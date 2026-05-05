import { usePerformanceStore } from "../../stores/performanceStore";
import { PanelRow } from "./PanelRow";
import { MetricDisplay } from "./MetricDisplay";
import {
  formatPercent,
  formatTemperature,
  formatBytes,
  formatMemory,
  formatVideoMemory,
  formatFrequency,
} from "../../utils/formatters";

export function PerformancePanel() {
  const snapshot = usePerformanceStore((s) => s.snapshot);
  const helperStatus = usePerformanceStore((s) => s.helperStatus);
  const helperMessage = usePerformanceStore((s) => s.helperMessage);

  // No snapshot yet
  if (!snapshot) {
    let text = "Waiting for hardware data...";
    if (helperStatus === "error" && helperMessage) {
      text = helperMessage;
    }
    return (
      <div className="flex h-full items-center justify-center px-4 text-center">
        <span className="text-[10px] leading-relaxed text-gray-400">
          {text}
        </span>
      </div>
    );
  }

  const { cpu, gpu, memory, network, disk } = snapshot;

  return (
    <div className="flex flex-col gap-[1px] py-0.5" data-tauri-drag-region>
      {/* CPU */}
      <PanelRow icon="∎" label="CPU">
        <MetricDisplay value={formatPercent(cpu.usagePercent)} bold />
        <MetricDisplay
          value={formatTemperature(cpu.temperatureC)}
          accentClassName="text-orange-300"
        />
        <MetricDisplay
          value={formatFrequency(cpu.frequencyMhz)}
          accentClassName="text-gray-400"
        />
      </PanelRow>

      {/* GPU */}
      <PanelRow icon="◈" label="GPU">
        <MetricDisplay value={formatPercent(gpu.usagePercent)} bold />
        <MetricDisplay
          value={formatTemperature(gpu.temperatureC)}
          accentClassName="text-orange-300"
        />
        <MetricDisplay
          value={formatVideoMemory(gpu.memoryUsedMb, gpu.memoryTotalMb)}
          accentClassName="text-purple-300"
        />
      </PanelRow>

      {/* RAM */}
      <PanelRow icon="▣" label="RAM">
        <MetricDisplay value={formatPercent(memory.usagePercent)} bold />
        <MetricDisplay
          value={formatMemory(memory.usedGb)}
          label="/"
          accentClassName="text-gray-400"
        />
        <MetricDisplay
          value={formatMemory(memory.totalGb)}
          accentClassName="text-gray-400"
        />
      </PanelRow>

      {/* NET */}
      <PanelRow icon="↓" label="NET">
        <MetricDisplay
          value={formatBytes(network.downloadBps)}
          accentClassName="text-green-300"
        />
        <MetricDisplay
          value={formatBytes(network.uploadBps)}
          label="↑"
          accentClassName="text-blue-300"
        />
      </PanelRow>

      {/* DSK */}
      <PanelRow icon="◉" label="DSK">
        <MetricDisplay
          value={formatBytes(disk.readBps)}
          label="R"
          accentClassName="text-yellow-300"
        />
        <MetricDisplay
          value={formatBytes(disk.writeBps)}
          label="W"
          accentClassName="text-pink-300"
        />
      </PanelRow>
    </div>
  );
}
