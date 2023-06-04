import { useCallback, useRef } from "react";

import { ApiService } from "../../services";
import { calculateGridSize } from "../../utils";

export const useResize = () => {

  const debounceTimer = useRef<any>(null);

  const applicationDidResize = useCallback(() => {
    clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(() => {
      const { columns, rows } = calculateGridSize();
      ApiService.sendEcsEvent({
        General: { OnApplicationResize: [columns, rows] },
      });
    }, 30);

  }, []);

  const registerWindowResize = useCallback(() => {
    window.addEventListener("resize", applicationDidResize);
  }, []);

  const unregisterWindowResize = useCallback(() => {
    clearTimeout(debounceTimer.current);
    window.removeEventListener("resize", applicationDidResize);
  }, []);

  return {
    registerWindowResize,
    unregisterWindowResize,
  }
};