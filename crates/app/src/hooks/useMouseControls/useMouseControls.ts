import { useCallback } from "react";
import { MouseEvent } from "react";
import { ApiService } from "../../services";
import { toGridCoordinate } from "../../utils";
import { useViewContext } from "../../providers/View/useViewContext";

export const useMouseControls = () => {
  const { nextView } = useViewContext();

  const handleMouseMove = useCallback(
    (event: MouseEvent) => {
      if (nextView == null) {
        return;
      }

      if (nextView.id !== "Game") {
        return;
      }

    },
    [nextView]
  );

  const handleMouseDown = useCallback(async (event: MouseEvent) => {
    if (nextView == null) {
      return;
    }

    if (nextView.id !== "Game") {
      return;
    }

    const { clientX, clientY } = event;
    const next = toGridCoordinate(clientX, clientY);

    await ApiService.sendEcsEvent({
      Renderer: {
        OnUpdateSelectedCell: {
          Custom: [next.columns, next.rows]
        }
      }
    });

  }, [nextView]);

  return {
    handleMouseDown,
    handleMouseMove,
  }
};
