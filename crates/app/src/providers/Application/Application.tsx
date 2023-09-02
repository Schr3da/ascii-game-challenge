import {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
  useState,
} from "react";

import { ApiService, Platforms } from "../../services";
import { useResize } from "../../hooks";
import { ApplicationContextValue } from "./Application.types";
import { calculateGridSize } from "../../utils";

export const ApplicationContext = createContext<ApplicationContextValue | null>(
  null
);

export const ApplicationProvider = ({ children }: PropsWithChildren) => {
  const [isInitialised, setIsInitialised] = useState(false);

  const [platform, setPlatform] = useState(Platforms.Web);

  const {
    windowWidth,
    windowHeight,
    registerWindowResize,
    unregisterWindowResize,
  } = useResize();

  const applicationDidMount = useCallback(async () => {
    if (isInitialised) {
      return;
    }

    const platform = await ApiService.getPlatform();

    const { columns, rows } = calculateGridSize();
    await ApiService.webviewDidMount(columns, rows);

    registerWindowResize();

    setPlatform(platform);
    setIsInitialised(true);
  }, []);

  const applicationWillUnmount = useCallback(() => {
    unregisterWindowResize();
  }, []);

  useEffect(() => {
    applicationDidMount();
    return applicationWillUnmount;
  }, []);

  return (
    <ApplicationContext.Provider
      value={{
        platform,
        windowWidth,
        windowHeight,
      }}
    >
      {isInitialised && children}
    </ApplicationContext.Provider>
  );
};
