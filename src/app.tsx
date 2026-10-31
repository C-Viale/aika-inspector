import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useMemo, useState } from "react";
import * as THREE from "three";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import {
  FolderData,
  getFolderStructure,
  readMesh,
  readTexture,
  TextureData,
} from "./actions/files";
import { MeshViewer } from "./components/mesh-viewer";
import { Sidebar } from "./components/sidebar";
import { TextureViewer } from "./components/texture-viewer";
import { ALLOWED_FILES } from "./constants";
import { AppContext } from "./contexts/app-context";
import { ModelData } from "./types";

type ModelExporter = {
  id: string;
  label: string;
  extension: string;
  export(root: THREE.Object3D): Promise<Uint8Array | string>;
};

const glbExporter: ModelExporter = {
  id: "glb",
  label: "GLB (binary glTF)",
  extension: "glb",
  async export(root) {
    const result = await new GLTFExporter().parseAsync(root, { binary: true });
    return new Uint8Array(result as ArrayBuffer);
  },
};

const objExporter: ModelExporter = {
  id: "obj",
  label: "Wavefront OBJ",
  extension: "obj",
  async export(root) {
    return new OBJExporter().parse(root);
  },
};

const EXPORTERS: ModelExporter[] = [glbExporter, objExporter];

export default function App() {
  const [backgroundColor, setBackgroundColor] = useState("black");
  const [folder, setFolder] = useState<FolderData | null>(null);
  const [selectedFilePath, setSelectedFilePath] = useState("");
  const [error, setError] = useState("");

  const [loading, setLoading] = useState(false);

  const [loadedMesh, setLoadedMesh] = useState<ModelData | null>();
  const [loadedTexture, setLoadedTexture] = useState<TextureData | null>(null);

  const model: THREE.Object3D | null = useMemo(() => {
    if (!loadedMesh) return null;
    return modelDataToThreeObject(loadedMesh);
  }, [loadedMesh]);

  const loadMesh = async (path: string) => {
    const response = await readMesh(path);

    disposeMesh();
    disposeTexture();
    setSelectedFilePath(path);

    if ("data" in response) {
      setLoadedMesh(response.data);
      setError("");
    } else {
      setError(response.error);
    }
  };

  const loadTexture = async (path: string) => {
    const response = await readTexture(path);

    disposeMesh();
    disposeTexture();
    setSelectedFilePath(path);

    if ("data" in response) {
      setLoadedTexture(response.data);
      setError("");
    } else {
      setError(response.error);
    }
  };

  const disposeTexture = () => {
    if (!loadedTexture) return;
    URL.revokeObjectURL(loadedTexture.src);
    setLoadedTexture(null);
  };

  const disposeMesh = () => {
    if (!model) return;
    disposeThreeObject(model);
    setLoadedMesh(null);
  };

  const load = async (path: string) => {
    if (!path) return;

    const extension = path.split(".").at(-1);
    const type = ALLOWED_FILES[extension as any];

    setLoading(true);
    if (type == "mesh") await loadMesh(path);
    if (type === "texture") await loadTexture(path);
    setLoading(false);
  };

  const exportModel = async (exporterId: string) => {
    if (!model) return;

    const exporter = EXPORTERS.find((x) => x.id === exporterId)!;
    const path = await save({
      defaultPath: selectedFilePath,
      filters: [{ name: exporter.label, extensions: [exporter.extension] }],
    });
    if (!path) return;

    try {
      const output = await exporter.export(model);
      const bytes =
        typeof output === "string" ? new TextEncoder().encode(output) : output;
      await invoke("write_file", bytes, {
        headers: { path: encodeURIComponent(path) },
      });
    } catch (e) {
      console.log(
        `Export failed: ${e instanceof Error ? e.message : String(e)}`,
      );
    }
  };

  const openFolderDialog = async () => {
    const folderPath = await open({
      directory: true,
      multiple: false,
    });

    if (!folderPath) return;

    const response = await getFolderStructure(folderPath);

    if ("data" in response) {
      setFolder(response.data);
      setError("");
    } else {
      setError(response.error);
    }
  };

  return (
    <AppContext
      value={{ selectedFilePath, folder, backgroundColor, setBackgroundColor }}
    >
      <div className="app-layout">
        <Sidebar
          folder={folder}
          selectedFilePath={selectedFilePath}
          onPathSelected={load}
          onExportClick={exportModel}
          onOpenFolderClick={openFolderDialog}
        />
        <div className="viewport" style={{ backgroundColor }}>
          {loadedMesh && (
            <MeshViewer
              data={loadedMesh}
              modelKey={selectedFilePath}
              model={model}
            />
          )}
          {loadedTexture && <TextureViewer data={loadedTexture} />}
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

          <div className="settings-bg-select">
            <div
              className="settings-bg-select-item"
              style={{ backgroundColor: "black" }}
              onClick={() => {
                console.log("uai");
                setBackgroundColor("black");
              }}
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

function modelDataToThreeObject(ir: ModelData) {
  const group = new THREE.Object3D();
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute(
    "position",
    new THREE.BufferAttribute(
      new Float32Array(ir.vertices.flatMap((v) => [v.x, v.y, v.z])),
      3,
    ),
  );

  geometry.setIndex(ir.triangles.flatMap((t) => [t.x, t.y, t.z]));

  const material = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    side: THREE.DoubleSide,
  });
  const mesh = new THREE.Mesh(geometry, material);

  geometry.computeBoundingSphere();

  group.add(mesh);
  return group;
}

function disposeThreeObject(obj: THREE.Object3D) {
  obj.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (!mesh.isMesh) return;
    mesh.geometry?.dispose();
    const mats = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
    mats.forEach((m) => m?.dispose());
  });
}
