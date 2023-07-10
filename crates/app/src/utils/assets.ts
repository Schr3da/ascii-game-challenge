import * as PIXI from "pixi.js";

import { toAbsoluteSize } from "./grid";
import { Ascii } from "../shared";

const createRenderTexture = () => {
  const size = toAbsoluteSize();

  const baseRenderTexture = new PIXI.BaseRenderTexture({
    width: size.width,
    height: size.height,
    scaleMode: PIXI.SCALE_MODES.LINEAR,
    resolution: 2,
  });

  return new PIXI.RenderTexture(baseRenderTexture);
}

const createTextStyle = (color: string, fontSize: number) => {
  return new PIXI.TextStyle({
    align: "center",
    fontSize,
    fontWeight: "300",
    fill: [color, color], 
    stroke: color,
    strokeThickness: 1,
    letterSpacing: 1,
    dropShadowBlur: 0,
    dropShadowAngle: 0,
    dropShadowDistance: 0,
    wordWrap: false,
    wordWrapWidth: 1,
  });
}

export const createBackgroundTexture = (color: string, renderer: PIXI.IRenderer<PIXI.ICanvas> | null): PIXI.RenderTexture => {
  const renderTexture = createRenderTexture();

  let graphic = new PIXI.Graphics();
  graphic.clear();
  graphic.beginFill(color);
  graphic.drawRect(0, 0, renderTexture.width, renderTexture.height);
  graphic.endFill();

  if (renderer == null) {
    return renderTexture;
  }

  renderer.render(graphic, { renderTexture });
  return renderTexture;
}

export const createSymbolTexture = (
  symbol: string,
  color: string,
  renderer: PIXI.IRenderer<PIXI.ICanvas> | null
): PIXI.RenderTexture => {
  const renderTexture = createRenderTexture();
  const width = renderTexture.width * 0.8;
  const fontSize = Math.floor(width);
  const style = createTextStyle(color, fontSize);
  const text = new PIXI.Text(symbol, style);
  text.anchor.set(0.5);
  text.position = new PIXI.Point(renderTexture.width * 0.5, text.height* 0.5);

  if (renderer == null) {
    return renderTexture;
  }

  renderer.render(text, { renderTexture });
  return renderTexture;
}

export const symbolToString = (next: Ascii): string => {
  switch (next) {
    case "space":
      return " ";
    case "plus":
      return "+";
    case "tilde":
      return "~";
    case "doubleTilde":
      return "≈";
  }
}