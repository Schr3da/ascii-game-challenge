import { RenderTexture, Texture } from "pixi.js";

export type AssetsContextValue = {
  assetWidth: number;
  assetHeight: number;
  textures: Map<string, RenderTexture>;
};
