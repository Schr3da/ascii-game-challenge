import { Container, Sprite } from "@pixi/react";
import { GameGridProps } from "./GameGrid.types";
import { toAbsolutePosition } from "../../../../../utils";
import { Fragment, useCallback, useMemo } from "react";
import { GameCellData } from "../GameView.types";
import { useGameTextureContext } from "../../../../../providers";

export const GameGrid = ({ data }: GameGridProps) => {
  const { textures, assetWidth, assetHeight } = useGameTextureContext();

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
    (id: string, data: GameCellData | null) => {
      if (data == null) {
        return null;
      }

      const cell = data[0];
      const position = toAbsolutePosition(data[1]);

      return (
        <Fragment>
          <Sprite
            key={`${id}-bg`}
            width={assetWidth}
            height={assetHeight}
            position={position}
            texture={textures.getBackgroundTexture(cell.id)}
          />
          <Sprite
            key={`${id}-symbol`}
            width={assetWidth}
            height={assetHeight}
            position={position}
            texture={textures.getSymbolTexture(cell.id)}
          />
        </Fragment>
      );
    },
    [textures, assetWidth, assetHeight]
  );

  const renderSelectedCell = useCallback(() => {
    if (selectedCell == null) {
      return null;
    }

    const cell = selectedCell[0];
    const position = toAbsolutePosition(selectedCell[1]);
    const [background, symbol] = textures.createSelectedTextures(cell);

    return (
      <Fragment>
        <Sprite
          key={`${cell.id}-selected-bg`}
          width={assetWidth}
          height={assetHeight}
          position={position}
          texture={background}
        />
        <Sprite
          key={`${cell.id}-selected-symbol`}
          width={assetWidth}
          height={assetHeight}
          position={position}
          texture={symbol}
        />
      </Fragment>
    );
  }, [selectedCell, assetWidth, assetHeight]);

  return (
    <Container>
      {data[0].map((d, i) => renderCell(`grid-cell-${i}`, d))}
      {renderSelectedCell()}
    </Container>
  );
};
