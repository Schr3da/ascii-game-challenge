import { useContext } from "react";
import { PopupContext} from "./Popup";
import { PopupContextValues as PopupContextValue } from "./Popup.types";

export const usePopupContext = (): PopupContextValue => {
  const ctx = useContext(PopupContext);

  if (ctx == null) {
    throw new Error("Unable to find popup context in hierachy");
  }

  return ctx;
};
