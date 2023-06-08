import { useCallback, useRef, useState } from "react";

import { ApiService } from "../../services";
import { calculateGridSize } from "../../utils";

export const useResize = () => {
  const debounceTimer = useRef<NodeJS.Timeout>();

  const [windowWidth, setWindowWidth] = useState(window.innerWidth);

  const [windowHeight, setWindowHeight] = useState(window.innerHeight);

  const applicationDidResize = useCallback(() => {
    clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(() => {
      setWindowWidth(window.innerWidth);
      setWindowHeight(window.innerHeight);

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
    windowWidth,
    windowHeight,
    registerWindowResize,
    unregisterWindowResize,
  }
};