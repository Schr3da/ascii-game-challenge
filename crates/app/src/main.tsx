import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import App from "./App";
import {
  ApplicationProvider,
  EcsProvider,
  NavigationProvider,
  AssetProvider,
} from "./providers";

import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <ApplicationProvider>
      <EcsProvider>
        <AssetProvider>
          <NavigationProvider>
            <App />
          </NavigationProvider>
        </AssetProvider>
      </EcsProvider>
    </ApplicationProvider>
  </BrowserRouter>
);
