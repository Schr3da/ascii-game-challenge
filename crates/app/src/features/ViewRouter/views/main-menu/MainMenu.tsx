import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared";
import { useEcsContext } from "../../../../providers";

const NewGameId: ViewComponentIds = { Main: "NewGame" };

const OptionId: ViewComponentIds = { Main: "Options" };

const QuitId: ViewComponentIds = { Main: "Quit" };

export const MainMenu = () => {
  const { isViewComponentSelected } = useEcsContext();

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-4">
      <Button id={NewGameId} isSelected={isViewComponentSelected(NewGameId)}>
        New Game
      </Button>
      <Button id={OptionId} isSelected={isViewComponentSelected(OptionId)}>
        Options
      </Button>
      <Button id={QuitId} isSelected={isViewComponentSelected(QuitId)}>
        Quit
      </Button>
    </div>
  );
};
