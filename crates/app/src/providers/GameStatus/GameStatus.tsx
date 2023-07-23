import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { GameStatusContextValue } from "./GameStatus.types";
import { GameStatus } from "../../shared";

export const GameStatusContext = createContext<GameStatusContextValue | null>(
  null
);

export const GameStatusProvider = ({ children }: PropsWithChildren) => {
  const [status, setStatus] = useState<GameStatus>("GameDidNotStart");

  const processEvent = useCallback((next: GameStatus) => {
    setStatus(next);
  }, []);

  useSubscribe("GameStatusSubscription", processEvent);

  return (
    <GameStatusContext.Provider
      value={{
        status,
      }}
    >
      {children}
    </GameStatusContext.Provider>
  );
};
