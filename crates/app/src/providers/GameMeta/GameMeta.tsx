import { createContext, PropsWithChildren, useCallback, useState } from "react";
import { useSubscribe } from "../../hooks";
import { GameMetaContextValue} from "./GameMeta.types";
import { GameMeta, GameStatus, SelectedCell } from "../../shared";

export const GameMetaContext = createContext<GameMetaContextValue| null>(
  null
);

export const GameMetaProvider = ({ children }: PropsWithChildren) => {
  const [status, setStatus] = useState<GameStatus>("GameDidNotStart");
  const [cursor, setCursor] = useState<SelectedCell | null>(null);

  const processEvent = useCallback((next: GameMeta) => {
    setStatus(next.status);
    setCursor(next.cursor);
  }, []);

  useSubscribe("GameMetaSubscription", processEvent);

  return (
    <GameMetaContext.Provider
      value={{
        status,
        cursor,
      }}
    >
      {children}
    </GameMetaContext.Provider>
  );
};
