import { invoke } from "@tauri-apps/api/core";
import { useMemo, useState } from "react";
import * as THREE from "three";
import { FloatingInfo } from "./components/floating-info";
import { Sidebar } from "./components/sidebar";
import { ModelData } from "./types";
import { Viewer } from "./viewer";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import { save } from "@tauri-apps/plugin-dialog";

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
  const [selectedPath, setSelectedPath] = useState("");
  const [loaded, setLoaded] = useState<ModelData | null>();

  const model: THREE.Object3D | null = useMemo(() => {
    if (!loaded) return null;
    return modelDataToThreeObject(loaded);
  }, [loaded]);

  const reload = () => {
    load(selectedPath);
  };

  const load = async (path: string) => {
    if (!path) return;

    if (model) {
      disposeThreeObject(model);
      setLoaded(null);
    }

    try {
      const buffer = await invoke<ModelData>("read_file", { path });
      setLoaded(buffer);
      setSelectedPath(path);
    } catch {
      console.log("error");
    }
  };

  const exportModel = async (exporterId: string) => {
    if (!model) return;

    const exporter = EXPORTERS.find((x) => x.id === exporterId)!;
    const path = await save({
      defaultPath: selectedPath,
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

  return (
    <div className="app-layout">
      <Sidebar
        selectedPath={selectedPath}
        onPathSelected={load}
        onExportClick={exportModel}
      />
      <Viewer modelKey={selectedPath} model={model} />
      {loaded && <FloatingInfo data={loaded} />}
    </div>
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
