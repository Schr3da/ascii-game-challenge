import { Route, Routes } from "react-router-dom";
import { MainMenu, Options, PixiCanvas } from "./views";
import { NavigationRoutes } from "../../providers/Navigation/Navigation.types";

export const ViewRouter = () => {
  return (
    <Routes>
      <Route path={NavigationRoutes.MainMenu} element={<MainMenu />}/>
      <Route path={NavigationRoutes.Options} element={<Options />} />
      <Route path={NavigationRoutes.Canvas} element={<PixiCanvas />} />
    </Routes>
  );
};
