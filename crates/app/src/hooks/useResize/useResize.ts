import { useCallback, useState } from "react";

import { ApiService } from "../../services";
import { calculateGridSize } from "../../utils";
import {useDebounce} from "../useDebounce";

export const useResize = () => {
  const {debounce, clearDebounce} = useDebounce();

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
    debounce(handleResize, 16);
  }, [handleResize, debounce]);

  const registerWindowResize = useCallback(() => {
    window.addEventListener("resize", applicationWillResize);
  }, []);

  const unregisterWindowResize = useCallback(() => {
    clearDebounce();
    window.removeEventListener("resize", applicationWillResize);
  }, [clearDebounce]);

  return {
    windowWidth,
    windowHeight,
    registerWindowResize,
    unregisterWindowResize,
  };
};
