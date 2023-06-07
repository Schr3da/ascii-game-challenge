import { Stage, Text } from "@pixi/react";
import { MouseEvent, useCallback } from "react";

import { useEcsContext } from "../../../../providers";
import { TextStyle } from "pixi.js";
import { useKeyboardControls } from "../../../../hooks/useKeyboardControls/useKeyboardControls";

export const PixiCanvas = () => {
  const { nextView: view } = useEcsContext();

  const { handleKeyUp } = useKeyboardControls(view);

  const handleClick = useCallback((event: MouseEvent<HTMLCanvasElement>) => {
    const target = event.target as HTMLCanvasElement;
    if (target.className.indexOf("pixi-canvas") === -1) {
      return;
    }
    // handle mouse click event;
  }, []);

  return (
    <Stage
      tabIndex={1}
      className="pixi-canvas"
      width={window.innerWidth}
      height={window.innerHeight}
      onClick={handleClick}
      onKeyUp={handleKeyUp}
    >
      <Text
        text="Canvas"
        x={window.innerWidth * 0.5}
        y={window.innerHeight * 0.5}
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
  );
};
