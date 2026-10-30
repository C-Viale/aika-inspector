import { createContext } from "react";
import { FolderData } from "../actions/files";

export const AppContext = createContext<Context>({
  backgroundColor: "",
  setBackgroundColor: () => null,
  folder: null,
  selectedFilePath: "",
});

type Context = {
  backgroundColor: string;
  setBackgroundColor(value: string): void;
  folder: FolderData | null;
  selectedFilePath: string;
};
