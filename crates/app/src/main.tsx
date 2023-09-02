import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import App from "./App";
import {
  ApplicationProvider,
  NavigationProvider,
  AssetProvider,
  PopupProvider,
  GeneralProvider,
  UiProvider,
} from "./providers";

import "./styles.css";
import { ViewProvider } from "./providers/View/View";
import { Composer } from "./features";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <Composer
      components={[
        ApplicationProvider,
        GeneralProvider,
        UiProvider,
        ViewProvider,
        PopupProvider,
        AssetProvider,
        NavigationProvider,
      ]}
    >
      <App />
    </Composer>
  </BrowserRouter>
);
