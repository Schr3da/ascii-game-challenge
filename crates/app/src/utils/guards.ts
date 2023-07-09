import { GameViewFooterState, GameViewHeaderState, Cell, Position, SelectedCell } from "../shared";

export const isGameViewHeaderState = (data: any): data is GameViewHeaderState => {
  return data != null && data.GameHeader != null;
};

export const isGameViewFooterState = (data: any): data is GameViewFooterState => {
  return data != null;
};

export const isGameCanvas = (data: any): data is { GameCanvas: [[Cell, Position][], SelectedCell | null] } => {
  return data != null && Array.isArray(data.GameCanvas)
}