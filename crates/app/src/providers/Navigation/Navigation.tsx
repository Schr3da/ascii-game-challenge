import {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
} from "react";
import { useNavigate } from "react-router-dom";
import { NavigationRoutes } from "./Navigation.types";
import { useEcsContext } from "../Ecs";
import { UiView } from "../../shared";

export const NavigationContext = createContext<null>(null);

export const NavigationProvider = ({ children }: PropsWithChildren) => {
  const navigate = useNavigate();

  const { previousView, nextView } = useEcsContext();

  const handleNavigation = useCallback(
    (view: UiView | null) => {
      if (view == null) {
        return navigate(NavigationRoutes.MainMenu);
      }

      switch (view.id) {
        case "Main":
          return navigate(NavigationRoutes.MainMenu);
        case "Options":
          return navigate(NavigationRoutes.Options);
        case "Game":
          return navigate(NavigationRoutes.Game);
      }
    },
    [navigate]
  );

  useEffect(() => {
    if (nextView == null) {
      return handleNavigation(nextView);
    }

    if (previousView == null) {
      return handleNavigation(nextView);
    }

    if (nextView.id === previousView.id) {
      return;
    }

    handleNavigation(nextView);
  }, [previousView, nextView, handleNavigation]);

  return (
    <NavigationContext.Provider value={null}>
      {children}
    </NavigationContext.Provider>
  );
};
