import { useContext } from "react";
import { GameStatusContextValue } from "./GameStatus.types";
import { GameStatusContext } from "./GameStatus";

export const useGameStatus = (): GameStatusContextValue => {
  const ctx = useContext(GameStatusContext);

  if (ctx == null) {
    throw new Error("Unable to find gamestatus context in hierachy");
  }

  return ctx;
};
