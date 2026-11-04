import { getFolderStructure, readFile } from "#actions/files";
import { AudioViewer } from "#components/audio-viewer";
import { MeshViewer } from "#components/mesh-viewer";
import { Sidebar } from "#components/sidebar";
import { TextureViewer } from "#components/texture-viewer";
import { AppContext } from "#contexts/app-context";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { FolderData, LoadedFile } from "./types";
import { getFileNameFromPath } from "#utils/functions/path";

export default function App() {
  const [backgroundColor, setBackgroundColor] = useState("black");
  const [folder, setFolder] = useState<FolderData | null>(null);
  const [selectedFilePath, setSelectedFilePath] = useState("");

  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const [loadedFile, setLoadedFile] = useState<LoadedFile | null>(null);

  const disposeObjectURL = () => {
    if (!loadedFile) return;
    if (!["texture", "audio"].includes(loadedFile.type)) return;
    URL.revokeObjectURL(loadedFile.data.src);
  };

  const load = async (path: string) => {
    if (!path) return;
    if (path === selectedFilePath) return;

    setSelectedFilePath(path);
    setLoading(true);
    const response = await readFile(path);
    disposeObjectURL();

    if (response.isOk()) {
      setLoadedFile(response.data!);
      setError("");
    } else {
      setLoadedFile(null);
      setError(response.error!);
    }

    setLoading(false);
  };

  const openFolderDialog = async () => {
    const folderPath = await open({
      directory: true,
      multiple: false,
    });

    if (!folderPath) return;

    const response = await getFolderStructure(folderPath);

    if (response.isOk()) {
      setFolder(response.data!);
      setError("");
    } else {
      setError(response.error!);
    }
  };

  const selectedFileName = getFileNameFromPath(selectedFilePath, false);

  return (
    <AppContext value={{ selectedFilePath, folder, backgroundColor, setBackgroundColor }}>
      <div className="app-layout">
        <Sidebar
          folder={folder}
          selectedFilePath={selectedFilePath}
          onPathSelected={load}
          onOpenFolderClick={openFolderDialog}
        />
        <div className="viewport" style={{ backgroundColor }}>
          {loadedFile?.type === "mesh" && (
            <MeshViewer data={loadedFile.data} selectedFileName={selectedFileName} />
          )}
          {loadedFile?.type === "texture" && (
            <>
              <TextureViewer data={loadedFile.data} selectedFileName={selectedFileName} />
              <div className="settings-bg-select">
                <div
                  className="settings-bg-select-item"
                  style={{ backgroundColor: "black" }}
                  onClick={() => setBackgroundColor("black")}
                ></div>
                <div
                  className="settings-bg-select-item"
                  style={{ backgroundColor: "grey" }}
                  onClick={() => setBackgroundColor("grey")}
                ></div>
                <div
                  className="settings-bg-select-item"
                  style={{ backgroundColor: "white" }}
                  onClick={() => setBackgroundColor("white")}
                ></div>
              </div>
            </>
          )}
          {loadedFile?.type === "audio" && (
            <AudioViewer selectedFileName={selectedFilePath} src={loadedFile.data.src} />
          )}
          {error && (
            <div
              style={{
                color: "white",
                padding: ".5rem",
                backgroundColor: "#750c0c",
                position: "absolute",
                bottom: 0,
                left: "50%",
                transform: "translateX(-50%)",
              }}
            >
              {error}
            </div>
          )}
        </div>
        {loading && (
          <div className="loading-container">
            <div>Loading file...</div>
          </div>
        )}
      </div>
    </AppContext>
  );
}
