import {ReactNode} from "react";

export type TypoProps = {
  className?: string;
  children: ReactNode;
  size?: "sm" | "md" | "lg";
  weight?: 400 | 500 | 600;
}
