import { Sprite, Stage } from "@pixi/react";
import { MouseEvent, useCallback, KeyboardEvent } from "react";

import { useEcsContext } from "../../providers";
import { ApiService } from "../../services";

export const PixiCanvas = () => {
  const { nextEvent } = useEcsContext();

  const handleClick = useCallback((event: MouseEvent<HTMLCanvasElement>) => {
    const target = event.target as HTMLCanvasElement;
    if (target.className.indexOf("pixi-canvas") === -1) {
      return;
    }

    // handle mouse click event;
  }, []);

  const handleKeyUp = useCallback((event: KeyboardEvent<HTMLCanvasElement>) => {
    switch (event.key) {
      case "ArrowUp":
        ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
      case "ArrowDown":
        ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
      default:
        return;
    }
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
      <Sprite
        image="https://pixijs.io/pixi-react/img/bunny.png"
        x={window.innerWidth * 0.5}
        y={window.innerHeight * 0.5}
        anchor={{ x: 0.5, y: 0.5 }}
      />
    </Stage>
  );
};
