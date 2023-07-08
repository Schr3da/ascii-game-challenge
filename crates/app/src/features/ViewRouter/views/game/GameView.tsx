import { TextStyle } from "pixi.js";
import { Stage, Text } from "@pixi/react";
import { useMemo, useRef } from "react";
import { useWrapperSize } from "../../../../hooks";
import { useEcsContext } from "../../../../providers";
import { GameHeader } from "./GameHeader";
import { GameFooter } from "./GameFooter";

export const GameView = () => {
  const ref = useRef<HTMLDivElement | null>(null);

  const { width, height } = useWrapperSize(ref);

  const { nextView } = useEcsContext();

  const isGameView = useMemo(() => {
    return nextView?.id === "Game";
  }, [nextView]);

  if (!isGameView) {
    return null;
  }

    return (
    <div className="relative w-full h-full flex flex-col">
      <GameHeader className="shrink-0" state={null} />
      <div ref={ref} className="relative flex-1 overflow-hidden">
        <Stage className="pixi-canvas" width={width} height={height}>
          <Text
            text="C"
            x={width * 0.5}
            y={height * 0.5}
            anchor={0.5}
            style={
              new TextStyle({
                align: "center",
                fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
                fontSize: 14,
                fontWeight: "400",
                fill: ["#01d27e", "#01d27e"], // gradient
                stroke: "#01d27e",
                strokeThickness: 1,
                letterSpacing: 2,
                dropShadowColor: "#ccced2",
                dropShadowBlur: 0,
                dropShadowAngle: 0,
                dropShadowDistance: 0,
                wordWrap: true,
                wordWrapWidth: 440,
              })
            }
          />
        </Stage>
      </div>
      <GameFooter className="shrink-0" state={null} />
    </div>
  );
};
