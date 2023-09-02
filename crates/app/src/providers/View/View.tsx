import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { UiView, ViewComponentIds } from "../../shared.d";
import { ViewContextValue } from "./View.types";

export const ViewContext = createContext<ViewContextValue | null>(null);

export const ViewProvider = ({ children }: PropsWithChildren) => {
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
      const stringifiedViewId = JSON.stringify(view?.state.selected_id);

      setSelectedViewId(stringifiedViewId);
      setPreviousView(nextView);
      setNextView(view);
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
