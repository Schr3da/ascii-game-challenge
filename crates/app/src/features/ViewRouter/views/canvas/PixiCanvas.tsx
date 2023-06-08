import { TextStyle } from "pixi.js";
import { Stage, Text } from "@pixi/react";
import { useRef } from "react";
import { useWrapperSize } from "../../../../hooks";

export const PixiCanvas = () => {
  const ref = useRef<HTMLDivElement | null>(null);

  const { width, height } = useWrapperSize(ref);

  return (
    <div ref={ref} className="relative w-full h-full">
      <Stage className="pixi-canvas" width={width} height={height}>
        <Text
          text="Canvas"
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
  );
};
