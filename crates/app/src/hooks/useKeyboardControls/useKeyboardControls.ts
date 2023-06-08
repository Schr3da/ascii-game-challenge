import { useCallback, useEffect } from "react";
import { ApiService } from "../../services";
import { useEcsContext } from "../../providers";

export const useKeyboardControls = () => {
  const { nextView } = useEcsContext();

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      event.preventDefault();

      if (nextView == null) {
        return;
      }

      switch (event.key) {
        case "q":
          return ApiService.sendEcsEvent({ General: "OnApplicationWillClose" });
        case "Escape":
          return ApiService.sendEcsEvent({ Ui: "OnCloseView" });
        case "ArrowUp":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Previous" } });
        case "Tab":
        case "ArrowDown":
          return ApiService.sendEcsEvent({ Ui: { OnSelect: "Next" } });
        case "Enter":
          return ApiService.sendEcsEvent({
            Ui: { OnClick: nextView.state.selected_id },
          });
        default:
          return;
      }
    },
    [nextView]
  );

  useEffect(() => {
    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, [handleKeyUp]);
  nextView;
};
