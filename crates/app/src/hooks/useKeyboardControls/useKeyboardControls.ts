import { useCallback, useEffect } from "react";
import { ApiService } from "../../services";
import { UiView } from "../../shared.d";
import { useViewContext } from "../../providers/View/useViewContext";

export const useKeyboardControls = () => {
  const { nextView, } = useViewContext();

  const handleGameViewControls = useCallback(async (event: KeyboardEvent) => {
    switch (event.key) {
      case "Escape":
        return await ApiService.sendEcsEvent({ Ui: "OnCloseView" });
      case "ArrowUp":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Up" } });
      case "ArrowLeft":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Left" } });
      case "ArrowRight":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Right" } });
      case "ArrowDown":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Down" } });
      default:
        return;
    }
  }, []);

  const handleBasicViewControls = useCallback(async (event: KeyboardEvent, view: UiView) => {
    switch (event.key) {
      case "q":
        return await ApiService.sendEcsEvent({ General: "OnApplicationWillClose" });
      case "Escape":
        return await ApiService.sendEcsEvent({ Ui: "OnCloseView" });
      case "ArrowUp":
        return await ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
      case "ArrowLeft":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Left" } });
      case "ArrowRight":
        return await ApiService.sendEcsEvent({ Renderer: { OnUpdateSelectedCell: "Right" } });
      case "Tab":
      case "ArrowDown":
        return await ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
      case "Enter":
        return await ApiService.sendEcsEvent({
          Ui: { OnClick: view.state.selected_id },
        });
      default:
        return;
    }
  }, []);

  const handleKeyUp = useCallback(
    async (event: KeyboardEvent) => {
      event.preventDefault();

      if (nextView == null) {
        return;
      }

      if (nextView.id === "Game") {
        return handleGameViewControls(event);
      }

      await handleBasicViewControls(event, nextView);
    },
    [nextView, handleGameViewControls, handleBasicViewControls]
  );

  useEffect(() => {
    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, [handleKeyUp]);
  nextView;
};
