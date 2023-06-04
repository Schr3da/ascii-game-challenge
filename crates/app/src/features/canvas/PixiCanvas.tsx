import { Sprite, Stage } from "@pixi/react";

import { useEcsContext } from "../../providers";

export const PixiCanvas = () => {
  
  const { nextEvent } = useEcsContext();

  return (
    <Stage width={window.innerWidth} height={window.innerHeight}>
      <Sprite
        image="https://pixijs.io/pixi-react/img/bunny.png"
        x={window.innerWidth * 0.5}
        y={window.innerHeight * 0.5}
        anchor={{ x: 0.5, y: 0.5 }}
      />
    </Stage>
  );
};
