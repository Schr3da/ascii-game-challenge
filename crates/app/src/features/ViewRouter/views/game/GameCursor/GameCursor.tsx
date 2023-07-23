import { useMemo } from "react";
import { useGameMetaContext } from "../../../../../providers";
import {
  defaultPosition,
  defaultTexture,
  toAbsolutePosition,
} from "../../../../../utils";
import { GameCell } from "../GameCell";
import { GameCursorProps } from "./GameCursor.types";

export const GameCursor = ({ textures, width, height }: GameCursorProps) => {
  const { cursor } = useGameMetaContext();

  let position = useMemo(() => {
    if (cursor == null) {
      return defaultPosition;
    }

    return toAbsolutePosition({
      ...cursor.frame,
      y: cursor.top + cursor.frame.y,
    });
  }, [cursor]);

  let [background, symbol] = useMemo(() => {
    if (cursor == null) {
      return [defaultTexture, defaultTexture];
    }

    return textures.createSelectedTextures(cursor.cell);
  }, [cursor]);

  return (
    <GameCell
      width={width}
      height={height}
      position={position}
      symbol={symbol}
      background={background}
    />
  );
};
