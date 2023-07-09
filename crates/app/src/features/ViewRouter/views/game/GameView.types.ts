import { Cell, Position, SelectedCell } from "../../../../shared";

export type GameCellData = [Cell, Position];

export type GameViewGridData = [[Cell, Position][], SelectedCell | null]
