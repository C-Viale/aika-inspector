import { useContext } from "react";
import { TextureData } from "../../actions/files";
import { AppContext } from "../../contexts/app-context";
import { FloatingInfo } from "../floating-info";
import "./styles.css";

interface TextureViewerProps {
  data: TextureData | null;
}

export function TextureViewer({ data }: TextureViewerProps) {
  const { setBackgroundColor } = useContext(AppContext);

  return (
    <div className="texture-viewer">
      <img src={data?.src} />

      <FloatingInfo
        data={{
          Magic: data?.magic,
          "Compression Type": data?.compressionType,
          Width: data?.width,
          Height: data?.height,
        }}
      />

      <div className="bg-select">
        <div
          className="bg-select-item"
          style={{ backgroundColor: "black" }}
          onClick={() => setBackgroundColor("black")}
        ></div>
        <div
          className="bg-select-item"
          style={{ backgroundColor: "grey" }}
          onClick={() => setBackgroundColor("grey")}
        ></div>
        <div
          className="bg-select-item"
          style={{ backgroundColor: "white" }}
          onClick={() => setBackgroundColor("white")}
        ></div>
      </div>
    </div>
  );
}
