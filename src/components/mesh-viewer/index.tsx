import { Bounds, Center, OrbitControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import * as THREE from "three";
import { ModelData } from "../../types";
import { FloatingInfo } from "../floating-info";

interface MeshViewerProps {
  data: ModelData | null;
  modelKey: string;
  model: THREE.Object3D | null;
}

export function MeshViewer({ data, modelKey, model }: MeshViewerProps) {
  return (
    <>
      <FloatingInfo
        data={{
          Vertices: data?.vertices.length,
          Normals: data?.normals.length,
          UV: data?.uv.length,
          Triangles: data?.triangles.length,
        }}
      />
      <Canvas
        style={{ height: "100vh" }}
        camera={{ position: [3, 2, 4], fov: 45 }}
        dpr={[1, 2]}
      >
        <hemisphereLight args={["#ffffff", "#8a9299", 1.1]} />
        <directionalLight position={[5, 8, 5]} intensity={1.6} />
        {model && (
          <Bounds key={modelKey} fit clip observe margin={1.4}>
            <Center>
              <primitive object={model} />
            </Center>
          </Bounds>
        )}
        <OrbitControls makeDefault />
      </Canvas>
    </>
  );
}
