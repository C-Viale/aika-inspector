import "./styles.css";

interface FloatingInfoProps {
  data: Record<string, string | number | null | undefined>;
}

export function FloatingInfo({ data }: FloatingInfoProps) {
  return (
    <div className="floating-info">
      {Object.entries(data).map(([key, value]) => (
        <p>
          <span className="info-key">{key}</span>
          <span className="info-value" data-none={!value}>
            {value ?? "-"}
          </span>
        </p>
      ))}
    </div>
  );
}
