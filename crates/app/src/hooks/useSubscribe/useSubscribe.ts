import { useEffect, useRef } from "react";
import { SubscriptionCallback, sharedInstance } from "../../services";

export const useSubscribe = (cb: SubscriptionCallback) => {

  const id = useRef("");
  
  useEffect(() => {
    sharedInstance().then((instance) => {
      id.current = instance.subscribe(cb);
    });

    return () => {
      sharedInstance().then((instance) => {
        instance.unsubscribe(id.current);
      });
    };
  }, [cb]);
};
