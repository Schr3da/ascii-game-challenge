import { Container, Sprite } from "@pixi/react";
import { useAssetsContext } from "../../../../../providers";
import { GameGridProps } from "./GameGrid.types";
import { toAbsolutePosition } from "../../../../../utils";
import { Fragment, useCallback, useMemo } from "react";
import { GameCellData } from "../GameView.types";

export const GameGrid = ({ data }: GameGridProps) => {
  const { textures, assetWidth, assetHeight } = useAssetsContext();

  const selectedCell = useMemo((): GameCellData | null => {
    if (data == null) {
      return null;
    }

    const next = data[1];
    if (next == null) {
      return null;
    }

    const position = {
        ...next.frame,
        y: next.top + next.frame.y,
      };

    return [next.cell, position];
  }, [data]);

  const renderCell = useCallback(
    (
      id: string,
      data: GameCellData | null,
      background: string,
      symbol: string
    ) => {
      if (data == null) {
        return null;
      }

      const _cell = data[0];
      const position = toAbsolutePosition(data[1]);

      return (
        <Fragment>
          <Sprite
            key={`${id}-bg`}
            width={assetWidth}
            height={assetHeight}
            position={position}
            texture={textures.get(background)}
          />
          <Sprite
            key={`${id}-symbol`}
            width={assetWidth}
            height={assetHeight}
            position={position}
            texture={textures.get(symbol)}
          />
        </Fragment>
      );
    },
    [textures, assetWidth, assetHeight]
  );

  return (
    <Container>
      {data[0].map((d, i) =>
        renderCell(`grid-cell-${i}`, d, "default-background", "default-symbol")
      )}
      {selectedCell &&
        renderCell(
          `selected-grid-cell`,
          selectedCell,
          "default-background-selected",
          "default-symbol-selected"
        )}
    </Container>
  );
};
