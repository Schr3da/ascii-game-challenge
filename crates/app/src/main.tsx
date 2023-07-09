import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import App from "./App";
import {
  ApplicationProvider,
  AssetsContext,
  EcsProvider,
  NavigationProvider,
  AssetsProvider,
} from "./providers";

import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <ApplicationProvider>
      <AssetsProvider>
        <EcsProvider>
          <NavigationProvider>
            <App />
          </NavigationProvider>
        </EcsProvider>
      </AssetsProvider>
    </ApplicationProvider>
  </BrowserRouter>
);
