import * as PIXI from "pixi.js";

import { symbolToString, createBackgroundTexture, createSymbolTexture, rgbToHex } from "../../utils";
import { AsciiIds, Cell } from "../../shared";

export class TextureService {

  public static rawAssets: { [key in AsciiIds]?: Cell } = {};

  public renderer: PIXI.IRenderer<PIXI.ICanvas> | null = null;

  private textures = new Map<string, PIXI.RenderTexture>();

  public createTextures = () => {
    this.textures.clear();

    (Object.keys(TextureService.rawAssets) as AsciiIds[]).forEach((key) => {
      if (key == null) {
        return;
      }

      const next = TextureService.rawAssets[key];
      if (next == null) {
        return;
      }

      this.textures.set(
        `${key}-background`,
        createBackgroundTexture(rgbToHex(next.background), this.renderer)
      );

      this.textures.set(
        `${key}-symbol`,
        createSymbolTexture(
          symbolToString(next.symbol),
          rgbToHex(next.foreground),
          this.renderer
        )
      );
    });
  }

  public getBackgroundTexture = (id: AsciiIds): PIXI.RenderTexture => {
    const next = this.textures.get(`${id}-background`);

    if (next == null) {
      throw new Error(`Background texture not found ${id}`);
    }

    return next;
  }

  public getSymbolTexture = (id: AsciiIds): PIXI.RenderTexture => {
    const next = this.textures.get(`${id}-symbol`);

    if (next == null) {
      throw new Error(`Symbol texture not found ${id}`);
    }

    return next;
  }

  public createSelectedTextures = (
    next: Cell,
  ): [PIXI.RenderTexture, PIXI.RenderTexture] => {
    const backgroundTexture = createBackgroundTexture(rgbToHex(next.background), this.renderer);

    const symbolTexture = createSymbolTexture(
      symbolToString(next.symbol),
      rgbToHex(next.foreground),
      this.renderer
    );

    return [backgroundTexture, symbolTexture];
  }
}