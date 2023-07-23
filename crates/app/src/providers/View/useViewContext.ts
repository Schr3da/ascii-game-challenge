import { useContext } from "react";
import { ViewContext } from "./View";
import { ViewContextValue } from "./View.types";

export const useViewContext = (): ViewContextValue => {
  const ctx = useContext(ViewContext);

  if (ctx == null) {
    throw new Error("Unable to find view context in hierachy");
  }

  return ctx;
};
