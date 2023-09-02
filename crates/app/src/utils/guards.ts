import { GameViewGridData } from "../features/ViewRouter/views/game/GameView.types";
import {
  GameViewFooterState,
  GameViewHeaderState,
  Cell,
  AsciiIds,
} from "../shared.d";

export const isGameViewHeaderState = (
  data: any
): data is GameViewHeaderState => {
  return data != null && data.GameHeader != null;
};

export const isGameViewFooterState = (
  data: any
): data is GameViewFooterState => {
  return data != null;
};

export const isGameCanvas = (
  data: any
): data is { GameCanvas: GameViewGridData } => {
  return data != null && Array.isArray(data.GameCanvas);
};

export const isApplicationDidLoadAssetsEvent = (
  data: any | null
): data is { OnApplicationDidLoadAssets: Record<AsciiIds, Cell> } => {
  return data != null && data.OnApplicationDidLoadAssets != null;
};
