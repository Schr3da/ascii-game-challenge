import * as PIXI from "pixi.js";
import {
  PropsWithChildren,
  createContext,
  useEffect,
  useRef,
  useState,
} from "react";

import {
  createBackgroundTexture,
  createSymbolTexture,
  toAbsoluteSize,
} from "../../utils";
import { useApp } from "@pixi/react";
import { GameTexturesContextValue } from "./GameTextures.types";
import { TextureService } from "../../services";

export const GameTexturesContext =
  createContext<GameTexturesContextValue | null>(null);

export const GameTexturesProvider = ({ children }: PropsWithChildren) => {
  const { renderer } = useApp();

  const textures = useRef(new TextureService());

  const [size] = useState(toAbsoluteSize());

  const [isInitialised, setIsInitialised] = useState(false);

  useEffect(() => {
    textures.current.renderer = renderer;
    textures.current.createTextures();
    setIsInitialised(true);
  }, []);

  return (
    <GameTexturesContext.Provider
      value={{
        assetWidth: size.width,
        assetHeight: size.height,
        textures: textures.current,
      }}
    >
      {isInitialised && children}
    </GameTexturesContext.Provider>
  );
};
