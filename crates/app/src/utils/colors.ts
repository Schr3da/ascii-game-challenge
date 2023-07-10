import { CellColors } from "../shared";

export const numberToHex = (value: number) => {
  return value.toString(16);
}

export const rgbToHex = (color: CellColors): string => {
  if (color == null || color.rgb == null) {
    return "#000000";
  }

  const next = (color.rgb || []).reduce((result, v) => {
    return result + numberToHex(v);
  }, "#");

  return next;
}