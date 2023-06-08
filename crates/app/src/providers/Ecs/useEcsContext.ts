import { useContext } from "react";
import { EcsContext } from "./Ecs";
import { EcsContextValue } from "./Ecs.types";

export const useEcsContext = (): EcsContextValue => {
  const ctx = useContext(EcsContext);

  if (ctx == null) {
    throw new Error("Unable to find ecs context in hierachy");
  }

  return ctx;
};
