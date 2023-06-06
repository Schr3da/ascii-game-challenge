import { PixiCanvas } from "./features";
import { EcsProvider, useApplicationContext } from "./providers";

import { WindowHeader } from "./features";
import { useMemo } from "react";
import { Platforms } from "./services";

const App = () => {
  const { platform } = useApplicationContext();

  const wrapperClassName = useMemo(() => {
    const next = "h-screen w-screen overflow-hidden bg-black";

    switch (platform) {
      case Platforms.Macos:
        return next + " rounded-xl";
      default:
        return next + " rounded-md";
    }
  }, [platform]);

  return (
    <div className={wrapperClassName}>
      <EcsProvider>
        <WindowHeader platform={platform} />
        <PixiCanvas />
      </EcsProvider>
    </div>
  );
};

export default App;
