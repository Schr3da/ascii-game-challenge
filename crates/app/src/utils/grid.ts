import { config } from "./config";

export const calculateGridSize = () => {
  const size = config.tileSize;
  const columns = Math.floor(window.innerWidth / size);
  const rows = Math.floor(window.innerHeight / size);

  return { columns, rows };
};
