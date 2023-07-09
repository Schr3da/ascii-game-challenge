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
import { AssetsContextValue } from "./Assets.types";
import { useApp } from "@pixi/react";

export const AssetsContext = createContext<AssetsContextValue | null>(null);

export const AssetsProvider = ({ children }: PropsWithChildren) => {
  const { renderer } = useApp();

  const textures = useRef(new Map<string, PIXI.RenderTexture>());

  const [size] = useState(toAbsoluteSize());

  const [isInitialised, setIsInitialised] = useState(false);

  useEffect(() => {
    textures.current.set(
      "default-background",
      createBackgroundTexture("#000000", renderer)
    );

    textures.current.set(
      "default-symbol",
      createSymbolTexture("#ff0000", renderer)
    );

    textures.current.set(
      "default-background-selected",
      createBackgroundTexture("#ffffff", renderer)
    );

    textures.current.set(
      "default-symbol-selected",
      createSymbolTexture("#00ff00", renderer)
    );

    setIsInitialised(true);
  }, []);

  return (
    <AssetsContext.Provider
      value={{
        assetWidth: size.width,
        assetHeight: size.height,
        textures: textures.current,
      }}
    >
      {isInitialised && children}
    </AssetsContext.Provider>
  );
};
