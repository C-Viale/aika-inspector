import { invoke } from "@tauri-apps/api/core";
import { Response } from "./types";
import { ModelData } from "../types";
import { sortFolder } from "../utils/functions";

export type FolderData = {
  name: string;
  path: string;
  subfolders: FolderData[];
  files: string[];
};

type ReadTextureResponse = {
  magic: string;
  compression_type: string;
  width: number;
  height: number;
  buffer: number[];
};

export type TextureData = {
  magic: string;
  compressionType: string;
  width: number;
  height: number;
  src: string;
};

export async function getFolderStructure(
  path: string,
): Promise<Response<FolderData>> {
  try {
    const data = await invoke<FolderData>("get_folder_structure", {
      path,
    });

    sortFolder(data, "asc");

    return { data };
  } catch (error) {
    return parseError(error);
  }
}

export async function readMesh(path: string): Promise<Response<ModelData>> {
  try {
    const data = await invoke<ModelData>("read_mesh", { path });
    return { data };
  } catch (error) {
    return parseError(error);
  }
}

export async function readTexture(
  path: string,
): Promise<Response<TextureData>> {
  try {
    const data = await invoke<ReadTextureResponse>("read_texture", { path });

    const blob = new Blob([new Uint8Array(data.buffer)]);
    const src = URL.createObjectURL(blob);

    return {
      data: {
        magic: data.magic,
        compressionType: data.compression_type,
        width: data.width,
        height: data.height,
        src,
      },
    };
  } catch (error) {
    return parseError(error);
  }
}

function parseError(err: unknown): { error: string } {
  if (!err) return { error: "undefined error" };
  if (typeof err !== "object") return { error: err.toString() };

  if ("error" in err) {
    return { error: err.error as string };
  }

  return { error: "unknown error" };
}

const sort = (a: string, b: string) => a.localeCompare(b);

function s(a: FolderData) {
  a.files.sort(sort);
  a.subfolders.forEach(s);
}
