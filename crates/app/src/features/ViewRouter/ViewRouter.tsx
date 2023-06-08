import { Route, Routes } from "react-router-dom";
import { MainMenu, Options, PixiCanvas } from "./views";
import { NavigationRoutes } from "../../providers/Navigation/Navigation.types";
import { useKeyboardControls } from "../../hooks";

export const ViewRouter = () => {

  useKeyboardControls();

  return (
    <Routes>
      <Route path={NavigationRoutes.MainMenu} element={<MainMenu/>} index/>
      <Route path={NavigationRoutes.Options} element={<Options />} />
      <Route path={NavigationRoutes.Canvas} element={<PixiCanvas />} />
    </Routes>
  );
};
