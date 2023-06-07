import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import App from "./App";
import {
  ApplicationProvider,
  EcsProvider,
  NavigationProvider,
} from "./providers";

import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <ApplicationProvider>
      <EcsProvider>
        <NavigationProvider>
          <App />
        </NavigationProvider>
      </EcsProvider>
    </ApplicationProvider>
  </BrowserRouter>
);
