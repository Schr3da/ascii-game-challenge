import {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
  useState,
} from "react";

import { ApiService } from "../../services";
import { useResize } from "../../hooks";

export const ApplicationContext = createContext(null);

export const ApplicationProvider = ({ children }: PropsWithChildren) => {
  const [isInitialised, setIsInitialised] = useState(false);

  const { registerWindowResize, unregisterWindowResize } = useResize();

  const applicationDidMount = useCallback(async () => {
    if (isInitialised) {
      return;
    }

    await ApiService.webviewDidMount();
    registerWindowResize();

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
    <ApplicationContext.Provider value={null}>
      {isInitialised && children}
    </ApplicationContext.Provider>
  );
};
