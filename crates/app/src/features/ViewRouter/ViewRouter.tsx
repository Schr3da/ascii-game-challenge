import { Route, Routes } from "react-router-dom";
import { MainMenuView, OptionsView, GameView } from "./views";
import { NavigationRoutes } from "../../providers/Navigation/Navigation.types";
import { useKeyboardControls} from "../../hooks";
import { ViewRouterProps } from "./ViewRouter.types";

export const ViewRouter = ({ className }: ViewRouterProps) => {
  useKeyboardControls();

  return (
    <div className={className}>
      <Routes>
        <Route
          path={NavigationRoutes.MainMenu}
          element={<MainMenuView />}
          index
        />
        <Route path={NavigationRoutes.Options} element={<OptionsView />} />
        <Route path={NavigationRoutes.Game} element={<GameView />} />
      </Routes>
    </div>
  );
};
