import { useEffect, useRef } from "react";
import { SubscriptionCallback, sharedInstance } from "../../services";
import {EcsSubscriptionIds} from "../../shared.d";

export const useSubscribe = (topic: EcsSubscriptionIds, cb: SubscriptionCallback) => {

  const id = useRef("");
  
  useEffect(() => {
    sharedInstance().then((instance) => {
      id.current = instance.subscribe(topic, cb);
    });

    return () => {
      sharedInstance().then((instance) => {
        instance.unsubscribe(topic, id.current);
      });
    };
  }, [cb]);
};
