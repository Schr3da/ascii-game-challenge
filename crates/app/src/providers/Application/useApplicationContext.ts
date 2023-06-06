import { useContext } from "react"
import { ApplicationContext } from "./Application";
import { ApplicationContextValue } from "./Application.types";

export const useApplicationContext = (): ApplicationContextValue => {
  const ctx = useContext(ApplicationContext);

  if (ctx == null) {
    throw new Error("Unable to find application context in hierachy");
  }

  return ctx;
}