import clsx from "clsx";
import { usePopupContext } from "../../../../providers";

import { PopupProps } from "./Popup.types";
import { useMemo } from "react";
import { Typo } from "../Typo";

export const Popup = ({ position }: PopupProps) => {
  const { isPopupVisible, nextPopup } = usePopupContext();

  const popupPosition = useMemo(() => {
    if (position === "left") {
      return "w-72 left-6 bottom-6 top-0";
    }

    return "w-72 right-6 bottom-6 top-0";
  }, [position]);

  const popupTranslation = useMemo(() => {
    if (isPopupVisible) {
      return "translate-x-0";
    }

    if (position === "left") {
      return "-translate-x-80";
    }

    return "translate-x-80";
  }, [isPopupVisible, position]);

  return (
    <div
      className={clsx(
        "absolute bg-gray-900 transition-transform duration-300 rounded-2xl p-6",
        "flex flex-col overflow-hidden",
        popupPosition,
        popupTranslation
      )}
    >
      <div className="relative flex-1 overflow-hidden space-y-2">
        <Typo className="text-center" size="lg" weight={500}>
          Popup Data
        </Typo>
        {nextPopup && (
          <div className="overflow-scroll break-all">{JSON.stringify(nextPopup)}</div>
        )}
      </div>
      <div className="text-center font-bold">
        <Typo size="lg" weight={500}>
          Press ESC to close
        </Typo>
      </div>
    </div>
  );
};
