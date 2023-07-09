import { Graphics, TextStyle } from "pixi.js";
import {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";

import { Graphics as GraphicsComponent, Text } from "@pixi/react";
import { toAbsoluteSize } from "../../utils";
import { AssetsContextValue } from "./Assets.types";

export const AssetsContext = createContext<AssetsContextValue | null>(null);

export const AssetsProvider = ({ children }: PropsWithChildren) => {
  const backgrounds = useRef(new Map());

  const asciis = useRef(new Map());

  const [size] = useState(toAbsoluteSize());

  const [halfWidth] = useState(size.width * 0.5);

  const [halfHeight] = useState(size.height * 0.5);

  const textStyle = useMemo(() => {
    return new TextStyle({
      align: "center",
      fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
      fontSize: Math.ceil(size.width * 0.8),
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
    });
  }, []);

  const addBackground = useCallback((key: string, color: string) => {
    const draw = (ctx: Graphics) => {
      ctx.clear();
      ctx.beginFill(color);
      ctx.drawRect(0, 0, size.width, size.height);
      ctx.endFill();
    };

    const next = <GraphicsComponent draw={draw} />;
    backgrounds.current.set(key, next);
  }, []);

  const addAscii = useCallback((key: string, symbol: string) => {
    const next = (
      <Text
        text={symbol}
        x={halfWidth}
        y={halfHeight}
        anchor={0.5}
        style={textStyle}
      />
    );

    asciis.current.set(key, next);
  }, []);

  useEffect(() => {
    addBackground("default", "#ffffff");
    addAscii("default", "C");
  }, []);

  return (
    <AssetsContext.Provider
      value={{
        assetWidth: size.width,
        assetHeight: size.height,
        backgrounds: backgrounds.current,
        asciis: asciis.current,
      }}
    >
      {children}
    </AssetsContext.Provider>
  );
};
