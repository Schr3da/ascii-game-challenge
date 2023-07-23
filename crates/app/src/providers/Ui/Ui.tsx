import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { UiSubscription } from "../../shared.d";
import { UiContextValue } from "./Ui.types";

const isUiSubscription = (event: any): event is UiSubscription =>
  event.UnknownUiSubscription;

export const UiContext = createContext<UiContextValue | null>(null);

export const UiProvider = ({ children }: PropsWithChildren) => {

  const [previousUiEvent, _setPreviousUiEvent] = useState<UiSubscription>(
    "UnknownUiSubscription"
  );

  const [nextUiEvent, _setUiEvent] = useState<UiSubscription>(
    "UnknownUiSubscription"
  );

  const processEvent = useCallback((event: unknown) => {
    if (!isUiSubscription(event)) {
      return;
    }
  }, []);

  useSubscribe("GeneralSubscription", processEvent);

  return <UiContext.Provider value={{
    previousUiEvent,
    nextUiEvent, 
  }}>{children}</UiContext.Provider>;
};
