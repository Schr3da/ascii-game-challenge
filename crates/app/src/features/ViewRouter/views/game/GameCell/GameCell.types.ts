import { RenderTexture } from "pixi.js";
import { Position } from "../../../../../shared.d";

export type GameCellProps = {
  width: number;
  height: number;
  position: Position;
  symbol: RenderTexture;
  background: RenderTexture;
};
