import clsx from "clsx";
import { CommonHeaderProps } from "./CommonHeader.types";

export const CommonHeader = ({ className }: CommonHeaderProps) => {
  return <div data-tauri-drag-region className={clsx("", className)}></div>;
};
