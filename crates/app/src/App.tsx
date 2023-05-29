import { useCallback } from "react";

import { useDidMount, useSubscribe } from "./hooks";

import "./App.css";

const App = () => {

  const processEvent = useCallback((event: {}) => {
    console.log("received event", event);
  }, []);

  useSubscribe(processEvent);
  useDidMount();

  return <div className="container"></div>;
};

export default App;
