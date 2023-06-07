import { useCallback } from "react";
import { ApiService } from "../../../../services";

export const MainMenu = () => {

  const handleNewGame = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: { Main: "NewGame" } } });
  }, []);

  const handleOptions= useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: { Main: "Options" } } });
  }, []);

  const handleQuit = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: { Main: "Quit" } } });
  }, []);

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-2">
      <div onClick={handleNewGame}>New Game</div>
      <div onClick={handleOptions}>Options</div>
      <div onClick={handleQuit}>Quit</div>
    </div>
  );
};
