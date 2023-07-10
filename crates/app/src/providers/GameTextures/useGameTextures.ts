import { useContext } from "react";
import { GameTexturesContext } from "./GameTextures";
import { GameTexturesContextValue } from "./GameTextures.types";

export const useGameTextureContext = (): GameTexturesContextValue => {
  const ctx = useContext(GameTexturesContext);

  if (ctx == null) {
    throw new Error("Unable to find GameTextures context in hierachy");
  }

  return ctx;
};
