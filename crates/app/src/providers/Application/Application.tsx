import {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
  useState,
} from "react";

import { TauriService } from "../../services";

export const ApplicationContext = createContext(null);

export const ApplicationProvider = ({ children }: PropsWithChildren) => {
  const [isInitialised, setIsInitialised] = useState(false);

  const applicationDidMount = useCallback(async () => {
    if (isInitialised) {
      return;
    }

    await TauriService.webviewDidMount();
    setIsInitialised(true);
  }, []);

  useEffect(() => {
    applicationDidMount();
  }, []);

  return (
    <ApplicationContext.Provider value={null}>
      {isInitialised && children}
    </ApplicationContext.Provider>
  );
};
