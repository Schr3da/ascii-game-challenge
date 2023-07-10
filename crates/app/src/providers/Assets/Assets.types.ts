import { AsciiIds, Cell } from "../../shared";

export type AssetProviderContextValue = {
  assets: Record<AsciiIds, Cell>;
}