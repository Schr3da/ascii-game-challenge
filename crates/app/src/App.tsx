import { useCallback, useState } from "react";

import { useSubscribe } from "./hooks";
import { SubscriptionEvents } from "./shared";

const App = () => {
  let [data, setData] = useState<any[]>([]);

  const processEvent = useCallback(
    (event: SubscriptionEvents) => {
      setData([...data, event]);
    },
    [data]
  );

  useSubscribe(processEvent);

  return <div className="container">{JSON.stringify(data)}</div>;
};

export default App;
