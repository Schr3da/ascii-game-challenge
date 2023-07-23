import { useContext } from "react";
import { GameMetaContextValue} from "./GameMeta.types";
import { GameMetaContext} from "./GameMeta";

export const useGameMetaContext = (): GameMetaContextValue=> {
  const ctx = useContext(GameMetaContext);

  if (ctx == null) {
    throw new Error("Unable to find game meta context in hierachy");
  }

  return ctx;
};
