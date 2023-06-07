import { Button } from "../../components";
import { ViewComponentIds } from "../../../../shared";
import { useEcsContext } from "../../../../providers";
import { useKeyboardControls } from "../../../../hooks";

const BackId: ViewComponentIds = { Options: "Back" };

export const Options = () => {
  const { nextView } = useEcsContext();
  const { handleKeyUp } = useKeyboardControls(nextView);

  return (
    <div
      className="w-full h-full flex flex-col justify-center items-center space-y-4"
      tabIndex={1}
      onKeyUp={handleKeyUp}
    >
      <Button id={BackId}>Back to menu</Button>
    </div>
  );
};
