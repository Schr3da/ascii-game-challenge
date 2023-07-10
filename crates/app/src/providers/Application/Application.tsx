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

    const { columns, rows } = calculateGridSize();
    const platform = await ApiService.getPlatform();

    await ApiService.webviewDidMount();
    registerWindowResize();

    setPlatform(platform);
    setIsInitialised(true);

    await ApiService.sendEcsEvent({
      General: { OnApplicationWillInitialise: [columns, rows] },
    });
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
