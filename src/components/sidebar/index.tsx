import { Button } from "#components/button";
import { AppContext } from "#contexts/app-context";
import { ChangeEvent, useContext, useMemo, useRef, useState } from "react";
import { BiChevronDown, BiChevronRight } from "react-icons/bi";
import { FcFile, FcFolder } from "react-icons/fc";
import "./styles.css";
import { FolderData } from "#/types";
import { SettingsContext } from "#contexts/settings-context/context";
import { PiMoon, PiSun } from "react-icons/pi";
import { MdPlayArrow, MdPlayDisabled } from "react-icons/md";

interface SidebarProps {
  folder: FolderData | null;
  selectedFilePath: string;
  onPathSelected(path: string): void;
  onOpenFolderClick(): void;
}

export function Sidebar({
  onPathSelected,
  // onExportClick,
  onOpenFolderClick,
}: SidebarProps) {
  const { folder } = useContext(AppContext);
  const { theme, toggleTheme, autoplayAudio, toggleAutoplayAudio } = useContext(SettingsContext);

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

      <footer>
        <div>
          <Button onClick={toggleTheme} title="Theme">
            {theme === "light" ? <PiSun size={16} /> : <PiMoon size={16} />}
          </Button>
          <Button onClick={toggleAutoplayAudio} title="Autoplay">
            {autoplayAudio ? <MdPlayArrow size={16} /> : <MdPlayDisabled size={16} />}
          </Button>
        </div>
        <span>0.1.0</span>
      </footer>
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
    return data.files.filter((filename) => filename.toLowerCase().includes(filterKey));
  }, [data.files, filter]);

  return (
    <li className="filelist-folder">
      <div className="filelist-folder-label" onClick={toggleVisible}>
        {!disableCollapse && <>{visible ? <BiChevronDown /> : <BiChevronRight />}</>}
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
              data-selected={selectedFilePath === data.path + "/" + file || null}
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
