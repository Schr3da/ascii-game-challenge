import { useCallback, useRef, useState } from "react";

import { ApiService } from "../../services";
import { calculateGridSize } from "../../utils";

export const useResize = () => {
  const debounceTimer = useRef<NodeJS.Timeout>();

  const [windowWidth, setWindowWidth] = useState(window.innerWidth);

  const [windowHeight, setWindowHeight] = useState(window.innerHeight);

  const handleResize = useCallback(async () => {
    setWindowWidth(window.innerWidth);
    setWindowHeight(window.innerHeight);

    const { columns, rows } = calculateGridSize();

    await ApiService.sendEcsEvent({
      General: { OnApplicationResize: [columns, rows] },
    });
  }, []);

  const applicationWillResize = useCallback(() => {
    clearTimeout(debounceTimer.current);
    debounceTimer.current = setTimeout(handleResize, 16);
  }, [handleResize]);

  const registerWindowResize = useCallback(() => {
    window.addEventListener("resize", applicationWillResize);
  }, []);

  const unregisterWindowResize = useCallback(() => {
    clearTimeout(debounceTimer.current);
    window.removeEventListener("resize", applicationWillResize);
  }, []);

  return {
    windowWidth,
    windowHeight,
    registerWindowResize,
    unregisterWindowResize,
  };
};
