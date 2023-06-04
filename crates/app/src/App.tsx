import { PixiCanvas } from "./features";
import { EcsProvider } from "./providers";

const App = () => {
  return (
    <div className="screen-w screen-h overflow-hidden">
      <EcsProvider>
        <PixiCanvas/>
      </EcsProvider>
    </div>
  );
};

export default App;
