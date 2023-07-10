import { PropsWithChildren, createContext, useEffect } from "react";
import { TextureService } from "../../services";
import { useEcsContext } from "../Ecs";
import { isApplicationDidLoadAssetsEvent } from "../../utils";

export const AssetContext = createContext(null);

export const AssetProvider = ({ children }: PropsWithChildren) => {
  const { nextGeneralEvent } = useEcsContext();

  useEffect(() => {
    if (nextGeneralEvent == null) {
      return;
    }

    if (!isApplicationDidLoadAssetsEvent(nextGeneralEvent)) {
      return;
    }

    TextureService.rawAssets = nextGeneralEvent.OnApplicationDidLoadAssets;
  }, [nextGeneralEvent]);

  return <AssetContext.Provider value={null}>{children}</AssetContext.Provider>;
};
