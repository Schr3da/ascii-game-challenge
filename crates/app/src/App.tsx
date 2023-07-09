import clsx from "clsx";

import { ViewRouter } from "./features";
import { useApplicationContext } from "./providers";

import { WindowHeader } from "./features";
import { useMemo } from "react";
import { Platforms } from "./services";

const App = () => {
  const { platform } = useApplicationContext();

  const wrapperClassName = useMemo(() => {
    const next =
      "h-screen w-screen overflow-hidden bg-black text-white pb-0 flex flex-col";

    switch (platform) {
      case Platforms.Macos:
        return clsx(next, "rounded-xl");
      default:
        return clsx(next, "rounded-xl");
    }
  }, [platform]);

  return (
    <div className={wrapperClassName}>
      <WindowHeader className="basis-7 shrink-0 grow-0 px-2" platform={platform} />
      <ViewRouter className="flex-1 overflow-hidden" />
    </div>
  );
};

export default App;
