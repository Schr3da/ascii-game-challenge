import { PropsWithChildren, createContext, useCallback, useState } from "react";

import { useSubscribe } from "../../hooks";
import { EcsContextValue } from "./Ecs.types";
import { SubscriptionEventTypes } from "../../services";
import {
  GeneralSubscription,
  RenderSubscription,
  UiSubscription,
  UiView,
} from "../../shared";

export const EcsContext = createContext<EcsContextValue | null>(null);

const isGeneralSubscription = (event: any): event is GeneralSubscription =>
  event.OnApplicationDidStart ||
  event.OnApplicationDidInitialise ||
  event.OnApplicationDidClose;

const isUiSubscription = (event: any): event is UiSubscription =>
  event.UnknownUiSubscription;

const isRendererSubscription = (event: any): event is RenderSubscription =>
  event.OnWorldDidUpdate;

export const EcsProvider = ({ children }: PropsWithChildren) => {
  const [nextView, setNextView] = useState<UiView | null>(null);
  const [previousView, setPreviousView] = useState<UiView | null>(null);

  const [previousGeneralEvent, setPreviousGeneralEvent] =
    useState<GeneralSubscription>("OnApplicationDidStart");

  const [nextGeneralEvent, setNextGeneralEvent] = useState<GeneralSubscription>(
    "OnApplicationDidStart"
  );

  const [previousUiEvent, setPreviousUiEvent] = useState<UiSubscription>(
    "UnknownUiSubscription"
  );

  const [nextUiEvent, setUiEvent] = useState<UiSubscription>(
    "UnknownUiSubscription"
  );

  const [previousRendererEvent, setPreviousRendererEvent] =
    useState<RenderSubscription>({ OnWorldDidUpdate: null });

  const [nextRendererEvent, setRendererEvent] = useState<RenderSubscription>({
    OnWorldDidUpdate: null,
  });

  const handleGeneralSubscription = useCallback(
    (event: GeneralSubscription) => {
      setPreviousGeneralEvent(nextGeneralEvent);
      return setNextGeneralEvent(event);
    },
    [nextGeneralEvent]
  );

  const handleUiSubscription = useCallback(
    (event: UiSubscription) => {
      setPreviousUiEvent(nextUiEvent);
      setUiEvent(event);
    },
    [nextUiEvent]
  );

  const handleRendererSubscription = useCallback(
    (event: RenderSubscription) => {
      setPreviousView(nextView);
      setNextView(event.OnWorldDidUpdate);
      setPreviousRendererEvent(nextRendererEvent);
      setRendererEvent(event);
    },
    [nextRendererEvent, nextView]
  );

  const processEvent = useCallback(
    (event: SubscriptionEventTypes) => {
      if (isGeneralSubscription(event)) {
        return handleGeneralSubscription(event);
      }

      if (isUiSubscription(event)) {
        return handleUiSubscription(event);
      }

      if (isRendererSubscription(event)) {
        return handleRendererSubscription(event);
      }
    },
    [
      handleGeneralSubscription,
      handleUiSubscription,
      handleRendererSubscription,
    ]
  );

  useSubscribe(processEvent);

  return (
    <EcsContext.Provider
      value={{
        previousView,
        nextView,
        previousGeneralEvent,
        nextGeneralEvent,
        previousUiEvent,
        nextUiEvent,
        previousRendererEvent,
        nextRendererEvent,
      }}
    >
      {children}
    </EcsContext.Provider>
  );
};
