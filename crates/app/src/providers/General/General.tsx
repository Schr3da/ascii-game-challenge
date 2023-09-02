import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { GeneralContextValue } from "./General.types";
import { GeneralSubscription } from "../../shared.d";

export const GeneralContext = createContext<GeneralContextValue | null>(null);

export const GeneralProvider = ({ children }: PropsWithChildren) => {
  const [previousGeneralEvent, setPreviousGeneralEvent] =
    useState<GeneralSubscription>("OnApplicationDidStart");

  const [nextGeneralEvent, setNextGeneralEvent] = useState<GeneralSubscription>(
    "OnApplicationDidStart"
  );

  const isGeneralSubscription = useCallback(
    (event: any): event is GeneralSubscription =>
      event.OnApplicationDidStart ||
      event.OnApplicationDidInitialise ||
      event.OnApplicationDidLoadAssets ||
      event.OnApplicationDidClose,
    []
  );

  const processEvent = useCallback(
    (event: unknown) => {
      if (!isGeneralSubscription(event)) {
        return;
      }

      setPreviousGeneralEvent(nextGeneralEvent);
      setNextGeneralEvent(event);
    },
    [nextGeneralEvent]
  );

  useSubscribe("GeneralSubscription", processEvent);

  return (
    <GeneralContext.Provider
      value={{
        previousGeneralEvent,
        nextGeneralEvent,
      }}
    >
      {children}
    </GeneralContext.Provider>
  );
};
