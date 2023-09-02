import clsx from "clsx";
import { useCallback, useMemo } from "react";
import { ApiService } from "../../../services";
import { MacosHeaderProps } from "./MacosHeader.types";

export const MacosHeader = ({ className }: MacosHeaderProps) => {
  const buttonStyles = useMemo(
    () => ({
      width: 13,
      height: 13,
    }),
    []
  );

  const applyButtonClassName = useCallback((color: String) => {
    return clsx("border-1 border-white rounded-full", color);
  }, []);

  return (
    <div
      data-tauri-drag-region
      className={clsx(
        "flex flex-column space-x-2 items-center z-10",
        className
      )}
    >
      <div
        className={applyButtonClassName("bg-red-400")}
        style={buttonStyles}
        onClick={ApiService.closeApplication}
      />
      <div
        className={applyButtonClassName("bg-yellow-400")}
        style={buttonStyles}
        onClick={ApiService.minimiseApplication}
      />
      <div
        className={applyButtonClassName("bg-green-400")}
        style={buttonStyles}
        onClick={ApiService.maximiseApplication}
      />
    </div>
  );
};
