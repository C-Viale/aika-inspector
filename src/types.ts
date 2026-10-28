export type Vector2 = { x: number; y: number };
export type Vector3 = { x: number; y: number; z: number };

export type ModelData = {
  vertices: Vector3[];
  triangles: Vector3[];
  normals: Vector3[];
  uv: Vector2[];
};

export type MshData = {
  magic: number;
  unused_0: number;
  chunk_key: number;
  unknow_0: number;
  chunk_size: number;
  unknow_1: number;
  bone_count: number;
  vertices_count: number;
  triangles_position_count: number;
  vertices: Vector3[];
  triangles: Vector3[];
  normals: Vector3[];
  uv: Vector2[];
};

export type FileListItem = {
  name: string;
  path: string;
};
