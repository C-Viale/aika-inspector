import { createContext } from "react";

export const SettingsContext = createContext<SettingsContextValue>({
  theme: "light",
  toggleTheme: () => null,

  autoplayAudio: false,
  toggleAutoplayAudio: () => null,

  volume: 0.5,
  setVolume: () => null,
});

type SettingsContextValue = {
  theme: "light" | "dark";
  toggleTheme(): void;
  autoplayAudio: boolean;
  toggleAutoplayAudio(): void;
  volume: number;
  setVolume(value: number): void;
};
