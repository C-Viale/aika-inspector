export function getFileNameFromPath(pathStr: string, extension = true) {
  const filename = pathStr.replace(/^.*[\\\/]/, "");
  if (extension) return filename;
  return filename.replace(/\.\w+$/, "");
}
