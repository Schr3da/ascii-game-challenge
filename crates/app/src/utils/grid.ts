import { Position } from "../shared";
import { config } from "./config";

export const calculateGridSize = () => {
  const size = config.tileSize;
  const columns = Math.ceil(window.innerWidth / size) ;
  const rows = Math.ceil(window.innerHeight / size) - 1;

  return { columns, rows };
};

export const toAbsolutePosition = (next: Position): Position => ({
  x: next.x * config.tileSize,
  y: next.y * config.tileSize,
});

export const toAbsoluteSize = () => ({
  width: 1 * config.tileSize,
  height: 1 * config.tileSize,
});
