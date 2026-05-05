import type { ReactNode } from "react";

interface PanelRowProps {
  icon: string;
  label: string;
  children: ReactNode;
}

export function PanelRow({ icon, label, children }: PanelRowProps) {
  return (
    <div className="flex items-center gap-1.5 px-2.5 py-[3px]">
      <span className="flex w-9 shrink-0 items-center gap-0.5 text-[10px] text-gray-400">
        <span className="leading-none">{icon}</span>
        <span className="font-medium leading-none">{label}</span>
      </span>
      <div className="flex flex-1 items-center justify-end gap-2.5">
        {children}
      </div>
    </div>
  );
}
