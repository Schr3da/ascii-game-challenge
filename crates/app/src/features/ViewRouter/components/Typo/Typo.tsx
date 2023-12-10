import clsx from "clsx";
import { TypoProps } from "./Typo.types";
import { useMemo } from "react";

export const Typo = ({ children, className, size, weight }: TypoProps) => {
  const fontSize = useMemo(() => {
    if (size === "lg") {
      return "font-lg";
    }

    if (size === "md") {
      return "font-md";
    }

    return "font-sm";
  }, [size]);

  const fontWeight = useMemo(() => {
    if (weight === 600) {
      return "font-bold";
    }

    if (weight === 500) {
      return "font-semibold";
    }

    return "font-normal";
  }, [weight]);

  return (
    <div className={clsx(fontSize, fontWeight, className)}>{children}</div>
  );
};
