import { ReactNode } from "react";
import { ViewComponentIds } from "../../../../shared.d";

export type ButtonProps = {
  id: ViewComponentIds;
  isSelected: boolean;
  children: ReactNode;
};
