interface MetricDisplayProps {
  value: string;
  label?: string;
  accentClassName?: string;
  bold?: boolean;
}

export function MetricDisplay({
  value,
  label,
  accentClassName,
  bold = false,
}: MetricDisplayProps) {
  return (
    <span
      className={`inline-flex items-baseline gap-0.5 whitespace-nowrap ${
        accentClassName ?? "text-gray-200"
      }`}
    >
      {label && <span className="text-[9px] text-gray-500">{label}</span>}
      <span className={`text-[11px] font-mono ${bold ? "font-semibold" : ""}`}>
        {value}
      </span>
    </span>
  );
}