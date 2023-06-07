import { useContext } from "react"
import { NavigationContext } from "./Navigation"

export const useNavigationContext = () => {
  const ctx = useContext(NavigationContext);

  if (ctx == null) {
    throw new Error("Unabel to find navigation context in hierarchy");
  }

  return ctx;
}