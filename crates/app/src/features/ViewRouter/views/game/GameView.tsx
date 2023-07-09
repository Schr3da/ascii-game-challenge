import { Stage } from "@pixi/react";
import { useMemo, useRef } from "react";
import { useWrapperSize } from "../../../../hooks";
import { useEcsContext } from "../../../../providers";
import { GameHeader } from "./GameHeader";
import { GameFooter } from "./GameFooter";
import { GameTile } from "./GameTiles";
import { isGameCanvas, toAbsolutePosition, toAbsoluteSize } from "../../../../utils";
import { GameTileData, GameViewData } from "./GameView.types";

const defaultCanvasData: GameViewData = [Array<GameTileData>(), null];

export const GameView = () => {
  const ref = useRef<HTMLDivElement | null>(null);

  const { width, height } = useWrapperSize(ref);

  const { nextView } = useEcsContext();

  const isGameView = useMemo(() => {
    if (nextView == null) {
      return false;
    }

    return nextView.id === "Game";
  }, [nextView]);

  const data = useMemo(() => {
    if (nextView == null) {
      return defaultCanvasData;
    }

    if (!isGameView) {
      return defaultCanvasData;
    }

    let child = nextView.children[1];
    if (!isGameCanvas(child)) {
      return defaultCanvasData;
    }

    return child.GameCanvas;
  }, [nextView, isGameView]);

  return (
    <div className="relative w-full h-full flex flex-col">
      <GameHeader className="px-2 shrink-0" state={null} />
      <div ref={ref} className="relative flex-1 overflow-hidden">
        <Stage className="pixi-canvas" width={width} height={height}>
          {data[0].map((d, i) => {
            let cell = d[0];
            let position = toAbsolutePosition(d[1]);
            let size = toAbsoluteSize();

            console.log(cell, position, size);

            return <GameTile
              key={`game-tile-${i}`}
              cell={cell}
              width={size.width}
              height={size.height}
              position={position}
            />
          })}
        </Stage>
      </div>
      <GameFooter className="px-2 shrink-0" state={null} />
    </div>
  );
};
