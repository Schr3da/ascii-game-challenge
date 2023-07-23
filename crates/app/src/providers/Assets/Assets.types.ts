import { AsciiIds, Cell } from "../../shared.d";

export type AssetProviderContextValue = {
  assets: Record<AsciiIds, Cell>;
}
