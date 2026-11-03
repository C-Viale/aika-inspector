import * as THREE from "three";
import { MeshData } from "#/types";

export function modelDataToThreeObject(ir: MeshData) {
  // #region OK

  const group = new THREE.Object3D();
  const geometry = new THREE.BufferGeometry();

  const position = new Float32Array(ir.vertices.flatMap((v) => [v.x, v.y, v.z]));
  const triangles = ir.triangles.flatMap((t) => [t.x, t.y, t.z]);

  geometry.setAttribute("position", new THREE.BufferAttribute(position, 3));
  geometry.setIndex(triangles);

  if (ir.normals.length > 0) {
    const normals = new Float32Array(ir.normals.flatMap((v) => [v.x, v.y, v.z]));
    geometry.setAttribute("normal", new THREE.BufferAttribute(normals, 3));
  }

  if (ir.uv.length > 0) {
    const uv = new Float32Array(ir.uv.flatMap((v) => [v.x, v.y]));
    geometry.setAttribute("uv", new THREE.BufferAttribute(uv, 2));
  }

  const material = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    side: THREE.DoubleSide,
  });

  if (ir.bones.length === 0) {
    let mesh = new THREE.Mesh(geometry, material);
    geometry.computeBoundingSphere();
    group.add(mesh);
    return group;
  }

  // #endregion

  // ! NOT OK

  const bones: THREE.Bone[] = [];
  const inverses: THREE.Matrix4[] = [];

  // const skinIndices: number[] = [];
  // const skinWeights: number[] = [];

  // geometry.setAttribute("skinIndex", new THREE.Uint16BufferAttribute(skinIndices, 4));
  // geometry.setAttribute("skinWeight", new THREE.Float32BufferAttribute(skinWeights, 4));

  let mesh = new THREE.Mesh(geometry, material);
  // const skeleton = new THREE.Skeleton(bones, inverses);
  // mesh.bind(skeleton, new THREE.Matrix4());
  geometry.computeBoundingSphere();
  group.add(mesh);

  return group;
}

export function disposeThreeObject(obj: THREE.Object3D) {
  obj.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (!mesh.isMesh) return;
    mesh.geometry?.dispose();
    const mats = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
    mats.forEach((m) => m?.dispose());
  });
}
