import { useCallback, useMemo } from "react"
import { Graphics, TextStyle } from "pixi.js";
import { Container, Graphics as GraphicsComponent, Text } from "@pixi/react";

import { GameTileProps } from "./GameTile.types"

export const GameTile = ({ width, height, position, cell }: GameTileProps) => {

  const draw = useCallback((ctx: Graphics) => {
    ctx.clear();
    ctx.beginFill("#ffffff");
    ctx.drawRect(0, 0, width, height);
    ctx.endFill();
  }, [width, height, cell, position]);

  const text = useMemo(() => <Text
    text="C"
    x={width * 0.5}
    y={height * 0.5}
    anchor={0.5}
    style={
      new TextStyle({
        align: "center",
        fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
        fontSize: Math.ceil(width * 0.8),
        fontWeight: "400",
        fill: ["#01d27e", "#01d27e"], // gradient
        stroke: "#01d27e",
        strokeThickness: 1,
        letterSpacing: 1,
        dropShadowColor: "#ccced2",
        dropShadowBlur: 0,
        dropShadowAngle: 0,
        dropShadowDistance: 0,
        wordWrap: false,
        wordWrapWidth: 1,
      })
    }
  />, [width, height, cell, position]);

  return <Container width={width} height={height} position={position}>
    <GraphicsComponent draw={draw} />
    {text}
  </Container>
}
