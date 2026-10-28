import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useMemo, useState } from "react";
import { FileListItem } from "../../types";
import { Button } from "../button";
import { Select } from "../select";
import "./styles.css";

interface SidebarProps {
  selectedPath: string;
  onPathSelected(path: string): void;
  onExportClick(exporterId: string): Promise<void>;
}

export function Sidebar({
  selectedPath,
  onPathSelected,
  onExportClick,
}: SidebarProps) {
  const [folder, setFolder] = useState("");
  const [filter, setFilter] = useState("");
  const [fileList, setFileList] = useState<FileListItem[]>([]);
  const [selectedExportType, setSelectedExportType] = useState("");

  const openFolderDialog = async () => {
    const folderPath = await open({
      directory: true,
      multiple: false,
    });

    if (!folderPath) return;

    const list = await invoke<FileListItem[]>("get_filelist_from_folder", {
      folderPath,
    });
    setFileList(list.sort((a, b) => a.name.localeCompare(b.name)));
    setFolder(folderPath);
  };

  const filteredList = useMemo(() => {
    const filterKey = filter.toLowerCase().trim();
    return fileList.filter((i) => i.name.toLowerCase().includes(filterKey));
  }, [fileList, filter]);

  return (
    <div className="sidebar">
      <header>
        <Button onClick={openFolderDialog}>Open folder</Button>
        {folder || "No selected folder."}
        <input
          autoComplete="off"
          autoCorrect="off"
          className="filter"
          placeholder="filter"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
      </header>
      <ul className="filelist">
        {filteredList.map((i) => (
          <li
            key={i.name}
            onClick={() => onPathSelected(i.path)}
            data-selected={selectedPath === i.path || null}
          >
            {i.name}
          </li>
        ))}
      </ul>
      <div>
        <h3>Export options</h3>

        <Select
          value={selectedExportType}
          onChange={(e) => setSelectedExportType(e.currentTarget.value)}
          options={[
            { label: "Wavefront Obj", value: "obj" },
            { label: "GLB (binary glTF)", value: "glb" },
          ]}
        />

        <Button onClick={() => onExportClick(selectedExportType)}>
          Export
        </Button>
      </div>
    </div>
  );
}
