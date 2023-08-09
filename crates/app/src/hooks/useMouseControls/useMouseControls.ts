import { useCallback } from "react";
import { MouseEvent } from "react";
import { ApiService } from "../../services";
import { toGridCoordinate } from "../../utils";
import { useViewContext } from "../../providers/View/useViewContext";
import { useDebounce } from "../useDebounce";

export const useMouseControls = () => {
  const { nextView } = useViewContext();

  const { debounce } = useDebounce();

  const handleMouseMove = useCallback(
    ({ clientX, clientY }: MouseEvent) => {
      if (nextView == null) {
        return;
      }

      if (nextView.id !== "Game") {
        return;
      }

      let { columns, rows } = toGridCoordinate(clientX, clientY);

      debounce(
        () =>
          ApiService.sendEcsEvent({
            Renderer: {
              OnUpdateSelectedCell: {
                Custom: [columns, rows],
              },
            },
          }),
        8
      );
    },
    [nextView, debounce]
  );

  const handleMouseDown = useCallback(
    async (event: MouseEvent) => {
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
            Custom: [next.columns, next.rows],
          },
        },
      });
    },
    [nextView]
  );

  return {
    handleMouseDown,
    handleMouseMove,
  };
};
