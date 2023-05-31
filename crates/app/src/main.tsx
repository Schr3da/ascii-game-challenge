import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { ApplicationProvider } from "./providers";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ApplicationProvider>
    <App />
  </ApplicationProvider>
);
