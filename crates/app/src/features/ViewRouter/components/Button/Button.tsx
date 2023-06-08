import clsx from "clsx";

import { ButtonProps } from "./Button.types";
import { useCallback } from "react";
import { ApiService } from "../../../../services";

export const Button = ({ id, children, isSelected }: ButtonProps) => {
  const handleClick = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: id } });
  }, [id]);

  return (
    <div
      className={clsx(  
        "outline outline-offset-2 outline-2 rounded-lg px-2 py-1 cursor-pointer",
        "transition-color duration-300",
        "hover:bg-gray-800 hover:outline-gray-800",
        isSelected ? "outline-blue-500 font-bold" : "outline-transparent"
      )}
      onClick={handleClick}
    >
      {children}
    </div>
  );
};
