import "./styles.css";

interface ProgressBar {
  min: number;
  max: number;
  value: number;
  step: string;
  onChange(value: number): void;
  width?: string;
}

export function ProgressBar({ min, max, value, step, width, onChange }: ProgressBar) {
  return (
    <div className="progress-bar" style={{ width }}>
      <input
        type="range"
        min={min}
        max={max}
        value={value}
        step={step}
        onChange={(e) => onChange(+e.target.value)}
      />
      <progress max={max} value={value} className="progress-bar-fill" />
    </div>
  );
}
