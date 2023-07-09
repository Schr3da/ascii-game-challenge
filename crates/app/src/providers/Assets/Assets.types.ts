import { ReactNode } from "react";

export type AssetsContextValue = {
  assetWidth: number;
  assetHeight: number;
  backgrounds: Map<string, ReactNode>;
  asciis: Map<string, ReactNode>;
};
