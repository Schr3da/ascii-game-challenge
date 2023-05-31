import { useEffect } from "react";
import { SubscriptionCallback, sharedInstance } from "../../services";

export const useSubscribe = (cb: SubscriptionCallback) => {
  useEffect(() => {
    let id = "";
    sharedInstance().then((instance) => {
      id = instance.subscribe(cb);
    });

    return () => {
      sharedInstance().then((instance) => {
        instance.unsubscribe(id);
      });
    }
  }, [cb]);
}