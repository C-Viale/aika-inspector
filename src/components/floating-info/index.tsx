import "./styles.css";

interface FloatingInfoProps {
  data: Record<string, string | number | null | undefined>;
}

export function FloatingInfo({ data }: FloatingInfoProps) {
  return (
    <div className="floating-info">
      {Object.entries(data).map(([key, value]) => (
        <p>
          {key}: <strong>{value}</strong>
        </p>
      ))}
    </div>
  );
}
