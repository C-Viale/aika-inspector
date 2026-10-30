import { ChangeEvent, useContext, useMemo, useRef, useState } from "react";
import { BiChevronDown, BiChevronRight } from "react-icons/bi";
import { FcFile, FcFolder } from "react-icons/fc";
import { FolderData } from "../../actions/files";
import { AppContext } from "../../contexts/app-context";
import { Button } from "../button";
import "./styles.css";

interface SidebarProps {
  folder: FolderData | null;
  selectedFilePath: string;
  onPathSelected(path: string): void;
  onOpenFolderClick(): void;
  onExportClick(exporterId: string): Promise<void>;
}

export function Sidebar({
  onPathSelected,
  // onExportClick,
  onOpenFolderClick,
}: SidebarProps) {
  const { folder } = useContext(AppContext);
  const [filter, setFilter] = useState("");

  const filterTimeout = useRef(0);

  const onFilterChange = (event: ChangeEvent<HTMLInputElement>) => {
    if (filterTimeout.current) {
      clearTimeout(filterTimeout.current);
    }

    filterTimeout.current = setTimeout(() => {
      setFilter(event.target.value);
    }, 500);
  };

  // const [fileList, setFileList] = useState<FolderData[]>([]);
  // const [selectedExportType, setSelectedExportType] = useState("");

  // const filteredList = useMemo(() => {
  //   const filterKey = filter.toLowerCase().trim();
  //   return fileList.filter((i) => i.name.toLowerCase().includes(filterKey));
  // }, [fileList, filter]);

  return (
    <div className="sidebar">
      <header>
        {/* <Button
          onClick={() =>
            onPathSelected("/Users/caio/Downloads/files/Texture/Bow_01.jit")
          }
        >
          TEST
        </Button> */}
        <Button onClick={onOpenFolderClick}>Open folder</Button>
        <input
          autoComplete="off"
          autoCorrect="off"
          className="filter"
          placeholder="Search files..."
          // value={filter}
          onChange={onFilterChange}
        />
      </header>
      <ul className="filelist">
        {folder && (
          <FileListFolder
            data={folder}
            filter={filter}
            onPathSelected={onPathSelected}
            disableCollapse
          />
        )}
      </ul>
      {/* <div>
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
      </div> */}
    </div>
  );
}

interface FileListFolderProps {
  data: FolderData;
  filter: string;
  disableCollapse: boolean;
  onPathSelected(path: string): void;
}

function FileListFolder({
  data,
  filter,
  onPathSelected,
  disableCollapse = false,
}: FileListFolderProps) {
  const { selectedFilePath } = useContext(AppContext);
  const [visible, setVisible] = useState(disableCollapse);

  const toggleVisible = () => {
    if (disableCollapse) return;
    setVisible((prev) => !prev);
  };

  const filteredFiles = useMemo(() => {
    const filterKey = filter.toLowerCase().trim();
    return data.files.filter((filename) =>
      filename.toLowerCase().includes(filterKey),
    );
  }, [data.files, filter]);

  return (
    <li className="filelist-folder">
      <div className="filelist-folder-label" onClick={toggleVisible}>
        {!disableCollapse && (
          <>{visible ? <BiChevronDown /> : <BiChevronRight />}</>
        )}
        <FcFolder />
        {data.name}
      </div>
      {visible && (
        <ul className="filelist-folder-children">
          {data.subfolders.map((sub) => (
            <FileListFolder
              key={sub.name}
              data={sub}
              filter={filter}
              onPathSelected={onPathSelected}
              disableCollapse={false}
            />
          ))}
          {filteredFiles.map((file) => (
            <li
              key={file}
              className="filelist-file"
              onClick={() => onPathSelected(data.path + "/" + file)}
              data-selected={
                selectedFilePath === data.path + "/" + file || null
              }
            >
              <FcFile />
              {file}
            </li>
          ))}
        </ul>
      )}
    </li>
  );
}
