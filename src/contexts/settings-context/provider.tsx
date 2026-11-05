import { useLocalStorage } from "#/hooks/useLocalStorage";
import { ReactNode, useEffect } from "react";
import { SettingsContext } from "./context";
import { STORAGE_KEYS } from "#utils/constants";

interface SettingsContextProviderProps {
  children: ReactNode;
}

export function SettingsContextProvider({ children }: SettingsContextProviderProps) {
  const [theme, setTheme] = useLocalStorage<"light" | "dark">(STORAGE_KEYS.theme, "light");
  const [autoplayAudio, setAutoplayAudio] = useLocalStorage(
    STORAGE_KEYS.autoplayAudio,
    false,
    "boolean",
  );
  const [volume, setVolume] = useLocalStorage(STORAGE_KEYS.volume, 0.5, "number");

  const toggleTheme = () => {
    setTheme((prev) => (prev === "light" ? "dark" : "light"));
  };

  const toggleAutoplayAudio = () => {
    setAutoplayAudio((prev) => !prev);
  };

  useEffect(() => {
    document.body.classList.remove(theme === "light" ? "dark" : "light");
    document.body.classList.add(theme);
  }, [theme]);

  return (
    <SettingsContext
      value={{ theme, toggleTheme, autoplayAudio, toggleAutoplayAudio, volume, setVolume }}
    >
      {children}
    </SettingsContext>
  );
}
