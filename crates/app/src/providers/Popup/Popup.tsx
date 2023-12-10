import {
  createContext,
  PropsWithChildren,
  useCallback,
  useMemo,
  useState,
} from "react";
import { useSubscribe } from "../../hooks";
import { UiView, ViewComponentIds } from "../../shared.d";
import { PopupContextValues } from "./Popup.types";

export const PopupContext = createContext<PopupContextValues | null>(null);

export const PopupProvider = ({ children }: PropsWithChildren) => {
  const [nextPopup, setNextPopupView] = useState<UiView | null>(null);

  const [previousPopup, setPreviousPopupView] = useState<UiView | null>(null);

  const [selectedPopupId, setSelectedPopupId] = useState("");

  const isPopupVisible = useMemo(() => nextPopup !== null, [nextPopup]);

  const isPopupSelected = useCallback(
    (next: ViewComponentIds) => {
      return JSON.stringify(next) === selectedPopupId;
    },
    [selectedPopupId]
  );

  const processEvent = useCallback(
    (view: UiView | null) => {
      setPreviousPopupView(nextPopup);
      setNextPopupView(view);

      const stringifiedPopupId = JSON.stringify(view?.state.selected_id);
      setSelectedPopupId(stringifiedPopupId);
    },
    [nextPopup]
  );

  useSubscribe("PopupSubscription", processEvent);

  return (
    <PopupContext.Provider
      value={{
        nextPopup,
        previousPopup,
        selectedPopupId,
        isPopupSelected,
        isPopupVisible,
      }}
    >
      {children}
    </PopupContext.Provider>
  );
};
