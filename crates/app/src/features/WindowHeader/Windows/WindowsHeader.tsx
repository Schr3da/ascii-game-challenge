import clsx from "clsx";

import { useCallback, useMemo } from "react";
import { ApiService } from "../../../services";

export const WindowsHeader = () => {
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
      className="flex flex-column h-7 space-x-2 justify-end items-center p-2 z-10"
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
