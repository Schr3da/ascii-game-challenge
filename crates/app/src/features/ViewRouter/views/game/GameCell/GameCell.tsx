import { Sprite } from "@pixi/react";
import { Fragment } from "react";
import { GameCellProps } from "./GameCell.types";

export const GameCell = ({
  width,
  height,
  position,
  background,
  symbol,
}: GameCellProps) => {
  return (
    <Fragment>
      <Sprite
        width={width}
        height={height}
        position={position}
        texture={background}
      />
      <Sprite
        width={width}
        height={height}
        position={position}
        texture={symbol}
      />
    </Fragment>
  );
};
