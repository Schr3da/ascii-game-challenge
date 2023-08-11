import { useCallback, useEffect } from "react";
import { ApiService } from "../../services";
import { useViewContext } from "../../providers/View/useViewContext";

export const useKeyboardControls = () => {
  const handleInput = useCallback(async ({ key }: KeyboardEvent) => {
    console.log(key);
    switch (key) {
      case "Escape":
        return await ApiService.sendKeyboardPressEvent("Esc");
      case "ArrowUp":
        return await ApiService.sendKeyboardPressEvent("UpArrow");
      case "ArrowLeft":
        return await ApiService.sendKeyboardPressEvent("LeftArrow");
      case "ArrowRight":
        return await ApiService.sendKeyboardPressEvent("RightArrow");
      case "ArrowDown":
        return await ApiService.sendKeyboardPressEvent("DownArrow");
      case "Enter": 
        return await ApiService.sendKeyboardPressEvent("Enter");
      default:
        return await ApiService.sendKeyboardPressEvent({ Char: key });
    }
  }, []);

  const handleKeyUp = useCallback(
    async (event: KeyboardEvent) => {
      event.preventDefault();
      await handleInput(event);
    },
    [handleInput]
  );

  useEffect(() => {
    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, [handleKeyUp]);
};
