import { FolderData } from "#actions/files";

function sortStrAsc(a: string, b: string) {
  return a.localeCompare(b);
}

function sortStrDesc(a: string, b: string) {
  return b.localeCompare(a);
}

export function sortFolder(data: FolderData, direction: "asc" | "desc") {
  if (direction === "asc") {
    data.files.sort(sortStrAsc);
    data.subfolders.sort((a, b) => sortStrAsc(a.name, b.name));
  } else {
    data.files.sort(sortStrDesc);
    data.subfolders.sort((a, b) => sortStrDesc(a.name, b.name));
  }

  data.subfolders.forEach((f) => sortFolder(f, direction));
}
