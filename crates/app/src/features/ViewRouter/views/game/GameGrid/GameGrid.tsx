import { Container } from "@pixi/react";
import { GameGridProps } from "./GameGrid.types";
import { toAbsolutePosition } from "../../../../../utils";
import { useMemo } from "react";
import { useGameTextureContext } from "../../../../../providers";
import { GameCell } from "../GameCell";
import { GameCursor } from "../GameCursor";

export const GameGrid = ({ data }: GameGridProps) => {
  const { textures, assetWidth, assetHeight } = useGameTextureContext();

  const cells = useMemo(() => {
    if (data == null) {
      return null;
    }

    return data.map((d, i) => {
      const cell = d[0];
      const position = toAbsolutePosition(d[1]);

      return (
        <GameCell
          key={`grid-game-cell-${i}`}
          width={assetWidth}
          height={assetHeight}
          position={position}
          symbol={textures.getSymbolTexture(cell.id)}
          background={textures.getBackgroundTexture(cell.id)}
        />
      );
    });
  }, [data, textures, assetWidth, assetHeight]);

  return (
    <Container>
      {cells}
      <GameCursor width={assetWidth} height={assetHeight} textures={textures} />
    </Container>
  );
};
