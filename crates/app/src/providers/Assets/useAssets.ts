import { useContext } from "react";
import { AssetsContext } from "./Assets";
import { AssetsContextValue} from "./Assets.types";

export const useAssetsContext = (): AssetsContextValue=> {
  const ctx = useContext(AssetsContext);

  if (ctx == null) {
    throw new Error("Unable to find assets context in hierachy");
  }

  return ctx;
};
