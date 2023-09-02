import * as React from "react";
import clsx from "clsx";

import { useMemo } from "react";
import { GameHeaderProps } from "./GameHeader.types";
import { isGameViewHeaderState } from "../../../../../utils";

export const GameHeader = ({ className, state }: GameHeaderProps) => {
  const data = useMemo(() => {
    if (state == null) {
      return;
    }

    const { view_data } = state;

    if (!isGameViewHeaderState(view_data)) {
      return null;
    }

    return view_data;
  }, [state]);

  return (
    <div className={clsx("flex flex-row", className)}>
      {data && (
        <React.Fragment>
          <div>{data.currentDays}</div>
          <div>{data.currentHours}</div>
          <div>{data.currentMinutes}</div>
          <div>{data.tickCount}</div>
        </React.Fragment>
      )}
    </div>
  );
};
