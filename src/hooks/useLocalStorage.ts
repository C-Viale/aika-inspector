import { useEffect, useState } from "react";

type Type = "JSON" | "number" | "string" | "boolean";

export function useLocalStorage<T>(key: string, defaultValue: T, type: Type = "string") {
  const [state, setState] = useState<T>(() => {
    try {
      const storageValue = get(key, type);
      return (storageValue ?? defaultValue) as T;
    } catch {
      save(key, defaultValue, type);
      return defaultValue;
    }
  });

  useEffect(() => {
    save(key, state, type);
  }, [key, state, type]);

  return [state, setState] as const;
}

function save(key: string, value: unknown, type: Type) {
  let val = "";

  if (type === "JSON") val = JSON.stringify(value);
  else val = String(value);

  localStorage.setItem(key, val);
}

function get(key: string, type: Type): unknown {
  const raw = localStorage.getItem(key);

  if (!raw) return;

  let val: unknown;

  if (type === "JSON") val = JSON.parse(raw);
  else if (type === "number") val = Number(raw);
  else if (type === "boolean") val = raw === "true";
  else val = raw;

  return val;
}
