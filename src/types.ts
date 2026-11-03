export type Vector2 = { x: number; y: number };
export type Vector3 = { x: number; y: number; z: number };

export type FileType = "mesh" | "texture" | "audio" | "binary";

// #region responses
export type ReadFileResponse =
  | {
      type: "mesh";
      data: MeshData;
    }
  | {
      type: "texture";
      data: ReadTextureData;
    }
  | {
      type: "audio";
      data: ReadAudioData;
    }
  | {
      type: "binary";
      data: any;
    };

type ReadTextureData = {
  magic: string;
  compression_type: string;
  width: number;
  height: number;
  buffer: number[];
};

type ReadAudioData = {
  compression: "mp3";
  buffer: number[];
};
// #endregion

// #region loaded

export type MeshData = {
  vertices: Vector3[];
  triangles: Vector3[];
  normals: Vector3[];
  uv: Vector2[];
  bones: Bone[];
  bone_weights: BoneWeight[];
};

export type TextureData = {
  magic: string;
  compressionType: string;
  width: number;
  height: number;
  src: string;
};

export type AudioData = {
  src: string;
};

export type Bone = {
  id: string;
  transform: readonly [
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
  ];
};

export type BoneWeight = {
  bones_ids: readonly [number, number, number, number];
  weights: readonly [number, number, number, number];
};

export type FileListItem = {
  name: string;
  path: string;
};

export type LoadedFile =
  | {
      type: "mesh";
      data: MeshData;
    }
  | {
      type: "texture";
      data: TextureData;
    }
  | {
      type: "audio";
      data: AudioData;
    }
  | {
      type: "binary";
      data: any;
    };

export type FolderData = {
  name: string;
  path: string;
  subfolders: FolderData[];
  files: string[];
};
// #endregion
