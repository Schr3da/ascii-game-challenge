import { useCallback, useEffect } from "react";
import { ApiService } from "../../services";
import { useEcsContext } from "../../providers";
import { UiView } from "../../shared";

export const useKeyboardControls = () => {
  const { nextView, } = useEcsContext();

  const handleGameViewControls = useCallback((event: KeyboardEvent) => {
    switch (event.key) {
      case "Escape":
        return ApiService.sendEcsEvent({ Ui: "OnCloseView" });
      case "ArrowUp":
        return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Up" } });
      case "ArrowLeft":
        return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Left" } });
      case "ArrowRight":
        return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Right" } });
      case "ArrowDown":
        return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Down" } });
      default:
        return;
    }
  }, []);

  const handleBasicViewControls = useCallback((event: KeyboardEvent, view: UiView) => {
      switch (event.key) {
        case "q":
          return ApiService.sendEcsEvent({ General: "OnApplicationWillClose" });
        case "Escape":
          return ApiService.sendEcsEvent({ Ui: "OnCloseView" });
        case "ArrowUp":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
        case "ArrowLeft":
          return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Left" } });
        case "ArrowRight":
          return ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Right" } });
        case "Tab":
        case "ArrowDown":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
        case "Enter":
          return ApiService.sendEcsEvent({
            Ui: { OnClick: view.state.selected_id },
          });
        default:
          return;
      }
  }, []);

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      event.preventDefault();

      if (nextView == null) {
        return;
      }

      if (nextView.id === "Game"){
        return handleGameViewControls(event);
      }

      handleBasicViewControls(event, nextView);
    },
    [nextView, handleGameViewControls, handleBasicViewControls]
  );

  useEffect(() => {
    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, [handleKeyUp]);
  nextView;
};
