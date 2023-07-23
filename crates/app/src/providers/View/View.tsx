import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { GameStatus, UiView, ViewComponentIds } from "../../shared.d";
import { ViewContextValue } from "./View.types";

export const ViewContext = createContext<ViewContextValue | null>(null);

export const ViewProvider = ({ children }: PropsWithChildren) => {
  const [gameStatus, setGameStatus] = useState<GameStatus>("GameDidNotStart");

  const [nextView, setNextView] = useState<UiView | null>(null);

  const [previousView, setPreviousView] = useState<UiView | null>(null);

  const [selectedViewId, setSelectedViewId] = useState("");

  const isViewSelected = useCallback(
    (next: ViewComponentIds) => {
      return JSON.stringify(next) === selectedViewId;
    },
    [selectedViewId]
  );

  const processEvent = useCallback(
    (view: UiView | null) => {
      setPreviousView(nextView);
      setNextView(view);

      const stringifiedViewId = JSON.stringify(view?.state.selected_id);
      setSelectedViewId(stringifiedViewId);
    },
    [nextView]
  );

  useSubscribe("ViewSubscription", processEvent);

  return (
    <ViewContext.Provider
      value={{
        nextView,
        previousView,
        selectedViewId,
        isViewSelected,
      }}
    >
      {children}
    </ViewContext.Provider>
  );
};
