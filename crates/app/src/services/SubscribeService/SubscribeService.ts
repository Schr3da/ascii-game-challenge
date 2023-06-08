import {
  Subscriptions,
  SubscriptionCallback,
  SubscriptionEventTypes,
} from "./SubscribeService.types";
import { ApiService } from "../ApiService";

class SubscribeService {
  public isInitialised = false;

  private subscriptions: Subscriptions = {};

  public init = async () => {
    await ApiService.ecsSubscriptionListener(this.handleEvent);
    await ApiService.webviewDidSubscribe();
    this.isInitialised = true;
  };

  private handleEvent = (event: SubscriptionEventTypes) => {
    Object.values(this.subscriptions).forEach((cb) => {
      cb && cb(event);
    });
  };

  public subscribe = (cb: SubscriptionCallback): string => {
    const nextId = `subscriber-id-${Object.keys(this.subscriptions).length}`;
    this.subscriptions[nextId] = cb;
    return nextId;
  };

  public unsubscribe = (id: string) => {
    this.subscriptions[id] = null;
  };

  public dispose() {
    ApiService.disposeEcsSubscriptionListener();

    for (let id in this.subscriptions) {
      this.unsubscribe(id);
    }

    this.subscriptions = {};
  }
}

const instance = new SubscribeService();

export const sharedInstance = async (): Promise<SubscribeService> => {
  if (!instance.isInitialised) {
    await instance.init();
  }
  return instance;
};
