import { createContext } from "react";

export const SettingsContext = createContext<SettingsContextValue>({
  theme: "light",
  toggleTheme: () => null,

  autoplayAudio: false,
  toggleAutoplayAudio: () => null,
});

type SettingsContextValue = {
  theme: "light" | "dark";
  toggleTheme(): void;
  autoplayAudio: boolean;
  toggleAutoplayAudio(): void;
};
