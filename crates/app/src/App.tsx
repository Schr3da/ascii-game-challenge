import { useCallback } from "react";

import "./App.css";
import { useDidMount, useSubscribe } from "./hooks";

const App = () => {

  const processEvent = useCallback((event: {}) => {
    console.log("received event", event);
  }, []);

  useSubscribe(processEvent);
  useDidMount();

  return <div className="container"></div>;
};

export default App;
