import { MeshData } from "#/types";
import { FloatingInfo } from "#components/floating-info";
import { disposeThreeObject, modelDataToThreeObject } from "#utils/functions/three";
import { Bounds, Center, OrbitControls, useHelper } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { useEffect, useMemo, useRef } from "react";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import * as THREE from "three";

interface MeshViewerProps {
  selectedFileName: string;
  data: MeshData;
}

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

export function MeshViewer({ data, selectedFileName }: MeshViewerProps) {
  const prev = useRef<THREE.Object3D>(null);

  const model: THREE.Object3D | null = useMemo(() => {
    if (!data) return null;
    if (prev.current) disposeThreeObject(prev.current);

    const obj = modelDataToThreeObject(data);
    prev.current = obj;
    return obj;
  }, [data]);

  const exportModel = async (exporterId: string) => {
    if (!model) return;

    // const exporter = EXPORTERS.find((x) => x.id === exporterId)!;
    // // const path = await save({
    // //   defaultPath: selectedFilePath,
    // //   filters: [{ name: exporter.label, extensions: [exporter.extension] }],
    // // });
    // if (!path) return;

    // try {
    //   const output = await exporter.export(model);
    //   const bytes = typeof output === "string" ? new TextEncoder().encode(output) : output;
    //   await invoke("write_file", bytes, {
    //     headers: { path: encodeURIComponent(path) },
    //   });
    // } catch (e) {
    //   console.log(`Export failed: ${e instanceof Error ? e.message : String(e)}`);
    // }
  };

  // useEffect(() => {
  //   return () => {
  //     if (model) disposeThreeObject(model);
  //   };
  // }, [data]);

  return (
    <>
      <FloatingInfo
        data={{
          File: selectedFileName,
          Vertices: data?.vertices.length,
          Normals: data?.normals.length,
          UV: data?.uv.length,
          Triangles: data?.triangles.length,
        }}
      />
      <Canvas style={{ height: "100vh" }} camera={{ position: [3, 2, 4], fov: 45 }} dpr={[1, 2]}>
        <hemisphereLight args={["#ffffff", "#8a9299", 1.1]} />
        <directionalLight position={[5, 8, 5]} intensity={1.6} />
        {model && <Model model={model} modelKey={selectedFileName} />}
        <OrbitControls makeDefault />
      </Canvas>
    </>
  );
}

interface ModelProps {
  modelKey: string;
  model: THREE.Object3D;
}
function Model({ modelKey, model }: ModelProps) {
  const ref = useRef<any>(null);
  useHelper(ref, THREE.SkeletonHelper);

  return (
    <Bounds key={modelKey} fit clip observe margin={1.4}>
      <Center>
        <primitive ref={ref} object={model} />
      </Center>
    </Bounds>
  );
}
