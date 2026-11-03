import { FileType } from "#/types";

type AllowedFiles = Record<string, FileType>;

export const ALLOWED_FILES: AllowedFiles = {
  jit: "texture",
  msh: "mesh",
  ms3: "mesh",
  MS3: "mesh",
  mp3: "audio",
  bin: "binary",
};
