import type { ComponentPropsWithRef } from "react";
import "./styles.css";

interface ButtonProps extends ComponentPropsWithRef<"button"> {}

export function Button({ type = "button", ...props }: ButtonProps) {
  return <button {...props} type={type} className="button" />;
}
