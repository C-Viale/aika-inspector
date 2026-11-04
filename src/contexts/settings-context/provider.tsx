import { useLocalStorage } from "#/hooks/useLocalStorage";
import { ReactNode, useEffect } from "react";
import { SettingsContext } from "./context";

interface SettingsContextProviderProps {
  children: ReactNode;
}

export function SettingsContextProvider({ children }: SettingsContextProviderProps) {
  const [theme, setTheme] = useLocalStorage<"light" | "dark">("app-theme", "light");
  const [autoplayAudio, setAutoplayAudio] = useLocalStorage("app-autoplayaudio", false, "boolean");

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
    <SettingsContext value={{ theme, toggleTheme, autoplayAudio, toggleAutoplayAudio }}>
      {children}
    </SettingsContext>
  );
}
