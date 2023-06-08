import { TextStyle } from "pixi.js";
import { Stage, Text } from "@pixi/react";

export const PixiCanvas = () => {
  return (
    <Stage
      className="pixi-canvas"
      width={window.innerWidth}
      height={window.innerHeight}
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
