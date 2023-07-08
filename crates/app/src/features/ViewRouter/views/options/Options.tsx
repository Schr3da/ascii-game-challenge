import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared";
import { useEcsContext } from "../../../../providers";

const BackId: ViewComponentIds = { Options: "Back" };

export const OptionsView = () => {
  const { isViewComponentSelected } = useEcsContext();

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-4">
      <Button id={BackId} isSelected={isViewComponentSelected(BackId)}>
        Back to menu
      </Button>
    </div>
  );
};
