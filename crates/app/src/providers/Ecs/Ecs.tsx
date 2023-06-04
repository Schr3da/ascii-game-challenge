import { PropsWithChildren, createContext, useCallback, useState } from "react";
import { SubscriptionEvents } from "../../shared";
import { useSubscribe } from "../../hooks";
import { EcsContextValue } from "./Ecs.types";

const defaultEvent = (): SubscriptionEvents => ({
  General: "OnApplicationDidStart",
});

export const EcsContext = createContext<EcsContextValue | null>(null);

export const EcsProvider = ({ children }: PropsWithChildren) => {
  const [nextEvent, setNextEvent] = useState<SubscriptionEvents>(
    defaultEvent()
  );

  const [previousEvent, setPreviousEvent] = useState<SubscriptionEvents>(
    defaultEvent()
  );

  const processEvent = useCallback(
    (event: SubscriptionEvents) => {
      setPreviousEvent(nextEvent);
      setNextEvent(event);
    },
    [nextEvent]
  );

  useSubscribe(processEvent);

  return (
    <EcsContext.Provider
      value={{
        previousEvent,
        nextEvent,
      }}
    >
      {children}
    </EcsContext.Provider>
  );
};
