import { Route, Routes } from "react-router-dom";
import { MainMenu, Options, PixiCanvas } from "./views";
import { useEffect } from "react";
import { calculateGridSize } from "../../utils";
import { ApiService } from "../../services";
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
