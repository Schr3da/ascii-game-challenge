import clsx from "clsx";

import { useCallback, useMemo } from "react";
import { ApiService } from "../../../services";
import { WindowsHeaderProps } from "./WindowsHeader.types";

export const WindowsHeader = ({ className }: WindowsHeaderProps) => {
  const buttonStyles = useMemo(
    () => ({
      width: 13,
      height: 13,
    }),
    []
  );

  const applyButtonClassName = useCallback((color: String) => {
    return clsx("rounded-sm", color);
  }, []);

  return (
    <div
      data-tauri-drag-region
      className={clsx(
        "flex flex-column space-x-2 items-center justify-end z-10",
        className
      )}
    >
      <div
        className={applyButtonClassName("bg-gray-500")}
        style={buttonStyles}
        onClick={ApiService.minimiseApplication}
      />
      <div
        className={applyButtonClassName("bg-gray-500")}
        style={buttonStyles}
        onClick={ApiService.maximiseApplication}
      />
      <div
        className={applyButtonClassName("bg-red-400")}
        style={buttonStyles}
        onClick={ApiService.closeApplication}
      />
    </div>
  );
};
