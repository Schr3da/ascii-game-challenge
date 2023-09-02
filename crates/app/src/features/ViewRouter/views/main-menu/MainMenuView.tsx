import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared.d";
import { useViewContext } from "../../../../providers/View/useViewContext";

const NewGameId: ViewComponentIds = { Main: "NewGame" };

const OptionId: ViewComponentIds = { Main: "Options" };

const QuitId: ViewComponentIds = { Main: "Quit" };

export const MainMenuView = () => {
  const { isViewSelected} = useViewContext();

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-4">
      <Button id={NewGameId} isSelected={isViewSelected(NewGameId)}>
        New Game
      </Button>
      <Button id={OptionId} isSelected={isViewSelected(OptionId)}>
        Options
      </Button>
      <Button id={QuitId} isSelected={isViewSelected(QuitId)}>
        Quit
      </Button>
    </div>
  );
};
