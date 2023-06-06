import { Route, Routes } from "react-router-dom";
import { PixiCanvas } from "../canvas";
import { useEffect } from "react";
import { calculateGridSize } from "../../utils";
import { ApiService } from "../../services";

export const ViewRouter = () => {
  useEffect(() => {
    const { columns, rows } = calculateGridSize();
    ApiService.sendEcsEvent({
      General: { OnApplicationWillInitialise: [columns, rows] },
    });
  }, []);

  return (
    <Routes>
      <Route path="/" element={<div>Menu</div>} index/>
      <Route path="/canvas" element={<PixiCanvas />}/>
      <Route path="/options" element={<div>Options</div>} />
    </Routes>
  );
};
