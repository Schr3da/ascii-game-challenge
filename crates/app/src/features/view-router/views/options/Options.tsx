import { useCallback } from "react";
import { ApiService } from "../../../../services";

export const Options = () => {
  const handleBack = useCallback(() => {
    ApiService.sendEcsEvent({ Ui: { OnClick: { Options: "Back" } } });
  }, []);

  return (
    <div className="w-full h-full flex flex-col justify-center items-center space-y-2">
      <div onClick={handleBack}>Back to menu</div>
    </div>
  );
};
