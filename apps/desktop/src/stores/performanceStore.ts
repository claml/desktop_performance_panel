import { create } from "zustand";
import type { HardwareSnapshot } from "../types";

export type HelperStatus = "unknown" | "running" | "stopped" | "error";

interface PerformanceState {
  snapshot: HardwareSnapshot | null;
  helperStatus: HelperStatus;
  helperMessage: string | null;
  lastSnapshotAt: number | null;

  setSnapshot: (s: HardwareSnapshot) => void;
  setHelperStatus: (status: HelperStatus) => void;
  setHelperMessage: (message: string | null) => void;
}

export const usePerformanceStore = create<PerformanceState>((set) => ({
  snapshot: null,
  helperStatus: "unknown",
  helperMessage: null,
  lastSnapshotAt: null,

  setSnapshot: (snapshot) =>
    set({ snapshot, lastSnapshotAt: Date.now() }),

  setHelperStatus: (status) =>
    set((prev) => ({
      helperStatus: status,
      helperMessage: status === "running" ? null : prev.helperMessage,
    })),

  setHelperMessage: (message) => set({ helperMessage: message }),
}));
