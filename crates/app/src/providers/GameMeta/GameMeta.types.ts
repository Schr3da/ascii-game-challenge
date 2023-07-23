import { GameStatus, SelectedCell } from "../../shared"

export type GameMetaContextValue = {
  cursor: SelectedCell | null;
  status: GameStatus;
}
