import { PointerEvent, useEffect, useRef, useState } from "react";
import { TextureData } from "../../actions/files";
import { FloatingInfo } from "../floating-info";
import "./styles.css";

interface TextureViewerProps {
  data: TextureData | null;
}

const MIN_SCALE = 0.5;
const MAX_SCALE = 8;
const ZOOM_SPEED = 0.0015;

export function TextureViewer({ data }: TextureViewerProps) {
  const ref = useRef<HTMLDivElement>(null);
  const drag = useRef<any>(null);

  const [dragging, setDragging] = useState(false);
  const [view, setView] = useState({ scale: 1, x: 0, y: 0 });

  const onPointerDown = (e: PointerEvent) => {
    if (e.button == 2) return;
    e.currentTarget.setPointerCapture(e.pointerId);
    drag.current = { px: e.clientX, py: e.clientY };
    setDragging(true);
  };

  const onPointerMove = (e: PointerEvent) => {
    if (!drag.current) return;

    const deltaX = e.clientX - drag.current.px;
    const deltaY = e.clientY - drag.current.py;

    drag.current = { px: e.clientX, py: e.clientY };

    setView((v) => ({ ...v, x: v.x + deltaX, y: v.y + deltaY }));
  };

  const endDrag = () => {
    drag.current = null;
    setDragging(false);
  };

  const reset = () => setView({ scale: 1, x: 0, y: 0 });

  useEffect(() => {
    const containerRef = ref.current;
    if (!containerRef) return;

    const onWheel = (event: WheelEvent) => {
      event.preventDefault();
      const rect = containerRef.getBoundingClientRect();
      const containerX = event.clientX - rect.left;
      const containerY = event.clientY - rect.top;

      setView((v) => {
        const next = Math.min(
          MAX_SCALE,
          Math.max(MIN_SCALE, v.scale * Math.exp(-event.deltaY * ZOOM_SPEED)),
        );

        const ratio = next / v.scale;

        return {
          scale: next,
          x: containerX - (containerX - v.x) * ratio,
          y: containerY - (containerY - v.y) * ratio,
        };
      });
    };

    containerRef.addEventListener("wheel", onWheel, { passive: false });
    return () => containerRef.removeEventListener("wheel", onWheel);
  }, []);

  useEffect(() => {
    reset();
  }, [data]);

  return (
    <div
      ref={ref}
      className="texture-viewer"
      onScroll={(e) => e.preventDefault()}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={endDrag}
      onPointerCancel={endDrag}
      onDoubleClick={reset}
      style={{
        cursor: dragging ? "grabbing" : "grab",
      }}
    >
      <img
        className="texture"
        src={data?.src}
        draggable={false}
        style={{
          transform: `translate(${view.x}px, ${view.y}px) scale(${view.scale})`,
        }}
      />

      <FloatingInfo
        data={{
          Magic: data?.magic,
          "Compression Type": data?.compressionType,
          Width: data?.width,
          Height: data?.height,
        }}
      />
    </div>
  );
}
