import { Cell, Position, SelectedCell } from "../../../../shared";

export type GameTileData = [Cell, Position];

export type GameViewData = [[Cell, Position][], SelectedCell | null]
