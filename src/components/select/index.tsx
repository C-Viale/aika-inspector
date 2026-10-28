import type { ComponentPropsWithRef } from "react";
import "./styles.css";

interface SelectProps extends ComponentPropsWithRef<"select"> {
  options: { label: string; value: string }[];
}

export function Select({ options, ...props }: SelectProps) {
  return (
    <select {...props} className="select">
      {options.map((option) => (
        <option key={option.value} value={option.value}>
          {option.label}
        </option>
      ))}
    </select>
  );
}
