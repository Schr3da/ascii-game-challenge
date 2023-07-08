import { GameViewFooterState, GameViewHeaderState } from "../shared";

export const isGameViewHeaderState = (data: any): data is GameViewHeaderState => {
  return data != null && data.GameHeader != null;
};

export const isGameViewFooterState = (data: any): data is GameViewFooterState => {
  return data != null;
};