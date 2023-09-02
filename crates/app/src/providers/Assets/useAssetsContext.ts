import { useContext } from "react";
import { AssetContext } from "./Assets";
import { AssetProviderContextValue } from "./Assets.types";

export const useAssetsContext = (): AssetProviderContextValue => {
  const ctx = useContext(AssetContext);

  if (ctx == null) {
    throw new Error("Unable to find assets context in hierachy");
  }

  return ctx;
};
