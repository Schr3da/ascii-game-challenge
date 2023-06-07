import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared";
import { useEcsContext } from "../../../../providers";
import { useKeyboardControls } from "../../../../hooks";

const NewGameId: ViewComponentIds = { Main: "NewGame" };

const OptionId: ViewComponentIds = { Main: "Options" };

const QuitId: ViewComponentIds = { Main: "Quit" };

export const MainMenu = () => {
  const { nextView } = useEcsContext();

  const { handleKeyUp } = useKeyboardControls(nextView);

  return (
    <div
      className="w-full h-full flex flex-col justify-center items-center space-y-4"
      tabIndex={1}
      onKeyUp={handleKeyUp}
    >
      <Button id={NewGameId}>New Game</Button>
      <Button id={OptionId}>Options</Button>
      <Button id={QuitId}>Quit</Button>
    </div>
  );
};
