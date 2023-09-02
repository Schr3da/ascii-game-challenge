import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared.d";
import { useViewContext } from "../../../../providers/View/useViewContext";

const BackId: ViewComponentIds = { Options: "Back" };

export const OptionsView = () => {
  const { isViewSelected } = useViewContext();

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-4">
      <Button id={BackId} isSelected={isViewSelected(BackId)}>
        Back to menu
      </Button>
    </div>
  );
};
