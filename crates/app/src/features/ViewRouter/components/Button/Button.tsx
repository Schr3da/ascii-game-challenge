import clsx from "clsx";

import { ButtonProps } from "./Button.types";
import { useCallback, useEffect, useRef } from "react";
import { ApiService } from "../../../../services";

export const Button = ({
  id,
  children,
}: ButtonProps) => {
  const handleClick = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: id } });
  }, [id]);

  const handleFocus = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnSelectById: id } });
  }, [id]);

  return (
    <div>
      <button
        className={clsx(
          "outline-2 outline-blue-500 rounded-lg px-2 py-1 transition-color duration-300",
          "hover:bg-gray-800 hover:font-semibold",
          "focus:bg-gray-800 focus:font-semibold"
        )}
        onClick={handleClick}
        onFocus={handleFocus}
      >
        {children}
      </button>
    </div>
  );
};
