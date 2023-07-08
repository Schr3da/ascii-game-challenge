import * as React from "react";
import clsx from "clsx";

import { useMemo } from "react";
import { GameFooterProps } from "./GameFooter.types";
import { isGameViewFooterState } from "../../../../../utils";

export const GameFooter = ({ className, state }: GameFooterProps) => {
  const data = useMemo(() => {
    if (state == null) {
      return null;
    }

    const { view_data } = state;

    if (!isGameViewFooterState(view_data)) {
      return null;
    }

    return view_data;
  }, [state]);

  return (
    <div className={clsx("flex flex-column overflow-hidden", className)}>
      <div className="flex-1">[ESC] Menu</div>
      <div className="flex-1">Gold</div>
      <div className="flex-1">Wood</div>
      <div className="flex-1">Stone</div>
    </div>
  );
};
