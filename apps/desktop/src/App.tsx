import { getCurrentWindow } from "@tauri-apps/api/window";
import { PerformancePanel } from "./components/panel/PerformancePanel";
import { useHardwareSnapshot } from "./hooks/useHardwareSnapshot";

function App() {
  useHardwareSnapshot();

  const handleStartDrag = async (event: React.PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;

    try {
      await getCurrentWindow().startDragging();
    } catch (error) {
      console.error("Failed to start window dragging:", error);
    }
  };

  return (
    <div
      data-tauri-drag-region
      onPointerDown={handleStartDrag}
      className="flex h-[200px] w-[320px] select-none bg-transparent p-2"
    >
      <div
        data-tauri-drag-region
        className="flex h-full w-full flex-col overflow-hidden rounded-xl border border-white/10 bg-gray-900/60 text-white shadow-2xl backdrop-blur-md"
      >
        {/* Title bar */}
        <div data-tauri-drag-region className="flex h-[22px] shrink-0 items-center justify-center border-b border-white/5">
          <span className="text-[10px] font-medium tracking-wide text-gray-400">
            Desktop Performance Panel
          </span>
        </div>

        {/* Performance rows */}
        <div className="flex-1">
          <PerformancePanel />
        </div>
      </div>
    </div>
  );
}

export default App;
