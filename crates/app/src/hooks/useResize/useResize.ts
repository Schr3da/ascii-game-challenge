import { useCallback, useRef } from "react";
import { ApiService } from "../../services";
import { config } from "../../config";

export const useResize = () => {

  const debounceTimer = useRef<any>(null);

  const applicationDidResize = useCallback(() => {
    clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(() => {
      const size = config.tileSize;
      const columns = Math.floor(window.innerWidth / size);
      const rows = Math.floor(window.innerHeight / size);

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