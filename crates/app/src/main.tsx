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
  GameStatusProvider,
} from "./providers";

import "./styles.css";
import { ViewProvider } from "./providers/View/View";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <ApplicationProvider>
      <GeneralProvider>
        <GameStatusProvider>
          <UiProvider>
            <ViewProvider>
              <PopupProvider>
                <AssetProvider>
                  <NavigationProvider>
                    <App />
                  </NavigationProvider>
                </AssetProvider>
              </PopupProvider>
            </ViewProvider>
          </UiProvider>
        </GameStatusProvider>
      </GeneralProvider>
    </ApplicationProvider>
  </BrowserRouter>
);
