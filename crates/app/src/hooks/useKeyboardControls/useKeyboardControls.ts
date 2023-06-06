import { useCallback, KeyboardEvent } from "react";
import { ApiService } from "../../services";
import { UiView } from "../../shared";

export const useKeyboardControls = (view: UiView | null) => {
  const handleKeyUp = useCallback(
    (event: KeyboardEvent<HTMLCanvasElement>) => {
      if (view == null) {
        return;
      }

      switch (event.key) {
        case "q":
          return ApiService.sendEcsEvent({ General: "OnApplicationWillClose" });
        case "ArrowUp":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
        case "ArrowDown":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
        case "Enter":
          return ApiService.sendEcsEvent({
            Ui: { OnClick: view.state.selected_id },
          });
        default:
          return;
      }
    },
    [view]
  );

  return {
    handleKeyUp
  }
}