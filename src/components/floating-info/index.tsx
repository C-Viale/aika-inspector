import { ModelData } from "../../types";
import "./styles.css";

interface FloatingInfoProps {
  data: ModelData;
}

export function FloatingInfo({ data }: FloatingInfoProps) {
  return (
    <div className="floating-info">
      <p>Vertices: {data.vertices.length}</p>
      <p>Normals: {data.normals.length}</p>
      <p>UV: {data.uv.length}</p>
      <p>Triangles: {data.triangles.length}</p>
    </div>
  );
}
