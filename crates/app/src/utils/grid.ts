import { Position } from "../shared";
import { config } from "./config";

export const calculateGridSize = () => {
  const size = config.tileSize;
  const columns = Math.floor(window.innerWidth / size);
  const rows = Math.floor(window.innerHeight / size) - 1;

  return { columns, rows };
};

export const toGridCoordinate = (xAbsolute: number, yAbsolute: number) => {
  const size = config.tileSize;
  const columns = Math.floor(xAbsolute / size);
  const rows = Math.floor(yAbsolute / size) - 1;

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
