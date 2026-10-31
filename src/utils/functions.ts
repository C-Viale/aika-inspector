import { FolderData } from "../actions/files";

function sortFileAsc(a: string, b: string) {
  return a.localeCompare(b);
}

function sortFileDesc(a: string, b: string) {
  return b.localeCompare(a);
}

export function sortFolder(data: FolderData, direction: "asc" | "desc") {
  if (direction === "asc") data.files.sort(sortFileAsc);
  else data.files.sort(sortFileDesc);

  data.subfolders.forEach((f) => sortFolder(f, direction));
}

export function clamp(value: number, min: number, max: number) {
  if (value <= min) return min;
  if (value >= max) return max;
  return value;
}
