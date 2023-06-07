import clsx from "clsx";

import { ViewRouter } from "./features";
import { useApplicationContext } from "./providers";

import { WindowHeader } from "./features";
import { useMemo } from "react";
import { Platforms } from "./services";

const App = () => {
  const { platform } = useApplicationContext();

  const wrapperClassName = useMemo(() => {
    const next = "h-screen w-screen overflow-hidden bg-black text-white px-2 py-2";

    switch (platform) {
      case Platforms.Macos:
        return clsx(next, "rounded-xl");
      default:
        return clsx(next, "rounded-md");
    }
  }, [platform]);

  return (
    <div className={wrapperClassName}>
      <WindowHeader platform={platform} />
      <ViewRouter />
    </div>
  );
};

export default App;
