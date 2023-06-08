import { Route, Routes } from "react-router-dom";
import { MainMenu, Options, PixiCanvas } from "./views";
import { NavigationRoutes } from "../../providers/Navigation/Navigation.types";
import { useKeyboardControls } from "../../hooks";
import { ViewRouterProps } from "./ViewRouter.types";

export const ViewRouter = ({ className }: ViewRouterProps) => {
  useKeyboardControls();

  return (
    <div className={className}>
      <Routes>
        <Route path={NavigationRoutes.MainMenu} element={<MainMenu />} index />
        <Route path={NavigationRoutes.Options} element={<Options />} />
        <Route path={NavigationRoutes.Canvas} element={<PixiCanvas />} />
      </Routes>
    </div>
  );
};
