import { FolderData } from "#/types";
import { createContext } from "react";

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
