import { useEffect } from "react";
import { usePerformanceStore } from "../stores/performanceStore";
import { bridgeListen, EVENTS } from "../services/tauriBridge";
import type { HardwareSnapshot } from "../types";

interface HardwareStatusPayload {
  status: string;
  pid: number | null;
  message: string | null;
}

export function useHardwareSnapshot() {
  const setSnapshot = usePerformanceStore((s) => s.setSnapshot);
  const setHelperStatus = usePerformanceStore((s) => s.setHelperStatus);
  const setHelperMessage = usePerformanceStore((s) => s.setHelperMessage);

  useEffect(() => {
    const u: Array<() => void> = [];

    // hardware:snapshot — real data from Rust backend
    bridgeListen<HardwareSnapshot>(EVENTS.HARDWARE_SNAPSHOT, (payload) => {
      setSnapshot(payload);
    }).then((fn) => u.push(fn));

    // hardware:status — helper process lifecycle
    bridgeListen<HardwareStatusPayload>(EVENTS.HARDWARE_STATUS, (payload) => {
      const status = mapStatus(payload.status);
      setHelperStatus(status);
      if (payload.message) {
        setHelperMessage(payload.message);
      }
    }).then((fn) => u.push(fn));

    // helper:message — debug-only forwarding
    bridgeListen(EVENTS.HELPER_MESSAGE, (payload) => {
      console.debug("[helper:message]", payload);
    }).then((fn) => u.push(fn));

    return () => {
      u.forEach((fn) => fn());
    };
  }, [setSnapshot, setHelperStatus, setHelperMessage]);
}

function mapStatus(raw: string): "running" | "stopped" | "error" {
  switch (raw) {
    case "running":
      return "running";
    case "stopped":
      return "stopped";
    case "error":
      return "error";
    default:
      return "stopped";
  }
}
