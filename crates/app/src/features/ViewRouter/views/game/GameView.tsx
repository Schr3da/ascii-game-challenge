import { Stage } from "@pixi/react";
import { useMemo, useRef } from "react";
import { useMouseControls, useWrapperSize } from "../../../../hooks";
import { GameTexturesProvider, useEcsContext } from "../../../../providers";
import { GameHeader } from "./GameHeader";
import { isGameCanvas } from "../../../../utils";
import { GameCellData, GameViewGridData } from "./GameView.types";
import { GameGrid } from "./GameGrid";

const defaultCanvasData: GameViewGridData = [Array<GameCellData>(), null];

export const GameView = () => {
  const ref = useRef<HTMLDivElement | null>(null);

  const { handleMouseDown, handleMouseMove } = useMouseControls();

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
        <Stage
          className="pixi-canvas"
          width={width}
          height={height}
          onMouseMove={handleMouseMove}
          onMouseDown={handleMouseDown}
        >
          <GameTexturesProvider>
            <GameGrid data={data} />
          </GameTexturesProvider>
        </Stage>
      </div>
    </div>
  );
};
