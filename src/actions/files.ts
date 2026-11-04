import { FolderData, LoadedFile, ReadFileResponse } from "#/types";
import { sortFolder } from "#utils/functions/array";
import { invoke } from "@tauri-apps/api/core";
import { ActionResult } from "./types";

export async function getFolderStructure(path: string): Promise<ActionResult<FolderData>> {
  try {
    const data = await invoke<FolderData>("get_folder_structure", {
      path,
    });

    sortFolder(data, "asc");

    return ActionResult.ok(data);
  } catch (error) {
    return ActionResult.error<FolderData>(parseError(error));
  }
}

export async function readFile(path: string): Promise<ActionResult<LoadedFile>> {
  try {
    const { type, data } = await invoke<ReadFileResponse>("read_file", { path });

    if (type === "audio") {
      const blob = new Blob([new Uint8Array(data.buffer)], { type: "audio/mpeg" });
      const src = URL.createObjectURL(blob);
      return ActionResult.ok({ type: "audio", data: { src } });
    }

    if (type === "texture") {
      const blob = new Blob([new Uint8Array(data.buffer)]);
      const src = URL.createObjectURL(blob);
      return ActionResult.ok({
        type: "texture",
        data: {
          magic: data.magic,
          compressionType: data.compression_type,
          width: data.width,
          height: data.height,
          src,
        },
      });
    }

    return ActionResult.ok({ type, data });
  } catch (error) {
    return ActionResult.error<LoadedFile>(parseError(error));
  }
}

export async function writeFile(path: string, body: Uint8Array) {
  try {
    await invoke<ReadFileResponse>("write_file", { path, body });
    return ActionResult.ok(true);
  } catch (error) {
    return ActionResult.error<LoadedFile>(parseError(error));
  }
}

function parseError(err: unknown): string {
  if (!err) return "Undefined error";
  if (typeof err !== "object") return err.toString();
  if ("error" in err) return err.error as string;
  return "Unknown error";
}
