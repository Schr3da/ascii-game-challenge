import { ReactNode } from "react";
import { ViewComponentIds } from "../../../../shared";

export type ButtonProps = {
  id: ViewComponentIds;
  isSelected: boolean;
  children: ReactNode;
};
