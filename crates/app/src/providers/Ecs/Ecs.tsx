import { PropsWithChildren, createContext, useCallback, useState } from "react";

import { useSubscribe } from "../../hooks";
import { EcsContextValue } from "./Ecs.types";
import { SubscriptionEventTypes } from "../../services";
import {
  GameStatus,
  GeneralSubscription,
  RenderSubscription,
  UiSubscription,
  UiView,
  ViewComponentIds,
} from "../../shared";

export const EcsContext = createContext<EcsContextValue | null>(null);

const isGeneralSubscription = (event: any): event is GeneralSubscription =>
  event.OnApplicationDidStart ||
  event.OnApplicationDidInitialise ||
  event.OnApplicationDidLoadAssets || 
  event.OnApplicationDidClose;

const isUiSubscription = (event: any): event is UiSubscription =>
  event.UnknownUiSubscription;

const isRendererSubscription = (event: any): event is RenderSubscription =>
  event.OnWorldDidUpdate;

export const EcsProvider = ({ children }: PropsWithChildren) => {
  const [gameStatus, setGameStatus] = useState<GameStatus>("GameDidNotStart");

  const [nextView, setNextView] = useState<UiView | null>(null);
  const [previousView, setPreviousView] = useState<UiView | null>(null);
  const [selectedViewComponentId, setSelectedViewComponentId] = useState("");

  const [nextPopupView, setNextPopupView] = useState<UiView | null>(null);
  const [previousPopupView, setPreviousPopupView] = useState<UiView | null>(null);
  const [selectedPopupViewComponentId, setSelectedPopupViewComponentId] = useState("");

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
    useState<RenderSubscription>({ OnWorldDidUpdate: [null, null, gameStatus] });

  const [nextRendererEvent, setRendererEvent] = useState<RenderSubscription>({
    OnWorldDidUpdate: [null, null, gameStatus],
  });

  const isViewComponentSelected = useCallback(
    (next: ViewComponentIds) => {
      return JSON.stringify(next) === selectedViewComponentId;
    },
    [selectedViewComponentId]
  );

  const isPopupViewComponentSelected = useCallback(
    (next: ViewComponentIds) => {
      return JSON.stringify(next) === selectedPopupViewComponentId;
    },
    [selectedPopupViewComponentId]
  );

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
      const [view, popup, status] = event.OnWorldDidUpdate;

      const stringifiedViewId = JSON.stringify(view?.state.selected_id);
      setSelectedViewComponentId(stringifiedViewId);
      setPreviousView(nextView);
      setNextView(view);

      const stringifiedPopupId = JSON.stringify(popup?.state.selected_id);
      setSelectedPopupViewComponentId(stringifiedPopupId);
      setPreviousPopupView(nextPopupView);
      setNextPopupView(popup);

      setGameStatus(status);
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
        previousPopupView,
        nextPopupView,
        previousGeneralEvent,
        nextGeneralEvent,
        previousUiEvent,
        nextUiEvent,
        previousRendererEvent,
        nextRendererEvent,
        isViewComponentSelected,
        isPopupViewComponentSelected
      }}
    >
      {children}
    </EcsContext.Provider>
  );
};
