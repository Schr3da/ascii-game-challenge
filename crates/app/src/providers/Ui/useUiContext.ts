import { useContext } from "react";
import { UiContext } from "./Ui";
import { UiContextValue } from "./Ui.types";

export const useUiContext = (): UiContextValue => {
  const ctx = useContext(UiContext);

  if (ctx == null) {
    throw new Error("Unable to find ui context in hierachy");
  }

  return ctx;
};
