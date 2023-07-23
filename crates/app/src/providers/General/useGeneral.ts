import { useContext } from "react";
import { GeneralContext } from "./General";
import { GeneralContextValue } from "./General.types";

export const useGeneralContext = (): GeneralContextValue => {
  const ctx = useContext(GeneralContext);

  if (ctx == null) {
    throw new Error("Unable to find general context in hierachy");
  }

  return ctx;
};
