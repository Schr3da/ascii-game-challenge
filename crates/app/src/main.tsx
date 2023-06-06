import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { ApplicationProvider } from "./providers";
import { BrowserRouter } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <ApplicationProvider>
      <App />
    </ApplicationProvider>
  </BrowserRouter>
);
