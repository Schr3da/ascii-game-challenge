import { Stage, Text } from "@pixi/react";
import { MouseEvent, useCallback, KeyboardEvent, useEffect } from "react";

import { useEcsContext } from "../../providers";
import { ApiService } from "../../services";
import { TextStyle } from "pixi.js";
import { calculateGridSize } from "../../utils";

export const PixiCanvas = () => {
  const { view } = useEcsContext();

  const handleClick = useCallback((event: MouseEvent<HTMLCanvasElement>) => {
    const target = event.target as HTMLCanvasElement;
    if (target.className.indexOf("pixi-canvas") === -1) {
      return;
    }
    // handle mouse click event;
  }, []);

  const handleKeyUp = useCallback(
    (event: KeyboardEvent<HTMLCanvasElement>) => {
      if (view == null) {
        return;
      }

      switch (event.key) {
        case "q":
          return ApiService.sendEcsEvent({ General: "OnApplicationWillClose" });
        case "ArrowUp":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
        case "ArrowDown":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
        case "Enter":
          return ApiService.sendEcsEvent({
            Ui: { OnClick: view.state.selected_id },
          });
        default:
          return;
      }
    },
    [view]
  );

  useEffect(() => {
    const { columns, rows } = calculateGridSize();
    ApiService.sendEcsEvent({
      General: { OnApplicationWillInitialise: [columns, rows] },
    });
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
        text="asds"
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
